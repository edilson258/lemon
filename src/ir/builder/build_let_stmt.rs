use crate::{ast, checker::types::TypeId, ir::ir};

use super::Builder;

impl Builder<'_> {
	pub fn build_let_stmt(&mut self, let_stmt: &ast::LetStmt) {
		let bind = self.build_binding(&let_stmt.name);
		let value = self.build_expr(&let_stmt.expr);
		if bind.type_id == TypeId::PRINTLN {
			return;
		}
		let instr = ir::OwnInstr { value, type_id: bind.type_id, dest: bind.register };
		self.add_instr(ir::Instr::Own(instr));
	}

	pub fn build_binding(&mut self, bind: &ast::Binding) -> ir::Bind {
		let register = self.ctx.get_register();
		self.ctx.add_value(bind.lexeme(), register);
		let type_id = self.get_type_id(bind.type_id);
		self.can_free_value(register, type_id);
		ir::Bind { register, type_id }
	}
}
