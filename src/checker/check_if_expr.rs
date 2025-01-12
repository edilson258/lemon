use super::types::TypeId;
use super::{Checker, TyResult};
use crate::ast;

impl Checker<'_> {
	pub fn check_if_expr(&mut self, if_expr: &mut ast::IfExpr) -> TyResult<TypeId> {
		let cond_type = self.check_expr(&mut if_expr.cond)?;
		self.equal_type_expected(TypeId::BOOL, cond_type, if_expr.cond.get_range())?;

		let then_type = self.check_stmt(&mut if_expr.then)?;

		if let Some(otherwise) = &mut if_expr.otherwise {
			let otherwise_type = self.check_stmt(otherwise)?;
			return Ok(otherwise_type);
		}

		Ok(then_type)
	}
}
