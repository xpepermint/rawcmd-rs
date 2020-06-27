#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    General,
    UnknownCommand(String),
    MissingCommandResolver(String),
    UnknownFlag(String),
    MissingFlagValue(String),
    InvalidFlagValue(String),
}
