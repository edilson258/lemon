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
		let other_block = then_block.next_block();

		let instr = ir::JmpIfInstr { cond, l0: then_block, l1: other_block };

		self.ir_ctx.add_instr(instr.into());

		self.ir_ctx.switch_to_block(then_block);

		self.build_block_stmt_without_blcok_id(&expr.then);

		if let Some(otherwise) = &expr.otherwise {
			self.ir_ctx.switch_to_block(other_block);
			self.build_block_stmt_without_blcok_id(otherwise);
		}

		if expr.otherwise.is_some() {
			self.ir_ctx.switch_to_block(other_block);
		} else {
			let merge_block = self.ir_ctx.new_block();
			self.ir_ctx.switch_to_block(merge_block);
		}
		// remove block id
		Register::new(0)
	}
}
