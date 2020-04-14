#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
  UnknownCommand = 1,
  UnknownFlag = 2,
  MissingFlagValue = 3,
  MissingResolver = 4,
}
