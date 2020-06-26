use std::error;
use std::fmt;
use crate::ErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {

    pub fn new<M>(kind: ErrorKind, message: M) -> Self
        where
        M: Into<String>,
    {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exit_code(&self) -> i32 {
        match self.kind { // [64 - 113]
            ErrorKind::UnknownCommand => 65,
            ErrorKind::UnknownFlag => 66,
            ErrorKind::MissingFlagValue => 67,
            ErrorKind::InvalidFlagValue => 69,
            ErrorKind::MissingResolver => 68,
            ErrorKind::CommandFailed => 1,
        }
    }
}

impl error::Error for Error {

    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
