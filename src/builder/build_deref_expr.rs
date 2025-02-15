use crate::{ast, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_deref_expr(&mut self, deref_expr: &mut ast::DerefExpr) -> IrBasicValue {
		todo!("{:?}", deref_expr);
	}
}
