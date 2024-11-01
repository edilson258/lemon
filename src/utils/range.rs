#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}

impl Range {
  pub fn new(start: usize, end: usize) -> Range {
    Range { start, end }
  }
  pub fn merge(&mut self, range: &Range) {
    if range.start < self.start {
      self.start = range.start;
    }
    if range.end > self.end {
      self.end = range.end;
    }
  }
}
