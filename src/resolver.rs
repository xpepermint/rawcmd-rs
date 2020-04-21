use crate::intent::Intent;

pub type Resolver = fn(Intent) -> Result<usize, usize>;
