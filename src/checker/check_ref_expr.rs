use crate::ast::{self};

use super::context::value::Value;
use super::diags::TypeCheckError;
// use super::types::TypeId;
use super::types::{RefType, Type, TypeId};
use super::{Checker, TypeResult};

impl Checker<'_> {
	pub fn check_ref_expr(&mut self, ref_expr: &ast::RefExpr) -> TypeResult<TypeId> {
		let found_value = self.check_expr_ref(&ref_expr.expr)?;
		let ref_type = RefType::new(ref_expr.mutable.is_some(), found_value.type_id);

		let borrow_id = self.ctx.add_borrow(found_value.id, ref_expr.mutable.is_some());

		if borrow_id.is_none() {
			return Err(TypeCheckError::borrow_conflict(ref_expr.get_range()));
		}
		Ok(self.ctx.type_store.add_type(Type::Ref(ref_type)))
	}

	fn check_expr_ref(&mut self, expr: &ast::Expr) -> TypeResult<Value> {
		match expr {
			ast::Expr::Ident(ident) => self.check_ident_ref_expr(ident),
			_ => todo!(),
		}
	}

	fn check_ident_ref_expr(&mut self, ident: &ast::Ident) -> TypeResult<Value> {
		let value = match self.ctx.get_value(ident.lexeme()) {
			Some(value) => *value,
			None => return Err(TypeCheckError::not_found_value(ident.lexeme(), ident.get_range())),
		};
		Ok(value)
	}
}
