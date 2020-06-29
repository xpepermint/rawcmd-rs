#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    GeneralError,
    UnknownCommand(String),
    MissingCommandResolver(String),
    CommandFailed(String), // prepared
    UnknownFlag(String),
    MissingFlagValue(String),
    InvalidFlagValue(String), // prepared
    InvalidParamValue(usize,), // prepared
    ToManyParams(usize, usize),
}
