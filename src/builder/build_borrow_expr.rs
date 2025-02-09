use crate::{ast, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_borrow_expr(&mut self, borrow_expr: &mut ast::BorrowExpr) -> IrBasicValue {
		let mut value = self.build_expr(&mut borrow_expr.expr);
		let type_id = self.build_type(borrow_expr.type_id, borrow_expr.get_range());
		value.with_new_type(type_id)
	}
}
