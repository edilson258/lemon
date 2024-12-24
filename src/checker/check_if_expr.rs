use crate::ast;

use super::{Checker, TypeResult};

use super::types::TypeId;

impl Checker<'_> {
	pub fn check_if_expr(&mut self, if_expr: &mut ast::IfExpr) -> TypeResult<TypeId> {
		let cond_type = self.check_expr(&mut if_expr.cond)?;
		self.equal_type_id(TypeId::BOOL, cond_type, if_expr.get_range())?;
		let then_type = self.check_stmt(&mut if_expr.then)?;
		if let Some(otherwise) = &mut if_expr.otherwise {
			let otherwise_type = self.check_stmt(otherwise)?;
			self.equal_type_id(then_type, otherwise_type, otherwise.get_range())?;
			return Ok(otherwise_type);
		}
		Ok(then_type)
	}
}
