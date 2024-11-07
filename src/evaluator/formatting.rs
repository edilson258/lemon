#![allow(dead_code, unused_variables)]
use core::fmt;

use super::value::Value;
use super::value::{ArrayValue, BoolValue, FnValue, NullValue, NumValue, ObjectValue, StringValue};

struct Formatting<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Formatting<F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0(f)
  }
}

pub fn display_value<'a>(v: &'a Value) -> impl fmt::Display + 'a {
  Formatting(move |f| match v {
    Value::Null(NullValue {}) => write!(f, "nil"),
    Value::Num(NumValue { value }) => write!(f, "{}", value),
    Value::String(StringValue { value }) => write!(f, "{}", value),
    Value::Bool(BoolValue { value }) => write!(f, "{}", value),
    Value::Array(ArrayValue { value }) => write!(f, "{}", display_array(v)),
    Value::Object(ObjectValue { value }) => write!(f, "{}", display_object(v)),
    Value::Fn(FnValue { pats, stmt, ctx, name }) => {
      write!(f, "fn{}({}) = ...", name.as_ref().map(|n| format!("{} ", n)).unwrap_or("".to_string()), pats.join(", "))
    }
    Value::StdFn(name, _) => write!(f, "std::{}", name),
  })
}

pub fn display_object<'a>(v: &'a Value) -> impl fmt::Display + 'a {
  Formatting(move |f| match v {
    Value::Object(ObjectValue { value }) => {
      write!(f, "{{")?;
      for (k, v) in value.iter() {
        write!(f, "{}: {}", k, display_value(v))?;
      }
      write!(f, "}}")
    }
    _ => write!(f, "{}", display_value(v)),
  })
}

pub fn display_array<'a>(v: &'a Value) -> impl fmt::Display + 'a {
  Formatting(move |f| match v {
    Value::Array(ArrayValue { value }) => {
      write!(f, "[")?;
      for (i, v) in value.iter().enumerate() {
        write!(f, "{}", display_value(v))?;
        if i != value.len() - 1 {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
    _ => write!(f, "{}", display_value(v)),
  })
}
