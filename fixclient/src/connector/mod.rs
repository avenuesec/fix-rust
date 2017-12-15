
///! This module attempts to implement the common fix handler,
///  and just calls the user code through the [UserHandler] which
///  is somewhat similar to the way the quickfix engine is designed.

use std::{io};

use fix::fixmessagegen::*;
use fix::frame::FixFrame;
use super::Sender;

use mio::{Token};
use mio_more::timer;

pub mod handler;
pub mod session;
pub mod fsstore;
pub mod memstore;
pub mod statemachine;


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

pub struct MessageStoreState {
    sender_seq: i32,
    target_seq: i32,
}

impl MessageStoreState {
    pub fn new() -> MessageStoreState {
        MessageStoreState::new_with(1, 1)
    }
    pub fn new_with(sender_seq: i32, target_seq: i32) -> MessageStoreState {
        MessageStoreState {
            sender_seq,
            target_seq,
        }
    }

    fn incr_sender_seq_num(&mut self) -> i32 {
        let temp = self.sender_seq;
        self.sender_seq = self.sender_seq + 1;
        temp
    }
    fn incr_target_seq_num(&mut self) -> i32 {
        let temp = self.target_seq;
        self.target_seq = self.target_seq + 1;
        temp
    }
    fn reset(&mut self) {
        self.sender_seq = 1;
        self.target_seq = 1;
    }

    fn overwrite_seqs(&mut self, sender: i32, target: i32) {
        self.sender_seq = sender;
        self.target_seq = target;
    }
}

pub trait MessageStore {

    fn init(&mut self, sender: Sender);

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn received(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn query(&mut self, begin: i32, end: i32) -> io::Result<Vec<FixFrame>>;

    fn incr_sender_seq_num(&mut self) -> io::Result<i32>;
    fn incr_target_seq_num(&mut self) -> io::Result<i32>;

    fn reset_seqs(&mut self) -> io::Result<()>;

    fn get_state(&self) -> &MessageStoreState;

    fn close(self) -> io::Result<()>;

    fn next_sender_seq_num(&self) -> i32 {
        self.get_state().sender_seq
    }
    fn next_target_seq_num(&self) -> i32 {
        self.get_state().target_seq
    }

    fn overwrite_target_seq(&mut self, new_seq: i32) -> io::Result<()>;
}

pub trait SessionState {

    fn init(&mut self, sender: Sender);

    fn build(&mut self, message: FixMessage, fill_seq: bool) -> io::Result<FixFrame>;

    fn build_for_resend(&mut self, original: FixFrame) -> io::Result<FixFrame>;

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn received(&mut self, frame: &FixFrame) -> io::Result<()>;

    fn new_timeout(&mut self, timeout: &timer::Timeout, event_kind: Token);

    fn on_timeout(&mut self, event_kind: Token);

    fn close(self) -> io::Result<()>;
}


