use crate::{
	ast,
	ir::{
		ir::{self},
		Value,
	},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_import_expr(&mut self, ret_stmt: &ast::ImportExpr) -> Value {
		let module = ret_stmt.get_path().split("/").last().unwrap().to_string();
		let dest = self.ctx.get_register();
		let instr = ir::ImportInstr::new(module);
		self.add_instr(ir::Instr::Import(instr));
		Value::Register(dest)
	}
}
