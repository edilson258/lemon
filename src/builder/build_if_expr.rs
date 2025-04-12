use crate::ast;
use crate::ir;

use super::Builder;

impl Builder<'_> {
	pub fn build_if_expr(&mut self, if_expr: &mut ast::IfExpr) -> ir::IrBasicValue {
		let then_block = self.ctx.current_block.create_new_block();
		let otherwise_block = self.ctx.current_block.create_new_block();
		let merge_block = self.ctx.current_block.create_new_block();

		// cond
		let cond_range = if_expr.cond.get_range();
		let cond = self.build_expr(&mut if_expr.cond);
		let cond_instr = ir::JmpIfInstr::new(cond, then_block.into(), otherwise_block.into());
		let result = self.ctx.current_block.append_instr(cond_instr.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(cond_range).report(self.loader)
		});

		// then
		let then_range = if_expr.then.get_range();
		let result = self.ctx.current_block.switch_to_label(then_block.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(then_range).report(self.loader)
		});
		let then = self.build_expr(&mut if_expr.then);
		let jmp_instr = ir::JmpInstr::new(merge_block.into());
		let result = self.ctx.current_block.append_instr(jmp_instr.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(then_range).report(self.loader)
		});

		// otherwise
		let otherwise_range = if_expr.otherwise.get_range();
		let result = self.ctx.current_block.switch_to_label(otherwise_block.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(otherwise_range).report(self.loader)
		});
		let otherwise = self.build_expr(&mut if_expr.otherwise);
		let jmp_instr = ir::JmpInstr::new(merge_block.into());
		let result = self.ctx.current_block.append_instr(jmp_instr.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(otherwise_range).report(self.loader)
		});

		let result = self.ctx.current_block.switch_to_label(merge_block.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(otherwise_range).report(self.loader)
		});

		then
	}
}
