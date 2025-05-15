use crate::ast;
use crate::ir;

use super::Builder;

impl Builder<'_> {
	pub fn build_if_expr(&mut self, if_expr: &mut ast::IfExpr) -> ir::IrBasicValue {
		let range = if_expr.get_range();
		let type_id = self.lookup_event_type(range);
		let dest = self.create_basic_value(type_id);
		let alloc_instr = ir::SallocInstr::new(dest.clone(), type_id);
		self.append_instr(alloc_instr.into(), Some(range));

		// blocks
		let then_block = self.ctx.current_block.create_new_block();
		let otherwise_block = self.ctx.current_block.create_new_block();
		let merge_block = self.ctx.current_block.create_new_block();

		// cond
		let cond = self.build_expr(&mut if_expr.cond);
		let cond_instr = ir::JmpIfInstr::new(cond, then_block.into(), otherwise_block.into());
		self.append_instr(cond_instr.into(), Some(range));

		// then
		let result = self.ctx.current_block.switch_to_label(then_block);
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
		});
		let then = self.build_expr(&mut if_expr.then);
		let then = self.ensure_loaded(then, range);
		let set_instr = ir::UnInstr::new(dest.clone(), then);
		self.append_instr(ir::Instr::Set(set_instr), Some(range));
		let jmp_instr = ir::JmpInstr::new(merge_block.into());
		self.append_instr(jmp_instr.into(), Some(range));

		// otherwise
		let result = self.ctx.current_block.switch_to_label(otherwise_block);
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
		});
		let otherwise = self.build_expr(&mut if_expr.otherwise);
		let otherwise = self.ensure_loaded(otherwise, range).with_new_type(type_id);
		let set_instr = ir::UnInstr::new(dest.clone(), otherwise);
		self.append_instr(ir::Instr::Set(set_instr), Some(range));
		let jmp_instr = ir::JmpInstr::new(merge_block.into());
		self.append_instr(jmp_instr.into(), Some(range));

		// pass control to the merge block
		let result = self.ctx.current_block.switch_to_label(merge_block);
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
		});
		dest
	}
}
