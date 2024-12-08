use crate::range::{Range, TraitRange};
use serde::{Deserialize, Serialize};

use super::visitor::Visitor;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstType {
  Numb(NumbType),
  Float(FloatType),
  Bool(BaseType),
  String(BaseType),
  Char(BaseType),
  Ident(IdentType),
  Fn(FnType),
}

impl AstType {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    match self {
      AstType::Numb(num) => visitor.visit_numb_type(num),
      AstType::Float(float) => visitor.visit_float_type(float),
      AstType::Bool(bool) => visitor.visit_base_type(bool),
      AstType::String(string) => visitor.visit_base_type(string),
      AstType::Char(char) => visitor.visit_base_type(char),
      AstType::Ident(ident) => visitor.visit_ident_type(ident),
      AstType::Fn(fn_type) => visitor.visit_fn_type(fn_type),
    }
  }
}

impl TraitRange for AstType {
  fn range(&self) -> Range {
    match self {
      AstType::Numb(num) => num.range(),
      AstType::Float(float) => float.range(),
      AstType::Bool(bool) => bool.range(),
      AstType::String(string) => string.range(),
      AstType::Char(char) => char.range(),
      AstType::Ident(ident) => ident.range(),
      AstType::Fn(fn_type) => fn_type.range(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseType {
  pub range: Range,
}

impl BaseType {
  pub fn range(&self) -> Range {
    self.range.clone()
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_base_type(self)
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumbType {
  pub range: Range,
  pub bits: u8, // 0 = depends on arch
  pub signed: bool,
}

impl NumbType {
  pub fn display(&self) -> String {
    match (self.bits, self.signed) {
      (8, true) => "i8".to_owned(),
      (8, false) => "u8".to_owned(),
      (16, true) => "i16".to_owned(),
      (16, false) => "u16".to_owned(),
      (32, true) => "i32".to_owned(),
      (32, false) => "u32".to_owned(),
      (64, true) => "i64".to_owned(),
      (64, false) => "u64".to_owned(),
      (_, true) if self.bits < 8 => "isize".to_owned(),
      (_, false) if self.bits < 8 => "usize".to_owned(),
      _ => panic!("error: unsupported number type"),
    }
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_numb_type(self)
  }
}

impl TraitRange for NumbType {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatType {
  pub range: Range,
  pub bits: u8, // only 32 and 64 are supported
}

impl FloatType {
  pub fn display(&self) -> String {
    match self.bits {
      32 => "f32".to_owned(),
      64 => "f64".to_owned(),
      _ => panic!("error: unsupported float type"),
    }
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_float_type(self)
  }
}

impl TraitRange for FloatType {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentType {
  pub range: Range,
  pub text: String,
}

impl IdentType {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_ident_type(self)
  }
}

impl TraitRange for IdentType {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

// fn(params_types...): ret_type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnType {
  pub params: Vec<AstType>,
  pub ret_type: Option<Box<AstType>>,
  pub range: Range, // fn range
}

impl FnType {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_fn_type(self)
  }
}

impl TraitRange for FnType {
  fn range(&self) -> Range {
    if let Some(ret_type) = &self.ret_type {
      return self.range.merged_with(&ret_type.range());
    }
    self.range.clone()
  }
}
