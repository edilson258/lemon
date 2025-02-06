use crate::{
	ast,
	ir::ir::{self},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_ret_stmt(&mut self, ret_stmt: &ast::RetStmt) {
		let value = ret_stmt.expr.as_ref().map(|expr| self.build_expr(expr));
		let type_id = match self.ir_ctx.get_ret_type() {
			Some(type_id) => *type_id,
			None => self.get_type_id(ret_stmt.get_type_id()),
		};

		if !type_id.is_known() {
			// todo: void or unit is known type
			self.ir_ctx.set_ret_owner(value.unwrap());
		}

		let instr = ir::RetInstr { value, type_id };
		self.ir_ctx.add_instr(instr.into());
	}
}
