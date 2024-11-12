#![allow(dead_code)]

use std::{collections::HashMap, path::PathBuf};

use crate::{
  diag::Diag,
  evaluator::value::{NativeFnValue, Value},
  range::Range,
};

use super::NativeModule;

pub fn create_module() -> NativeModule {
  let mut module: NativeModule = HashMap::new();

  module.insert("tcp_connect", NativeFnValue::new("tcp_connect", tcp_connect));
  module.insert("tcp_listen", NativeFnValue::new("tcp_listen", tcp_listen));
  module.insert("tcp_accept", NativeFnValue::new("tcp_accept", tcp_accept));
  module.insert("tcp_read", NativeFnValue::new("tcp_read", tcp_read));
  module.insert("tcp_write", NativeFnValue::new("tcp_write", tcp_write));
  module.insert("tcp_close", NativeFnValue::new("tcp_close", tcp_close));
  module
}

fn tcp_connect(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  return Err(Diag::create_err("not implemented".to_owned(), range.clone(), path.clone()));
}

fn tcp_listen(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  return Err(Diag::create_err("not implemented".to_owned(), range.clone(), path.clone()));
}

fn tcp_accept(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  return Err(Diag::create_err("not implemented".to_owned(), range.clone(), path.clone()));
}

fn tcp_read(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  return Err(Diag::create_err("not implemented".to_owned(), range.clone(), path.clone()));
}

fn tcp_write(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  return Err(Diag::create_err("not implemented".to_owned(), range.clone(), path.clone()));
}

fn tcp_close(args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
  return Err(Diag::create_err("not implemented".to_owned(), range.clone(), path.clone()));
}
