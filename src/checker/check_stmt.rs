use crate::{ast, range::TraitRange};

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
  pub fn check_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) -> CheckerResult<Type> {
    todo!()
  }
  pub fn check_let_stmt(&mut self, let_stmt: &ast::LetStmt) -> CheckerResult<Type> {
    let value_type = match self.check_expr(&let_stmt.expr)? {
      Some(value) => value,
      None => return Ok(None),
    };

    if let Some(binding) = self.check_binding(&let_stmt.name)? {
      if !binding.eq(&value_type) {
        let range = let_stmt.name.ident.range();
        let diag = TypeErr::Mismatched(binding.to_string(), value_type.to_string(), range);
        return Err(diag.into());
      }
    }

    self.ctx.add_value(let_stmt.get_name().to_owned(), value_type);
    Ok(None)
  }

  pub fn check_block_stmt(&mut self, block: &ast::BlockStmt) -> CheckerResult<Type> {
    let mut rt = None;
    for stmt in &block.stmts {
      rt = self.check_stmt(&stmt)?;
    }
    Ok(rt)
  }
}
