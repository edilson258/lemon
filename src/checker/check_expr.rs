use crate::ast;

use super::types::TypeId;
use super::{Checker, TyResult};
impl Checker<'_> {
	pub fn check_expr(&mut self, expr: &mut ast::Expr) -> TyResult<TypeId> {
		match expr {
			ast::Expr::Binary(binary_expr) => self.check_binary_expr(binary_expr),
			ast::Expr::Literal(literal) => self.check_literal(literal),
			ast::Expr::Deref(deref_expr) => self.check_deref_expr(deref_expr),
			ast::Expr::Borrow(borrow_expr) => self.check_borrow_expr(borrow_expr),
			ast::Expr::Assign(assign_expr) => self.check_assign_expr(assign_expr),
			ast::Expr::Ident(ident_expr) => self.check_ident_expr(ident_expr),
			ast::Expr::Call(call_expr) => self.check_call_expr(call_expr),
			ast::Expr::If(if_expr) => self.check_if_expr(if_expr),
			ast::Expr::Import(import_expr) => self.check_import_expr(import_expr),
			_ => todo!(),
		}
	}
}
