#![allow(dead_code)]
use crate::ast;

use super::ctx::Ctx;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Null(NullValue),
  Num(NumValue),
  String(StringValue),
  Bool(BoolValue),
  Fn(FnValue),
  Array(ArrayValue),
  Object(ObjectValue),
}

impl Value {
  pub fn is_null(&self) -> bool {
    matches!(self, Value::Null(_))
  }
  pub fn is_number(&self) -> bool {
    matches!(self, Value::Num(_))
  }

  pub fn is_string(&self) -> bool {
    matches!(self, Value::String(_))
  }

  pub fn is_bool(&self) -> bool {
    matches!(self, Value::Bool(_))
  }

  pub fn is_array(&self) -> bool {
    matches!(self, Value::Array(_))
  }

  pub fn is_object(&self) -> bool {
    matches!(self, Value::Object(_))
  }

  pub fn is_fn(&self) -> bool {
    matches!(self, Value::Fn(_))
  }

  pub fn is_eq(&self, value: &Value) -> bool {
    match (self, value) {
      (Value::Num(lt), Value::Num(rt)) => lt.get() == rt.get(),
      (Value::String(lt), Value::String(rt)) => lt.get() == rt.get(),
      (Value::Bool(lt), Value::Bool(rt)) => lt.get() == rt.get(),
      (Value::Null(_), Value::Null(_)) => true,
      (Value::Array(lt), Value::Array(rt)) => lt.is_eq(rt),
      (Value::Object(lt), Value::Object(rt)) => lt.is_eq(rt),
      _ => false,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullValue {}

#[derive(Debug, Clone, PartialEq)]
pub struct NumValue {
  pub value: f64,
}

impl NumValue {
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

  pub fn to_int(&self) -> i64 {
    self.value as i64
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue {
  pub value: String,
}

impl StringValue {
  pub fn get(&self) -> &String {
    &self.value
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

  pub fn pop(&mut self) -> Option<char> {
    self.value.pop()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolValue {
  pub value: bool,
}

impl BoolValue {
  pub fn get(&self) -> bool {
    self.value
  }

  pub fn set(&mut self, value: bool) {
    self.value = value;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayValue {
  pub value: Vec<Value>,
}

impl ArrayValue {
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

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectValue {
  pub value: Vec<(String, Value)>,
}

impl ObjectValue {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.value.iter().find(|(k, _)| k == key).map(|(_, v)| v)
  }

  pub fn set(&mut self, key: String, value: Value) {
    self.value.push((key, value));
  }

  pub fn has(&self, key: &str) -> bool {
    self.value.iter().any(|(k, _)| k == key)
  }

  pub fn keys(&self) -> Vec<String> {
    self.value.iter().map(|(k, _)| k.clone()).collect()
  }

  pub fn values(&self) -> Vec<Value> {
    self.value.iter().map(|(_, v)| v.clone()).collect()
  }

  pub fn is_eq(&self, value: &ObjectValue) -> bool {
    self.value.iter().zip(value.value.iter()).all(|(lt, rt)| lt.0 == rt.0 && lt.1.is_eq(&rt.1))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnValue {
  pub ctx: Ctx,
  pub pats: Vec<String>,
  pub stmt: Box<ast::Stmts>,
}

impl FnValue {
  pub fn new(ctx: Ctx, pats: Vec<String>, stmt: Box<ast::Stmts>) -> Self {
    Self { ctx, pats, stmt }
  }
}

pub fn create_null() -> Value {
  Value::Null(NullValue {})
}

pub fn create_num(value: f64) -> Value {
  Value::Num(NumValue { value })
}

pub fn create_string(value: String) -> Value {
  Value::String(StringValue { value })
}

pub fn create_bool(value: bool) -> Value {
  Value::Bool(BoolValue { value })
}

pub fn create_array(value: Vec<Value>) -> Value {
  Value::Array(ArrayValue { value })
}

pub fn create_object(value: Vec<(String, Value)>) -> Value {
  Value::Object(ObjectValue { value })
}
