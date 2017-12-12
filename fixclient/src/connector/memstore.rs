
use std::io::{self};

use super::{MessageStore, MessageStoreState};
use super::super::sender::Sender;
use super::super::FixSessionConfig;
use fix::frame::FixFrame;


pub struct MemoryMessageStore {
    state: MessageStoreState,
    messages : Vec<FixFrame>,
}

impl MemoryMessageStore {

    pub fn new( cfg: &FixSessionConfig ) -> io::Result<MemoryMessageStore> {
        Ok( MemoryMessageStore {
            state : MessageStoreState::new(),
            messages: vec![],
        })
    }

    pub fn add_to_store(&mut self, frame: FixFrame) {
        self.messages.push( frame );
    }
}

impl MessageStore for MemoryMessageStore {

    fn init(&mut self, sender: Sender) {

    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {
        self.messages.push( frame.clone() );
        Ok( () )
    }

    fn received(&mut self, frame: &FixFrame) -> io::Result<()> {
        Ok( () )
    }

    fn query(&mut self, begin: i32, end: i32) -> io::Result<Vec<FixFrame>> {
        let mut subset = vec![];

        for frame in self.messages.iter() {
            let seq = frame.header.msg_seq_num;

            if seq >= begin && (end == 0 || seq <= end) {
                subset.push( frame.clone() );
            }
        }

        Ok( subset )
    }

    fn incr_sender_seq_num(&mut self) -> io::Result<i32> {
        Ok ( self.state.incr_sender_seq_num() )
    }
    fn incr_target_seq_num(&mut self) -> io::Result<i32> {
        Ok ( self.state.incr_target_seq_num() )
    }
    fn reset_seqs(&mut self) -> io::Result<()> {
        self.state.reset();
        Ok( () )
    }

    fn get_state(&self) -> &MessageStoreState {
        &self.state
    }

    fn close(self) -> io::Result<()> {
        Ok( () )
    }
}
