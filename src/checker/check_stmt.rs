use crate::ast::{self};

use super::{Checker, CkrResult};

impl<'ckr> Checker<'ckr> {
  pub fn check_stmt(&mut self, stmt: &'ckr ast::Stmt) -> CkrResult {
    match stmt {
      ast::Stmt::Fn(fn_stmt) => self.check_fn_stmt(fn_stmt),
      ast::Stmt::Expr(expr) => self.check_expr(expr),
      ast::Stmt::Let(let_stmt) => self.check_let_stmt(let_stmt),
      ast::Stmt::Block(block) => self.check_block_stmt(block),
    }
  }

  pub fn check_let_stmt(&mut self, let_stmt: &'ckr ast::LetStmt) -> CkrResult {
    let value_symbol = self.check_expr(&let_stmt.expr)?.unwrap();

    if let Some(binding) = self.check_binding(&let_stmt.name)? {
      let range = let_stmt.expr.range();
      self.assign_compatible(binding.as_ref_ty(), value_symbol.as_ref_ty(), range)?;
    }

    if let_stmt.is_mut() {
      self.ctx.add_value_mut(let_stmt.get_name(), value_symbol.as_ty());
    } else {
      self.ctx.add_value(let_stmt.get_name(), value_symbol.as_ty());
    }
    Ok(None)
  }

  pub fn check_block_stmt(&mut self, block: &'ckr ast::BlockStmt) -> CkrResult {
    for stmt in &block.stmts {
      let result = self.check_stmt(stmt)?;
      if result.is_some() {
        return Ok(result);
      }
    }
    Ok(None)
  }
}
