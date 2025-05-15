use crate::{
	ast,
	ir::{self, IrBasicValue, UnInstr},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_assign_expr(&mut self, assign_expr: &mut ast::AssignExpr) -> IrBasicValue {
		let range = assign_expr.get_range();
		let src = self.build_expr(&mut assign_expr.right);
		let dest = self.build_dest_expr(&mut assign_expr.left);
		// let type_id = self.lookup_event_type(range);
		// let basic_type = self.type_store.lookup_type(type_id).unwrap_or_else(|| {
		// 	let message = error_build!("not found type id {} in type store", type_id.as_usize());
		// 	message.report(self.loader);
		// });
		// if src.is_register() {
		let src = self.ensure_loaded(src, range);
		let instr = UnInstr::new(dest, src);
		self.append_instr(ir::Instr::Set(instr), Some(range));
		IrBasicValue::default()
		// }

		// let instr = UnInstr::new(dest, src);
		// self.append_instr(ir::Instr::Halloc(instr), Some(range));

		// IrBasicValue::default()
	}

	fn build_dest_expr(&mut self, right: &mut ast::Expr) -> IrBasicValue {
		match right {
			ast::Expr::Deref(deref_expr) => self.build_expr(&mut deref_expr.expr),
			_ => self.build_expr(right),
		}
	}
}
