use crate::{Intent, Result};

pub type CommandResolver<C> = fn(intent: Intent, context: C) -> Result<usize>;
