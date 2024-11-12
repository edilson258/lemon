#![allow(dead_code)]
use std::{collections::HashMap, io::Bytes};

use crate::ast;

use super::{ctx::Ctx, native::NativeFn};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Null(NullValue),
  Num(NumValue),
  String(StringValue),
  Bool(BoolValue),
  Fn(FnValue),
  Array(ArrayValue),
  Object(ObjectValue),
  NativeFn(NativeFnValue),
  Buffer(BufferValue),
  Bytes(BytesValue),
  Stream(StreamValue),
}

impl Value {
  pub fn is_null(&self) -> bool {
    matches!(self, Value::Null(_))
  }
  pub fn is_number(&self) -> bool {
    matches!(self, Value::Num(_))
  }

  pub fn is_buffer(&self) -> bool {
    matches!(self, Value::Buffer(_))
  }

  pub fn is_bytes(&self) -> bool {
    matches!(self, Value::Bytes(_))
  }

  pub fn is_stream(&self) -> bool {
    matches!(self, Value::Stream(_))
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
      (Value::Buffer(lt), Value::Buffer(rt)) => lt.is_eq(rt),
      (Value::Bytes(lt), Value::Bytes(rt)) => lt.is_eq(rt),
      (Value::Stream(lt), Value::Stream(rt)) => lt.is_eq(rt),
      (Value::NativeFn(lt), Value::NativeFn(rt)) => lt.is_eq(rt),
      _ => false,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFnValue {
  pub name: &'static str,
  pub native: NativeFn,
}

impl NativeFnValue {
  pub fn new(name: &'static str, native: NativeFn) -> Self {
    Self { name, native }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_native(&self) -> &NativeFn {
    &self.native
  }

  pub fn is_eq(&self, other: &NativeFnValue) -> bool {
    self.name == other.name && self.native.eq(&other.native)
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
  pub value: HashMap<String, Value>,
}

impl ObjectValue {
  pub fn new() -> Self {
    Self { value: HashMap::new() }
  }
  pub fn get(&self, key: &str) -> Option<Value> {
    self.value.get(key).map(|v| v.to_owned())
  }
  pub fn set(&mut self, key: String, value: Value) {
    self.value.insert(key, value);
  }
  pub fn values(&self) -> Vec<Value> {
    self.value.iter().map(|(_, v)| v.clone()).collect()
  }

  pub fn is_eq(&self, value: &ObjectValue) -> bool {
    self.value.iter().zip(value.value.iter()).all(|(lt, rt)| lt.0 == rt.0 && lt.1.is_eq(&rt.1))
  }

  pub fn with_native(natives: &HashMap<&'static str, NativeFnValue>) -> Self {
    let mut obj = ObjectValue { value: HashMap::with_capacity(natives.len()) };
    for (key, native) in natives {
      obj.value.insert(key.to_string(), Value::NativeFn(native.clone()));
    }
    obj
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

  pub fn create_self(&mut self, value: Value) {
    self.ctx.set("self".to_owned(), value);
  }

  pub fn set_value(&mut self, key: &str, value: Value) {
    self.ctx.set(key.to_owned(), value);
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

pub fn create_object(value: HashMap<String, Value>) -> Value {
  Value::Object(ObjectValue { value })
}

pub fn create_buffer(value: Vec<u8>) -> Value {
  Value::Buffer(BufferValue { value })
}

pub fn create_bytes(value: Vec<u8>) -> Value {
  Value::Bytes(BytesValue::new(value))
}

pub fn create_stream() -> Value {
  Value::Stream(StreamValue::new())
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferValue {
  pub value: Vec<u8>,
}

impl BufferValue {
  pub fn new() -> Self {
    Self { value: Vec::new() }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { value: Vec::with_capacity(capacity) }
  }

  pub fn from_vec(value: Vec<u8>) -> Self {
    Self { value }
  }

  pub fn len(&self) -> usize {
    self.value.len()
  }

  pub fn is_empty(&self) -> bool {
    self.value.is_empty()
  }

  pub fn capacity(&self) -> usize {
    self.value.capacity()
  }

  pub fn clear(&mut self) {
    self.value.clear();
  }

  pub fn extend_from_slice(&mut self, other: &[u8]) {
    self.value.extend_from_slice(other);
  }

  pub fn as_slice(&self) -> &[u8] {
    &self.value
  }

  pub fn to_vec(&self) -> Vec<u8> {
    self.value.clone()
  }

  pub fn is_eq(&self, other: &BufferValue) -> bool {
    self.value == other.value
  }
}

// BytesValue para iteração sobre bytes
#[derive(Debug, Clone, PartialEq)]
pub struct BytesValue {
  pub value: Vec<u8>,
  position: usize,
}

impl BytesValue {
  pub fn new(value: Vec<u8>) -> Self {
    Self { value, position: 0 }
  }

  pub fn next(&mut self) -> Option<u8> {
    if self.position < self.value.len() {
      let byte = self.value[self.position];
      self.position += 1;
      Some(byte)
    } else {
      None
    }
  }

  pub fn reset(&mut self) {
    self.position = 0;
  }

  pub fn position(&self) -> usize {
    self.position
  }

  pub fn is_eq(&self, other: &BytesValue) -> bool {
    self.value == other.value
  }
}

// StreamValue para leitura de streams
#[derive(Debug, Clone, PartialEq)]
pub struct StreamValue {
  pub buffer: Vec<u8>,
  pub position: usize,
}

impl StreamValue {
  pub fn new() -> Self {
    Self { buffer: Vec::new(), position: 0 }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { buffer: Vec::with_capacity(capacity), position: 0 }
  }

  pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    let available = self.buffer.len() - self.position;
    let to_read = std::cmp::min(available, buf.len());

    if to_read == 0 {
      return Ok(0);
    }

    buf[..to_read].copy_from_slice(&self.buffer[self.position..self.position + to_read]);
    self.position += to_read;
    Ok(to_read)
  }

  pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.buffer.extend_from_slice(buf);
    Ok(buf.len())
  }

  pub fn is_eq(&self, other: &StreamValue) -> bool {
    self.buffer == other.buffer
  }
}
