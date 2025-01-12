use crate::{
	ast,
	ir::ir::{self},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_ret_stmt(&mut self, ret_stmt: &ast::RetStmt) {
		let value = ret_stmt.expr.as_ref().map(|expr| self.build_expr(expr));
		let type_id = self.get_type_id(ret_stmt.get_type_id());
		let instr = ir::RetInstr { value, type_id };
		self.ir_ctx.add_instr(instr.into());
	}
}
