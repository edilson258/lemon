#![allow(dead_code, unused_variables)]
mod buffer;
mod fmt;
mod io;
mod net;
mod process;
mod time;
use std::{collections::HashMap, path::PathBuf};

use crate::{diag::Diag, range::Range};

use super::value::{NativeFnValue, Value};

pub type NativeFn = fn(Vec<Value>, &PathBuf, &Range) -> Result<Value, Diag>;

pub type NativeModule = HashMap<&'static str, NativeFnValue>;

pub struct NativeRegistry {
  modules: HashMap<String, NativeModule>,
}

impl NativeRegistry {
  pub fn new() -> Self {
    let mut registry = Self { modules: HashMap::new() };
    registry.register("io", io::create_module());
    registry.register("net", net::create_module());
    registry.register("process", process::create_module());
    registry.register("time", time::create_module());
    registry.register("buffer", buffer::create_module());
    registry.register("fmt", fmt::create_module());
    registry
  }

  pub fn register(&mut self, name: &str, module: NativeModule) {
    self.modules.insert(name.to_owned(), module);
  }

  pub fn get_fn(&self, module: &str, name: &str) -> Option<&NativeFnValue> {
    self.modules.get(module)?.get(name)
  }
  pub fn get_module_fns(&self, module: &str) -> Option<&HashMap<&'static str, NativeFnValue>> {
    self.modules.get(module)
  }

  pub fn get_nested(&self, pattern: &mut Vec<String>) -> (Option<&NativeModule>, Option<&NativeFnValue>) {
    if pattern.len() < 1 || !pattern[0].eq("core") {
      return (None, None);
    }
    match pattern.len() {
      3 => {
        let root = pattern.get(1).and_then(|module| self.modules.get(module));
        let sub_module = pattern.get(2).and_then(|module| root.and_then(|root| root.get(module.as_str())));
        (root, sub_module)
      }
      2 => {
        let root = pattern.get(1).and_then(|module| self.modules.get(module));
        (root, None)
      }
      _ => (None, None),
    }
  }
}
