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
			self.add_instr(ir::Instr::Ret(register));
		} else {
			self.add_instr(ir::Instr::Ret(None));
		}
	}
}
