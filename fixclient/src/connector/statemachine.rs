
use std::string::ToString;
use std::result::Result;
use std::io;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;

use super::MessageStore;

#[derive(PartialEq)]
enum FixPartyState {
    /// Initial state
    Connected,

    /// Processing logon stage
    Logon,

    /// Logout initiated
    Logout,

    /// Gap detected, so negotiating and awaiting resend or reset
    /// At this point, no new app messages should be sent
    /// and messages past gap should not be delivered to application code
    MessageSynchronization,

    /// All is fine
    Operational,
}
pub struct FixStateTransition <Store : MessageStore> {
    them:  FixPartyState,
    us:    FixPartyState,
    store: Rc<Store>,
}

/// make sure the state is kept sane
impl <Store : MessageStore> FixStateTransition <Store> {

    pub fn new( store: Rc<Store> ) -> FixStateTransition <Store> {
        FixStateTransition {
            us:   FixPartyState::Connected,
            them: FixPartyState::Connected,
            store,
        }
    }

    /// logon message sent
    pub fn outgoing_logon(&mut self) -> io::Result<()> {
        if self.us == FixPartyState::Connected {
            self.us = FixPartyState::Logon;
            self.them = FixPartyState::Logon;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("cannot not go to logon_sent from {}", self.to_string()).as_str() ) )
        }
    }

    /// logon message recv (confirmation)
    pub fn incoming_logon(&mut self) -> io::Result<()> {
        if self.us == FixPartyState::Logon {
            self.them = FixPartyState::MessageSynchronization;
            self.us = FixPartyState::MessageSynchronization;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("logon_recv without sending a logon? current: {}", self).as_str() ) )
        }
    }

    /// logout message sent
    pub fn outgoing_logout(&mut self) -> io::Result<()> {
        self.us = FixPartyState::Logout;
        Ok( () )
    }

    /// logout message recv
    pub fn incoming_logout(&mut self) -> io::Result<()> {
        self.us = FixPartyState::Logout;
        // transition to disconnected?
        Ok( () )
    }

    pub fn incoming_sync_complete(&mut self) {
        self.them = FixPartyState::Operational;
    }

    pub fn outgoing_sync_complete(&mut self) {
        self.us = FixPartyState::Operational;
    }

    /// other side requested resend of missing messages (gap fill)
//    pub fn incoming_resend(&mut self) -> io::Result<()> {
//        if self.is_operational() {
//            self.them = FixPartyState::AwaitingResend;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("incoming_resend not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

    /// we requested resend of missing messages when a gap is detected
//    pub fn outgoing_resend(&mut self) -> io::Result<()> {
//        if self.is_operational() {
//            self.us = FixPartyState::AwaitingResend;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("outgoing_resend not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

//    pub fn confirm_resent_sent(&mut self) -> io::Result<()>  {
//        if self.them == FixPartyState::AwaitingResend {
//            self.them = FixPartyState::Operational;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("confirm_resent_sent not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

//    pub fn confirm_resent_recv(&mut self) -> io::Result<()>  {
//        if self.us == FixPartyState::AwaitingResend {
//            self.us = FixPartyState::Operational;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("confirm_resent_recv not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

//    pub fn incoming_seqreset(&mut self) -> io::Result<()> {
//        // since this can happen in many situations, it's always acceptable
//        // Operational and AwaitingResend would be the most obvious
//
//        Ok( () )
//    }
//
//    pub fn outgoing_seqreset(&mut self) -> io::Result<()> {
//        // since this can happen in many situations, it's always acceptable
//        // Operational and AwaitingResend would be the most obvious
//
//        Ok( () )
//    }

//    pub fn disconnect(&mut self) {
//        self.them = FixPartyState::Disconnected;
//        self.us = FixPartyState::Disconnected;
//    }

    pub fn is_operational(&self) -> bool {
        self.us == FixPartyState::Operational && self.them == FixPartyState::Operational
    }
}

impl <Store : MessageStore> Display for FixStateTransition <Store> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str( &format!("us {} - them {}", self.us.to_string(), self.them.to_string()) )
    }
}

impl ToString for FixPartyState {
    fn to_string(&self) -> String {
        match self {
            &FixPartyState::Connected               => "connected",
            // &FixPartyState::Disconnected     => "disconnected",
            &FixPartyState::Logon                   => "logon",
            &FixPartyState::Logout                  => "logout",
            &FixPartyState::MessageSynchronization  => "sync",
            &FixPartyState::Operational             => "operational",
            _ => unreachable!()
        }.to_owned()
    }
}
