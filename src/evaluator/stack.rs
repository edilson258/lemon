#![allow(dead_code)]

use std::path::PathBuf;

use crate::{diag::Diag, range::Range};

use super::value::Value;

const STACK_SIZE: usize = 999;
pub struct CallStack {
  frames: Vec<Vec<Value>>,
  path: PathBuf,
}

pub struct StackTrace {
  pub paths: Vec<PathBuf>,
}

impl CallStack {
  pub fn new(path: PathBuf) -> Self {
    Self { frames: Vec::with_capacity(STACK_SIZE), path }
  }

  pub fn push_frame(&mut self, range: Range) -> Result<(), Diag> {
    if self.overflow() {
      return Err(Diag::create_err("stack overflow in recursive call".to_owned(), range, self.path.clone()));
    }
    self.frames.push(Vec::new());
    Ok(())
  }

  pub fn pop_frame(&mut self) {
    self.frames.pop();
  }

  pub fn push(&mut self, value: Value) {
    if let Some(frame) = self.frames.last_mut() {
      frame.push(value);
    }
  }

  pub fn pop(&mut self) -> Option<Value> {
    self.frames.last_mut().and_then(|frame| frame.pop())
  }

  pub fn get_last_value(&self) -> Option<&Value> {
    self.frames.last().and_then(|frame| frame.last())
  }
  pub fn overflow(&self) -> bool {
    self.frames.len() >= STACK_SIZE
  }
}
