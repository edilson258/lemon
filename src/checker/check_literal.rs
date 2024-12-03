use crate::ast::{self};

use super::{
  diags::errs::TypeErr,
  types::{FloatValue, NumbValue, Type},
  Checker, CheckerResult,
};

impl<'a> Checker<'a> {
  pub fn check_literal(&mut self, lit: &ast::Literal) -> CheckerResult<Type> {
    match lit {
      ast::Literal::Num(num) => self.check_num_literal(num),
      ast::Literal::String(string) => self.check_string_literal(string),
      ast::Literal::Bool(bool) => self.check_bool_literal(bool),
      ast::Literal::Null(null) => self.check_null_literal(null),
    }
  }

  pub fn check_num_literal(&mut self, num: &ast::NumLiteral) -> CheckerResult<Type> {
    if num.as_dot() {
      let bits = self.check_float_bits(num)?.unwrap();
      let float = FloatValue { bits };
      return Ok(Some(Type::Float(float)));
    }
    let bits = self.check_num_bits(&num)?.unwrap();
    let numb = NumbValue { bits: Some(bits), signed: false };
    return Ok(Some(Type::Numb(numb)));
  }

  pub fn check_num_bits(&mut self, num: &ast::NumLiteral) -> CheckerResult<u8> {
    let diag = TypeErr::Unsupported(num.range());
    let value = match u128::from_str_radix(&num.text, num.base as u32) {
      Ok(v) => v,
      _ => return Err(diag.into()),
    };

    let bits: u8 = match (128 - value.leading_zeros()) as u8 {
      0..=8 => 8,
      9..=16 => 16,
      17..=32 => 32,
      33..=64 => 64,
      65..=128 => 128,
      _ => return Err(diag.into()),
    };
    Ok(Some(bits))
  }

  pub fn check_float_bits(&mut self, num: &ast::NumLiteral) -> CheckerResult<u8> {
    // todo: improve this...
    match num.text.parse::<f32>() {
      Ok(num) if !num.is_nan() && !num.is_infinite() => return Ok(Some(32)),
      _ => {}
    }
    match num.text.parse::<f64>() {
      Ok(num) if !num.is_nan() && !num.is_infinite() => return Ok(Some(64)),
      _ => {}
    }
    let diag = TypeErr::Unsupported(num.range());
    return Err(diag.into());
  }

  pub fn check_string_literal(&mut self, string: &ast::StringLiteral) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_bool_literal(&mut self, bool: &ast::BoolLiteral) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_null_literal(&mut self, null: &ast::BaseExpr) -> CheckerResult<Type> {
    todo!()
  }
}
