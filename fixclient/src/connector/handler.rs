
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
use super::{MessageStore, SessionState, UserHandlerFactory, UserSender};

// needs to be driven by:
use super::super::{FixHandler, FixSessionConfig};


pub struct DefaultHandler <State,UserF>
    where State : SessionState,
          UserF : UserHandlerFactory {
    sender: Sender,
    sender_comp_id: String,
    target_comp_id: String,
    user_handler: UserF::Handler,
    user_handler_factory: PhantomData<UserF>,
    state : State,
    heart_bt: i32,
}

impl <State,UserF> DefaultHandler <State,UserF>
    where State : SessionState,
          UserF : UserHandlerFactory {

    pub fn new(sender: Sender, settings: FixSessionConfig, state: State, mut user_handler_f: UserF) -> Self {
        info!("send new");

        let user_handler = { 
            user_handler_f.build(UserSender { sender: sender.clone() }) 
        };

        DefaultHandler {
            sender: sender.clone(), 
            sender_comp_id: settings.sender_comp.to_owned(),
            target_comp_id: settings.target_comp.to_owned(),
            user_handler,
            user_handler_factory: PhantomData::default(),
            heart_bt: settings.heart_beat as i32,
            state,
        }
    }

    pub fn init(&mut self) {
        info!("handler init");

        self.state.init( self.sender.clone() );

        // Start login process
        let flds = LogonFields {
            encrypt_method: Field_EncryptMethod_Enum::NONE,
            heart_bt_int: self.heart_bt,
            reset_seq_num_flag: Some(true), // TODO: review this
            .. Default::default()
        };
        let logon_message = FixMessage::Logon(Box::new(flds));

        self.send( logon_message );
    }
    
    fn send(&mut self, message: FixMessage) -> io::Result<()> {
        info!("send init");

        let frame = self.state.build(message)?;

        self.state.sent(&frame)?;

        self.sender.send(frame)?;

        Ok( () )
    }
}

impl <State,UserF> FixHandler for DefaultHandler <State,UserF>
    where //Store : MessageStore,
          State : SessionState, 
          UserF : UserHandlerFactory {

    // cancelled with a timeout has been set on the timer
    fn new_timeout(&mut self, timeout: timer::Timeout, event_kind: Token ) {

        info!("new_timeout called for event_kind: {:?}", event_kind);

        self.state.new_timeout(&timeout, event_kind);
    }

    // cancelled when a timeout has been triggered
    fn on_timeout(&mut self, event_kind: Token) -> io::Result<()> {
        
        let now = Utc::now();
        info!("on_timeout called for {:?} event_kind: {:?}", now, event_kind);

        self.state.on_timeout(event_kind);

        Ok( () )
    }

    fn on_message(&mut self, frame: FixFrame) -> io::Result<()> {
        info!("on_message called for {:?}", frame);

        self.state.received(&frame)?;

        Ok( () )
    }

    fn on_network_error(&mut self) {
        info!("handler on_network_error");

        // indicates the handler is about to be destroyed, so we should close everything
    }

    // invoked when someone calls Sender2.send(fixmessage)
    // this allow us to build the frame, store, validate, etc
    fn before_send(&mut self, message: FixMessage) {
        info!("handler before_send");

        self.send(message);
    }
}



