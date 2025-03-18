use crate::ast::{self};

use super::{context::scope::ScopeKind, types::TypeId, Checker, TyResult};

impl Checker<'_> {
	pub fn check_block_stmt(&mut self, block: &mut ast::BlockStmt) -> TyResult<TypeId> {
		// todo: warn unreachable code
		self.ctx.enter_scope(ScopeKind::block_scope());
		let mut ret_type = TypeId::UNIT;
		for stmt in block.stmts.iter_mut() {
			self.ctx.flow.set_paths_return(stmt.ends_with_ret());
			ret_type = self.check_stmt(stmt)?;
		}
		self.ctx.exit_scope();
		Ok(ret_type)
	}
}
