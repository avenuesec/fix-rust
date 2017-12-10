
//! default handler that delegates to pipelines and coordinates session state and message persistence

// use std::sync::Arc;
use std::io;
use std::marker::PhantomData;

use chrono::{Utc};

use mio::{Token};
use mio_more::timer;

use fix::frame::*;
use fix::fixmessagegen::*;
use super::super::Sender;
use super::{SessionState, UserHandlerFactory, UserSender};

// needs to be driven by:
use super::super::{FixHandler, FixSessionConfig};


pub struct DefaultHandler <State,UserF>
    where State : SessionState,
          UserF : UserHandlerFactory {
    sender: Sender,
    user_handler: UserF::Handler,
    user_handler_factory: PhantomData<UserF>,
    state : State,
    heart_bt: i32,
}

impl <State,UserF> DefaultHandler <State,UserF>
    where State : SessionState,
          UserF : UserHandlerFactory {

    pub fn new(sender: Sender, settings: FixSessionConfig, state: State, mut user_handler_f: UserF) -> Self {
        // info!("send new");

        let user_handler = { 
            user_handler_f.build(UserSender { sender: sender.clone() }) 
        };

        DefaultHandler {
            sender: sender.clone(), 
            user_handler,
            user_handler_factory: PhantomData::default(),
            heart_bt: settings.heart_beat as i32,
            state,
        }
    }

    pub fn init(&mut self) {
        info!("DefaultHandler init");

        self.state.init( self.sender.clone() );

    }

    fn send(&mut self, message: FixMessage) -> io::Result<()> {
        info!("DefaultHandler send");

        let frame = self.state.build(message)?;

        self.state.sent(&frame)?;

        self.sender.send(frame)?;

        Ok( () )
    }

    fn resend(&mut self, frame: FixFrame) -> io::Result<()> {
        info!("DefaultHandler resend");

        let frame = self.state.build_for_resend(frame)?;

        // should we persist resends?
        // self.state.sent(&frame)?;

        self.sender.send(frame)?;

        Ok( () )
    }
}

impl <State,UserF> FixHandler for DefaultHandler <State,UserF>
    where State : SessionState,
          UserF : UserHandlerFactory {

    // cancelled with a timeout has been set on the timer
    fn new_timeout(&mut self, timeout: timer::Timeout, event_kind: Token ) {

        info!("DefaultHandler new_timeout called for event_kind: {:?}", event_kind);

        self.state.new_timeout(&timeout, event_kind);
    }

    // cancelled when a timeout has been triggered
    fn on_timeout(&mut self, event_kind: Token) -> io::Result<()> {
        
        let now = Utc::now();
        info!("DefaultHandler on_timeout called for {:?} event_kind: {:?}", now, event_kind);

        self.state.on_timeout(event_kind);

        Ok( () )
    }

    fn on_message(&mut self, frame: FixFrame) -> io::Result<()> {
        info!("DefaultHandler on_message called for {:?}", frame);

        self.state.received(&frame)?;

        Ok( () )
    }

    fn on_network_error(self) {
        info!("DefaultHandler handler on_network_error");

        // indicates the handler is about to be destroyed, so we should close everything

        self.state.close();
    }

    /// invoked when someone calls Sender.send(fixmessage)
    /// this allow us to build the frame, store, validate, etc
    fn before_send(&mut self, message: FixMessage) {
        info!("DefaultHandler handler before_send");

        if let Err(e) = self.send(message) {
            error!("before_send error  {:?}", e);
        }
    }

    /// Used when re-sending a message, hence we need the original frame
    fn before_resend(&mut self, message: FixFrame) {
        info!("DefaultHandler handler before_resend");

        if let Err(e) = self.resend(message) {
            error!("before_resend error  {:?}", e);
        }
    }
}



