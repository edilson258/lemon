#[derive(Debug, Clone, PartialEq)]
pub struct NullValue;

impl NullValue {}
#[derive(Debug, Clone, PartialEq)]
pub struct NumValue {
  pub value: f64,
}

impl NumValue {
  pub fn new(value: f64) -> Self {
    Self { value }
  }

  pub fn get(&self) -> f64 {
    self.value
  }

  pub fn set(&mut self, value: f64) {
    self.value = value;
  }

  pub fn to_string(&self) -> String {
    self.value.to_string()
  }

  pub fn to_bool(&self) -> bool {
    self.value != 0.0
  }

  pub fn to_usize(&self) -> usize {
    self.value as usize
  }

  pub fn to_int(&self) -> i64 {
    self.value as i64
  }

  pub fn is_neg(&self) -> bool {
    self.value < 0.0
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue {
  pub value: String,
}

impl StringValue {
  pub fn new(value: String) -> Self {
    Self { value }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { value: String::with_capacity(capacity) }
  }
  pub fn get(&self) -> &str {
    self.value.as_str()
  }
  pub fn set(&mut self, value: String) {
    self.value = value;
  }

  pub fn len(&self) -> usize {
    self.value.len()
  }

  pub fn chars(&self) -> Vec<char> {
    self.value.chars().collect()
  }

  pub fn push(&mut self, value: char) {
    self.value.push(value);
  }

  pub fn push_str(&mut self, value: &str) {
    self.value.push_str(value);
  }

  pub fn pop(&mut self) -> Option<char> {
    self.value.pop()
  }

  pub fn is_empty(&self) -> bool {
    self.value.is_empty()
  }

  pub fn as_bytes(&self) -> Vec<u8> {
    self.value.as_bytes().to_vec()
  }

  pub fn is_eq(&self, value: &StringValue) -> bool {
    self.value == value.value
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolValue {
  pub value: bool,
}

impl BoolValue {
  pub fn new(value: bool) -> Self {
    Self { value }
  }
  pub fn get(&self) -> bool {
    self.value
  }

  pub fn set(&mut self, value: bool) {
    self.value = value;
  }
  pub fn not(&self) -> Self {
    Self { value: !self.value }
  }
  pub fn and(&self, other: &Self) -> Self {
    Self { value: self.value && other.value }
  }
  pub fn or(&self, other: &Self) -> Self {
    Self { value: self.value || other.value }
  }
  pub fn xor(&self, other: &Self) -> Self {
    Self { value: self.value ^ other.value }
  }
}
