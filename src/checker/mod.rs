#![allow(dead_code, unused_variables)]

use crate::{
  ast::{self, Operator},
  diag,
  range::{Range, TraitRange},
};
use types::Type;

mod check_expr;
mod check_fn;
mod check_literal;
mod check_stmt;
mod check_type;
// mod check_unary;
mod check_binary;
// mod check_group;
// mod check_call;
// mod check_if;
// mod check_break;
// mod check_ret;
// mod check_assign;
// mod check_while;
mod check_if;
pub mod context;
mod diags;
pub mod types;

type CheckerResult<T> = Result<Option<T>, diag::Diag>;

pub struct Checker<'a> {
  ctx: context::Context,
  diag_group: &'a mut diag::DiagGroup<'a>,
}

impl<'a> Checker<'a> {
  pub fn new(diag_group: &'a mut diag::DiagGroup<'a>, ctx: context::Context) -> Self {
    Self { ctx, diag_group }
  }

  pub fn check_program(&mut self, ast: &ast::Program) -> CheckerResult<Type> {
    ast.stmts.split_last().map_or(Ok(None), |(last, rest)| {
      for stmt in rest {
        self.check_stmt(stmt)?;
      }
      self.check_stmt(last)
    })
  }
  pub fn check_binding(&mut self, binding: &ast::Binding) -> CheckerResult<Type> {
    if let Some(ty) = &binding.ty {
      let kind = self.check_type(ty)?;
      return Ok(kind);
    }
    return Ok(None);
  }

  pub fn operator_supported(&self, left: &Type, right: &Type, operator: &Operator) -> bool {
    left.can_operated(operator) && right.can_operated(operator) && left.same_set(right)
  }

  pub fn range_of_last_stmt_or_block(&self, stmt: &Box<ast::Stmt>) -> Range {
    match stmt.as_ref().last_stmt_range() {
      Some(range) => range,
      None => stmt.as_ref().range(),
    }
  }

  pub fn resulting_type(&self, left: &Type, right: &Type) -> Type {
    match (left, right) {
      (Type::Float(l), Type::Float(r)) => Type::Float(l.higher_bits(&r)),
      (Type::Numb(l), Type::Numb(r)) => Type::Numb(l.higher_bits(&r)),
      (_, r) => r.clone(),
    }
  }
}
