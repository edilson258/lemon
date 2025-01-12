use crate::{
	ast,
	ir::{
		ir::{self, IrValue},
		Register,
	},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_if_expr(&mut self, expr: &ast::IfExpr) -> Register {
		let cond = self.build_expr(&expr.cond);

		let then_block = self.ir_ctx.new_block();
		let merge_block = self.ir_ctx.new_block();

		let other_block = if expr.otherwise.is_some() { self.ir_ctx.new_block() } else { merge_block };

		let instr = ir::JmpIfInstr { cond, l0: then_block, l1: other_block };

		self.ir_ctx.add_instr(instr.into());

		self.ir_ctx.switch_to_block(then_block);

		self.build_stmt(&expr.then);

		if let Some(otherwise) = &expr.otherwise {
			self.ir_ctx.switch_to_block(other_block);
			self.build_stmt(otherwise);
		}

		self.ir_ctx.switch_to_block(merge_block);

		Register::new(0)
	}
}
