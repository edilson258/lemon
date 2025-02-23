use crate::{ast, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_associate_expr(&mut self, _associate_expr: &mut ast::AssociateExpr) -> IrBasicValue {
		// let self_name = associate_expr.self_name.lexeme();
		// let method_name = associate_expr.method.lexeme();
		// todo: wait for support passing function as argument
		todo!("get associated method, passing function as argument");
	}
}
