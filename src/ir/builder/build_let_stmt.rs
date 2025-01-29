use crate::{ast, ir::ir};

use super::Builder;

impl Builder<'_> {
	pub fn build_let_stmt(&mut self, let_stmt: &ast::LetStmt) {
		// todo: own on heap
		let type_id = self.get_type_id(let_stmt.get_type_id());
		let value = self.build_expr(&let_stmt.expr);
		let register = self.ir_ctx.new_register();

		self.ir_ctx.register_struct(value, register);

		if type_id.is_known() {
			// we don't need to transfer ownership jus copy
			let value = value.into();
			let instr = ir::StoreInstr { type_id, value, dest: register };
			self.ir_ctx.add_instr(instr.into());
		} else {
			let instr = ir::OwnInstr { type_id, value, dest: register };
			self.ir_ctx.add_instr(instr.into());
		}
		self.ir_ctx.add_value(let_stmt.lexeme(), register);
	}
}
