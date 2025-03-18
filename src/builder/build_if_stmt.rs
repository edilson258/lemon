use crate::ast;
use crate::ir::{self};

use super::Builder;

impl Builder<'_> {
	pub fn build_if_stmt(&mut self, if_expr: &mut ast::IfStmt) {
		let cond = self.build_expr(&mut if_expr.cond);
		let then_block = self.ctx.current_block.create_new_block();

		#[rustfmt::skip]
		let otherwise_block = if_expr.otherwise.as_ref().map(|_|
			self.ctx.current_block.create_new_block()
		);

		let merge_block = self.ctx.current_block.create_new_block();
		let otherwise = otherwise_block.unwrap_or(merge_block);

		let instr = ir::JmpIfInstr::new(cond, then_block.into(), otherwise.into());
		self.ctx.current_block.append_instr(instr.into());

		self.ctx.current_block.switch_to_label(then_block);
		self.build_stmt(&mut if_expr.then);
		if !self.ctx.current_block.has_returned {
			let jump = ir::JmpInstr::new(merge_block.into());
			self.ctx.current_block.append_instr(jump.into());
		}

		if let Some(otherwise) = &mut if_expr.otherwise {
			self.ctx.current_block.switch_to_label(otherwise_block.unwrap());
			self.build_stmt(otherwise);

			if !self.ctx.current_block.has_returned {
				let jump = ir::JmpInstr::new(merge_block.into());
				self.ctx.current_block.append_instr(jump.into());
			}
		}
		self.ctx.current_block.switch_to_label(merge_block);
	}
}
