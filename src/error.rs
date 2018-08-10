use std::fmt;
use ws;

#[derive(Debug)]
pub enum Error {
    SocketError(ws::Error),
    // Custom
    StringError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match *self {
            Error::SocketError(ref err) => err.to_string(),
            Error::StringError(ref err) => err.clone(),
        };
        write!(f, "Oh no: {}", err_msg)
    }
}

impl From<ws::Error> for Error {
    fn from(err: ws::Error) -> Self {
        Error::SocketError(err)
    }
}
