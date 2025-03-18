use crate::ast;
use crate::ir::{Instr, SallocInstr, UnInstr};

use super::Builder;

impl Builder<'_> {
	pub fn build_let_stmt(&mut self, let_stmt: &mut ast::LetStmt) {
		let type_id = self.build_type(let_stmt.type_id, let_stmt.get_range());
		let mut src = self.build_expr(&mut let_stmt.expr);
		let src = src.with_new_type(type_id);
		let name = let_stmt.lexeme();
		let dest = self.ctx.create_register(type_id);
		self.ctx.define_local_variable(name.to_string(), dest.clone());

		if !self.type_store.is_borrow(type_id) && src.is_register() {
			let value = self.resolve_value(src);
			if let Some(size) = self.is_need_heap_allocation(value.get_type()) {
				let unary_instr = UnInstr::new(dest.clone(), size.into());
				self.ctx.register_unbound_value(dest.clone());
				self.ctx.current_block.append_instr(Instr::Halloc(unary_instr));
			} else {
				let salloc = SallocInstr::new(dest.clone(), type_id);
				self.ctx.current_block.append_instr(salloc.into());
			}
			self.ctx.current_block.append_instr(Instr::Set(UnInstr::new(dest, value)));
			return;
		}

		if let Some(size) = self.is_need_heap_allocation(src.get_type()) {
			self.ctx.register_unbound_value(dest.clone());
			let unary_instr = UnInstr::new(dest.clone(), size.into());
			self.ctx.current_block.append_instr(Instr::Halloc(unary_instr));
		} else {
			let salloc = SallocInstr::new(dest.clone(), type_id);
			self.ctx.current_block.append_instr(salloc.into());
		}

		self.ctx.current_block.append_instr(Instr::Set(UnInstr::new(dest, src)));
	}
}
