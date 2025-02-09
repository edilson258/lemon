use crate::ast;

use super::Builder;

impl Builder<'_> {
	pub fn build_block_stmt(&mut self, block_stmt: &mut ast::BlockStmt) {
		block_stmt.stmts.iter_mut().for_each(|stmt| {
			self.build_stmt(stmt);
		});
	}
}
