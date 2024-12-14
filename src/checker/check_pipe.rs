use crate::ast::{self};

use super::{diags::errs::TypeErr, Checker, CkrResult};

impl<'ckr> Checker<'ckr> {
  pub fn check_pipe(&mut self, pipe_expr: &'ckr ast::PipeExpr) -> CkrResult {
    let left_type = self.check_expr(&pipe_expr.left)?;
    if left_type.is_none() {
      return Err(TypeErr::not_found("left", pipe_expr.left.range()));
    }
    self.ctx.add_pipe_arg(left_type.unwrap());
    let right_type = self.check_expr(&pipe_expr.right)?;
    if let Some(right_type) = &right_type {
      let right_type = right_type.as_ref_ty();
      let pipe_areg = self.ctx.take_pipe_arg();
      if right_type.is_fn() && pipe_areg.is_some() {
        let arg = pipe_areg.unwrap();
        let fn_type = right_type.get_fn().unwrap();
        if fn_type.args.len() != 1 {
          let diag = TypeErr::mismatched_args(1, fn_type.args.len(), pipe_expr.right.range());
          return Err(diag);
        }
        if !fn_type.args[0].eq(&arg) {
          let diag = TypeErr::mismatched_args(1, fn_type.args.len(), pipe_expr.right.range());
          return Err(diag);
        }
        let ret_type = fn_type.get_ret().map(|ty| ty.as_symbol());
        return Ok(ret_type);
      }
    }
    Ok(right_type)
  }
}
