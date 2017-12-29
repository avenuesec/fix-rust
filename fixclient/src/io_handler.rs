//! Handles the main IO event loop

use std::usize;
use std::io;
use std::time::Duration;

use mio::{Poll, PollOpt, Events, Ready, Token};
use mio::tcp::{TcpStream};
use mio_more::{channel,timer};
use slab::Slab;
use std::net::SocketAddr;

use super::{FixHandlerFactory, FixHandler, Sender, AdvSender};
use super::conn;
use fix::frame;
use fix::fixmessagegen;


const TOKEN_QUEUE   : Token = Token(0); // (usize::MAX - 1);
const TOKEN_TIMER   : Token = Token(1); // (usize::MAX - 2);
const TOKEN_RESOLVE : Token = Token(2); // (usize::MAX - 3); // hack for now
const EVKIND_RECONN : Token = Token(3); // (usize::MAX - 4);
const RESERVED_TOKENS : usize = 4;

pub struct IoHandler <F : FixHandlerFactory> {
    factory: F,
    queue_tx: channel::SyncSender<Command>,
    queue_rx: channel::Receiver<Command>,
    timer: timer::Timer<ConnetionTimeoutSettings>,
    is_running : bool,
    token_2conn: Slab<Option<conn::Conn<F::Handler>>>,
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

        let mut slab = Slab::with_capacity(20); // todo: param

        // reserves RESERVED_TOKENS to avoid mistepping over special tokens
        for _ in 0..RESERVED_TOKENS {
            slab.vacant_entry().insert( None );
        }

        IoHandler {
            poll,
            factory,
            queue_rx: rx,
            queue_tx: tx,
            is_running: true,
            timer,
            token_2conn : slab,
            reconnect_queue : Vec::new(),
        }
    }

    pub fn connect(&mut self, addr: &SocketAddr) -> io::Result<()> {

        self.do_connect( addr.clone() )
    }

    fn reconnect(&mut self, token: Token) {

        let addr = {
            let conn = self.token_2conn.get(token.0).unwrap().as_ref().unwrap();
            conn.addr.clone()
        };

        // Signal `on_network_error` to handler
        {
            // self.token_2conn.get_mut(token.0).as_mut().unwrap().handler.on_network_error();
            // remove token/conn from token_2conn
            let conn = self.token_2conn.remove(token.0).unwrap(); // conn taken over with remove is called
            conn.consume_with_error(); // consume self, so everything is dropped
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
        let _ = self.timer.set_timeout( duration, timeout_info );
    }

    fn do_connect(&mut self, addr: SocketAddr) -> io::Result<()> {
        // TODO when is reconect, when to stop trying to reconnect?

        let conn_res = TcpStream::connect( &addr );
        if conn_res.is_err() {
            error!("Could not create socket to {:?}", addr);
            return Err(conn_res.unwrap_err());
        }
        let socket = conn_res.unwrap();
        let _ = socket.set_nodelay(true); // TODO: config param

        let token = {
            let entry = self.token_2conn.vacant_entry(); // <- needs to be more robust
            let token = entry.key();

            debug!("new conn token created: {:?}", token);

            // on_connected - creates handler with a sender
            let sender = Sender::new(Token(token), self.queue_tx.clone());
            let handler = self.factory.on_started( &addr, sender );

            let connection = conn::Conn::new(Token(token), socket, handler, addr);

            entry.insert(Some(connection));
            token
        };

        let res = self.poll.register( &self.token_2conn[token].as_ref().unwrap().socket,
                                       self.token_2conn[token].as_ref().unwrap().token,
                                       self.token_2conn[token].as_ref().unwrap().events,
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

                    if let Some(conn) = self.token_2conn.get_mut(token.0).unwrap().as_mut() {
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
                    if let Some(_) = self.token_2conn.get(token.0) {
                        if self.token_2conn[token.0].as_mut().unwrap().in_error {
                            info!("removed connection due to previous error {:?}", token);
                            self.token_2conn.remove(token.0);
                        } else if let Err(err) = self.schedule( self.token_2conn[token.0].as_ref().unwrap() ) {
                            // self.token_2conn[token.0].unwrap().error(err);
                            self.token_2conn.remove(token.0).unwrap().error(err);
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
                if let Some(conn) = self.token_2conn.get_mut(cmd.token.0).unwrap().as_mut() {
                    if let Err(err) = conn.send( payload ) {
                        conn.error( err );
                    }
                } else {
                    // connection removed?
                }
            },

            CommandAction::Disconnect => {

                if self.token_2conn.contains(cmd.token.0) {
                    if let Some(conn) = self.token_2conn.remove(cmd.token.0) {
                        conn.disconnect();
                    }
                }
            },

            CommandAction::SetTimeout { timeout_in_ms, event_kind } => {
                if let Some(conn) = self.token_2conn.get_mut(cmd.token.0).unwrap().as_mut() {
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
                        Err(_err) => {
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

                let token_to_use = self.resolve_connection_token(cmd.token);

                if let Some(opt) = self.token_2conn.get_mut(token_to_use.0) {
                    if let Some(conn) = opt.as_mut() {
                        // sends back to handler
                        conn.handler.before_send( message );
                    } else {
                        warn!("Conn not found to send SendBackToHandler. token {:?}", token_to_use);
                    }
                }

                return;
            },

            CommandAction::SendFrameBackToHandler(frame) => {

                let token_to_use = self.resolve_connection_token(cmd.token);

                if let Some(opt) = self.token_2conn.get_mut(token_to_use.0) {
                    if let Some(conn) = opt.as_mut() {
                        // sends back to handler
                        conn.handler.before_resend( frame );
                    } else {
                        warn!("Conn not found to send SendFrameBackToHandler. token {:?}", token_to_use);
                    }
                }

                return;
            },

            // _ => unreachable!()
        };

        if let Some(_) = self.token_2conn.get(cmd.token.0) {
            if let Err(err) = self.schedule( self.token_2conn[cmd.token.0].as_ref().unwrap() ) {
                self.token_2conn[cmd.token.0].as_mut().unwrap().error(err);
            }
        }
    }

    fn resolve_connection_token(&self, token: Token) -> Token {
        match token {
            TOKEN_RESOLVE => {
                // the caller doesn't know which connection,
                // so for now we get the first available.
                // TODO: caller needs to give a hint on which fix session she wants to use
                let mut tk = Token(0);
                for t in 0..100 { // 0 to 99 is totally arbitrary
                    tk = Token(t);
                    if self.token_2conn.contains( tk.0 ) {
                        break;
                    }
                }
                tk // assumes it was found - bad!
            },
            token => token
        }
    }

    fn handle_timeout(&mut self, ConnetionTimeoutSettings { connection_token: t, event_kind } : ConnetionTimeoutSettings) {
        if let Some(t) = t {
            if let Some(conn) = self.token_2conn.get_mut(t.0).unwrap().as_mut() {
                if let Err(_err) = conn.handler.on_timeout(event_kind) {
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
                    let _ = self.do_connect( addr );
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
    pub token: Token,
    pub action: CommandAction,
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
    /// Required for re-sending scenarios
    SendFrameBackToHandler(frame::FixFrame),
    SetTimeout {
        timeout_in_ms: u32,
        event_kind: Token,
    },
    CancelTimeout (timer::Timeout),
    Disconnect,
}
