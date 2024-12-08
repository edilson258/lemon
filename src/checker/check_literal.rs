use crate::ast::{self};

use super::{
  types::{FloatValue, NumbValue, Type},
  CheckResult, Checker,
};

impl<'ckr> Checker<'ckr> {
  pub fn check_literal(&mut self, lit: &ast::Literal) -> CheckResult<Type> {
    match lit {
      ast::Literal::Num(num) => self.check_num_literal(num),
      ast::Literal::String(string) => Ok(Some(Type::String)),
      ast::Literal::Bool(bool) => Ok(Some(Type::Bool)),
      ast::Literal::Char(char) => Ok(Some(Type::Char)),
      ast::Literal::Null(null) => todo!(),
    }
  }

  pub fn check_num_literal(&mut self, num: &ast::NumLiteral) -> CheckResult<Type> {
    if num.as_dot() {
      let bits = self.check_float_bits(num)?.unwrap();
      let float = FloatValue { bits };
      return Ok(Some(Type::Float(float)));
    }
    let bits = self.check_num_bits(num)?.unwrap();
    let numb = NumbValue { bits: Some(bits), signed: false };
    Ok(Some(Type::Numb(numb)))
  }

  pub fn check_num_bits(&mut self, num: &ast::NumLiteral) -> CheckResult<u8> {
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
    Ok(Some(bits))
  }

  pub fn check_float_bits(&mut self, num: &ast::NumLiteral) -> CheckResult<u8> {
    // todo: improve this...
    match num.text.parse::<f32>() {
      Ok(num) if !num.is_nan() && !num.is_infinite() => return Ok(Some(32)),
      _ => {}
    }
    match num.text.parse::<f64>() {
      Ok(num) if !num.is_nan() && !num.is_infinite() => return Ok(Some(64)),
      _ => {}
    }
    unreachable!()
  }
}
