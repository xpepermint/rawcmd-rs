use crate::intent::Intent;

pub type CommandResolver = fn(Intent) -> Result<usize, usize>;
