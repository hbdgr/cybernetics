use std::fmt;
use ws;

#[derive(Debug)]
pub enum Error {
    SocketError(ws::Error),
    WrongPassword,
    // signature verification failure
    SignVerifyFail,
    // Custom
    StringError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match *self {
            Error::SocketError(ref err) => err.to_string(),
            Error::WrongPassword => "Wrong password".into(),
            Error::SignVerifyFail => "Signature verification failure".into(),
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
