use crate::ast;
use crate::ir::{Instr, SallocInstr, UnInstr};

use super::Builder;

impl Builder<'_> {
	pub fn build_let_stmt(&mut self, let_stmt: &mut ast::LetStmt) {
		let type_id = self.build_type(let_stmt.type_id, let_stmt.get_range());
		let mut src = self.build_expr(&mut let_stmt.expr);
		let src = src.with_new_type(type_id);

		let name = let_stmt.lexeme();

		let dest = self.ctx.new_register(type_id);
		self.ctx.add_local(name.to_string(), dest.clone());

		if let Some(basic_type) = self.type_store.get_type(type_id) {
			if !basic_type.is_borrow() && src.is_register() {
				let value = self.resolve_value(src);
				let salloc = SallocInstr::new(dest.clone(), type_id);

				self.ctx.block.add_instr(salloc.into());
				self.ctx.block.add_instr(Instr::Set(UnInstr::new(dest, value)));
				return;
			}
		}

		let salloc = SallocInstr::new(dest.clone(), type_id);
		self.ctx.block.add_instr(salloc.into());
		self.ctx.block.add_instr(Instr::Set(UnInstr::new(dest, src)));
	}
}
