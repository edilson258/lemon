use super::context::scope::ScopeKind;
use super::types::TypeId;
use super::{CheckResult, Checker, ExpectSome};
use crate::ast;

impl Checker<'_> {
	pub fn check_while_stmt(&mut self, while_stmt: &mut ast::WhileStmt) -> CheckResult {
		let test_range = while_stmt.test.get_range();
		let test_type = self.check_expr(&mut while_stmt.test).some(test_range)?;

		self.equal_type_expected(TypeId::BOOL, test_type.type_id, test_range)?;

		self.ctx.enter_scope(ScopeKind::loop_scope());
		let body_range = while_stmt.body.get_range();
		let body_type = self.check_stmt(&mut while_stmt.body).some(body_range)?;

		self.ctx.exit_scope();
		self.equal_type_expected(TypeId::UNIT, body_type.type_id, body_range)?;

		Ok(None)
	}
}
