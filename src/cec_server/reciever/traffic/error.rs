/// The error of the traffic

use std::fmt;

/// Server Result
pub type Result<T> = std::result::Result<T, TrafficError>;

/// Error that might be caused in the server
#[derive(Debug)]
pub enum TrafficError{
    CouldNotSendResponse(std::sync::mpsc::SendError<String>),
    NoResponse,
}

impl fmt::Display for TrafficError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TrafficError::CouldNotSendResponse(ref e) => write!(f, "Could not send to CEC traffic because: {}", e),
            TrafficError::NoResponse => write!(f, "No response avaliable"),
        }
    }
}

impl std::error::Error for TrafficError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        match *self {
            TrafficError::CouldNotSendResponse(ref e) => Some(e),
            TrafficError::NoResponse => None,
        }
    }
}