use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MalformedFileError {
    reason: String,
}

impl MalformedFileError {
    pub fn new<T>(reason: T) -> Self
    where T: ToString
    {
        Self {
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for MalformedFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Malformed file : type({})", self.reason)
    }
}

impl Error for MalformedFileError {}
