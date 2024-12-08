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

  pub fn unwrap_numb(&self) -> &NumbValue {
    match self {
      Type::Numb(v) => v,
      _ => panic!("Type is not a number type"),
    }
  }

  pub fn unwrap_float(&self) -> &FloatValue {
    match self {
      Type::Float(v) => v,
      _ => panic!("Type is not a float type"),
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

  #[rustfmt::skip]
  pub fn can_operated(&self, operator: &Operator) -> bool {
    matches!((self, operator),
      // number, float allow `+`, `-`, `*`, `/`
      (Type::Numb(_) | Type::Float(_),
      Operator::ADD | Operator::SUB | Operator::MUL | Operator::DIV)
      |
      // number only allow `%`
      (Type::Numb(_), Operator::MOD)
      |
      // number, float, bool, char, string allow `==`, `!=`, `<`, `>`, `<=`, `>=`
      (Type::Numb(_) | Type::Float(_) | Type::Bool   | Type::Char   | Type::String,
      Operator::EQ   | Operator::NOT  | Operator::LT | Operator::GT | Operator::LE
      | Operator::GE | Operator::NOTEQ)

      // bool only allow `&&`, `||`
      | (Type::Bool, Operator::AND | Operator::OR)
      )
  }

  pub fn same_set(&self, target: &Type) -> bool {
    match (self, target) {
      (Type::Numb(lt), Type::Numb(rt)) => lt.same_set(rt),
      (Type::Float(_), Type::Float(_)) => true,
      (Type::Bool, Type::Bool) | (Type::Char, Type::Char) | (Type::String, Type::String) => true,
      _ => false,
    }
  }
  pub fn fits_in(&self, target: &Type) -> bool {
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

  pub fn same_set(&self, target: &FnValue) -> bool {
    if self.has_ret() != target.has_ret() {
      return false;
    }
    self.params.iter().zip(target.params.iter()).all(|(l, r)| l.same_set(r))
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

  pub fn same_set(&self, target: &Self) -> bool {
    if self.bits.is_none() && target.bits.is_none() {
      return self.signed == target.signed;
    }
    self.bits.is_some() && target.bits.is_some()
  }

  pub fn higher_bits(&self, target: &Self) -> Self {
    if self.bits.is_none() && target.bits.is_none() {
      return self.signed.then(|| self.clone()).unwrap_or(target.clone());
    }
    if self.bits.is_none() || target.bits.is_none() {
      return self.clone();
    }
    let bits = self.bits.unwrap().max(target.bits.unwrap());
    NumbValue { bits: Some(bits), signed: self.signed }
  }
}

#[derive(Debug, Clone)]
pub struct FloatValue {
  pub bits: u8,
}

impl FloatValue {
  pub fn higher_bits(&self, target: &Self) -> Self {
    if self.bits >= target.bits {
      return self.clone();
    }
    FloatValue { bits: self.bits }
  }
}

// impl  eq (==)  for Type
//

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
