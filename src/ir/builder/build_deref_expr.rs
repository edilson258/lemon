use crate::{
	ast,
	ir::{ir, Instr, IrValue, Register},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_deref_expr(&mut self, deref_exper: &ast::DerefExpr) -> Register {
		let value = self.build_expr(&deref_exper.expr);
		let type_id = self.get_type_id(deref_exper.type_id);
		let dest = self.ir_ctx.new_register();
		let instr = ir::UnaryInstr { type_id, value, dest };
		self.ir_ctx.add_instr(Instr::Deref(instr));
		dest
	}
}
