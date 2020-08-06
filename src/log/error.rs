/// The log error give a better explanation of what happened

use std::fmt;

/// Syntax sugar
pub type Result<T> = std::result::Result<T, Error>;

/// Error of the logger
#[derive(Debug)]
pub enum Error {
    CouldNotOpenLogFile(std::io::Error),
    DeadDaemon,
    DeadMaster,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::CouldNotOpenLogFile(ref e) => write!(f, "Could not open the the log file, error: {}", e),
            Error::DeadDaemon =>  write!(f, "The log server is exausted, make a new one"),
            Error::DeadMaster =>  write!(f, "The master log is poisoned, this log is unusable"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        match *self {
            Error::CouldNotOpenLogFile(ref e) => Some(e),
            Error::DeadDaemon => None,
            Error::DeadMaster => None,
        }
    }
}