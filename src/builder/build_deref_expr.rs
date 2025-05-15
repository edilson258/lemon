use crate::{
	ast,
	ir::{self, IrBasicValue},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_deref_expr(&mut self, deref_expr: &mut ast::DerefExpr) -> IrBasicValue {
		let value = self.build_expr(&mut deref_expr.expr);
		let dest = self.create_basic_value(value.type_id);
		let instr = ir::UnInstr::new(dest.clone(), value);
		self.append_instr(ir::Instr::Load(instr), Some(deref_expr.get_range()));
		dest
	}
}
