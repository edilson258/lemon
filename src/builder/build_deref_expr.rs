use crate::{ast, ir::IrValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_deref_expr(&mut self, deref_expr: &mut ast::DerefExpr) -> IrValue {
		todo!()
	}
}
