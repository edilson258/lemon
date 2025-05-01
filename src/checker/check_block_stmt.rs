use crate::ast;

use super::{context::scope::ScopeKind, CheckResult, Checker};

impl Checker<'_> {
	pub fn check_block_stmt(&mut self, block: &mut ast::BlockStmt) -> CheckResult {
		// todo: warn unreachable code
		self.ctx.enter_scope(ScopeKind::block_scope());
		let mut ret_type = None;
		for stmt in block.stmts.iter_mut() {
			ret_type = self.check_stmt(stmt)?;
		}
		self.ctx.exit_scope();
		Ok(ret_type)
	}
}
