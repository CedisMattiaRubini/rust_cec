use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJsonError(serde_json::Error),
    TcpError(std::io::Error),  
    DataExcess,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SerdeJsonError(ref e) => e.fmt(f),
            Error::TcpError(ref e) => e.fmt(f),
            Error::DataExcess => write!(f, "sending too much data"),
            /*Error::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            Error::Parse(ref e) => e.fmt(f),*/
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::SerdeJsonError(ref e) => Some(e),
            Error::TcpError(ref e) => Some(e),
            Error::DataExcess => None,
            /*DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),*/
        }
    }
}


// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
impl From<std::io::Error> for Error {
    // Error from std::io::Error
    fn from(err: std::io::Error) -> Error {
        Error::TcpError(err)
    }
}

impl From<serde_json::Error> for Error{
    // Error from 
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJsonError(err)
    }
}