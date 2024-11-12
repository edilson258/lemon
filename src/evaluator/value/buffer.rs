#[derive(Debug, Clone, PartialEq)]
pub struct BufferValue {
  pub value: Vec<u8>,
}

impl BufferValue {
  pub fn new() -> Self {
    Self { value: Vec::new() }
  }

  pub fn get(&self) -> &Vec<u8> {
    &self.value
  }

  pub fn with_slice(&self, start: usize, end: usize) -> Self {
    Self { value: self.value[start..end].to_vec() }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { value: Vec::with_capacity(capacity) }
  }
  pub fn from_vec(value: Vec<u8>) -> Self {
    Self { value }
  }

  pub fn is_empty(&self) -> bool {
    self.value.is_empty()
  }

  pub fn is_eq(&self, value: &BufferValue) -> bool {
    self.value == value.value
  }

  pub fn len(&self) -> usize {
    self.value.len()
  }

  pub fn push(&mut self, value: u8) {
    self.value.push(value);
  }

  pub fn pop(&mut self) -> Option<u8> {
    self.value.pop()
  }

  pub fn as_slice(&self) -> &[u8] {
    self.value.as_slice()
  }

  pub fn extend_from_slice(&mut self, value: &[u8]) {
    self.value.extend_from_slice(value);
  }
}
