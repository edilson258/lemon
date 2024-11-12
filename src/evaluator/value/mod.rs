#![allow(dead_code)]
mod buffer;
mod bytes;
mod collection;
mod function;
mod primitive;
mod stream;

pub use buffer::BufferValue;
pub use bytes::BytesValue;
pub use collection::{ArrayValue, ObjectValue};
pub use function::{FnValue, NativeFnValue};
pub use primitive::{BoolValue, NullValue, NumValue, StringValue};
// pub use stream::StreamValue;

use std::collections::HashMap;

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
  // Stream(StreamValue),
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
  pub fn is_fn(&self) -> bool {
    matches!(self, Value::Fn(_))
  }
  pub fn is_array(&self) -> bool {
    matches!(self, Value::Array(_))
  }
  pub fn is_object(&self) -> bool {
    matches!(self, Value::Object(_))
  }
  pub fn is_buffer(&self) -> bool {
    matches!(self, Value::Buffer(_))
  }
  pub fn is_bytes(&self) -> bool {
    matches!(self, Value::Bytes(_))
  }

  // pub fn is_stream(&self) -> bool {
  //   matches!(self, Value::Stream(_))
  // }

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
      // (Value::Stream(lt), Value::Stream(rt)) => lt.is_eq(rt),
      (Value::NativeFn(lt), Value::NativeFn(rt)) => lt.is_eq(rt),
      _ => false,
    }
  }
}

pub mod value_factory {
  use super::*;

  pub fn create_null() -> Value {
    Value::Null(NullValue)
  }
  pub fn create_num(value: f64) -> Value {
    Value::Num(NumValue::new(value))
  }
  pub fn create_string(value: String) -> Value {
    Value::String(StringValue::new(value))
  }
  pub fn create_bool(value: bool) -> Value {
    Value::Bool(BoolValue::new(value))
  }
  pub fn create_array(value: Vec<Value>) -> Value {
    Value::Array(ArrayValue::new(value))
  }
  pub fn create_object(value: HashMap<String, Value>) -> Value {
    Value::Object(ObjectValue::new_with_map(value))
  }
  pub fn create_buffer(value: Vec<u8>) -> Value {
    Value::Buffer(BufferValue::from_vec(value))
  }
  pub fn create_bytes(value: Vec<u8>) -> Value {
    Value::Bytes(BytesValue::new(value))
  }
  // pub fn create_stream() -> Value {
  //   Value::Stream(StreamValue::new())
  // }
}
