use crate::{
	ast,
	ir::{Instr, IrValue, SallocInstr, UnInstr},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_let_stmt(&mut self, let_stmt: &mut ast::LetStmt) {
		let type_id = self.build_type(let_stmt.type_id, let_stmt.get_range());

		let src = self.build_expr(&mut let_stmt.expr);
		let src = src.with_new_type(type_id);

		let name = let_stmt.lexeme();
		let dest = IrValue::new(name.to_owned(), type_id);
		let salloc = SallocInstr::new(dest, type_id);

		self.ctx.block.add_instr(salloc.into());

		let dest = IrValue::new(name.to_owned(), type_id);

		self.ctx.block.add_instr(Instr::Set(UnInstr::new(dest, src)));

		// let un_instr = UnInstr::new(dest, src);
		// self.ctx.block.add_instr(Instr::Mov(un/_instr));
	}
}
