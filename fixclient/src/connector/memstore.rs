
use std::io::{self};

use super::{MessageStore, MessageStoreState};
use super::super::sender::Sender;
use super::super::FixSessionConfig;
use fix::frame::FixFrame;


pub struct MemoryMessageStore {
    state: MessageStoreState,
}

impl MemoryMessageStore {

    pub fn new( cfg: &FixSessionConfig ) -> io::Result<MemoryMessageStore> {

        Ok( MemoryMessageStore {
            state : MessageStoreState::new(),
        } )
    }
}

impl MessageStore for MemoryMessageStore {

    fn init(&mut self, sender: Sender) {

    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {

        Ok( () )
    }

    fn received(&mut self, frame: &FixFrame) -> io::Result<()> {

        Ok( () )
    }

    fn query(&self, begin: u32, end: u32) -> Vec<FixFrame> {
        Vec::new()
    }

    fn incr_sender_seq_num(&mut self) -> io::Result<u32> {
        Ok ( self.state.incr_sender_seq_num() )
    }
    fn incr_target_seq_num(&mut self) -> io::Result<u32> {
        Ok ( self.state.incr_target_seq_num() )
    }
    fn reset_seqs(&mut self) -> io::Result<()> {
        self.state.reset();
        Ok( () )
    }

    fn get_state(&self) -> &MessageStoreState {
        &self.state
    }
}