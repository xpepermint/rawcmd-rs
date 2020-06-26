#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    CommandFailed,
    UnknownCommand,
    UnknownFlag,
    MissingFlagValue,
    InvalidFlagValue,
    MissingResolver,
}
