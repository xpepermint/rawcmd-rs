use std::error;
use std::fmt;
use crate::ErrorKind;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    status: i32,
    source: Option<Box<dyn error::Error + 'static>>,
}

impl Error {

    pub fn new(kind: ErrorKind) -> Self {
        Self {
            message: error_message(&kind),
            status: error_status(&kind),
            kind,
            source: None,
        }
    }

    pub fn with_source<E>(source: E, kind: ErrorKind) -> Self
        where
        E: std::error::Error + 'static,
    {
        Self {
            message: error_message(&kind),
            status: error_status(&kind),
            kind,
            source: Some(Box::new(source)),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn status(&self) -> &i32 {
        &self.status
    }

    pub fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl std::default::Default for Error {
    fn default() -> Self {
        Self {
            message: error_message(&ErrorKind::GeneralError),
            status: error_status(&ErrorKind::GeneralError),
            kind: ErrorKind::GeneralError,
            source: None,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

fn error_message(kind: &ErrorKind) -> String {
    match kind {
        ErrorKind::GeneralError => format!("Unknown error occurred while processing."),
        ErrorKind::UnknownCommand(name) => format!("The requested command `{}` does not exist.", name),
        ErrorKind::UnknownFlag(name) => format!("The provided flag `{}` does not exist.", name),
        ErrorKind::MissingCommandResolver(name) => format!("The requested command `{}` does not have a resolver.", name),
        ErrorKind::MissingFlagValue(name) => format!("The provided flag `{}` should have a value.", name),
        ErrorKind::InvalidFlagValue(name) => format!("The provided flag `{}` has invalid value.", name),
        ErrorKind::InvalidParamValue(index) => format!("The provided param `{}` has invalid value.", index),
        ErrorKind::ToManyParameters(expected, found) => format!("Expected `{}` parameters, found {}.", expected, found),
        ErrorKind::CommandFailed(name) => format!("The requested command `{}` failed to execute.", name),
    }
}

fn error_status(kind: &ErrorKind) -> i32 {
    match kind { // [64 - 113]
        ErrorKind::GeneralError => 1,
        ErrorKind::UnknownCommand(_) => 65,
        ErrorKind::UnknownFlag(_) => 66,
        ErrorKind::MissingCommandResolver(_) => 68,
        ErrorKind::MissingFlagValue(_) => 69,
        ErrorKind::InvalidFlagValue(_) => 70,
        ErrorKind::InvalidParamValue(_) => 71,
        ErrorKind::ToManyParameters(_, _) => 67,
        ErrorKind::CommandFailed(_) => 72,
    }
}
