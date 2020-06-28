#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    GeneralError,
    UnknownCommand(String),
    MissingCommandResolver(String),
    CommandFailed(String),
    UnknownFlag(String),
    MissingFlagValue(String),
    InvalidFlagValue(String),
    InvalidParamValue(usize,),
    ToManyParams(usize, usize),
}
