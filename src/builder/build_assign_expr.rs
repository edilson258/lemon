use crate::{
	ast,
	ir::{self, Instr, IrBasicValue, UnInstr},
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

		if !basic_type.is_borrow() || !basic_type.is_borrow_mut() {
			let dest_value = UnInstr::new(dest.clone(), src.clone());
			self.ctx.block.add_instr(Instr::Load(dest_value));
		}
		let instr = UnInstr::new(dest, src);
		self.ctx.block.add_instr(ir::Instr::Set(instr));
		IrBasicValue::default()
	}
}
