use super::types::TypeId;
use super::{CheckResult, Checker, ExpectSome};
use crate::ast;

impl Checker<'_> {
	pub fn check_if_stmt(&mut self, if_expr: &mut ast::IfStmt) -> CheckResult {
		let cond_range = if_expr.cond.get_range();
		let cond_type = self.check_expr(&mut if_expr.cond).some(cond_range)?;
		self.equal_type_expected(TypeId::BOOL, cond_type.type_id, cond_range)?;

		let then_range = if_expr.then.get_range();
		let then_typed = self.check_stmt(&mut if_expr.then).some(then_range)?;

		if let Some(otherwise) = &mut if_expr.otherwise {
			let otherwise_range = otherwise.get_range();
			let otherwise_typed = self.check_stmt(otherwise).some(otherwise_range)?;
			let then_typed = self.unify_types(then_typed.type_id, otherwise_typed.type_id)?;
			// return Ok(otherwise_typed );
		}
		// TODO: unify types of then and otherwise branches
		Ok(Some(then_typed))
	}
}
