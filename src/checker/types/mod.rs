use crate::ast::Operator;
use core::fmt;
pub mod show;

#[derive(Debug, Clone)]
pub enum Type {
  Numb(NumbValue),
  Float(FloatValue),
  Bool,
  Char,
  String,
  Fn(FnValue),
}

impl Type {
  pub fn is_fn(&self) -> bool {
    matches!(self, Type::Fn(_))
  }
  pub fn is_numb(&self) -> bool {
    matches!(self, Type::Numb(_))
  }

  pub fn get_numb(&self) -> Result<&NumbValue, &'static str> {
    match self {
      Type::Numb(v) => Ok(v),
      _ => Err("is not a number"),
    }
  }

  pub fn get_float(&self) -> Result<&FloatValue, &'static str> {
    match self {
      Type::Float(v) => Ok(v),
      _ => Err("is not a float"),
    }
  }

  pub fn is_float(&self) -> bool {
    matches!(self, Type::Float(_))
  }

  pub fn is_bool(&self) -> bool {
    matches!(self, Type::Bool)
  }
  pub fn is_char(&self) -> bool {
    matches!(self, Type::Char)
  }

  pub fn is_string(&self) -> bool {
    matches!(self, Type::String)
  }

  pub fn can_operate_with(&self, operator: &Operator) -> bool {
    use Operator::*;
    match self {
      Type::Numb(_) if matches!(operator, MOD) => true,
      Type::Numb(_) | Type::Float(_) if matches!(operator, ADD | SUB | MUL | DIV) => true,
      Type::Bool if matches!(operator, AND | OR) => true,
      Type::Numb(_) | Type::Float(_) | Type::Bool | Type::Char | Type::String => {
        matches!(operator, EQ | NOT | LT | GT | LE | GE | NOTEQ)
      }
      _ => false,
    }
  }

  pub fn is_cmp_with(&self, target: &Type) -> bool {
    match (self, target) {
      (Type::Numb(lt), Type::Numb(rt)) => lt.is_cmp_with(rt),
      (Type::Float(_), Type::Float(_)) => true,
      (Type::Bool, Type::Bool) | (Type::Char, Type::Char) | (Type::String, Type::String) => true,
      _ => false,
    }
  }

  pub fn fits_into(&self, target: &Type) -> bool {
    match (self, target) {
      (Type::Numb(left), Type::Numb(right)) => left.bits.unwrap_or(0) <= right.bits.unwrap_or(0),
      (Type::Float(left), Type::Float(right)) => left.bits <= right.bits,
      (Type::Float(_), Type::Numb(_)) => false,
      _ => false,
    }
  }
}

#[derive(Debug, Clone)]
pub struct FnValue {
  pub params: Vec<Type>,
  pub ret_type: Option<Box<Type>>,
}

impl FnValue {
  pub fn new(params: Vec<Type>, ret_type: Option<Box<Type>>) -> Self {
    FnValue { params, ret_type }
  }

  pub fn has_ret(&self) -> bool {
    self.ret_type.is_some()
  }

  pub fn is_cmp_with(&self, target: &FnValue) -> bool {
    if self.has_ret() != target.has_ret() {
      return false;
    }
    self.params.iter().zip(target.params.iter()).all(|(l, r)| l.is_cmp_with(r))
  }
}

#[derive(Debug, Clone)]
pub struct NumbValue {
  pub bits: Option<u8>,
  pub signed: bool,
}

impl NumbValue {
  pub fn is_signed(&self) -> bool {
    self.signed
  }

  pub fn is_arch(&self) -> bool {
    self.bits.is_none()
  }

  pub fn set_bits(&mut self, bits: Option<u8>) {
    self.bits = bits;
  }

  pub fn set_signed(&mut self, value: bool) {
    self.signed = value;
  }

  pub fn is_cmp_with(&self, other: &Self) -> bool {
    match (self.bits, other.bits) {
      (Some(b1), Some(b2)) => b1 == b2 && self.signed == other.signed,
      (None, None) => self.signed == other.signed,
      _ => false,
    }
  }

  pub fn higher_bits(&self, other: &Self) -> NumbValue {
    match (self.bits, other.bits) {
      (Some(b1), Some(b2)) => NumbValue { bits: Some(b1.max(b2)), signed: self.signed },
      _ => self.clone(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct FloatValue {
  pub bits: u8,
}

impl FloatValue {
  pub fn higher_bits(&self, other: &Self) -> Self {
    if self.bits >= other.bits {
      self.clone()
    } else {
      other.clone()
    }
  }
}

impl PartialEq for Type {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Type::Numb(lt), Type::Numb(rt)) => lt.eq(rt),
      (Type::Float(lt), Type::Float(rt)) => lt.eq(rt),
      (Type::Bool, Type::Bool) | (Type::Char, Type::Char) | (Type::String, Type::String) => true,
      _ => false,
    }
  }
}

impl PartialEq for FnValue {
  fn eq(&self, other: &Self) -> bool {
    if self.has_ret() != other.has_ret() {
      return false;
    }
    self.params.iter().zip(other.params.iter()).all(|(l, r)| l.eq(r))
  }
}

impl PartialEq for NumbValue {
  fn eq(&self, other: &Self) -> bool {
    if self.bits.is_none() && other.bits.is_none() {
      return self.signed == other.signed;
    }
    if self.bits.is_none() || other.bits.is_none() {
      return false;
    }
    self.bits.unwrap() == other.bits.unwrap()
  }
}

impl PartialEq for FloatValue {
  fn eq(&self, other: &Self) -> bool {
    self.bits == other.bits
  }
}
