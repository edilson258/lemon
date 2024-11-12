#![allow(dead_code)]

use crate::{
  diag::Diag,
  evaluator::{
    errors,
    value::{value_factory, NativeFnValue, Value},
  },
  range::Range,
};
use std::time;
use std::{collections::HashMap, path::PathBuf};

use super::NativeModule;

pub fn create_module() -> NativeModule {
  let mut module: NativeModule = HashMap::new();
  module.insert("now_ms", NativeFnValue::new("now_ms", time_now_ms));
  module.insert("now", NativeFnValue::new("now", now));
  module.insert("sleep", NativeFnValue::new("sleep", sleep));
  module
}

fn now(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if !args.is_empty() {
    let msg = format!("no expected args, found {}", args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }
  let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap_or_default().as_secs() as f64;
  Ok(value_factory::create_num(now))
}

fn time_now_ms(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if !args.is_empty() {
    let msg = format!("no expected args, found {}", args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }
  let now_ms = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap_or_default().as_millis() as f64;
  return Ok(value_factory::create_num(now_ms));
}

fn sleep(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 1 {
    let msg = errors::format_function_arity_mismatch(1, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }
  if let Value::Num(num_ms) = &args[0] {
    let duration = num_ms.get();
    if duration < 0.0 {
      let msg = format!("sleep duration must be positive, found {}", duration);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    std::thread::sleep(time::Duration::from_millis(duration as u64));
    return Ok(value_factory::create_null());
  }

  let msg = errors::format_expected_number(&args[0]);
  return Err(Diag::create_err(msg, range.clone(), path.clone()));
}
