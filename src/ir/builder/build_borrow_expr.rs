use crate::{
	ast,
	ir::{
		ir::{self, IrValue},
		Register,
	},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_borrow_expr(&mut self, borrow_expr: &ast::BorrowExpr) -> Register {
		let value = self.build_expr(&borrow_expr.expr);
		let type_id = self.get_type_id(borrow_expr.get_type_id());
		let dest = self.ir_ctx.new_register();
		let instr = ir::UnaryInstr { type_id, value, dest };
		if borrow_expr.mutable.is_some() {
			let instr = ir::Instr::BorrowMut(instr);
			self.ir_ctx.add_instr(instr);
		} else {
			let instr = ir::Instr::Borrow(instr);
			self.ir_ctx.add_instr(instr);
		}
		dest
	}
}
