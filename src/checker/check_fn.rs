use crate::{
  ast::{self, ast_type},
  diag::Diag,
  range::Range,
};

use super::{
  diags::errs::TypeErr,
  types::{FnValue, Type},
  CheckResult, Checker,
};

impl<'ckr> Checker<'ckr> {
  pub fn check_fn_stmt(&mut self, fn_stmt: &'ckr ast::FnStmt) -> CheckResult<Type> {
    let params_type = self.extract_params(&fn_stmt.params)?;
    let ret_type = self.extract_ret_type(&fn_stmt.ret_type)?;

    let params = params_type.iter().map(|p| p.1.to_owned()).collect();

    let fn_type = FnValue::new(params, ret_type.clone().map(Box::new));

    self.ctx.add_value(fn_stmt.text(), Type::Fn(fn_type));

    self.with_new_scope(|checker| {
      checker.register_params(&params_type);
      checker.check_fn_body(&fn_stmt.body, &ret_type)
    })
  }

  pub fn check_fn_expr(&mut self, fn_expr: &'ckr ast::FnExpr) -> CheckResult<Type> {
    let params_type = self.extract_params(&fn_expr.params)?;
    let ret_type = self.extract_ret_type(&fn_expr.ret_type)?;

    self.with_new_scope(|checker| {
      checker.register_params(&params_type);
      checker.check_fn_body(&fn_expr.body, &ret_type)?;
      let params = params_type.iter().map(|p| p.1.to_owned()).collect();
      let fn_type = FnValue::new(params, ret_type.map(Box::new));
      Ok(Some(Type::Fn(fn_type)))
    })
  }

  fn extract_params(
    &mut self,
    binds: &'ckr [ast::Binding],
  ) -> Result<Vec<(&'ckr str, Type)>, Diag> {
    let mut params_type = Vec::with_capacity(binds.len());
    for binding in binds {
      let kind = self.check_binding(binding)?.unwrap();
      params_type.push((binding.text(), kind));
    }
    Ok(params_type)
  }

  fn extract_ret_type(&mut self, ret_type: &Option<ast_type::AstType>) -> CheckResult<Type> {
    match ret_type {
      Some(ty) => Ok(self.check_type(ty)?),
      None => Ok(None),
    }
  }

  fn with_new_scope<F>(&mut self, execute: F) -> CheckResult<Type>
  where
    F: FnOnce(&mut Self) -> CheckResult<Type>,
  {
    self.ctx.enter_scope();
    let result = execute(self);
    self.ctx.exit_scope();
    result
  }

  fn register_params(&mut self, params: &[(&'ckr str, Type)]) {
    params.iter().for_each(|(name, ty)| {
      self.ctx.add_value(name, ty.clone());
    })
  }

  fn check_fn_body(&mut self, body: &'ckr ast::Stmt, ret_type: &Option<Type>) -> CheckResult<Type> {
    let body_type = self.check_stmt(body)?;
    let range = self.final_or_block_range(body);
    self.check_ret_type(ret_type, body_type, range)?;
    Ok(None)
  }


  #[rustfmt::skip]
  fn check_ret_type(&mut self, ret_ty: &Option<Type>, body_ty: Option<Type>, range: Range) -> Result<(), Diag> {
    match (ret_ty, body_ty) {
      (Some(ret), Some(body)) => Ok(self.assign_compatible(ret, &body, range)?),
      (Some(ret), None) => Err(TypeErr::expected_value(ret, range)),
      (None, Some(body)) => Err(TypeErr::no_expected_value(&body, range)),
      _ => Ok(()),
    }
  }
}
