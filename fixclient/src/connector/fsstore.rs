
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::fs::{self, File, OpenOptions, DirBuilder};
use std::path::{PathBuf, Path};

use bytes::{BytesMut};
use chrono::{DateTime, Utc};

use fix::frame::FixFrame;
use super::{MessageStore, MessageStoreState};
use super::super::FixSessionConfig;
use super::super::sender::Sender;


pub struct FSMessageStore {
    seqnums: File,
    messages: File,
    messages_pos: usize,
    headers: File,
    session: File,

    session_creation: DateTime<Utc>,

    sender: Option<Sender>,
    state: MessageStoreState,

    reusable_buf: BytesMut,
}

impl FSMessageStore {

    pub fn new( cfg: &FixSessionConfig ) -> io::Result<FSMessageStore> {

        let prefix = format!("{}_{}_{}", cfg.begin_string, cfg.target_comp, cfg.sender_comp).to_lowercase();
        let cfg_store = &cfg.store_dir;

        let seqs_path_buf     = to_path(cfg_store, & format!("{}.seqnums", prefix ))?;
        let messages_path_buf = to_path(cfg_store, & format!("{}.body", prefix ))?;
        let header_path_buf   = to_path(cfg_store, & format!("{}.header", prefix ))?;
        let session_path_buf  = to_path(cfg_store, & format!("{}.session", prefix ))?;

        let (sender_seq_num, target_seq_num) =
            FSMessageStore::restore_seqnums(seqs_path_buf.as_path())?;
        let session_creation =
            FSMessageStore::restore_session(session_path_buf.as_path())?;
        debug!("initial store state: sender_seq {} - target: {} - session creation {}", sender_seq_num, target_seq_num, session_creation);

        let mut seqs_file = OpenOptions::new().write(true).create(true).open(seqs_path_buf.as_path())?;
        let mut messages = OpenOptions::new().write(true).create(true).append(true).open(messages_path_buf.as_path())?;
        let mut headers  = OpenOptions::new().write(true).create(true).append(true).open(header_path_buf.as_path())?;
        let mut session = OpenOptions::new().write(true).create(true).truncate(true).open(session_path_buf.as_path())?;

        let messages_pos = messages.seek(SeekFrom::End(0))?;
        headers.seek(SeekFrom::End(0));

        let state = MessageStoreState::new_with(sender_seq_num, target_seq_num);

        let mut store = FSMessageStore {
            seqnums: seqs_file,
            session,
            messages,
            headers,
            messages_pos: messages_pos as usize,
            sender: None,
            session_creation,
            state,
            reusable_buf : BytesMut::new(),
        };

        store.persist_seqs()?;
        store.persist_session()?;

        Ok(store)
    }

    fn restore_seqnums(path : &Path) -> io::Result<(u32, u32)> {
        if !path.exists() {
            return Ok( ( 1, 1 ) ) // default
        }

        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        drop(file);
        if let Some(index) = buffer.find(" : ") {
            let sender_seq = u32::from_str_radix( &buffer[ ..index ]      , 10 ).unwrap();
            let target_seq = u32::from_str_radix( &buffer[ (index + 3).. ], 10 ).unwrap();

            Ok ( (sender_seq, target_seq) )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("malformed seqnums file {}", buffer)) )
        }
    }

    fn restore_session(path : &Path) -> io::Result<DateTime<Utc>> {
        if !path.exists() {
            return Ok( Utc::now() )
        }

        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        drop(file);
        let parse_result = buffer.parse();

        match parse_result {
            Ok( dt ) => {
                Ok( dt )
            },
            Err( _err ) => {
                Err( io::Error::new(io::ErrorKind::Other, format!("could not parse datetime from file {}", buffer)) )
            }
        }
    }

    pub fn get_sender_seq(&self) -> u32 {
        self.state.sender_seq
    }
    pub fn get_target_seq(&self) -> u32 {
        self.state.target_seq
    }

    // Only for unit tests
    pub fn delete_files( prefix: &str, store_dir: &str ) -> io::Result<()> {
        fs::remove_file( to_path(store_dir, &format!("{}.seqnums", prefix))?.as_path() )?;
        fs::remove_file( to_path(store_dir, &format!("{}.body", prefix))?.as_path() )?;
        fs::remove_file( to_path(store_dir, &format!("{}.header", prefix))?.as_path() )?;
        fs::remove_file( to_path(store_dir, &format!("{}.session", prefix))?.as_path() )?;
        Ok( () )
    }

    pub fn persist_seqs(&mut self) -> io::Result<()> {
        self.seqnums.seek(SeekFrom::Start(0))?;
        write!( self.seqnums, "{} : {}", self.state.sender_seq, self.state.target_seq )?;
        self.seqnums.flush()
    }

    pub fn persist_session(&mut self) -> io::Result<()> {
        self.session.seek(SeekFrom::Start(0))?;
        write!( self.session, "{:?}", self.session_creation )?;
        self.session.flush()
    }
}


impl MessageStore for FSMessageStore {

    fn init(&mut self, sender: Sender) {
        self.sender = Some(sender);
    }

    fn incr_sender_seq_num(&mut self) -> io::Result<u32> {
        let temp = self.state.incr_sender_seq_num();
        self.persist_seqs()?;
        Ok(temp)
    }

    fn incr_target_seq_num(&mut self) -> io::Result<u32> {
        let temp = self.state.incr_target_seq_num();
        self.persist_seqs()?;
        Ok(temp)
    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {
        // TODO persist msg + segment index (will io be an issue?)

        if self.reusable_buf.len() != 0 {
            unsafe { self.reusable_buf.set_len(0); } // will this cause the buffer free its capacity?
        }

        // writes immutable msg to buffer
        frame.write(&mut self.reusable_buf)?;

        // calculate offset
        let len = self.reusable_buf.len();
        self.messages_pos = self.messages_pos + len;

        // persists message
        self.messages.write_all( &mut self.reusable_buf )?;


        // persists offset
        write!( &mut self.headers, "{},{},{} ", frame.seq, self.messages_pos, len)?;

        // debug!("FSMessageStore offsets {}  {} ", self.messages_pos, len);

        Ok( () )
    }

    fn received(&mut self, _frame: &FixFrame) -> io::Result<()> {
        Ok( () )
    }

    fn reset_seqs(&mut self) -> io::Result<()> {
        self.state.sender_seq = 1;
        self.state.target_seq = 1;
        self.persist_seqs()?;
        Ok( () )
    }

    fn get_state(&self) -> &MessageStoreState {
        &self.state
    }
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