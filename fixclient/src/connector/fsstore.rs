
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::fs::{self, File, OpenOptions, DirBuilder};
use std::path::{PathBuf, Path};
use std::collections::HashMap;

use bytes::{BytesMut};
use chrono::{DateTime, Utc};
use nom::IResult;

use fix::frame::{self, FixFrame};
use super::{MessageStore, MessageStoreState};
use super::super::FixSessionConfig;
use super::super::sender::Sender;


pub struct FSMessageStore {
    seqnums: File,
    messages: File,
    messages_pos: usize,
    headers: File,
    session: File,
    offsets_map: HashMap<i32, (usize, usize)>,

    session_creation: DateTime<Utc>,

    sender: Option<Sender>,
    state: MessageStoreState,

    reusable_buf: BytesMut,
}

macro_rules! scanf {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
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
        let offsets_map =
            FSMessageStore::restore_offsets(header_path_buf.as_path())?;
        debug!("initial store state: sender_seq {} - target: {} - session creation {}", sender_seq_num, target_seq_num, session_creation);

        let seqs_file = OpenOptions::new().write(true).create(true).open(seqs_path_buf.as_path())?;
        let mut messages = OpenOptions::new().create(true).write(true).read(true).open(messages_path_buf.as_path())?;
        let mut headers  = OpenOptions::new().write(true).create(true).append(true).open(header_path_buf.as_path())?;
        let session = OpenOptions::new().write(true).create(true).truncate(true).open(session_path_buf.as_path())?;

        let messages_pos = messages.seek(SeekFrom::End(0))?;
        headers.seek(SeekFrom::End(0))?;

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
            offsets_map,
            reusable_buf : BytesMut::new(),
        };

        store.persist_seqs()?;
        store.persist_session()?;

        Ok(store)
    }

    fn restore_seqnums(path : &Path) -> io::Result<(i32, i32)> {
        if !path.exists() {
            return Ok( ( 1, 1 ) ) // default
        }

        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        drop(file);
        if let Some(index) = buffer.find(" : ") {
            let sender_seq = i32::from_str_radix( &buffer[ ..index ]      , 10 ).unwrap();
            let target_seq = i32::from_str_radix( &buffer[ (index + 3).. ], 10 ).unwrap();

            Ok ( (sender_seq, target_seq) )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("malformed seqnums file {}", buffer)) )
        }
    }

    fn restore_offsets(path : &Path) -> io::Result<HashMap<i32, (usize, usize)>> {
        let mut map = HashMap::new();

        if path.exists() {
            let mut file = OpenOptions::new().read(true).open(path)?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            for line in buf.split(' ') {
                if line == "" { continue };

                let (seq, offset, len) = scanf!(line, ',', i32, usize, usize);
                map.insert( seq.expect("bad seq"), (offset.expect("bad offset"), len.expect("bad len")) );
            }
        }

        Ok( map )
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

    pub fn get_sender_seq(&self) -> i32 {
        self.state.sender_seq
    }
    pub fn get_target_seq(&self) -> i32 {
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

    fn persist_message(&mut self, frame: &FixFrame) -> io::Result<(i32, usize, usize)> {
        if self.reusable_buf.len() != 0 {
            unsafe { self.reusable_buf.set_len(0); } // will this cause the buffer free its capacity?
        }

        // writes immutable msg to buffer
        frame.write(&mut self.reusable_buf)?;

        // calculate offset
        let len = self.reusable_buf.len();
        let pos = self.messages_pos;

        self.messages_pos = self.messages_pos + len; // updates pos

        // persists message
        self.messages.write_all( &mut self.reusable_buf )?;

        Ok ( (frame.header.msg_seq_num, pos, len) )
    }

    fn persist_message_offset(&mut self, offset: (i32, usize, usize)) -> io::Result<()> {

        let (seq, pos, len) = offset;

        // update our local cache
        self.offsets_map.insert(seq, (pos, len));

        write!( &mut self.headers, "{},{},{} ", seq, pos, len)
    }

    fn load_message(&mut self, offset: (usize, usize) ) -> io::Result<FixFrame> {
        // TODO: try cache first, then file

        let (pos, len) = offset;
        self.messages.seek(SeekFrom::Start(pos as u64))?;

        let mut buffer = vec![0; len];

        self.messages.read_exact( buffer.as_mut_slice() )?;

        match frame::parse(&buffer) {
            IResult::Done(_, fixframe) => Ok( fixframe ),
            _ => Err( io::Error::new(io::ErrorKind::Other, "error parsing frame from file") )
        }
    }
}


impl MessageStore for FSMessageStore {

    fn init(&mut self, sender: Sender) {
        self.sender = Some(sender);
    }

    fn incr_sender_seq_num(&mut self) -> io::Result<i32> {
        let temp = self.state.incr_sender_seq_num();
        self.persist_seqs()?;
        Ok(temp)
    }

    fn incr_target_seq_num(&mut self) -> io::Result<i32> {
        let temp = self.state.incr_target_seq_num();
        self.persist_seqs()?;
        Ok(temp)
    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {
        let offset = self.persist_message(frame)?;

        self.persist_message_offset(offset)?;

        Ok( () )
    }

    fn received(&mut self, _frame: &FixFrame) -> io::Result<()> {
        Ok( () )
    }

    fn query(&mut self, start: i32, end: i32) -> io::Result<Vec<FixFrame>> {

        // TODO: implement a cache so we dont do this much IO

        let upper_bound =
            (if end == 0 {
                self.get_sender_seq()
            } else {
                i32::min( end, self.get_sender_seq() )
            }) + 1;

        // if upper_bound < end then ResetSeqs is needed

        let capacity = (upper_bound - start) as usize;
        let mut messages = Vec::with_capacity( capacity );

        for seq in start..upper_bound {
            let offset =
                match self.offsets_map.get(&seq) {
                    Some(offset) => {
                        Some( (offset.0, offset.1) )
                    },
                    None => {
                        // bad, some seq is missing, the handle should send a
                        // seqreset message then
                        None
                    }
                };

            if offset.is_some() {
                if let Ok(frame) = self.load_message( offset.unwrap() ) {
                    messages.push( frame );
                }
            }
        }

        Ok( messages )
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

    fn close(self) -> io::Result<()> {

        // TODO: flush all files and close

        Ok( () )
    }

    fn overwrite_target_seq(&mut self, new_seq: i32) -> io::Result<()> {
        self.state.target_seq = new_seq;
        self.persist_seqs()?;
        Ok( () )
    }

    fn overwrite_sender_seq(&mut self, new_seq: i32) -> io::Result<()> {
        self.state.sender_seq = new_seq;
        self.persist_seqs()?;
        Ok( () )
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