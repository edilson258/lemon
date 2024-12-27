use crate::{ast, ir::ir};

use super::Builder;

impl Builder {
	pub fn build_let_stmt(&mut self, let_stmt: &ast::LetStmt) {
		let bind = self.build_binding(&let_stmt.name);
		let value = self.build_expr(&let_stmt.expr);
		let instr = ir::OwnInstr { value, type_id: bind.type_id, dest: bind.register };
		self.add_instr(ir::Instr::Own(instr));
	}

	pub fn build_binding(&mut self, bind: &ast::Binding) -> ir::Bind {
		let register = self.ctx.get_register();
		self.ctx.add_value(bind.lexeme(), register);
		let type_id = bind.type_id.unwrap();
		ir::Bind { register, type_id }
	}
}
