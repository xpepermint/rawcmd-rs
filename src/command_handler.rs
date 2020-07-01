use crate::{Error, Intent, Result};

pub type CommandHandler<C> = fn(err: Error, intent: &Intent, context: &mut C) -> Result<usize>;
