//! MessageLogger's implementation that just outputs to the file system

use std::io::{self, Read, Write, Seek, SeekFrom};
use std::fs::{self, File, OpenOptions, DirBuilder};
use std::path::{PathBuf, Path};

use fix::frame::{self, FixFrame};

use super::MessageLogger;
use super::super::FixSessionConfig;


pub struct FSLogger {
    received: File,
    sent: File,
}

impl FSLogger {
    pub fn new( cfg: &FixSessionConfig ) -> io::Result<FSLogger> {
        let prefix = format!("{}_{}_{}", cfg.begin_string, cfg.target_comp, cfg.sender_comp).to_lowercase();
        let cfg_log = &cfg.log_dir;

        let incoming_path_buf = to_path(cfg_log, & format!("{}.messages.recv", prefix ))?;
        let outgoing_path_buf = to_path(cfg_log, & format!("{}.messages.sent", prefix ))?;

        let mut received = OpenOptions::new().write(true).create(true).append(true).open(incoming_path_buf.as_path())?;
        let mut sent     = OpenOptions::new().write(true).create(true).append(true).open(outgoing_path_buf.as_path())?;

        // make sure we're at the end of the file
        received.seek(SeekFrom::End(0))?;
        sent.seek(SeekFrom::End(0))?;

        Ok( FSLogger {
            received, sent
        })
    }
}

impl MessageLogger for FSLogger {

    fn init(&mut self) {

    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {

        Ok( () )
    }

    fn received(&mut self, frame: &FixFrame) -> io::Result<()> {

        Ok( () )
    }

    fn close(self) -> io::Result<()> {

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