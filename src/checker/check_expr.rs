use crate::{ast, range::TraitRange};

use super::{diags::errs::TypeErr, types::Type, Checker, CheckerResult};

impl<'a> Checker<'a> {
  pub fn check_expr(&mut self, expr: &ast::Expr) -> CheckerResult<Type> {
    match expr {
      ast::Expr::Literal(lit) => self.check_literal(lit),
      ast::Expr::Ident(ident) => self.check_ident(ident),
      ast::Expr::Unary(unary) => self.check_unary(unary),
      ast::Expr::Binary(binary) => self.check_binary(binary),
      ast::Expr::Group(group) => self.check_group(group),
      ast::Expr::Call(call) => self.check_call(call),
      ast::Expr::If(if_expr) => self.check_if(if_expr),
      ast::Expr::Break(break_expr) => self.check_break(break_expr),
      ast::Expr::Ret(ret) => self.check_ret(ret),
      ast::Expr::Assign(assign) => self.check_assign(assign),
      ast::Expr::While(while_expr) => self.check_while(while_expr),
      ast::Expr::For(for_expr) => self.check_for(for_expr),
      ast::Expr::Fn(fn_expr) => self.check_fn(fn_expr),
      ast::Expr::Skip(base_expr) => self.check_skip(base_expr),
      ast::Expr::Pipe(pipe_expr) => self.check_pipe(pipe_expr),
      ast::Expr::Import(import_expr) => self.check_import(import_expr),
    }
  }

  pub fn check_ident(&mut self, ident: &ast::Ident) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_unary(&mut self, unary: &ast::UnaryExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_binary(&mut self, binary: &ast::BinaryExpr) -> CheckerResult<Type> {
    let lt_type = match self.check_expr(&binary.left)? {
      Some(left) => left,
      None => return Ok(None),
    };

    let rt_type = match self.check_expr(&binary.right)? {
      Some(right) => right,
      None => return Ok(None),
    };

    if !lt_type.supports_operator(&binary.operator) {
      let diag = TypeErr::UnsupportedOperator(
        lt_type.to_string(),
        rt_type.to_string(),
        binary.operator.clone(),
        binary.range(),
      );
      return Err(diag.into());
    }

    if !rt_type.supports_operator(&binary.operator) {
      let diag = TypeErr::UnsupportedOperator(
        lt_type.to_string(),
        rt_type.to_string(),
        binary.operator.clone(),
        binary.range(),
      );
      return Err(diag.into());
    }

    return Ok(Some(rt_type));
  }

  // check group expression
  pub fn check_group(&mut self, group: &ast::GroupExpr) -> CheckerResult<Type> {
    todo!()
  }

  /// check call expression
  pub fn check_call(&mut self, call: &ast::CallExpr) -> CheckerResult<Type> {
    todo!()
  }

  /// check if expression
  pub fn check_if(&mut self, if_expr: &ast::IfExpr) -> CheckerResult<Type> {
    todo!()
  }

  /// check break expression
  pub fn check_break(&mut self, break_expr: &ast::BaseExpr) -> CheckerResult<Type> {
    todo!()
  }

  /// check return expression
  pub fn check_ret(&mut self, ret: &ast::RetExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_assign(&mut self, assign: &ast::AssignExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_while(&mut self, while_expr: &ast::WhileExpr) -> CheckerResult<Type> {
    todo!()
  }

  fn check_for(&mut self, for_expr: &ast::ForExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_fn(&mut self, fn_expr: &ast::FnExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_skip(&mut self, base_expr: &ast::BaseExpr) -> CheckerResult<Type> {
    todo!()
  }

  fn check_pipe(&mut self, pipe_expr: &ast::PipeExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_import(&mut self, import_expr: &ast::ImportExpr) -> CheckerResult<Type> {
    todo!()
  }
}
