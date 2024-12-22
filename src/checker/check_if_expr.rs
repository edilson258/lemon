use crate::ast;

use super::{Checker, TypeResult};

use super::types::TypeId;

impl Checker<'_> {
	// todo: if expr should return a type
	pub fn check_if_expr(&mut self, if_expr: &ast::IfExpr) -> TypeResult<TypeId> {
		let cond_type = self.check_expr(&if_expr.cond)?;
		self.equal_type_id(TypeId::BOOL, cond_type, if_expr.get_range())?;
		let then_type = self.check_stmt(&if_expr.then)?;
		if let Some(otherwise) = &if_expr.otherwise {
			let otherwise_type = self.check_stmt(otherwise)?;
			self.equal_type_id(then_type, otherwise_type, otherwise.get_range())?;
			return Ok(TypeId::NOTHING);
		}
		Ok(TypeId::NOTHING)
	}
}
