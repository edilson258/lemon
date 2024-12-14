use crate::ast::{self};

use super::{
  context::scope::Symbol,
  types::{FloatValue, NumbValue, Type},
  Checker, CkrResult,
};

impl<'ckr> Checker<'ckr> {
  pub fn check_literal(&mut self, lit: &ast::Literal) -> CkrResult {
    let ty = match lit {
      ast::Literal::Num(num) => self.check_num_literal(num),
      ast::Literal::String(string) => Type::String,
      ast::Literal::Bool(bool) => Type::Bool,
      ast::Literal::Char(char) => Type::Char,
      ast::Literal::Null(null) => todo!(),
    };

    Ok(Some(Symbol::Type(ty)))
  }

  pub fn check_num_literal(&mut self, num: &ast::NumLiteral) -> Type {
    if num.as_dot() {
      let bits = self.check_float_bits(num).unwrap();
      let float = FloatValue { bits };
      return Type::Float(float);
    }
    let bits = self.check_num_bits(num).unwrap();
    let numb = NumbValue::new_signed(Some(bits));
    Type::Numb(numb)
  }

  pub fn check_num_bits(&mut self, num: &ast::NumLiteral) -> Option<u8> {
    // todo:  we need o check if the number is valid
    let value = u128::from_str_radix(&num.text, num.base as u32).unwrap();
    let bit_range = (128 - value.leading_zeros()) as u8;
    let bits = match bit_range {
      0..=8 => 8,
      9..=16 => 16,
      17..=32 => 32,
      33..=64 => 64,
      65..=128 => 128,
      _ => unreachable!(),
    };
    Some(bits)
  }

  pub fn check_float_bits(&mut self, num: &ast::NumLiteral) -> Option<u8> {
    // todo: improve this...
    match num.text.parse::<f32>() {
      Ok(num) if !num.is_nan() && !num.is_infinite() => return Some(32),
      _ => {}
    }
    match num.text.parse::<f64>() {
      Ok(num) if !num.is_nan() && !num.is_infinite() => return Some(64),
      _ => {}
    }
    unreachable!()
  }
}
