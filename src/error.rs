use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct StringError {
    what: String
}

impl StringError {
    pub fn new(msg: &str) -> StringError {
        let msg_owned = msg.to_string().clone();
        StringError { what: msg_owned }
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        self.what.as_str()
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no: {}", self.what)
    }
}
