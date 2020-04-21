use crate::intent::Intent;

pub trait Resolver {
  fn resolve(&self, intent: Intent) -> Result<usize, usize>;
}

impl<F: Fn(Intent) -> Result<usize, usize>> Resolver for F {
  fn resolve(&self, intent: Intent) -> Result<usize, usize> {
      self(intent)
  }
}
