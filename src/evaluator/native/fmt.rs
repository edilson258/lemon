#![allow(dead_code)]

use std::{collections::HashMap, path::PathBuf};

use crate::evaluator::formatting::display_value;
use crate::evaluator::value::{value_factory, NativeFnValue, Value};
use crate::{diag::Diag, evaluator::errors, range::Range};

use super::NativeModule;

pub fn create_module() -> NativeModule {
  let mut native: NativeModule = HashMap::new();
  native.insert("format", NativeFnValue::new("format", format));
  native
}

fn format(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  let pattern = match &args[0] {
    Value::String(string_value) => string_value,
    _ => {
      let msg = errors::format_mismatched_types("string as first arg", &args[0]);
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
  };

  let mut output = String::with_capacity(pattern.get().len() + args.len() * 10);
  let mut chars = pattern.get().chars().peekable();
  let mut arg_index = 0;

  while let Some(ch) = chars.next() {
    if ch == '{' && chars.peek() == Some(&'}') {
      chars.next();
      if arg_index + 1 < args.len() {
        let arg = display_value(&args[arg_index + 1]).to_string();
        output.push_str(&arg);
        arg_index += 1;
      } else {
        let msg = format!("missing arg for placeholder `{}`", arg_index + 1);
        return Err(Diag::create_err(msg, range.clone(), path.clone()));
      }
    } else {
      output.push(ch);
    }
  }

  if arg_index != args.len() - 1 {
    let msg = format!("expected {} args, found {}", arg_index, args.len() - 1);
    return Err(Diag::create_err(msg, range.clone(), path.clone()));
  }
  Ok(value_factory::create_string(output))
}
