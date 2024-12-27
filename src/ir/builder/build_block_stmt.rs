use crate::ast;

use super::Builder;

impl Builder {
	pub fn build_block_stmt(&mut self, block_stmt: &ast::BlockStmt) {
		self.ctx.enter_scope();
		for stmt in block_stmt.stmts.iter() {
			self.build_stmt(stmt);
		}
		self.ctx.exit_scope();
	}
}
