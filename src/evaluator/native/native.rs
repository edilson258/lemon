use std::{collections::HashMap, path};

use crate::{diag::Diag, evaluator::formatting::display_value, range::Range};

use super::{
  errors,
  value::{create_null, Value},
};

pub type NativeFn = fn(Vec<Value>, &path::PathBuf, &Range) -> Result<Value, Diag>;

pub fn create_native_functions() -> HashMap<&'static str, NativeFn> {
  let mut map: HashMap<&'static str, NativeFn> = HashMap::new();
  // io
  map.insert("println", |args, _path, _range| {
    let output = args.iter().map(|arg| display_value(arg).to_string()).collect::<Vec<String>>().join("");
    println!("{}", output);
    Ok(create_null())
  });
  map.insert("print", |args, _path, _range| {
    let output = args.iter().map(|arg| display_value(arg).to_string()).collect::<Vec<String>>().join("");
    println!("{}", output);
    Ok(create_null())
  });

  // exit
  map.insert("exit", |args, path, range| {
    if args.len() != 1 {
      let err = errors::format_function_arity_mismatch(1, args.len());
      return Err(Diag::create_err(err, range.clone(), path.clone()));
    }
    if let Value::Num(num) = args.get(0).unwrap() {
      std::process::exit(num.get() as i32);
    }
    let err = errors::format_expected_number(args.get(0).unwrap());
    return Err(Diag::create_err(err, range.clone(), path.clone()));
  });
  return map;
}
