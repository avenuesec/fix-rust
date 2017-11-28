//! Handles the main IO event loop

use std::usize;
use std::io;
use std::time::Duration;

use mio::{Poll, Token, PollOpt, Events, Ready};
use mio::tcp::{TcpStream};
use mio_more::{channel,timer};
use slab::Slab;
use std::net::SocketAddr;

use super::{FixHandlerFactory, FixHandler, Sender, AdvSender};
use super::conn;
use fix::frame;
use fix::fixmessagegen;


const TOKEN_QUEUE   : Token = Token(usize::MAX - 1);
const TOKEN_TIMER   : Token = Token(usize::MAX - 2);
const TOKEN_RESOLVE : Token = Token(usize::MAX - 3); // hack for now
const EVKIND_RECONN : Token = Token(usize::MAX - 4);


pub struct IoHandler <F : FixHandlerFactory> {
    factory: F,
    queue_tx: channel::SyncSender<Command>,
    queue_rx: channel::Receiver<Command>,
    timer: timer::Timer<ConnetionTimeoutSettings>,
    is_running : bool,
    token_2conn: Slab<conn::Conn<F::Handler>, Token>,
    poll: Poll,
    reconnect_queue : Vec<SocketAddr>,
}


#[derive(Debug, Clone, Copy)]
struct ConnetionTimeoutSettings {
    connection_token: Option<Token>,
    pub event_kind: Token,
    // interval: Duration,
}

const TIMER_TICK_MILLIS: u64 = 100;
const TIMER_WHEEL_SIZE: usize = 1024;
const TIMER_CAPACITY: usize = 65_536;


impl<F> IoHandler <F> 
    where F : FixHandlerFactory {
        
    pub fn new(factory: F, poll: Poll) -> IoHandler<F> {
        let (tx, rx) = channel::sync_channel(100); // # max items before send blocks

        let timer = timer::Builder::default()
            .tick_duration(Duration::from_millis(TIMER_TICK_MILLIS))
            .num_slots(TIMER_WHEEL_SIZE)
            .capacity(TIMER_CAPACITY)
            .build();

        IoHandler {
            poll,
            factory,
            queue_rx: rx,
            queue_tx: tx,
            is_running: true,
            timer,
            token_2conn : Slab::with_capacity(10), // todo: param
            reconnect_queue : Vec::new(),
        }
    }

    pub fn connect(&mut self, addr: &SocketAddr) -> io::Result<()> {

        self.do_connect( addr.clone() )
    }

    fn reconnect(&mut self, token: Token) {

        let addr = {
            let conn = self.token_2conn.get(token).unwrap();
            conn.addr.clone()
        };

        // Signal `on_network_error` to handler
        {
            self.token_2conn.get_mut(token).as_mut().unwrap().handler.on_network_error();
            // remove token/conn from token_2conn
            self.token_2conn.remove(token);
        }

        // save for later
        self.reconnect_queue.push( addr );

        // TODO: requests to push data to this recently removed conn will silently fail
        // TODO: .. ideally we'd need to save/enqueue them or outright reject them until the session has been re-established

        let duration = Duration::from_millis(3000); // <-- NOTE: consider exponential backoff algo here
        let timeout_info = ConnetionTimeoutSettings {
            connection_token: None,
            event_kind: EVKIND_RECONN
        };
        self.timer.set_timeout( duration, timeout_info );
    }

    fn do_connect(&mut self, addr: SocketAddr) -> io::Result<()> {
        // TODO when is reconect, when to stop trying to reconnect?

        let conn_res = TcpStream::connect( &addr );
        if conn_res.is_err() {
            error!("Could not create socket to {:?}", addr);
            return Err(conn_res.unwrap_err());
        }
        let socket = conn_res.unwrap();

        let token = {
            let entry = self.token_2conn.vacant_entry().unwrap(); // <- needs to be more robust
            let token = entry.index();

            // debug!("token created: {:?}", token);

            // on_connected - creates handler with a sender
            let sender = Sender::new(token, self.queue_tx.clone());
            let handler = self.factory.on_connected( &addr, sender );

            let connection = conn::Conn::new(token, socket, handler, addr);

            entry.insert(connection);
            token
        };

        let res = self.poll.register( &self.token_2conn[token].socket,
                                       self.token_2conn[token].token,
                                       self.token_2conn[token].events,
                                      PollOpt::edge() | PollOpt::oneshot() )
            .map_err(io::Error::from)
            .or_else(move |err| {
                self.token_2conn.remove(token).unwrap().consume(); // frees token and Conn
                Err(err) })?;
        Ok(res)
    }

    // Starts
    pub fn run(&mut self) -> io::Result<()> {
        // TODO: ensure this isn't called more than once?
        self.poll.register( &self.queue_rx, TOKEN_QUEUE, Ready::readable(), PollOpt::edge() | PollOpt::oneshot() )?;
        self.poll.register( &self.timer,    TOKEN_TIMER, Ready::readable(), PollOpt::edge() )?;
        
        self.is_running = true;

        let mut events = Events::with_capacity(1024);
        
        while self.is_running {
            // counter = counter + 1;
            let count = match self.poll.poll(&mut events, None) {
                Ok(c) => c,
                Err(err) => {
                    if err.kind() == io::ErrorKind::Interrupted {
                        self.is_running = false; // stop loop
                        0
                    } else {
                        return Err(io::Error::from(err));
                    }
                }
            };
            // debug!("Processing {} events", count);
            
            for i in 0..count {
                let event = events.get(i).unwrap();
                self.handle_event( event.token(), event.readiness() );
            }
        }

        self.poll.deregister( &self.queue_rx )?;
        self.poll.deregister( &self.timer )?;

        Ok( () )
    }

    pub fn create_adv_sender(&self) -> AdvSender {
        // NOTE: Using TOKEN_RESOLVE to indicate that the target connection 
        // needs to be resolved somehow, the sender doesn't have any info regarding the connection.
        // For now, since we have just one connection to an exchange, it's OK. 
        // when we have more exchanges to talk to, this will need to be fixed. 
        AdvSender::new(TOKEN_RESOLVE, self.queue_tx.clone())
    }

    fn handle_event(&mut self, token: Token, events: Ready) {
        debug!("handle event for {:?} events: {:?}", token, events);
        
        match token {

            TOKEN_TIMER => {
                while let Some(timeout) = self.timer.poll() {
                    self.handle_timeout(timeout);
                }
            }

            TOKEN_QUEUE => {
                for _ in 0..100 { // how many messages we want take off the queue per event loop iteration?
                    match self.queue_rx.try_recv() {
                        Ok(cmd) => self.handle_received_command(cmd),
                        _ => break // no more channel messages
                    }
                }
                // re-registers
                let opts = PollOpt::edge() | PollOpt::oneshot();
                let _ = self.poll.reregister(&self.queue_rx, TOKEN_QUEUE, Ready::readable(), opts);
            }

            _ => { 
                // socket events

                let must_reconnect = {

                    if let Some(conn) = self.token_2conn.get_mut(token) {
                        if events.is_readable() {
                            if let Err(err) = conn.read() {
                                error!("error reading: {}", err);
                                conn.error(err);
                            }
                        }
                        if events.is_writable() {
                            if let Err(err) = conn.write() {
                                error!("error writing: {}", err);
                                conn.error(err);
                            }
                        }
                        conn.in_error // if in error, trigger reconnect
                    } else {
                        false
                    }
                };

                if must_reconnect {
                    self.reconnect(token);
                } else {
                    if let Some(_) = self.token_2conn.get(token) {
                        if self.token_2conn[token].in_error {
                            info!("removed connection due to previous error {:?}", token);
                            self.token_2conn.remove(token);
                        } else if let Err(err) = self.schedule( &self.token_2conn[token] ) {
                            self.token_2conn[token].error(err);
                            self.token_2conn.remove(token);
                        }
                    }
                }
            }
        }
    }

    fn handle_received_command(&mut self, cmd: Command) {
        debug!("handle_received_command");
        
        match cmd.action {
            CommandAction::Message(payload) => {
                if let Some(conn) = self.token_2conn.get_mut(cmd.token) {
                    if let Err(err) = conn.send( payload ) {
                        conn.error( err );
                    }
                } else {
                    // connection removed?
                }
            },

            CommandAction::SetTimeout { timeout_in_ms, event_kind } => {
                if let Some(conn) = self.token_2conn.get_mut(cmd.token) {
                    let duration = Duration::from_millis(timeout_in_ms as u64);
                    let timeout_info = ConnetionTimeoutSettings { 
                        connection_token: Some(cmd.token),
                        // interval: duration 
                        event_kind
                    };
                    match self.timer.set_timeout( duration, timeout_info ) {
                        Ok(timeout) => {
                            // confirm with handler?
                            conn.handler.new_timeout( timeout, event_kind );
                        }, 
                        Err(err) => {
                            // then what?
                        }
                    }
                } else {
                    // connection removed?
                }
            },

            CommandAction::CancelTimeout(timeout) => {
                self.timer.cancel_timeout(&timeout);
                return;
            },

            CommandAction::SendBackToHandler(message) => {

                let token_to_use = match cmd.token {
                    TOKEN_RESOLVE => {
                        // the caller doesn't know which connection,
                        // so for now we get the first available.
                        // TODO: caller needs to give a hint on which fix session she wants to use
                        let mut tk = Token(0);
                        for t in 0..100 { // 0 to 99 is totally arbitrary
                            tk = Token(t);
                            if self.token_2conn.contains( tk ) {
                                break;
                            }
                        }
                        tk // assumes it was found - bad!
                    },
                    token => token
                };

                if let Some(conn) = self.token_2conn.get_mut(token_to_use) {
                    // sends back to handler
                    conn.handler.before_send( message );
                } else {
                    warn!("Conn not found to send SendBackToHandler. token {:?}", token_to_use);
                }

                return;
            }

            // _ => unreachable!()
        };

        if let Some(_) = self.token_2conn.get(cmd.token) {
            if let Err(err) = self.schedule( &self.token_2conn[cmd.token] ) {
                self.token_2conn[cmd.token].error(err);
            }
        }
    }

    fn handle_timeout(&mut self, ConnetionTimeoutSettings { connection_token: t, event_kind } : ConnetionTimeoutSettings) {
        if let Some(t) = t {
            if let Some(conn) = self.token_2conn.get_mut(t) {
                if let Err(err) = conn.handler.on_timeout(event_kind) {
                    // TODO: should we close the connection?
                }
            }
        } else {
            // an timeout without a connection is meant to affect all of them
            self.handle_global_timeout(event_kind);
        }
        // TODO: ensure the connection still valid, remove it (and handler) if not
    }

    fn handle_global_timeout(&mut self, event_kind : Token) {
        match event_kind {
            EVKIND_RECONN => {

                // for each addr in reconnect queue, start connection process
                while let Some(addr) = self.reconnect_queue.pop() {
                    self.do_connect( addr );
                }
            },
            _ => unreachable!("no other event supported yet")
        }
    }

    fn schedule(&self, conn: &conn::Conn<F::Handler>) -> io::Result<()> {
        debug!("scheduling {:?} for {:?}", conn.token, conn.events);
        Ok( self.poll.reregister( &conn.socket, conn.token, conn.events, PollOpt::edge() | PollOpt::oneshot() )? )
    }
}

#[derive(Debug)]
pub struct Command {
    token: Token,
    action: CommandAction,
}
impl Command {
    pub fn new( token: Token, cmd: CommandAction ) -> Command {
        Command {
            action: cmd,
            token
        }
    }
}

#[derive(Debug)]
pub enum CommandAction {
    Message(frame::FixFrame),
    SendBackToHandler(fixmessagegen::FixMessage),
    SetTimeout {
        timeout_in_ms: u32,
        event_kind: Token,
    },
    CancelTimeout (timer::Timeout),
}
