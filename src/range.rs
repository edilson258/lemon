#![allow(dead_code)]
use logos::Span;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Range {
  // set as u32?
  pub start: usize,
  pub end: usize,
}

impl Range {
  pub fn new(start: usize, end: usize) -> Range {
    Range { start, end }
  }

  pub fn merge(&mut self, range: &Range) {
    self.start = min(self.start, range.start);
    self.end = max(self.end, range.end);
  }

  pub fn merged_with(&self, range: &Range) -> Range {
    Range::new(min(self.start, range.start), max(self.end, range.end))
  }

  pub fn from_span(span: Span) -> Range {
    Range::new(span.start, span.end)
  }
}

// trait to acess range

pub trait TraitRange {
  fn range(&self) -> Range;
}
