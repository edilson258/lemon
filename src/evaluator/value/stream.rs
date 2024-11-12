#[derive(Debug, Clone, PartialEq)]
pub struct StreamValue {
  pub buffer: Vec<u8>,
  pub position: usize,
}

impl StreamValue {
  pub fn new() -> Self {
    Self { buffer: Vec::new(), position: 0 }
  }
}
