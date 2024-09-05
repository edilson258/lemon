use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}

impl Range {
  pub fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }
}

pub fn create_range_from(range_start: &Range, range_end: &Range) -> Range {
  Range { start: range_start.start, end: range_end.end }
}
