use crate::ast;

use super::Builder;

impl Builder<'_> {
	pub fn build_block_stmt(&mut self, block: &ast::BlockStmt) {
		self.ir_ctx.enter_scope();
		for stmt in block.stmts.iter() {
			self.build_stmt(stmt);
		}
		self.ir_ctx.exit_scope();
	}
}
