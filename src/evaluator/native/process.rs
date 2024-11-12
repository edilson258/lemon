#![allow(dead_code)]

use std::{collections::HashMap, path::PathBuf, process::Command};

use crate::{
  diag::Diag,
  evaluator::{
    errors,
    value::{value_factory, NativeFnValue, Value},
  },
  range::Range,
};

use super::NativeModule;

pub fn create_module() -> NativeModule {
  let mut module: NativeModule = HashMap::new();
  module.insert("spawn", NativeFnValue::new("spawn", spawn_process));
  module.insert("exit", NativeFnValue::new("exit", exit));
  module.insert("get_env", NativeFnValue::new("get_env", get_env));
  module.insert("set_env", NativeFnValue::new("set_env", set_env));
  module
}

fn spawn_process(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() < 1 {
    let msg = errors::format_function_arity_mismatch(1, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  let command = match &args[0] {
    Value::String(cmd) => cmd,
    _ => {
      let msg = errors::format_mismatched_types("string", &args[0]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  let process_args = args[1..]
    .iter()
    .map(|arg| {
      if let Value::String(s) = arg {
        Ok(s.get().to_string())
      } else {
        let msg = errors::format_mismatched_types("string", &arg);
        Err(Diag::create_err(msg, range.clone(), path.clone()))
      }
    })
    .collect::<Result<Vec<String>, Diag>>()?;

  match Command::new(command.get()).args(process_args).output() {
    Ok(output) => {
      let mut result = HashMap::new();
      let status = value_factory::create_num(output.status.code().unwrap_or(-1) as f64);
      result.insert("status".to_string(), status);
      let stdout = value_factory::create_string(String::from_utf8_lossy(&output.stdout).to_string());
      result.insert("stdout".to_string(), stdout);
      let stderr = value_factory::create_string(String::from_utf8_lossy(&output.stderr).to_string());
      result.insert("stderr".to_string(), stderr);
      Ok(value_factory::create_object(result))
    }
    Err(err) => {
      let msg = format!("spawn process failed: {}", err.to_string());
      Err(Diag::create_err(msg, range.clone(), path.clone()))
    }
  }
}

fn exit(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 1 {
    let msg = errors::format_function_arity_mismatch(1, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  if let Value::Num(num) = args.get(0).unwrap() {
    std::process::exit(num.get() as i32);
  }

  let msg = errors::format_expected_number(args.get(0).unwrap());
  return Err(Diag::create_err(msg, range.clone(), path.clone()));
}

fn get_env(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 1 {
    let msg = errors::format_function_arity_mismatch(1, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  let key = match &args[0] {
    Value::String(k) => k,
    _ => {
      let msg = errors::format_mismatched_types("string", &args[0]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  match std::env::var(key.get()) {
    Ok(value) => Ok(value_factory::create_string(value)),
    Err(_) => Ok(value_factory::create_null()),
  }
}

fn set_env(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  if args.len() != 2 {
    let msg = errors::format_function_arity_mismatch(2, args.len());
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }

  let key = match &args[0] {
    Value::String(k) => k,
    _ => {
      let msg = errors::format_mismatched_types("string", &args[0]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  let value = match &args[1] {
    Value::String(v) => v,
    _ => {
      let msg = errors::format_mismatched_types("string", &args[1]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  std::env::set_var(key.get(), value.get());
  Ok(value_factory::create_null())
}
