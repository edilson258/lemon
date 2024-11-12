#![allow(dead_code)]

use std::io::{self, Write};
use std::{collections::HashMap, path::PathBuf};

use crate::evaluator::formatting::display_value;
use crate::evaluator::value::{value_factory, NativeFnValue, Value};
use crate::{diag::Diag, evaluator::errors, range::Range};

use super::NativeModule;

pub fn create_module() -> NativeModule {
  let mut native: NativeModule = HashMap::new();
  native.insert("write_bytes", NativeFnValue::new("write_bytes", write_bytes));
  native.insert("read_bytes", NativeFnValue::new("read_bytes", read_bytes));
  native.insert("write_stdout", NativeFnValue::new("write_stdout", write_stdout));
  native.insert("read_stdin", NativeFnValue::new("read_stdin", read_stdin));
  native
}

fn write_bytes(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 2 {
    let msg = errors::format_function_arity_mismatch(2, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  let file_path = match &args[0] {
    Value::String(string_value) => string_value,
    _ => {
      let msg = errors::format_mismatched_types("string", &args[0]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  let bytes = match &args[1] {
    Value::String(string_value) => string_value.as_bytes(),
    Value::Bytes(bytes) => bytes.value.clone(),
    _ => {
      let msg = errors::format_mismatched_types("string", &args[1]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };
  match std::fs::write(path.join(file_path.get()), bytes) {
    Ok(_) => Ok(value_factory::create_null()),
    Err(e) => {
      let msg = errors::format_error(e);
      Err(Diag::create_err(msg, range.clone(), path.clone()))
    }
  }
}

fn read_bytes(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 1 {
    let msg = errors::format_function_arity_mismatch(1, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  let file_path = match &args[0] {
    Value::String(string_value) => string_value,
    _ => {
      let msg = errors::format_mismatched_types("string", &args[0]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };
  match std::fs::read(path.join(file_path.get())) {
    Ok(bytes) => Ok(value_factory::create_bytes(bytes)),
    Err(e) => {
      let msg = errors::format_error(e);
      Err(Diag::create_err(msg, range.clone(), path.clone()))
    }
  }
}

fn write_stdout(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 1 {
    let msg = errors::format_function_arity_mismatch(1, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  let stdout = io::stdout();
  let mut handle = stdout.lock();

  let output = match &args[0] {
    Value::String(string_value) => string_value.get().to_string(),
    _ => display_value(&args[0]).to_string(),
  };
  match handle.write_all(output.as_bytes()) {
    Ok(_) => {}
    Err(e) => {
      let msg = errors::format_error(e);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  match handle.flush() {
    Ok(_) => return Ok(value_factory::create_null()),
    Err(e) => {
      let msg = errors::format_error(e);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  }
}

fn read_stdin(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if !args.is_empty() {
    let msg = format!("no expected args, found {}", args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  let mut buffer = String::new();
  match std::io::stdin().read_line(&mut buffer) {
    Ok(_) => Ok(value_factory::create_string(buffer)),
    Err(e) => {
      let msg = errors::format_error(e);
      Err(Diag::create_err(msg, range.clone(), path.clone()))
    }
  }
}
