use crate::{Intent, Result};

pub type CommandResolver<C> = fn(intent: &Intent, context: &mut C) -> Result<i32>;
