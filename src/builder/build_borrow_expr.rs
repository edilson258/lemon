use crate::{ast, ir::IrValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_borrow_expr(&mut self, borrow_expr: &mut ast::BorrowExpr) -> IrValue {
		todo!()
	}
}
