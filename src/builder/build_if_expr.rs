use crate::ast;
use crate::ir::{self, IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_if_expr(&mut self, if_expr: &mut ast::IfExpr) -> IrBasicValue {
		let cond = self.build_expr(&mut if_expr.cond);
		let then_block = self.ctx.block.new_block();
		let otherwise_block = if_expr.otherwise.as_ref().map(|_| self.ctx.block.new_block());
		let merge_block = self.ctx.block.new_block();

		let otherwise = otherwise_block.unwrap_or(merge_block);

		let instr = ir::JmpIfInstr::new(cond, then_block.into(), otherwise.into());
		self.ctx.block.add_instr(instr.into());

		self.ctx.block.switch_to_block(then_block);
		self.build_stmt(&mut if_expr.then);

		if let Some(otherwise) = &mut if_expr.otherwise {
			self.ctx.block.switch_to_block(otherwise_block.unwrap());
			self.build_stmt(otherwise);
		}
		self.ctx.block.switch_to_block(merge_block);
		IrBasicValue::default()
	}
}
