use std::path::PathBuf;

use crate::{
  ast,
  diag::Diag,
  evaluator::{ctx::Ctx, native::NativeFn},
  range::Range,
};

use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct FnValue {
  pub ctx: Ctx,
  pub pats: Vec<String>,
  pub stmt: Box<ast::Stmts>,
}

impl FnValue {
  pub fn new(ctx: Ctx, pats: Vec<String>, stmt: Box<ast::Stmts>) -> Self {
    Self { ctx, pats, stmt }
  }

  pub fn create_self(&mut self, value: Value) {
    self.ctx.set("self".to_owned(), value);
  }

  pub fn set_value(&mut self, key: &str, value: Value) {
    self.ctx.set(key.to_owned(), value);
  }

  pub fn is_eq(&self, value: &FnValue) -> bool {
    if self.ctx.hash != value.ctx.hash {
      return false;
    }
    if self.pats.len() != value.pats.len() {
      return false;
    }
    return self.stmt.get_range() == value.stmt.get_range();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFnValue {
  pub name: String,
  pub native: NativeFn,
}

impl NativeFnValue {
  pub fn new(name: &str, native: NativeFn) -> Self {
    Self { name: name.to_owned(), native }
  }

  pub fn is_eq(&self, value: &NativeFnValue) -> bool {
    self.name.eq(&value.name) && self.native.eq(&value.native)
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_native(&self) -> &NativeFn {
    &self.native
  }

  pub fn apply(&self, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    (self.native)(args, path, range)
  }
}

type MethodFn = fn(value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag>;
#[derive(Debug, Clone, PartialEq)]
pub struct MethodFnValue {
  native: MethodFn,
}

impl MethodFnValue {
  pub fn new(native: MethodFn) -> Self {
    Self { native }
  }
  pub fn get_native(&self) -> &MethodFn {
    &self.native
  }

  pub fn apply(&self, value: &mut Value, args: Vec<Value>, path: &PathBuf, range: &Range) -> Result<Value, Diag> {
    (self.native)(value, args, path, range)
  }
}
