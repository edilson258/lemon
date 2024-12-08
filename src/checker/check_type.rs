use crate::ast::ast_type;

use super::{
  types::{FloatValue, FnValue, NumbValue, Type},
  CheckResult, Checker,
};

impl<'ckr> Checker<'ckr> {
  pub fn check_type(&mut self, ast_type: &ast_type::AstType) -> CheckResult<Type> {
    match ast_type {
      ast_type::AstType::Bool(bool) => Ok(Some(Type::Bool)),
      ast_type::AstType::Char(char) => Ok(Some(Type::Char)),
      ast_type::AstType::String(string) => Ok(Some(Type::String)),
      ast_type::AstType::Float(float) => Ok(Some(Type::Float(FloatValue { bits: float.bits }))),
      ast_type::AstType::Numb(numb) => self.check_numb_type(numb),
      ast_type::AstType::Fn(fn_type) => self.check_fn_type(fn_type),
      ast_type::AstType::Ident(ident) => self.check_ident_type(ident),
    }
  }

  pub fn check_numb_type(&mut self, ast_type: &ast_type::NumbType) -> CheckResult<Type> {
    let bits = if ast_type.bits >= 8 { Some(ast_type.bits) } else { None };
    let numb = NumbValue { bits, signed: ast_type.signed };
    Ok(Some(Type::Numb(numb)))
  }
  pub fn check_fn_type(&mut self, ast_type: &ast_type::FnType) -> CheckResult<Type> {
    let mut params = Vec::with_capacity(ast_type.params.len());

    for param in ast_type.params.iter() {
      params.push(self.check_type(param)?.unwrap()); // we expect the type to be checked
    }

    let ret_type = match ast_type.ret_type {
      Some(ref ty) => Some(Box::new(self.check_type(ty)?.unwrap())), // we expect the type to be checked
      None => None,
    };

    let fn_type = FnValue::new(params, ret_type);

    Ok(Some(Type::Fn(fn_type)))
  }

  pub fn check_ident_type(&mut self, ast_type: &ast_type::IdentType) -> CheckResult<Type> {
    todo!()
  }
}
