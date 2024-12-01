use crate::ast::{self, BASE_DECIMAL};

use super::{
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
    let value = num.text.parse::<f64>();
    let bits = 64; // default
    if num.as_dot && num.base == BASE_DECIMAL {
      let float = FloatValue { bits };
      return Ok(Some(Type::Float(float)));
    }
    // base: num.base
    let numb = NumbValue { bits: Some(bits), signed: false };
    return Ok(Some(Type::Numb(numb)));
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
