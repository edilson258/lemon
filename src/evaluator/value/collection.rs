// pub use collection::{ArrayValue, ObjectValue};

use std::collections::HashMap;

use super::{NativeFnValue, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayValue {
  pub value: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectValue {
  pub value: HashMap<String, Value>,
}

impl ArrayValue {
  pub fn new(value: Vec<Value>) -> Self {
    Self { value }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { value: Vec::with_capacity(capacity) }
  }

  pub fn extend(&mut self, value: &ArrayValue) {
    self.value.extend(value.value.iter().cloned());
  }
  pub fn get(&self, index: usize) -> Option<&Value> {
    self.value.get(index)
  }

  pub fn set(&mut self, index: usize, value: Value) {
    self.value[index] = value;
  }

  pub fn len(&self) -> usize {
    self.value.len()
  }

  pub fn pop(&mut self) -> Option<Value> {
    self.value.pop()
  }

  pub fn push(&mut self, value: Value) {
    self.value.push(value);
  }

  pub fn is_eq(&self, value: &ArrayValue) -> bool {
    self.value.len() == value.value.len() && self.value.iter().zip(value.value.iter()).all(|(lt, rt)| lt.is_eq(rt))
  }
}

impl ObjectValue {
  pub fn new() -> Self {
    Self { value: HashMap::new() }
  }

  pub fn with_native(natives: &HashMap<&'static str, NativeFnValue>) -> Self {
    let mut value: HashMap<String, Value> = HashMap::new();
    for (key, native) in natives.iter() {
      value.insert(key.to_string(), Value::NativeFn(native.clone()));
    }
    Self { value }
  }

  pub fn new_with_map(value: HashMap<String, Value>) -> Self {
    Self { value }
  }
  pub fn with_capacity(capacity: usize) -> Self {
    Self { value: HashMap::with_capacity(capacity) }
  }

  pub fn extend(&mut self, value: &ObjectValue) {
    self.value.extend(value.value.iter().map(|(k, v)| (k.to_owned(), v.to_owned())));
  }
  pub fn get(&self, key: &str) -> Option<Value> {
    self.value.get(key).map(|v| v.to_owned())
  }

  pub fn set(&mut self, key: &str, value: Value) {
    self.value.insert(key.to_owned(), value);
  }
  pub fn values(&self) -> Vec<Value> {
    self.value.iter().map(|(_, v)| v.clone()).collect()
  }

  pub fn is_eq(&self, value: &ObjectValue) -> bool {
    self.value.iter().zip(value.value.iter()).all(|(lt, rt)| lt.0 == rt.0 && lt.1.is_eq(&rt.1))
  }
}
