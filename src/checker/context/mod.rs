use std::collections::HashMap;
pub mod scope;
use super::types::Type;
use scope::{Scope, Symbol, Value};

#[derive(Debug)]
pub struct Context<'ctx> {
  pub types: HashMap<&'ctx str, Type>,
  pub scopes: Vec<Scope<'ctx>>,
  pub pipe_arg: Option<Type>,
}

impl<'ctx> Context<'ctx> {
  pub fn new() -> Self {
    Self { pipe_arg: None, types: HashMap::new(), scopes: vec![Scope::new()] }
  }

  pub fn add_type(&mut self, name: &'ctx str, kind: Type) {
    self.types.entry(name).or_insert(kind);
  }

  pub fn get_type(&self, name: &str) -> Option<&Type> {
    self.types.get(name)
  }

  pub fn add_value(&mut self, name: &'ctx str, ty: Type) {
    let scope = self.scopes.last_mut().unwrap(); // we always have at least one scope
    scope.add_value(name, ty);
  }

  pub fn add_value_mut(&mut self, name: &'ctx str, ty: Type) {
    let scope = self.scopes.last_mut().unwrap(); // we always have at least one scope
    scope.add_value_mut(name, ty);
  }

  pub fn set_value(&mut self, name: &'ctx str, ty: Type) -> bool {
    let scope = self.scopes.last_mut().unwrap(); // we always have at least one scope
    scope.set_value(name, ty)
  }

  pub fn get_value(&self, name: &str) -> Option<&Value> {
    self.scopes.iter().rev().find_map(|scope| scope.get_value(name))
  }

  pub fn add_pipe_arg(&mut self, symbol: Symbol) {
    self.pipe_arg = Some(symbol.as_ty());
  }
  pub fn take_pipe_arg(&mut self) -> Option<Type> {
    self.pipe_arg.take()
  }

  pub fn get_symbol(&self, name: &str) -> Option<Symbol> {
    let value = self.scopes.iter().rev().find_map(|scope| scope.get_value(name));
    value.map(|value| Symbol::Value(value.clone()))
  }

  pub fn get_value_mut(&mut self, name: &str) -> Option<&mut Value> {
    self.scopes.iter_mut().rev().find_map(|scope| scope.get_value_mut(name))
  }

  pub fn enter_scope(&mut self) {
    self.scopes.push(Scope::new());
  }

  pub fn exit_scope(&mut self) {
    self.scopes.pop();
  }
}
