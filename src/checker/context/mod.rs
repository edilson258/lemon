use std::collections::HashMap;

use super::types::Type;

#[derive(Debug)]
pub struct Context {
  pub types: HashMap<String, Type>,
  pub scopes: Vec<Scope>,
}

#[derive(Debug)]
pub struct Scope {
  pub values: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Value {
  pub name: String,
  pub kind: Type,
}

impl Value {
  pub fn new(name: String, kind: Type) -> Self {
    Self { name, kind }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }
  pub fn get_kind(&self) -> Type {
    self.kind.clone()
  }
}

impl Context {
  pub fn new() -> Self {
    Self { types: HashMap::new(), scopes: vec![Scope::new()] }
  }

  pub fn add_type(&mut self, name: String, kind: Type) {
    self.types.entry(name).or_insert(kind);
  }

  pub fn get_type(&self, name: &str) -> Option<&Type> {
    self.types.get(name)
  }

  pub fn add_value(&mut self, name: String, kind: Type) -> bool {
    if let Some(scope) = self.scopes.last_mut() {
      scope.add_value(name, kind)
    } else {
      false
    }
  }

  pub fn get_value(&self, name: &str) -> Option<&Value> {
    for scope in self.scopes.iter().rev() {
      if let Some(var) = scope.get_value(name) {
        return Some(var);
      }
    }
    None
  }

  pub fn enter_scope(&mut self) {
    self.scopes.push(Scope::new());
  }

  pub fn exit_scope(&mut self) {
    self.scopes.pop();
  }
}

impl Scope {
  pub fn new() -> Self {
    Self { values: HashMap::new() }
  }

  pub fn add_value(&mut self, name: String, kind: Type) -> bool {
    if self.values.contains_key(&name) {
      false
    } else {
      self.values.insert(name.clone(), Value { name, kind });
      true
    }
  }

  pub fn get_value(&self, name: &str) -> Option<&Value> {
    self.values.get(name)
  }
}
