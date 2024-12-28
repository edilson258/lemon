use crate::{
	ast,
	ir::ir::{self},
};

use super::Builder;

impl Builder {
	pub fn build_ret_stmt(&mut self, ret_stmt: &ast::RetStmt) {
		if let Some(expr) = &ret_stmt.expr {
			let value = self.build_expr(expr);
			let register = value.get_register();
			let instr = ir::RetInstr::new(ret_stmt.type_id, register);
			self.add_instr(ir::Instr::Ret(instr));
		} else {
			let instr = ir::RetInstr::new(ret_stmt.type_id, None);
			self.add_instr(ir::Instr::Ret(instr));
		}
	}
}
