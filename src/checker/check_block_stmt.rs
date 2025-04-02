use crate::{
	ast::{self},
	message::MessageResult,
};

use super::{context::scope::ScopeKind, types::TypeId, Checker};

impl Checker<'_> {
	pub fn check_block_stmt(&mut self, block: &mut ast::BlockStmt) -> MessageResult<TypeId> {
		// todo: warn unreachable code
		self.ctx.enter_scope(ScopeKind::block_scope());
		let mut ret_type = TypeId::UNIT;
		for stmt in block.stmts.iter_mut() {
			ret_type = self.check_stmt(stmt)?;
		}
		self.ctx.exit_scope();
		Ok(ret_type)
	}
}
