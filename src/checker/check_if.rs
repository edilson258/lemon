use crate::{ast, diag};

use super::{diags::errs::TypeErr, types::Type, Checker, CheckerResult};

impl<'a> Checker<'a> {
  pub fn check_if_expr(&mut self, if_expr: &ast::IfExpr) -> CheckerResult<Type> {
    let cond_type = self.extract_cond_type(&if_expr.cond)?;
    let then_type = self.check_stmt(&if_expr.then)?;

    let otherwise_type = match &if_expr.otherwise {
      Some(stmt) => self.check_stmt(&stmt)?.unwrap(),
      None => return Ok(then_type),
    };

    if then_type.is_none() {
      let otherwise_range = self.range_of_last_stmt_or_block(&if_expr.otherwise.as_ref().unwrap());
      let diag = TypeErr::NoExpectedValue(&otherwise_type, otherwise_range);
      return Err(diag.into());
    }

    let then_type = then_type.unwrap();

    if then_type != otherwise_type {
      let otherwise_range = self.range_of_last_stmt_or_block(&if_expr.otherwise.as_ref().unwrap());
      let diag = TypeErr::Mismatched(&then_type, &otherwise_type, otherwise_range);
      return Err(diag.into());
    }

    let result = self.resulting_type(&then_type, &otherwise_type);

    Ok(Some(result))
  }

  pub fn extract_cond_type(&mut self, cond_expr: &ast::Expr) -> Result<Type, diag::Diag> {
    let cond_type = match self.check_expr(&cond_expr)? {
      Some(cond_type) if cond_type.is_bool() => cond_type,
      Some(cond_type) => {
        let range = cond_expr.range();
        let diag = TypeErr::Mismatched(&Type::Bool, &cond_type, range);
        return Err(diag.into());
      }
      None => {
        let range = cond_expr.range();
        let diag = TypeErr::ExpectedValue(&Type::Bool, range);
        return Err(diag.into());
      }
    };
    Ok(cond_type)
  }
}
