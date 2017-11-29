
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::fs::{self, File, OpenOptions, DirBuilder};
use std::path::{PathBuf};

use super::MessageStore;
use fix::frame::FixFrame;
use super::super::FixSessionConfig;
use super::super::sender::Sender;


pub struct FSMessageStore {
    seqnums: File,
    sender_seq: u32,
    target_seq: u32,
    sender: Option<Sender>,
}

impl FSMessageStore {

    pub fn new( cfg: &FixSessionConfig ) -> io::Result<FSMessageStore> {

        let cfg_store = &cfg.store_dir;
        let mut sender_seq_num = 1;
        let mut target_seq_num = 1;

        let seqs_path_buf = FSMessageStore::to_path(cfg_store, "seqnums")?;
        if seqs_path_buf.as_path().exists() {
            let mut file = OpenOptions::new().read(true).open(seqs_path_buf.as_path())?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            if let Some(index) = buffer.find(" : ") {
                sender_seq_num = u32::from_str_radix( &buffer[ ..index ]      , 10 ).unwrap();
                target_seq_num = u32::from_str_radix( &buffer[ (index + 3).. ], 10 ).unwrap();
            }
            drop(file);
        }
        let seqs_file = OpenOptions::new().write(true).create(true).open(seqs_path_buf.as_path())?;

        debug!("exists: {:?} {}", seqs_path_buf, seqs_path_buf.as_path().exists());

        Ok(FSMessageStore {
            seqnums: seqs_file,
            sender_seq: sender_seq_num,
            target_seq: target_seq_num,
            sender: None,
        })
    }

    pub fn get_sender_seq(&self) -> u32 {
        self.sender_seq
    }
    pub fn get_target_seq(&self) -> u32 {
        self.target_seq
    }

    // Only for unit tests
    pub fn delete_files( store_dir: &str ) -> io::Result<()> {
        let seqs_path_buf = FSMessageStore::to_path(store_dir, "seqnums")?;
        fs::remove_file( seqs_path_buf.as_path() )?;
        Ok( () )
    }

    fn to_path( store: &str, file_name: &str ) -> io::Result<PathBuf> {
        let mut path_buf = PathBuf::new();
        path_buf.push( store );
        if !path_buf.as_path().exists() {
            DirBuilder::new().recursive(true).create( store )?;
        }

        path_buf.push( file_name );
        Ok ( path_buf )
    }

    pub fn persist_seqs(&mut self) -> io::Result<()> {
        self.seqnums.seek(SeekFrom::Start(0))?;
        write!( self.seqnums, "{} : {}", self.sender_seq, self.target_seq )?;
        self.seqnums.flush()
    }
}


impl MessageStore for FSMessageStore {

    fn init(&mut self, sender: Sender) {
        self.sender = Some(sender);
    }

    fn incr_sender_seq_num(&mut self) -> io::Result<u32> {
        let temp = self.sender_seq;
        self.sender_seq = self.sender_seq + 1;
        self.persist_seqs()?;
        Ok(temp)
    }

    fn incr_target_seq_num(&mut self) -> io::Result<u32> {
        let temp = self.target_seq;
        self.target_seq = self.target_seq + 1;
        self.persist_seqs()?;
        Ok(temp)
    }

    fn sent(&mut self, _frame: &FixFrame) -> io::Result<()> {

        Ok( () )
    }

    fn received(&mut self, _frame: &FixFrame) -> io::Result<()> {

        Ok( () )
    }

    fn reset_seqs(&mut self) -> io::Result<()> {
        self.sender_seq = 1;
        self.target_seq = 1;
        self.persist_seqs()?;
        Ok( () )
    }
}


