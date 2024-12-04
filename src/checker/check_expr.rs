use crate::ast::{self, Operator};

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
      ast::Expr::Fn(fn_expr) => self.check_fn_expr(fn_expr),
      ast::Expr::Skip(base_expr) => self.check_skip(base_expr),
      ast::Expr::Pipe(pipe_expr) => self.check_pipe(pipe_expr),
      ast::Expr::Import(import_expr) => self.check_import(import_expr),
    }
  }

  pub fn check_ident(&mut self, ident: &ast::Ident) -> CheckerResult<Type> {
    if let Some(value) = self.ctx.get_value(&ident.text) {
      return Ok(Some(value.get_kind()));
    }
    let range = ident.range.clone();
    let diag = TypeErr::NotFound(&ident.text, range);
    return Err(diag.into());
  }
  pub fn check_unary(&mut self, unary: &ast::UnaryExpr) -> CheckerResult<Type> {
    todo!()
  }
  pub fn check_binary(&mut self, binary: &ast::BinaryExpr) -> CheckerResult<Type> {
    let left_type = self.check_expr(&binary.left)?.unwrap();
    let right_type = self.check_expr(&binary.right)?.unwrap();
    let operator = &binary.operator;
    let range = binary.range.clone();
    if !self.operator_supported(&left_type, &right_type, operator) {
      let diag = TypeErr::NotSupported(&left_type, &right_type, &operator, range);
      return Err(diag.into());
    }

    Ok(Some(self.resulting_type(left_type, right_type)))
  }

  fn operator_supported(&self, left: &Type, right: &Type, operator: &Operator) -> bool {
    left.can_operated(operator) && right.can_operated(operator) && left.same_set(right)
  }

  fn resulting_type(&self, left: Type, right: Type) -> Type {
    match (left, right) {
      (Type::Float(l), Type::Float(r)) => Type::Float(l.higher_bits(&r)),
      (Type::Numb(l), Type::Numb(r)) => Type::Numb(l.higher_bits(&r)),
      (_, r) => r,
    }
  }

  pub fn check_group(&mut self, group: &ast::GroupExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_call(&mut self, call: &ast::CallExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_if(&mut self, if_expr: &ast::IfExpr) -> CheckerResult<Type> {
    todo!()
  }

  pub fn check_break(&mut self, break_expr: &ast::BaseExpr) -> CheckerResult<Type> {
    todo!()
  }

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
