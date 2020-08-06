/// The server error give a better explanation of what happened

use std::fmt;

use crate::log;

/// Server Result
pub type Result<T> = std::result::Result<T, ServerError>;

/// Error that might be caused in the server
#[derive(Debug)]
pub enum ServerError{
    /// During setup the cec daemon could not be spawned
    CecDaemonNotSpawned(std::io::Error),
    CecStdOutMissing,
    CecStdInMissing,
    LogError(log::Error),
    CecDaemonNotKilled(std::io::Error, Box<Option<ServerError>>),
    InvalidJson,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ServerError::CecDaemonNotSpawned(ref e) => write!(f, "The server cound not spawn the cec daemon because: {}", e),
            ServerError::CecStdOutMissing => write!(f, "Missing stdout of the cec daemon"),
            ServerError::CecStdInMissing => write!(f, "Missing stdin of the cec daemon"),
            ServerError::LogError(ref e) => write!(f, "Log error: {}", e),
            ServerError::CecDaemonNotKilled(ref kill_e, ref previous_e) => {
                match &*(*previous_e) {
                    None => write!(f, "The cec daemon could not be killed because: {}", kill_e),
                    Some(prev_e) => write!(f, "The cec daemon could not be killed because: {}. Trying to kill process because: {}", kill_e, prev_e),
                }
            }, // CecDaemonNotKilled
            ServerError::InvalidJson => write!(f, "invalid json"),
        }
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        match *self {
            ServerError::CecDaemonNotSpawned(ref e) => Some(e),
            ServerError::CecStdOutMissing => None,
            ServerError::CecStdInMissing => None,
            ServerError::LogError(ref e) => Some(e),
            ServerError::CecDaemonNotKilled(ref kill_e, _) => {
                Some(kill_e)
            },
            ServerError::InvalidJson => None,
        }
    }
}