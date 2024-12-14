use std::collections::HashMap;

use crate::checker::types::Type;

#[derive(Debug, Clone)]
pub enum Symbol {
  Value(Value),
  Type(Type),
}

impl Symbol {
  pub fn as_ref_ty(&self) -> &Type {
    match self {
      Symbol::Value(value) => &value.ty,
      Symbol::Type(ty) => ty,
    }
  }

  pub fn as_ty(&self) -> Type {
    match self {
      Symbol::Value(value) => value.ty.clone(),
      Symbol::Type(ty) => ty.clone(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Value {
  pub name: String,
  pub ty: Type,
  mutable: bool,
  pub infered: bool,
}

impl Value {
  pub fn new(name: &str, ty: Type) -> Self {
    Self { name: name.to_string(), ty, mutable: false, infered: false }
  }
  pub fn new_mutable(name: &str, ty: Type) -> Self {
    Self { name: name.to_string(), ty, mutable: true, infered: false }
  }

  pub fn get_name(&self) -> &str {
    self.name.as_str()
  }

  pub fn get_type(&self) -> &Type {
    &self.ty
  }
  pub fn set_type(&mut self, ty: Type) -> bool {
    if !self.mutable {
      return false;
    }
    self.ty = ty;
    self.set_as_immutable();
    true
  }

  pub fn set_as_immutable(&mut self) {
    self.mutable = false;
  }

  pub fn is_infered(&mut self) -> bool {
    self.infered
  }

  pub fn set_as_infered(&mut self) {
    self.infered = true
  }

  pub fn is_mutable(&self) -> bool {
    self.mutable
  }

  pub fn get_type_mut(&mut self) -> &mut Type {
    &mut self.ty
  }
}

#[derive(Debug)]
pub struct Scope<'sp> {
  pub values: HashMap<&'sp str, Value>,
}

impl<'sp> Scope<'sp> {
  pub fn new() -> Self {
    let values = HashMap::new();
    Self { values }
  }
  pub fn add_value(&mut self, name: &'sp str, ty: Type) {
    let value = Value::new(name, ty);
    self.values.entry(name).or_insert(value);
  }

  pub fn add_value_mut(&mut self, name: &'sp str, ty: Type) {
    let value = Value::new_mutable(name, ty);
    self.values.entry(name).or_insert(value);
  }

  pub fn get_value(&self, name: &str) -> Option<&Value> {
    self.values.get(name)
  }

  pub fn get_value_mut(&mut self, name: &str) -> Option<&mut Value> {
    self.values.get_mut(name)
  }

  pub fn set_value(&mut self, name: &'sp str, ty: Type) -> bool {
    if let Some(value) = self.values.get_mut(name) {
      return value.set_type(ty);
    }
    false
  }
}
