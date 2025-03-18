use crate::{
	ast,
	ir::{self, IrBasicValue, UnInstr},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_assign_expr(&mut self, assign_expr: &mut ast::AssignExpr) -> IrBasicValue {
		let src = self.build_expr(&mut assign_expr.right);
		let dest = self.build_expr(&mut assign_expr.left);
		let type_id = self.build_type(assign_expr.get_type_id(), assign_expr.get_range());

		let basic_type = self.type_store.get_type(type_id).unwrap_or_else(|| {
			throw_ir_build_error(
				format!("not found type id {} in type store", type_id.as_usize()).as_str(),
			);
		});

		if src.is_register() && !basic_type.is_borrow() {
			let register = self.resolve_value(src);
			let instr = UnInstr::new(dest, register);
			self.ctx.current_block.append_instr(ir::Instr::Set(instr));
			return IrBasicValue::default();
		}

		let instr = UnInstr::new(dest, src);
		self.ctx.current_block.append_instr(ir::Instr::Set(instr));
		IrBasicValue::default()
	}
}
