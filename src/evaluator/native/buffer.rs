#![allow(dead_code)]

use std::{collections::HashMap, path::PathBuf};

use crate::evaluator::formatting::display_value;
use crate::evaluator::value::{value_factory, BufferValue, NativeFnValue, Value};
use crate::{diag::Diag, evaluator::errors, range::Range};

use super::NativeModule;

const DEFAULT_BUFFER_SIZE: usize = 1024; // 1KB default size

pub fn create_module() -> NativeModule {
  let mut module: NativeModule = HashMap::new();
  module.insert("create", NativeFnValue::new("create", create_buffer));
  module.insert("write", NativeFnValue::new("write", buffer_write));
  module.insert("read", NativeFnValue::new("read", buffer_read));
  module.insert("to_string", NativeFnValue::new("to_string", buffer_to_string));
  module
}

fn validate_buffer(value: &Value) -> Result<&BufferValue, String> {
  match value {
    Value::Buffer(buf) => Ok(buf),
    _ => Err(format!("expected buffer, found {}", display_value(value))),
  }
}

fn validate_number(value: &Value, param_name: &str) -> Result<usize, String> {
  match value {
    Value::Num(num) if !num.is_neg() => Ok(num.to_usize()),
    Value::Num(_) => Err(format!("{} must be non-negative", param_name)),
    _ => Err(format!("{} must be a number", param_name)),
  }
}

fn create_buffer(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() > 1 {
    return Err(Diag::create_err(errors::format_function_arity_mismatch(1, args.len()), range.clone(), path.clone()));
  }

  let size = match args.get(0) {
    Some(Value::Num(num)) if !num.is_neg() => num.to_usize(),
    Some(Value::Num(_)) => {
      return Err(Diag::create_err("buffer size must be non-negative".to_string(), range.clone(), path.clone()))
    }
    None => DEFAULT_BUFFER_SIZE,
    _ => return Err(Diag::create_err("buffer size must be a number".to_string(), range.clone(), path.clone())),
  };

  Ok(Value::Buffer(BufferValue::with_capacity(size)))
}

fn buffer_write(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 2 {
    return Err(Diag::create_err(errors::format_function_arity_mismatch(2, args.len()), range.clone(), path.clone()));
  }

  let buffer = validate_buffer(&args[0]).map_err(|e| Diag::create_err(e, range.clone(), path.clone()))?;

  let new_buffer = match &args[1] {
    Value::String(s) => {
      let mut buf = buffer.clone();
      buf.extend_from_slice(&s.as_bytes());
      buf
    }
    Value::Buffer(src_buffer) => {
      let mut buf = buffer.clone();
      buf.extend_from_slice(&src_buffer.get());
      buf
    }
    _ => {
      let msg = errors::format_mismatched_types("string or buffer", &args[1]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  Ok(Value::Buffer(new_buffer))
}

fn buffer_read(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.is_empty() || args.len() > 3 {
    return Err(Diag::create_err(format!("expected 1-3 args, found {}", args.len()), range.clone(), path.clone()));
  }

  let buffer = validate_buffer(&args[0]).map_err(|e| Diag::create_err(e, range.clone(), path.clone()))?;

  let start = if let Some(value) = args.get(1) {
    validate_number(value, "offset").map_err(|e| Diag::create_err(e, range.clone(), path.clone()))?
  } else {
    0
  };

  let length = if let Some(value) = args.get(2) {
    validate_number(value, "length").map_err(|e| Diag::create_err(e, range.clone(), path.clone()))?
  } else {
    buffer.len().saturating_sub(start)
  };

  if start >= buffer.len() {
    return Err(Diag::create_err("read offset out of bounds".to_string(), range.clone(), path.clone()));
  }

  let end = start.saturating_add(length).min(buffer.len());
  Ok(Value::Buffer(buffer.with_slice(start, end)))
}

fn buffer_to_string(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 1 {
    return Err(Diag::create_err(format!("expected 1 arg, found {}", args.len()), range.clone(), path.clone()));
  }

  let buffer = validate_buffer(&args[0]).map_err(|e| Diag::create_err(e, range.clone(), path.clone()))?;

  String::from_utf8(buffer.get().clone())
    .map(value_factory::create_string)
    .map_err(|_| Diag::create_err("buffer contains invalid UTF-8 data".to_string(), range.clone(), path.clone()))
}
