use crate::{
	ast,
	ir::ir::{self, Value},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_borrow_expr(&mut self, borrow_expr: &ast::BorrowExpr) -> Value {
		todo!()
		// let dest = self.ctx.get_register();
		// let expr = self.build_expr(&ref_expr.expr);
		// let value = expr.get_register().unwrap();
		// let type_id = self.get_type_id(expr.get_type_id());

		// if ref_expr.mutable.is_some() {
		// 	let instr = ir::UnaryInstr { type_id, value, dest };
		// 	self.add_instr(ir::Instr::BorrowMut(instr));
		// 	return Value::new_register(dest);
		// }

		// let instr = ir::UnaryInstr { type_id, value, dest };
		// self.add_instr(ir::Instr::Borrow(instr));
		// Value::new_register(dest)
	}
}
