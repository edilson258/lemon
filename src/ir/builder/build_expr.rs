use crate::{
	ast,
	ir::{ir::IrValue, Register},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_expr(&mut self, expr: &ast::Expr) -> Register {
		match expr {
			ast::Expr::Binary(binary) => self.build_binary_expr(binary),
			ast::Expr::Literal(literal) => self.build_literal(literal),
			ast::Expr::If(if_expr) => self.build_if_expr(if_expr),
			ast::Expr::Ident(ident) => self.build_ident_expr(ident),
			ast::Expr::Call(call) => self.build_call_expr(call),
			ast::Expr::Deref(deref) => self.build_deref_expr(deref),
			ast::Expr::Borrow(borrow) => self.build_borrow_expr(borrow),
			ast::Expr::Assign(assign) => self.build_assign_expr(assign),
			ast::Expr::StructInit(struct_init) => self.build_struct_init_expr(struct_init),
			ast::Expr::Member(member) => self.build_member_expr(member),
			_ => todo!("expr: {:?}", expr),
		}
	}
}
