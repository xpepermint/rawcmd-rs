use crate::Result;

pub type ParamResolver = fn(input: Option<String>) -> Result<Option<String>>;
