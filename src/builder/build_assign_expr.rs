use crate::{
	ast, error_build,
	ir::{self, IrBasicValue, UnInstr},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_assign_expr(&mut self, assign_expr: &mut ast::AssignExpr) -> IrBasicValue {
		let range = assign_expr.get_range();
		let src = self.build_expr(&mut assign_expr.right);
		let dest = self.build_expr(&mut assign_expr.left);
		let type_id = self.lookup_event_type(range);

		let basic_type = self.type_store.lookup_type(type_id).unwrap_or_else(|| {
			let message = error_build!("not found type id {} in type store", type_id.as_usize());
			message.report(self.loader);
		});

		if src.is_register() && !basic_type.is_borrow() {
			let register = self.resolve_value(src, range);
			let instr = UnInstr::new(dest, register);
			let result = self.ctx.current_block.append_instr(ir::Instr::Set(instr));
			result.unwrap_or_else(|message| {
				message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
			});
			return IrBasicValue::default();
		}

		let instr = UnInstr::new(dest, src);
		let result = self.ctx.current_block.append_instr(ir::Instr::Set(instr));
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
		});
		IrBasicValue::default()
	}
}
