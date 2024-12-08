#![allow(dead_code, unused_variables)]

use crate::{
  ast::{self, Operator},
  diag::{self, Diag},
  range::{Range, TraitRange},
};
use context::Context;
use diags::errs::TypeErr;
use types::Type;

mod check_binary;
mod check_expr;
mod check_fn;
mod check_if;
mod check_literal;
mod check_stmt;
mod check_type;
pub mod context;
mod diags;
pub mod types;

type CheckResult<T> = Result<Option<T>, diag::Diag>;
pub struct Checker<'ckr> {
  ctx: Context<'ckr>,
  diag_group: &'ckr mut diag::DiagGroup<'ckr>,
}

impl<'ckr> Checker<'ckr> {
  pub fn new(diag_group: &'ckr mut diag::DiagGroup<'ckr>, ctx: Context<'ckr>) -> Self {
    Self { ctx, diag_group }
  }

  pub fn check_program(&mut self, ast: &'ckr ast::Program) -> CheckResult<Type> {
    ast.stmts.split_last().map_or(Ok(None), |(last, rest)| {
      for stmt in rest {
        self.check_stmt(stmt)?;
      }
      self.check_stmt(last)
    })
  }
  pub fn check_binding(&mut self, binding: &'ckr ast::Binding) -> CheckResult<Type> {
    if let Some(ty) = &binding.ty {
      let kind = self.check_type(ty)?;
      return Ok(kind);
    }
    Ok(None)
  }

  pub fn operator_supported(&self, left: &Type, right: &Type, operator: &Operator) -> bool {
    left.can_operated(operator) && right.can_operated(operator) && left.same_set(right)
  }

  pub fn range_of_last_stmt_or_block(&self, stmt: &'ckr ast::Stmt) -> Range {
    match stmt.last_stmt_range() {
      Some(range) => range,
      None => stmt.range(),
    }
  }

  pub fn resulting_type(&self, left: &Type, right: &Type) -> Type {
    match (left, right) {
      (Type::Float(l), Type::Float(r)) => Type::Float(l.higher_bits(r)),
      (Type::Numb(l), Type::Numb(r)) => Type::Numb(l.higher_bits(r)),
      (_, r) => r.clone(),
    }
  }

  pub fn check_assing_bind(&self, expect: &Type, found: &Type, range: Range) -> Result<(), Diag> {
    if !found.eq(expect) {
      return Err(TypeErr::mismatched(found, expect, range));
    }

    if !found.fits_in(expect) && !found.same_set(expect) {
      return Err(TypeErr::out_of_range(expect, found, range));
    }
    Ok(())
  }
}
