#![allow(dead_code, unused_variables)]

use crate::{ast, diag};
use types::Type;

mod check_expr;
mod check_literal;
mod check_stmt;
mod check_type;
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
    let mut rt = None;
    for stmt in &ast.stmts {
      rt = self.check_stmt(stmt)?;
    }
    Ok(rt)
  }
  pub fn check_binding(&mut self, binding: &ast::Binding) -> CheckerResult<Type> {
    if let Some(ty) = &binding.ty {
      let kind = self.check_type(ty)?;
      return Ok(kind);
    }
    return Ok(None);
  }
}
