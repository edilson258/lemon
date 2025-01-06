use crate::{ast, ir::ir};

use super::Builder;

impl Builder<'_> {
	pub fn build_const_del_stmt(&mut self, const_del: &ast::ConstDelStmt) {
		self.ctx.enter_comptime();
		let name = const_del.name.lexeme();
		let value = self.build_expr(&const_del.expr);
		let type_id = self.get_type_id(const_del.type_id);
		self.ctx.add_value(name, value.get_register().unwrap());
		let dest = self.ctx.get_register();
		let instr = ir::OwnInstr { type_id, value, dest };
		self.add_global(ir::Instr::Own(instr));
		self.ctx.exit_comptime();
	}
}
