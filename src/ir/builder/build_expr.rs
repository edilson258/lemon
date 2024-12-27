use crate::{ast, ir::ir::Value};

use super::Builder;

impl Builder {
	pub fn build_expr(&mut self, expr: &ast::Expr) -> Value {
		match expr {
			ast::Expr::Binary(binary) => self.build_binary_expr(binary),
			ast::Expr::Literal(literal) => self.build_literal(literal),
			ast::Expr::If(if_expr) => self.build_if_expr(if_expr),
			ast::Expr::Ident(ident) => self.build_ident_expr(ident),
			ast::Expr::Call(call) => self.build_call_expr(call),
			ast::Expr::Deref(deref) => self.build_deref_expr(deref),
			ast::Expr::Ref(ref_expr) => self.build_ref_expr(ref_expr),
			_ => todo!(),
		}
	}
}
