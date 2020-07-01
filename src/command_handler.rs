use crate::{Error, Result};

pub type CommandHandler<C> = fn(err: Error, context: &mut C) -> Result<i32>;
