use crate::ast::ast_type;

use super::{
  context::scope::Symbol,
  types::{FloatValue, FnValue, NumbValue, Type},
  Checker, CkrResult,
};

impl<'ckr> Checker<'ckr> {
  pub fn check_type(&mut self, ast_type: &ast_type::AstType) -> CkrResult {
    match ast_type {
      ast_type::AstType::Bool(bool) => self.check_bool_type(bool),
      ast_type::AstType::Char(char) => self.check_char_type(char),
      ast_type::AstType::String(string) => self.check_string_type(string),
      ast_type::AstType::Float(float) => self.check_float_type(float),
      ast_type::AstType::Numb(numb) => self.check_numb_type(numb),
      ast_type::AstType::Fn(fn_type) => self.check_fn_type(fn_type),
      ast_type::AstType::Ident(ident) => self.check_ident_type(ident),
    }
  }

  pub fn check_float_type(&mut self, ast_type: &ast_type::FloatType) -> CkrResult {
    let bits = ast_type.bits;
    let float = FloatValue { bits };
    let symbol = Symbol::Type(Type::Float(float));
    Ok(Some(symbol))
  }

  pub fn check_string_type(&mut self, ast_type: &ast_type::BaseType) -> CkrResult {
    let symbol = Symbol::Type(Type::String);
    Ok(Some(symbol))
  }

  pub fn check_char_type(&mut self, ast_type: &ast_type::BaseType) -> CkrResult {
    let symbol = Symbol::Type(Type::Char);
    Ok(Some(symbol))
  }

  pub fn check_bool_type(&mut self, ast_type: &ast_type::BaseType) -> CkrResult {
    let symbol = Symbol::Type(Type::Bool);
    Ok(Some(symbol))
  }

  pub fn check_numb_type(&mut self, ast_type: &ast_type::NumbType) -> CkrResult {
    let bits = if ast_type.bits >= 8 { Some(ast_type.bits) } else { None };
    let numb = NumbValue { bits, signed: ast_type.signed };
    let symbol = Symbol::Type(Type::Numb(numb));
    Ok(Some(symbol))
  }
  pub fn check_fn_type(&mut self, ast_type: &ast_type::FnType) -> CkrResult {
    let mut params = Vec::with_capacity(ast_type.params.len());

    for param in ast_type.params.iter() {
      params.push(self.check_type(param)?.unwrap().as_ty()); // we expect the type to be checked
    }

    let ret_type = match ast_type.ret_type {
      Some(ref ty) => Some(Box::new(self.check_type(ty)?.unwrap().as_ty())),
      None => None,
    };

    let fn_type = FnValue::new(params, ret_type);

    Ok(Some(Type::Fn(fn_type).as_symbol()))
  }

  pub fn check_ident_type(&mut self, ast_type: &ast_type::IdentType) -> CkrResult {
    todo!()
  }
}
