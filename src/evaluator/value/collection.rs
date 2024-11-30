use std::{collections::HashMap, path::PathBuf};

use crate::{diag::Diag, evaluator::errors, range::Range};

use super::{
  value_factory::{self},
  MethodFnValue, NativeFnValue, Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayValue {
  pub value: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectValue {
  pub value: HashMap<String, Value>,
}

impl ArrayValue {
  pub fn new(value: Vec<Value>) -> Self {
    Self { value }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { value: Vec::with_capacity(capacity) }
  }

  pub fn extend(&mut self, value: &ArrayValue) {
    self.value.extend(value.value.iter().cloned());
  }
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

  pub fn method_push(this: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 1 {
      let msg = errors::format_function_arity_mismatch(1, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = this.as_array_mut(range, path)?;
    array.push(args[0].clone());
    Ok(value_factory::create_null())
  }

  pub fn method_pop(this: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 0 {
      let msg = errors::format_function_arity_mismatch(0, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = this.as_array_mut(range, path)?;
    let value = array.pop().unwrap_or(value_factory::create_null());
    Ok(value)
  }

  pub fn method_shift(this: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 0 {
      let msg = errors::format_function_arity_mismatch(0, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = this.as_array_mut(range, path)?;
    let value = array.pop().unwrap_or(value_factory::create_null());
    array.set(0, value.clone());
    Ok(value)
  }

  pub fn method_unshift(this: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 1 {
      let msg = errors::format_function_arity_mismatch(1, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = this.as_array_mut(range, path)?;
    array.set(0, args[0].clone());
    Ok(value_factory::create_null())
  }

  pub fn method_splice(this: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 2 {
      let msg = errors::format_function_arity_mismatch(2, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = this.as_array_mut(range, path)?;
    let start = args[0].as_num(range, path)?.get() as usize;
    let delete_count = args[1].as_num(range, path)?.get() as usize;
    let mut values = Vec::with_capacity(delete_count);
    for _ in 0..delete_count {
      values.push(array.pop().unwrap_or(value_factory::create_null()));
    }
    array.value.splice(start..start, values);
    Ok(value_factory::create_null())
  }

  pub fn method_reverse(this: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 0 {
      let msg = errors::format_function_arity_mismatch(0, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = this.as_array_mut(range, path)?;
    array.value.reverse();
    Ok(value_factory::create_null())
  }

  pub fn method(&self, name: &str) -> Option<MethodFnValue> {
    match name {
      "push" => Some(MethodFnValue::new(Self::method_push)),
      "pop" => Some(MethodFnValue::new(Self::method_pop)),
      "shift" => Some(MethodFnValue::new(Self::method_shift)),
      "unshift" => Some(MethodFnValue::new(Self::method_unshift)),
      "splice" => Some(MethodFnValue::new(Self::method_splice)),
      "reverse" => Some(MethodFnValue::new(Self::method_reverse)),
      _ => None,
    }
  }
}

// ---- array methods -----

pub mod array_methods {
  use super::*;
  pub fn array_push(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 1 {
      let msg = errors::format_function_arity_mismatch(1, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = value.as_array_mut(range, path)?;
    array.push(args[0].clone());
    Ok(value_factory::create_null())
  }

  pub fn array_pop(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 0 {
      let msg = errors::format_function_arity_mismatch(0, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = value.as_array_mut(range, path)?;
    let value = array.pop().unwrap_or(value_factory::create_null());
    Ok(value)
  }

  pub fn array_shift(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 0 {
      let msg = errors::format_function_arity_mismatch(0, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = value.as_array_mut(range, path)?;
    let value = array.pop().unwrap_or(value_factory::create_null());
    array.set(0, value.clone());
    Ok(value)
  }

  pub fn array_unshift(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 1 {
      let msg = errors::format_function_arity_mismatch(1, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = value.as_array_mut(range, path)?;
    array.set(0, args[0].clone());
    Ok(value_factory::create_null())
  }

  pub fn array_splice(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 2 {
      let msg = errors::format_function_arity_mismatch(2, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = value.as_array_mut(range, path)?;
    let start = args[0].as_num(range, path)?.get() as usize;
    let delete_count = args[1].as_num(range, path)?.get() as usize;
    let mut values = Vec::with_capacity(delete_count);
    for _ in 0..delete_count {
      values.push(array.pop().unwrap_or(value_factory::create_null()));
    }
    array.value.splice(start..start, values);
    Ok(value_factory::create_null())
  }

  pub fn array_reverse(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    if args.len() != 0 {
      let msg = errors::format_function_arity_mismatch(0, args.len());
      return Err(Diag::create_err(msg, range.clone(), path.clone()));
    }
    let array = value.as_array_mut(range, path)?;
    array.value.reverse();
    Ok(value_factory::create_null())
  }
}

// pub fn array_sort(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
//   if args.len() != 0 {
//     let msg = errors::format_function_arity_mismatch(0, args.len());
//     return Err(Diag::create_err(msg, range.clone(), path.clone()));
//   }
//   let array = value.as_array_mut(range, path)?;
//   array.value.sort();
//   Ok(value_factory::create_null())
// }

// ----- ObjectValue -----

impl ObjectValue {
  pub fn new() -> Self {
    Self { value: HashMap::new() }
  }

  pub fn with_native(natives: &HashMap<&'static str, NativeFnValue>) -> Self {
    let mut value: HashMap<String, Value> = HashMap::new();
    for (key, native) in natives.iter() {
      value.insert(key.to_string(), Value::NativeFn(native.clone()));
    }
    Self { value }
  }

  pub fn new_with_map(value: HashMap<String, Value>) -> Self {
    Self { value }
  }
  pub fn with_capacity(capacity: usize) -> Self {
    Self { value: HashMap::with_capacity(capacity) }
  }

  pub fn extend(&mut self, value: &ObjectValue) {
    self.value.extend(value.value.iter().map(|(k, v)| (k.to_owned(), v.to_owned())));
  }
  pub fn get(&self, key: &str) -> Option<Value> {
    self.value.get(key).map(|v| v.to_owned())
  }

  pub fn set(&mut self, key: &str, value: Value) {
    self.value.insert(key.to_owned(), value);
  }
  pub fn values(&self) -> Vec<Value> {
    self.value.iter().map(|(_, v)| v.clone()).collect()
  }

  pub fn is_eq(&self, value: &ObjectValue) -> bool {
    self.value.iter().zip(value.value.iter()).all(|(lt, rt)| lt.0 == rt.0 && lt.1.is_eq(&rt.1))
  }
}
