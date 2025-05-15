use crate::{ast, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_borrow_expr(&mut self, borrow_expr: &mut ast::BorrowExpr) -> IrBasicValue {
		self.build_expr(&mut borrow_expr.expr)
	}
}
