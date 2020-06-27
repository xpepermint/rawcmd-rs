use std::error;
use std::fmt;
use crate::ErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    exit_code: i32,
}

impl Error {

    pub fn new(kind: ErrorKind) -> Self {
        Self {
            message: error_message(&kind),
            exit_code: error_exit_code(&kind),
            kind,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn exit_code(&self) -> &i32 {
        &self.exit_code
    }
}

impl std::default::Default for Error {

    fn default() -> Self {
        Self {
            message: error_message(&ErrorKind::General),
            exit_code: error_exit_code(&ErrorKind::General),
            kind: ErrorKind::General,
        }
    }
}

impl error::Error for Error {

    fn description(&self) -> &str {
        &self.message()
    }

    fn source(&self) -> Option<&'static dyn std::error::Error> {
        None
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

fn error_message(kind: &ErrorKind) -> String {
    match kind {
        ErrorKind::General => format!("Unknown error occurred while processing."),
        ErrorKind::UnknownCommand(name) => format!("The requested command \"{}\" does not exist.", name),
        ErrorKind::MissingCommandResolver(name) => format!("The requested command \"{}\" does not have has a resolver.", name),
        ErrorKind::UnknownFlag(name) => format!("The provided flag \"{}\" does not exist.", name),
        ErrorKind::MissingFlagValue(name) => format!("The provided flag \"{}\" should have a value.", name),
        ErrorKind::InvalidFlagValue(name) => format!("The provided flag \"{}\" has invalid value.", name),
    }
}

fn error_exit_code(kind: &ErrorKind) -> i32 {
    match kind { // [64 - 113]
        ErrorKind::General => 1,
        ErrorKind::UnknownCommand(_) => 65,
        ErrorKind::UnknownFlag(_) => 66,
        ErrorKind::MissingFlagValue(_) => 67,
        ErrorKind::InvalidFlagValue(_) => 69,
        ErrorKind::MissingCommandResolver(_) => 68,
    }
}
