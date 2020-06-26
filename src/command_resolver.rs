use crate::{Intent, Result};

pub type CommandResolver = fn(Intent) -> Result<usize>;
