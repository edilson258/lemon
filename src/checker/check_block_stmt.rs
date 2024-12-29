use crate::ast::{self};

use super::{context::scope::ScopeType, diags::TypeCheckWarn, types::TypeId, Checker, TypeResult};

impl Checker<'_> {
	pub fn check_block_stmt(&mut self, block: &mut ast::BlockStmt) -> TypeResult<TypeId> {
		// todo: warn unreachable code
		self.ctx.enter_scope(ScopeType::new_block());
		let mut ret_type = TypeId::NOTHING;
		for stmt in block.stmts.iter_mut() {
			self.ctx.flow.set_paths_return(stmt.ends_with_ret());

			if ret_type.is_nothing() {
				ret_type = self.check_stmt(stmt)?;
				continue;
			}
			self.ctx.flow.set_unreachable(true);
			let diag = TypeCheckWarn::unreachable(stmt.get_range());
			self.diag_group.add(diag);
		}
		self.ctx.exit_scope();
		Ok(ret_type)
	}
}
