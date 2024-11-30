#![allow(dead_code)]
use std::collections::HashMap;

use super::value::{ObjectValue, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Ctx {
  pub hash: HashMap<String, Value>,
  pub parent: Option<Box<Ctx>>,
  pub this: Option<Box<Value>>,
}

impl Ctx {
  pub fn new(parent: Option<Box<Ctx>>) -> Self {
    let this = None;
    Self { hash: HashMap::new(), parent, this }
  }
  pub fn get(&self, key: &str) -> Option<&Value> {
    if let Some(value) = self.hash.get(key) {
      return Some(value);
    }
    self.parent.as_ref().and_then(|parent| parent.get(key))
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
    if let Some(value) = self.hash.get_mut(key) {
      return Some(value);
    }
    self.parent.as_mut().and_then(|parent| parent.get_mut(key))
  }

  pub fn set(&mut self, key: String, value: Value) {
    self.hash.insert(key, value);
  }

  pub fn set_parent(&mut self, parent: Box<Ctx>) {
    self.parent = Some(parent);
  }

  pub fn get_parent(&self) -> Option<&Box<Ctx>> {
    self.parent.as_ref()
  }

  pub fn update(&mut self, key: String, curr: Value) {
    if let Some(value) = self.hash.get_mut(&key) {
      *value = curr;
    }
    self.parent.as_mut().and_then(|parent| parent.get_mut(&key));
  }

  pub fn to_object(&self) -> ObjectValue {
    let value = self.hash.clone();
    ObjectValue { value }
  }

  pub fn create_ctx_object(&mut self, value: HashMap<String, Value>) -> Ctx {
    let mut ctx = Ctx::new(Some(Box::new(self.clone())));
    for (key, value) in value.iter() {
      ctx.set(key.to_owned(), value.to_owned());
    }
    ctx
  }

  pub fn set_self(&mut self, value: Value) {
    self.this = Some(Box::new(value));
  }
  pub fn get_self_ctx(&mut self) -> Option<&mut Value> {
    match &mut self.this {
      Some(value) => Some(value),
      None => None,
    }
  }
}
