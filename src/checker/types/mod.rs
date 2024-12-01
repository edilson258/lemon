use crate::ast::Operator;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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

  pub fn supports_operator(&self, operator: &Operator) -> bool {
    match (self, operator) {
      (
        Type::Numb(_),
        Operator::ADD | Operator::SUB | Operator::MUL | Operator::DIV | Operator::MOD,
      ) => true,
      (Type::Float(_), Operator::ADD | Operator::SUB | Operator::MUL | Operator::DIV) => true,
      (
        Type::Numb(_) | Type::Float(_) | Type::Bool | Type::Char | Type::String,
        Operator::EQ | Operator::NOT | Operator::LT | Operator::GT,
      ) => true,
      (Type::Bool, Operator::AND | Operator::OR) => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnValue {
  pub params: Vec<Type>,
  pub ret_type: Option<Box<Type>>,
}

impl FnValue {
  pub fn has_ret(&self) -> bool {
    self.ret_type.is_some()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatValue {
  pub bits: u8,
}

// FnValue
impl fmt::Display for FnValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let params = self.params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
    if let Some(ret) = &self.ret_type {
      write!(f, "fn({}) -> {}", params, ret.to_string())
    } else {
      write!(f, "fn({})", params)
    }
  }
}

// NumbValue
impl fmt::Display for NumbValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(bits) = self.bits {
      write!(f, "{}{}", if self.signed { "i" } else { "u" }, bits)
    } else {
      write!(f, "{}", if self.signed { "isize" } else { "usize" })
    }
  }
}

impl fmt::Display for FloatValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "float({} bits)", self.bits)
  }
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Type::Numb(num) => write!(f, "{}", num),
      Type::Float(floa) => write!(f, "{}", floa),
      Type::Bool => write!(f, "bool"),
      Type::Char => write!(f, "char"),
      Type::String => write!(f, "string"),
      Type::Fn(fn_type) => write!(f, "{}", fn_type),
    }
  }
}
