use crate::{
  ast::{self},
  range::TraitRange,
};

use super::{diags::errs::TypeErr, Checker, CkrResult};

impl<'ckr> Checker<'ckr> {
  pub fn check_call(&mut self, call: &'ckr ast::CallExpr) -> CkrResult {
    let callee_type = self.check_expr(&call.callee)?;
    if callee_type.is_none() {
      return Err(TypeErr::not_found("callee", call.callee.range()));
    }

    let callee_type = callee_type.unwrap();

    if !callee_type.as_ref_ty().is_fn() {
      let diag = TypeErr::expected_fn(callee_type.as_ref_ty(), call.callee.range());
      return Err(diag);
    }

    let fn_type = callee_type.as_ref_ty().get_fn().unwrap(); // we know it's a fn

    let pipe_areg = self.ctx.take_pipe_arg();
    if fn_type.args.len() != (call.args.len() + pipe_areg.as_ref().map_or(0, |_| 1)) {
      let expected = fn_type.args.len();
      let found = call.args.len();
      return Err(TypeErr::mismatched_args(expected, found, call.range()));
    }
    if let Some(pipe_type) = pipe_areg {
      let result = fn_type.args.split_first().map_or::<CkrResult, _>(Ok(None), |(first, rest)| {
        if !first.eq(&pipe_type) {
          return Err(TypeErr::mismatched_type(first, &pipe_type, call.range()));
        }
        for (expected, expr) in rest.iter().zip(call.args.iter()) {
          let found = self.check_expr(expr)?.unwrap();
          self.assign_compatible(expected, found.as_ref_ty(), expr.range())?;
        }
        Ok(None)
      });
      return result;
    }
    for (expected, expr) in fn_type.args.iter().zip(call.args.iter()) {
      let found = self.check_expr(expr)?.unwrap();
      self.assign_compatible(expected, found.as_ref_ty(), expr.range())?;
    }
    Ok(fn_type.get_ret().map(|ty| ty.as_symbol()))
  }
}
