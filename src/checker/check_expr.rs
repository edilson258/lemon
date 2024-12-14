use crate::ast::{self};

use super::{context::scope::Symbol, diags::errs::TypeErr, Checker, CkrResult};

impl<'ckr> Checker<'ckr> {
  pub fn check_expr(&mut self, expr: &'ckr ast::Expr) -> CkrResult {
    match expr {
      ast::Expr::Literal(lit) => self.check_literal(lit),
      ast::Expr::Ident(ident) => self.check_ident(ident),
      ast::Expr::Unary(unary) => self.check_unary(unary),
      ast::Expr::Binary(binary) => self.check_binary_expr(binary),
      ast::Expr::Group(group) => self.check_group(group),
      ast::Expr::Call(call) => self.check_call(call),
      ast::Expr::If(if_expr) => self.check_if_expr(if_expr),
      ast::Expr::Break(break_expr) => self.check_break(break_expr),
      ast::Expr::Ret(ret) => self.check_ret(ret),
      ast::Expr::Assign(assign) => self.check_assign(assign),
      ast::Expr::While(while_expr) => self.check_while(while_expr),
      ast::Expr::For(for_expr) => self.check_for(for_expr),
      ast::Expr::Fn(fn_expr) => self.check_fn_expr(fn_expr),
      ast::Expr::Skip(base_expr) => self.check_skip(base_expr),
      ast::Expr::Pipe(pipe_expr) => self.check_pipe(pipe_expr),
      ast::Expr::Import(import_expr) => self.check_import(import_expr),
    }
  }

  pub fn check_assign(&mut self, assign: &'ckr ast::AssignExpr) -> CkrResult {
    // todo: improve this
    let left_type = self.check_expr(&assign.left)?;
    let left_type = left_type.unwrap();

    if let Symbol::Value(value) = &left_type {
      if !value.is_mutable() {
        let diag = TypeErr::cannot_mutate(&value.name, assign.left.range());
        return Err(diag);
      }

      let right_type = self.check_expr(&assign.right)?;

      if right_type.is_none() {
        return Err(TypeErr::not_found("right", assign.right.range()));
      }
      let right_type = right_type.unwrap();

      if value.get_type() != right_type.as_ref_ty() {
        let range = assign.right.range();
        let diag = TypeErr::mismatched_type(value.get_type(), right_type.as_ref_ty(), range);
        return Err(diag);
      }
    }
    Ok(None)
  }

  pub fn check_ident(&mut self, ident: &'ckr ast::Ident) -> CkrResult {
    if let Some(value) = self.ctx.get_symbol(&ident.text) {
      return Ok(Some(value));
    }
    let range = ident.range.clone();
    Err(TypeErr::not_found(&ident.text, range))
  }

  pub fn check_ret(&mut self, ret: &'ckr ast::RetExpr) -> CkrResult {
    if let Some(value) = &ret.value {
      return self.check_expr(value);
    }
    Ok(None)
  }

  pub fn check_unary(&mut self, unary: &'ckr ast::UnaryExpr) -> CkrResult {
    todo!()
  }

  pub fn check_group(&mut self, group: &'ckr ast::GroupExpr) -> CkrResult {
    todo!()
  }

  pub fn check_break(&mut self, break_expr: &'ckr ast::BaseExpr) -> CkrResult {
    todo!()
  }
  pub fn check_while(&mut self, while_expr: &'ckr ast::WhileExpr) -> CkrResult {
    todo!()
  }

  fn check_for(&mut self, for_expr: &'ckr ast::ForExpr) -> CkrResult {
    todo!()
  }

  pub fn check_skip(&mut self, base_expr: &'ckr ast::BaseExpr) -> CkrResult {
    todo!()
  }

  pub fn check_import(&mut self, import_expr: &'ckr ast::ImportExpr) -> CkrResult {
    todo!()
  }
}
