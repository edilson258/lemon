use super::Builder;
use crate::{ast, ir};

impl Builder<'_> {
	pub fn build_let_stmt(&mut self, let_stmt: &mut ast::LetStmt) {
		let range = let_stmt.get_range();
		let type_id = self.lookup_event_type(range);
		let mut src = self.build_expr(&mut let_stmt.expr).with_new_type(type_id);

		if !src.is_register() {
			let dest = self.create_basic_value(type_id);
			self.append_instr(ir::SallocInstr::new(dest.clone(), type_id).into(), Some(range));
			let instr = ir::UnInstr::new(dest.clone(), src.clone());
			self.append_instr(ir::Instr::Set(instr), Some(range));
			src = dest;
		}
		let src = src.with_new_type(type_id);
		let name = let_stmt.lexeme();
		self.ctx.define_local_variable(name.to_string(), src);
	}
}
