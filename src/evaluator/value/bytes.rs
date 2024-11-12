#[derive(Debug, Clone, PartialEq)]
pub struct BytesValue {
  pub value: Vec<u8>,
  pub position: usize,
}

impl BytesValue {
  pub fn new(value: Vec<u8>) -> Self {
    Self { value, position: 0 }
  }

  pub fn is_empty(&self) -> bool {
    self.value.is_empty()
  }

  pub fn is_eq(&self, value: &BytesValue) -> bool {
    self.value == value.value
  }
}
