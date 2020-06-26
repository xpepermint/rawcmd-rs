use crate::Result;

pub type FlagResolver = fn(Option<String>) -> Result<Option<String>>;
