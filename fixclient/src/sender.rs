
use std::io;
use mio::{Token};
use mio_more::{channel, timer};
use super::io_handler::{Command, CommandAction};
use fix::frame;
use fix::fixmessagegen::FixMessage;


// for advanced scenarios where you need the sender 
// before even the connections were established
#[derive(Clone)]
pub struct AdvSender {
    resolve_token: Token,
    queue_tx: channel::SyncSender<Command>,
}
impl AdvSender {

    pub fn new(resolve_token: Token, queue_tx: channel::SyncSender<Command>) -> AdvSender {
        AdvSender {
            resolve_token: resolve_token,
            queue_tx: queue_tx,
        }
    }

    pub fn send(&self, message: FixMessage) -> io::Result<()> {
        debug!("AdvSender sending SendBackToHandler {:?}", message);
        let cmd = Command::new( self.resolve_token, CommandAction::SendBackToHandler( message ) );
        if let Err(err) = self.queue_tx.send( cmd ) {
             return Err(io::Error::new( io::ErrorKind::Other, err) )
        }
        Ok( () )
    }
}


#[derive(Clone)]
pub struct Sender { 
    token: Token,
    queue_tx: channel::SyncSender<Command>,
}

impl Sender {
    pub fn new(token: Token, tx: channel::SyncSender<Command>) -> Sender {
        Sender {
            token: token,
            queue_tx: tx,
        }
    }

    pub fn send(&self, message: frame::FixFrame) -> io::Result<()> {
        debug!("enqueing frame for sending {:?}", message);

        let cmd = Command::new( self.token, CommandAction::Message( message ) );
        if let Err(err) = self.queue_tx.send( cmd ) {
            return Err(io::Error::new( io::ErrorKind::Other, err) )
        }
        Ok( () )
    }

    // only fired once
    pub fn set_timeout(&self, timeout_in_ms: u32, event_kind: Token) -> io::Result<()> {
        debug!("enqueing set_timeout {}", timeout_in_ms);

        let cmd = Command::new( self.token, CommandAction::SetTimeout { timeout_in_ms, event_kind } );
        if let Err(err) = self.queue_tx.send( cmd ) {
            return Err(io::Error::new( io::ErrorKind::Other, err) )
        }
        Ok( () )
    }

    // attempts to cancel it, but there are no guarantees it will
    pub fn cancel_timeout(&self, timeout: timer::Timeout) -> io::Result<()> {

        debug!("enqueing cancel_timeout {:?}", timeout);

        let cmd = Command::new( self.token, CommandAction::CancelTimeout(timeout) );

        if let Err(err) = self.queue_tx.send( cmd ) {
            return Err(io::Error::new( io::ErrorKind::Other, err) )
        }
        Ok( () )
    }

    pub fn send_self(&self, message: FixMessage) -> io::Result<()> {
        let cmd = Command::new( self.token, CommandAction::SendBackToHandler( message ) );
        if let Err(err) = self.queue_tx.send( cmd ) {
            return Err(io::Error::new( io::ErrorKind::Other, err) )
        }
        Ok( () )
    }
}
