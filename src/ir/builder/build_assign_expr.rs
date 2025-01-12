use crate::{
	ast,
	ir::{ir, IrValue, Register},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_assign_expr(&mut self, binary: &ast::AssignExpr) -> Register {
		let value = self.build_expr(&binary.right).into();
		let dest = self.build_expr(&binary.left);
		let type_id = self.get_type_id(binary.get_type_id());
		let instr = ir::StoreInstr { type_id, value, dest };
		self.ir_ctx.add_instr(instr.into());
		dest
	}
}
