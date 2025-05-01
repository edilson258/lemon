use crate::ast;
use crate::ir::{self};

use super::Builder;

impl Builder<'_> {
	pub fn build_if_stmt(&mut self, if_expr: &mut ast::IfStmt) {
		let range = if_expr.get_range();
		let cond = self.build_expr(&mut if_expr.cond);
		let then_block = self.ctx.current_block.create_new_block();

		#[rustfmt::skip]
		let otherwise_block = if_expr.otherwise.as_ref().map(|_|
			self.ctx.current_block.create_new_block()
		);

		let merge_block = self.ctx.current_block.create_new_block();
		let otherwise = otherwise_block.unwrap_or(merge_block);

		let instr = ir::JmpIfInstr::new(cond, then_block.into(), otherwise.into());
		let cond_range = if_expr.cond.get_range();
		let result = self.ctx.current_block.append_instr(instr.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(cond_range).report(self.loader)
		});

		let result = self.ctx.current_block.switch_to_label(then_block);
		let then_range = if_expr.then.get_range();
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(then_range).report(self.loader)
		});

		self.build_stmt(&mut if_expr.then);
		if !self.ctx.current_block.has_returned {
			let jump = ir::JmpInstr::new(merge_block.into());
			let result = self.ctx.current_block.append_instr(jump.into());
			result.unwrap_or_else(|message| {
				message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
			});
		}

		if let Some(otherwise) = &mut if_expr.otherwise {
			let result = self.ctx.current_block.switch_to_label(otherwise_block.unwrap());
			let otherwise_range = otherwise.get_range();
			result.unwrap_or_else(|message| {
				message.mod_id(self.mod_id_unchecked()).range(otherwise_range).report(self.loader)
			});
			self.build_stmt(otherwise);

			if !self.ctx.current_block.has_returned {
				let jump = ir::JmpInstr::new(merge_block.into());
				let result = self.ctx.current_block.append_instr(jump.into());
				result.unwrap_or_else(|message| {
					message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
				});
			}
		}
		let result = self.ctx.current_block.switch_to_label(merge_block);
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
		});
	}
}
