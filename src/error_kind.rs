#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    GeneralError,
    UnknownCommand(String),
    UnknownFlag(String),
    MissingCommandResolver(String),
    MissingFlagValue(String),
    InvalidFlagValue(String),
    CommandFailed(String),
}
