
use std::string::ToString;
use std::result::Result;
use std::io;
use std::fmt::{Display, Error, Formatter};


#[derive(PartialEq)]
enum FixState {
    Connected,
    Disconnected,
    Logon,
    Logout,
    AwaitingResend,
    Operational,
}
pub struct FixStateTransition {
    them: FixState,
    us:   FixState,
}

/// make sure the state is kept sane
impl FixStateTransition {

    pub fn new() -> FixStateTransition {
        FixStateTransition { us: FixState::Connected, them: FixState::Connected  }
    }

    /// logon message sent
    pub fn outgoing_logon(&mut self) -> io::Result<()> {
        if self.us == FixState::Connected {
            self.us = FixState::Logon;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("cannot not go to logon_sent from {}", self.to_string()).as_str() ) )
        }
    }

    /// logon message recv (confirmation)
    pub fn incoming_logon(&mut self) -> io::Result<()> {
        if self.us == FixState::Logon {
            self.them = FixState::Operational;
            self.us = FixState::Operational;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("logon_recv without sending a logon? current: {}", self).as_str() ) )
        }
    }

    /// logout message sent
    pub fn outgoing_logout(&mut self) -> io::Result<()> {
        self.us = FixState::Logout;
        Ok( () )
    }

    /// logout message recv
    pub fn incoming_logout(&mut self) -> io::Result<()> {
        self.us = FixState::Logout;
        // transition to disconnected?
        Ok( () )
    }

    /// other side requested resend of missing messages (gap fill)
    pub fn incoming_resend(&mut self) -> io::Result<()> {
        if self.is_operational() {
            self.them = FixState::AwaitingResend;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("incoming_resend not valid at this point. current: {}", self).as_str() ) )
        }
    }

    /// we requested resend of missing messages when a gap is detected
    pub fn outgoing_resend(&mut self) -> io::Result<()> {
        if self.is_operational() {
            self.us = FixState::AwaitingResend;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("outgoing_resend not valid at this point. current: {}", self).as_str() ) )
        }
    }

    pub fn confirm_resent_sent(&mut self) -> io::Result<()>  {
        if self.them == FixState::AwaitingResend {
            self.them = FixState::Operational;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("confirm_resent_sent not valid at this point. current: {}", self).as_str() ) )
        }
    }

    pub fn confirm_resent_recv(&mut self) -> io::Result<()>  {
        if self.us == FixState::AwaitingResend {
            self.us = FixState::Operational;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("confirm_resent_recv not valid at this point. current: {}", self).as_str() ) )
        }
    }

    pub fn incoming_seqreset(&mut self) -> io::Result<()> {
        // since this can happen in many situations, it's always acceptable
        // Operational and AwaitingResend would be the most obvious

        Ok( () )
    }

    pub fn outgoing_seqreset(&mut self) -> io::Result<()> {
        // since this can happen in many situations, it's always acceptable
        // Operational and AwaitingResend would be the most obvious

        Ok( () )
    }

//    pub fn disconnect(&mut self) {
//        self.them = FixState::Disconnected;
//        self.us = FixState::Disconnected;
//    }

    pub fn is_operational(&self) -> bool {
        self.us == FixState::Operational && self.them == FixState::Operational
    }
}

impl Display for FixStateTransition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str( &format!("us {} - them {}", self.us.to_string(), self.them.to_string()) )
    }
}

impl ToString for FixState {
    fn to_string(&self) -> String {
        match self {
            &FixState::Connected        => "connected",
            &FixState::Disconnected     => "disconnected",
            &FixState::Logon            => "logon",
            &FixState::Logout           => "logout",
            &FixState::AwaitingResend   => "await_resend",
            &FixState::Operational      => "operational",
            _ => unreachable!()
        }.to_owned()
    }
}
