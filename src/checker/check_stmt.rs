use crate::ast::{self};

use super::{types::Type, CheckResult, Checker};

impl<'ckr> Checker<'ckr> {
  pub fn check_stmt(&mut self, stmt: &'ckr ast::Stmt) -> CheckResult<Type> {
    match stmt {
      ast::Stmt::Fn(fn_stmt) => self.check_fn_stmt(fn_stmt),
      ast::Stmt::Expr(expr) => self.check_expr(expr),
      ast::Stmt::Let(let_stmt) => self.check_let_stmt(let_stmt),
      ast::Stmt::Block(block) => self.check_block_stmt(block),
    }
  }

  pub fn check_let_stmt(&mut self, let_stmt: &'ckr ast::LetStmt) -> CheckResult<Type> {
    let value_type = self.check_expr(&let_stmt.expr)?.unwrap();

    if let Some(binding) = self.check_binding(&let_stmt.name)? {
      let range = let_stmt.expr.range();
      self.assign_compatible(&binding, &value_type, range)?;
    }

    self.ctx.add_value(let_stmt.get_name(), value_type);
    Ok(None)
  }

  pub fn check_block_stmt(&mut self, block: &'ckr ast::BlockStmt) -> CheckResult<Type> {
    block.stmts.split_last().map_or(Ok(None), |(last, rest)| {
      for stmt in rest {
        self.check_stmt(stmt)?;
      }
      self.check_stmt(last)
    })
  }
}
