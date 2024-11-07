use std::collections::HashMap;

use crate::{diag::Diag, range::Range};

use super::value::Value;

const STACK_SIZE: usize = 999;
pub struct CallStack {
  frames: Vec<HashMap<String, Value>>,
  pointers: Vec<usize>,
}

impl CallStack {
  pub fn new() -> Self {
    Self { frames: Vec::new(), pointers: Vec::new() }
  }

  pub fn push_frame(&mut self, range: Range) -> Result<(), Diag> {
    if self.overflow() {
      return Err(Diag::create_err("stack overflow in recursive call".to_owned(), range));
    }
    self.frames.push(HashMap::new());
    self.pointers.push(self.frames.len() - 1);
    Ok(())
  }

  pub fn pop_frame(&mut self) {
    if let Some(frame_index) = self.pointers.pop() {
      self.frames.truncate(frame_index);
    }
  }

  pub fn push(&mut self, value: Value) {
    if let Some(frame) = self.frames.last_mut() {
      match &value {
        Value::Fn(fn_value) => {
          // todo: check if name is already in the frame
          frame.insert(fn_value.name.clone().unwrap(), value);
        }
        _ => {}
      }
    }
  }

  pub fn get(&self, name: &str) -> Option<&Value> {
    for frame in self.frames.iter().rev() {
      return frame.get(name);
    }
    return None;
  }
  pub fn overflow(&self) -> bool {
    self.pointers.len() > STACK_SIZE
  }
}
