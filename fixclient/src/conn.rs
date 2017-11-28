
use std::io;
use std::io::{Write};
use std::net::{SocketAddr};
use mio::{Token, Ready};
use mio::tcp::{TcpStream};
use bytes::{BytesMut, BufMut};

use fix::frame;
use nom::IResult;
use FixHandler;

pub struct Conn <T : FixHandler> {
    pub addr: SocketAddr,
    pub token : Token,
    pub socket: TcpStream,
    pub events: Ready,
    pub inbound:  BytesMut,
    pub outbound: BytesMut,
    pub handler : T,
    pub in_error : bool,
}

impl<T : FixHandler> Conn<T> {
    pub fn new(token: Token, socket: TcpStream, handler: T, addr: SocketAddr) -> Conn<T> {
        Conn {
            addr,
            token,
            socket,
            events:   Ready::readable(), // Ready::empty(),
            inbound:  BytesMut::new(),
            outbound: BytesMut::new(),
            handler,
            in_error: false,
        }
    }

    // puts payload onto outbound buffer and sets writing event
    pub fn send(&mut self, payload: frame::FixFrame ) -> io::Result<()> {
        // debug!("connection_send");

        payload.write( &mut self.outbound )?;

        // debug!("outbound length {}", self.outbound.len() );

        Ok( self.adjust_socket_events() )
    }

    pub fn error(&mut self, error: io::Error ) {
        // TODO: translate the error into proper callbacks on the handler
        self.in_error = true;
    }

    pub fn read(&mut self) -> io::Result<()> {
        
        if let Ok(total) = self.read_all() {
            debug!("total read {}", total);

            if total == 0 { 
                self.disconnected();
                return Err(io::Error::new(io::ErrorKind::ConnectionReset, "connection closed by server"))
            } else {
                self.handle_frames();
            }
        }
        self.adjust_socket_events();
        Ok( () )
    }

    pub fn write(&mut self) -> io::Result<usize> {
        debug!("writing {} to {:?}", self.outbound.len(), self.token);

        let mut written : usize = 0;
        let total_to_write = self.outbound.len();
        let res = loop { 
            match self.socket.write( &mut self.outbound ) {
                Ok(size) => {
                    if size != 0 {
                        self.outbound.split_to(size); // consume what was written
                        written = written + size;
                    }
                    if total_to_write == written { break Ok(written) }
                },
                Err(err) => {
                    if let io::ErrorKind::WouldBlock = err.kind() {
                        // early stop, needs another event triggered to continue
                        break Ok(written)
                    } else {
                        break Err(err)
                    }
                }
            }
        };
        debug!("writing done {:?} to {:?}", res, self.token);

        self.adjust_socket_events();
        res
    }

    fn read_all(&mut self) -> io::Result<usize> {
        // self.inbound.reserve( 1024 * 4 ); // might be no-op

        let mut total_read : usize = 0;

        loop { 
            // let mut b : &mut [u8] = &mut self.inbound;
            // debug!("buffer size {}", b.len());
            let enough = 1024 * 4;
            if self.inbound.remaining_mut() < enough {
                self.inbound.reserve( enough );
            }

            let r = unsafe {
                let mut bufs: [_; 16] = Default::default();
                let n = self.inbound.bytes_vec_mut(&mut bufs);
                self.socket.read_bufs(&mut bufs[..n])
            };
            match r {
                Ok(size) => {
                    total_read = total_read + size;
                    // debug!("read_bufs ret {} len {}", size, self.inbound.len());

                    // required to move pointer of the next write
                    unsafe { self.inbound.advance_mut(size); }

                    if size == 0 { 
                        error!("read size 0 for {:?}", self.token);
                        return Ok( total_read ) 
                    }
                },
                Err(err) => {
                    // would block is expected
                    if let io::ErrorKind::WouldBlock = err.kind() {
                        return Ok( total_read ) 
                    } else {
                        error!("read error {:?} for {:?}", err, self.token);
                        return Err(err)
                    }
                }
            }
        }
    }

    fn disconnected(&mut self) {

    }

    fn handle_frames(&mut self) {
        // generators would be awesome here :-\ (yield frame) etc..
        while self.inbound.len() != 0 {
            match self.decode_and_advance() {
                Ok(None) => {
                    // incomplete. we need to receive more data for a complete frame
                    break;
                }
                Ok(Some(frame)) => {
                    // dispatch to handler
                    if let Err(err) = self.handler.on_message( frame ) {
                        error!("error returned by on_message {:?} for {:?}", err, self.token);
                        // TODO: error should close this connection
                        break;
                    }
                }
                Err(err) => {
                    // problem!
                    error!("decode error {}", err);
                    break;
                }
            }
        }
    }

    fn decode_and_advance(&mut self) -> io::Result<Option<frame::FixFrame>> {
        // debug!("decode got full buffer: {:?}", self.inbound);

        let original_len = self.inbound.len();

        let (consumed, frame) = match frame::parse( &self.inbound ) {
            IResult::Error(e) => { 
                return Err(io::Error::new(io::ErrorKind::Other, e)) 
            },
            IResult::Incomplete(_) => { 
                return Ok(None)
            },
            IResult::Done(remaining, parsed_frame) => {
                let consumed = original_len - remaining.len();
                (consumed, parsed_frame)
            }
        };

        let discarded = self.inbound.split_to( consumed );
        // debug!("raw processed {:?} consumed {}", discarded, consumed);

        Ok( Some(frame) )
    }

    // end of the road for this:
    pub fn consume(self) { 

    }

    fn adjust_socket_events(&mut self) {
        self.events.remove( Ready::writable() );

        self.events.insert( Ready::readable() );
        
        if self.outbound.len() != 0 { // anything left to be sent?
            self.events.insert( Ready::writable() );
        }

        // debug!("adjust_socket_events set to {:?}", self.events);
    }
}
