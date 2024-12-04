use crate::{
  ast,
  range::{Range, TraitRange},
};

use super::{
  diags::errs::TypeErr,
  types::{FnValue, Type},
  Checker, CheckerResult,
};

impl<'a> Checker<'a> {
  pub fn check_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) -> CheckerResult<Type> {
    let mut params_type = Vec::with_capacity(fn_stmt.params.len());

    for binding in &fn_stmt.params {
      let kind = self.check_binding(binding)?.unwrap();
      params_type.push((binding.text(), kind));
    }

    let ret_type = match &fn_stmt.ret_type {
      Some(ret) => Some(Box::new(self.check_type(ret)?.unwrap())), // we can unwrap here because we
      None => None,
    };

    let params = params_type.iter().map(|(_, ty)| ty.to_owned()).collect();

    let fn_type = FnValue::new(params, ret_type.clone());

    self.ctx.add_value(fn_stmt.text().to_owned(), Type::Fn(fn_type));

    self.ctx.enter_scope();

    // fn scope
    for (name, ty) in params_type {
      self.ctx.add_value(name.to_owned(), ty);
    }

    let body_range = fn_stmt.body.last_stmt_range().unwrap_or_else(|| fn_stmt.body.range());

    let body_type = self.check_stmt(&fn_stmt.body)?;

    self.check_fn_ret_type(ret_type, body_type, body_range)?;

    self.ctx.exit_scope();
    Ok(None)
  }

  pub fn check_fn_expr(&mut self, fn_expr: &ast::FnExpr) -> CheckerResult<Type> {
    let mut params_type = Vec::with_capacity(fn_expr.params.len());

    for binding in &fn_expr.params {
      let kind = self.check_binding(binding)?.unwrap();
      params_type.push((binding.text(), kind));
    }

    let ret_type = match &fn_expr.ret_type {
      Some(ret) => Some(Box::new(self.check_type(ret)?.unwrap())), // we can unwrap here because we
      None => None,
    };

    let params = params_type.iter().map(|(_, ty)| ty.to_owned()).collect();

    let fn_type = FnValue::new(params, ret_type.clone());

    self.ctx.enter_scope();
    // fn scope
    for (name, ty) in params_type {
      self.ctx.add_value(name.to_owned(), ty);
    }
    let body_type = self.check_stmt(&fn_expr.body)?;

    let body_range = fn_expr.body.last_stmt_range().unwrap_or_else(|| fn_expr.body.range());

    self.check_fn_ret_type(ret_type, body_type, body_range)?;

    self.ctx.exit_scope();
    Ok(Some(Type::Fn(fn_type)))
  }

  pub fn check_fn_ret_type(
    &mut self,
    ret_type: Option<Box<Type>>,
    body_type: Option<Type>,
    range: Range,
  ) -> CheckerResult<()> {
    // todo: we need to exit the scope here?
    match (ret_type, body_type) {
      (Some(ret), Some(body)) if !body.fits_in(&ret) => {
        let diag = TypeErr::Mismatched(&ret, &body, range);
        self.ctx.exit_scope(); //  ensure that for every fn, we exit the scope
        return Err(diag.into());
      }
      (Some(ret), None) => {
        let diag = TypeErr::ExpectedValue(&ret, range);
        self.ctx.exit_scope(); //  ensure that for every fn, we exit the scope
        return Err(diag.into());
      }
      (None, Some(body)) => {
        let diag = TypeErr::NoExpectedValue(&body, range);
        self.ctx.exit_scope(); //  ensure that for every fn, we exit the scope
        return Err(diag.into());
      }
      _ => {}
    }
    return Ok(None);
  }
}
