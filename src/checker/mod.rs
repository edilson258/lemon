#![allow(dead_code, unused_variables)]

use crate::{
  ast::{self, Operator},
  diag::{self, Diag},
  range::Range,
};
use context::{scope::Symbol, Context};
use diags::errs::TypeErr;
use types::Type;

mod check_binary;
mod check_call;
mod check_expr;
mod check_fn;
mod check_if;
mod check_literal;
mod check_pipe;
mod check_stmt;
mod check_type;
pub mod context;
mod diags;
pub mod types;

type DiagResult<T> = Result<T, diag::Diag>;
type CkrResult = Result<Option<Symbol>, Diag>;

pub struct Checker<'ckr> {
  ctx: Context<'ckr>,
  diag_group: &'ckr mut diag::DiagGroup<'ckr>,
}

impl<'ckr> Checker<'ckr> {
  pub fn new(diag_group: &'ckr mut diag::DiagGroup<'ckr>, ctx: Context<'ckr>) -> Self {
    Self { ctx, diag_group }
  }

  pub fn check_program(&mut self, ast: &'ckr ast::Program) -> CkrResult {
    ast.stmts.split_last().map_or(Ok(None), |(last, rest)| {
      for stmt in rest {
        self.check_stmt(stmt)?;
      }
      self.check_stmt(last)
    })
  }
  pub fn check_binding(&mut self, binding: &'ckr ast::Binding) -> CkrResult {
    binding.ty.as_ref().map_or(Ok(None), |ty| self.check_type(ty))
  }

  pub fn operator_supported(&self, left: &Symbol, right: &Symbol, operator: &Operator) -> bool {
    let left = left.as_ref_ty();
    let right = right.as_ref_ty();
    if !left.can_operate_with(operator) || !right.can_operate_with(operator) {
      return false;
    }
    left.is_cmp_with(right)
  }

  pub fn take_common_type(&self, left: &Symbol, right: &Symbol) -> Type {
    match (left.as_ref_ty(), right.as_ref_ty()) {
      (Type::Float(l), Type::Float(r)) => Type::Float(l.higher_bits(r)),
      (Type::Numb(l), Type::Numb(r)) => Type::Numb(l.higher_bits(r)),
      (_, r) => r.clone(),
    }
  }

  pub fn assign_compatible(&self, expect: &Type, found: &Type, range: Range) -> Result<(), Diag> {
    if found.is_cmp_with(expect) {
      if !found.fits_into(expect) {
        return Err(TypeErr::out_of_range(expect, found, range));
      }
      return Ok(());
    }

    if !expect.eq(found) {
      return Err(TypeErr::mismatched_type(expect, found, range));
    }
    Ok(())
  }
}
