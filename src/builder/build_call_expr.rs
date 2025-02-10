use crate::{ast, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_call_expr(&mut self, call_expr: &mut ast::CallExpr) -> IrBasicValue {
		todo!("{:?}", call_expr);
	}
}
