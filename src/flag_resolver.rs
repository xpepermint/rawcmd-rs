use crate::Result;

pub type FlagResolver = fn(input: Option<String>) -> Result<Option<String>>;
