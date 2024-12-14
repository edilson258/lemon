use crate::{ast, diag, range::TraitRange};

use super::{context::scope::Symbol, diags::errs::TypeErr, types::Type, Checker, CkrResult};

impl<'ckr> Checker<'ckr> {
  pub fn check_if_expr(&mut self, if_expr: &'ckr ast::IfExpr) -> CkrResult {
    let cond_type = self.extract_cond_type(&if_expr.cond)?;
    let then_type = self.check_stmt(&if_expr.then)?;

    let otherwise_type = match &if_expr.otherwise {
      Some(stmt) => self.check_stmt(stmt)?,
      None => return Ok(then_type),
    };

    match (then_type, otherwise_type) {
      (Some(then_type), Some(otherwise_type)) => {
        let result = self.take_common_type(&cond_type, &then_type);
        Ok(Some(result.as_symbol()))
      }
      (Some(then_type), None) => {
        let range = if_expr.otherwise.as_ref().unwrap().range();
        Err(TypeErr::expected_value(then_type.as_ref_ty(), range))
      }
      (None, Some(otherwise_type)) => {
        let range = if_expr.then.range();
        Err(TypeErr::expected_value(otherwise_type.as_ref_ty(), range))
      }
      (None, None) => Ok(None),
    }
  }

  pub fn extract_cond_type(&mut self, cond_expr: &'ckr ast::Expr) -> Result<Symbol, diag::Diag> {
    let cond_type = match self.check_expr(cond_expr)? {
      Some(cond_type) if cond_type.as_ref_ty().is_bool() => cond_type,
      Some(cond_type) => {
        return Err(TypeErr::mismatched_type(
          &Type::Bool,
          cond_type.as_ref_ty(),
          cond_expr.range(),
        ));
      }
      None => {
        return Err(TypeErr::expected_value(&Type::Bool, cond_expr.range()));
      }
    };
    Ok(cond_type)
  }
}
