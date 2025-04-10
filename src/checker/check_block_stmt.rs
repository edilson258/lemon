use crate::{
	ast::{self},
	message::MessageResult,
};

use super::{context::scope::ScopeKind, Checker, TypedValue};

impl Checker<'_> {
	pub fn check_block_stmt(&mut self, block: &mut ast::BlockStmt) -> MessageResult<TypedValue> {
		// todo: warn unreachable code
		self.ctx.enter_scope(ScopeKind::block_scope());
		let mut ret_type = TypedValue::default();
		for stmt in block.stmts.iter_mut() {
			ret_type = self.check_stmt(stmt)?;
		}
		self.ctx.exit_scope();
		Ok(ret_type)
	}
}
