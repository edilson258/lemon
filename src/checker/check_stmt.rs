use crate::ast::{self};

use super::{diags::errs::TypeErr, types::Type, Checker, CheckerResult};

impl<'a> Checker<'a> {
  pub fn check_stmt(&mut self, stmt: &ast::Stmt) -> CheckerResult<Type> {
    match stmt {
      ast::Stmt::Fn(fn_stmt) => self.check_fn_stmt(fn_stmt),
      ast::Stmt::Expr(expr) => self.check_expr(expr),
      ast::Stmt::Let(let_stmt) => self.check_let_stmt(let_stmt),
      ast::Stmt::Block(block) => self.check_block_stmt(block),
    }
  }

  pub fn check_let_stmt(&mut self, let_stmt: &ast::LetStmt) -> CheckerResult<Type> {
    let value_type = self.check_expr(&let_stmt.expr)?.unwrap(); // we throw error in parser
    match self.check_binding(&let_stmt.name)? {
      Some(binding) if !binding.eq(&value_type) && !value_type.fits_in(&binding) => {
        let range = let_stmt.expr.range();
        // if are the sema set...  handle range out err...
        if binding.same_set(&value_type) {
          let diag = TypeErr::OutOfRange(&value_type, &binding, range);
          return Err(diag.into());
        }

        let diag = TypeErr::Mismatched(&binding, &value_type, range);
        Err(diag.into())
      }
      _ => {
        self.ctx.add_value(let_stmt.get_name().to_owned(), value_type);
        Ok(None)
      }
    }
  }

  pub fn check_block_stmt(&mut self, block: &ast::BlockStmt) -> CheckerResult<Type> {
    block.stmts.split_last().map_or(Ok(None), |(last, rest)| {
      for stmt in rest {
        self.check_stmt(stmt)?;
      }
      self.check_stmt(last)
    })
  }
}
