use crate::ast::ast_type;

use super::{
  types::{FloatValue, NumbValue, Type},
  Checker, CheckerResult,
};

impl<'a> Checker<'a> {
  pub fn check_type(&mut self, ast_type: &ast_type::AstType) -> CheckerResult<Type> {
    match ast_type {
      ast_type::AstType::Bool(bool) => self.check_bool_type(&bool),
      ast_type::AstType::Char(char) => self.check_char_type(&char),
      ast_type::AstType::Float(float) => self.check_float_type(&float),
      ast_type::AstType::Numb(numb) => self.check_numb_type(&numb),
      ast_type::AstType::String(string) => self.check_string_type(&string),
      ast_type::AstType::Fn(fn_type) => self.check_fn_type(&fn_type),
      ast_type::AstType::Ident(ident) => self.check_ident_type(&ident),
    }
  }

  pub fn check_float_type(&mut self, ast_type: &ast_type::FloatType) -> CheckerResult<Type> {
    let float = FloatValue { bits: ast_type.bits };
    return Ok(Some(Type::Float(float)));
  }

  pub fn check_numb_type(&mut self, ast_type: &ast_type::NumbType) -> CheckerResult<Type> {
    let bits = if ast_type.bits > 8 { Some(ast_type.bits) } else { None };
    let numb = NumbValue { bits, signed: ast_type.signed };
    return Ok(Some(Type::Numb(numb)));
  }

  fn check_bool_type(&mut self, ast_type: &ast_type::BaseType) -> CheckerResult<Type> {
    return Ok(Some(Type::Bool));
  }

  fn check_char_type(&mut self, ast_type: &ast_type::BaseType) -> CheckerResult<Type> {
    return Ok(Some(Type::Char));
  }

  pub fn check_string_type(&mut self, ast_type: &ast_type::BaseType) -> CheckerResult<Type> {
    return Ok(Some(Type::String));
  }

  pub fn check_fn_type(&mut self, ast_type: &ast_type::FnType) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_ident_type(&mut self, ast_type: &ast_type::IdentType) -> CheckerResult<Type> {
    todo!()
  }
}
