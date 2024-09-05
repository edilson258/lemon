use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Source<'s> {
  pub raw: String,
  pub name: &'s str,
  pub len: usize,
}

impl<'s> Source<'s> {
  pub fn new(raw: String, name: &'s str) -> Self {
    Self { len: raw.len(), raw, name }
  }
}
