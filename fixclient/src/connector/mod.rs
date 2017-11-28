use std::io;
use fix::fixmessagegen::*;
use fix::frame::FixFrame;
use super::Sender;

use mio::{Token};
use mio_more::timer;

pub mod handler;
pub mod session;
pub mod fsstore;


#[derive(Clone)]
pub struct UserSender {
    sender: Sender,
}
impl UserSender {
    pub fn send(&self, message: FixMessage) -> io::Result<()> {
        self.sender.send_self(message)
    }

    pub fn set_timeout(&self, timeout_in_ms: u32, event_kind: Token) -> io::Result<()> {
        self.sender.set_timeout(timeout_in_ms, event_kind)
    }

    pub fn cancel_timeout(&self, timeout: timer::Timeout) -> io::Result<()> {
        self.sender.cancel_timeout(timeout)
    }
}

pub trait UserHandlerFactory {
    type Handler: UserHandler;

    fn build(&mut self, sender: UserSender) -> Self::Handler;
}
pub trait UserHandler {
    fn on_new_order_single(&mut self, message: &NewOrderSingleFields) -> io::Result<()>;
}

// Super cool way of adapting a Fn to a trait
impl<F, H> UserHandlerFactory for F 
    where H : UserHandler,
          F : FnMut(UserSender) -> H {
    type Handler = H;

    fn build(&mut self, sender: UserSender) -> Self::Handler {
        // when the "event" is called, we just invoke the FnMut which should return a handler
        self(sender)
    }
}

pub trait MessageStore {

    fn init(&mut self, sender: Sender);

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn received(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn incr_sender_seq_num(&mut self) -> io::Result<u32>;
    fn incr_target_seq_num(&mut self) -> io::Result<u32>;
    fn reset_seqs(&mut self) -> io::Result<()>;
}

pub trait SessionState {

    fn init(&mut self, sender: Sender);

    fn build(&mut self, message: FixMessage) -> io::Result<FixFrame>;

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn received(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn new_timeout(&mut self, timeout: &timer::Timeout, event_kind: Token);

    fn on_timeout(&mut self, event_kind: Token);
}

