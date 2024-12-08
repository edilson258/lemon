use std::collections::HashMap;

use super::types::Type;

#[derive(Debug)]
pub struct Context<'ctx> {
  pub types: HashMap<&'ctx str, Type>,
  pub scopes: Vec<Scope<'ctx>>,
}

#[derive(Debug)]
pub struct Scope<'ctx> {
  pub values: HashMap<&'ctx str, Value<'ctx>>,
}

#[derive(Debug, Clone)]
pub struct Value<'ctx> {
  pub name: &'ctx str,
  pub ty: Type,
}

impl<'ctx> Value<'ctx> {
  pub fn new(name: &'ctx str, ty: Type) -> Self {
    Self { name, ty }
  }

  pub fn get_name(&self) -> &str {
    self.name
  }

  pub fn get_type(&self) -> Type {
    self.ty.clone()
  }
}

impl<'ctx> Context<'ctx> {
  pub fn new() -> Self {
    Self { types: HashMap::new(), scopes: vec![Scope::new()] }
  }

  pub fn add_type(&mut self, name: &'ctx str, kind: Type) {
    self.types.entry(name).or_insert(kind);
  }

  pub fn get_type(&self, name: &str) -> Option<&Type> {
    self.types.get(name)
  }

  pub fn add_value(&mut self, name: &'ctx str, kind: Type) {
    let scope = self.scopes.last_mut().unwrap(); // we always have at least one scope
    scope.add_value(name, kind);
  }

  pub fn get_value(&self, name: &str) -> Option<&Value> {
    self.scopes.iter().rev().find_map(|scope| scope.get_value(name))
  }

  pub fn enter_scope(&mut self) {
    self.scopes.push(Scope::new());
  }

  pub fn exit_scope(&mut self) {
    self.scopes.pop();
  }
}

impl<'ctx> Scope<'ctx> {
  pub fn new() -> Self {
    Self { values: HashMap::new() }
  }
  pub fn add_value(&mut self, name: &'ctx str, ty: Type) {
    let value = Value::new(name, ty);
    self.values.entry(name).or_insert(value);
  }

  pub fn get_value(&self, name: &str) -> Option<&Value> {
    self.values.get(name)
  }
}
