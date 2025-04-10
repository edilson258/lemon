use super::context::scope::ScopeKind;
use super::types::TypeId;
use super::{Checker, TypedValue};
use crate::ast;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_while_stmt(&mut self, while_stmt: &mut ast::WhileStmt) -> MessageResult<TypedValue> {
		let test_type = self.check_expr(&mut while_stmt.test)?;

		self.equal_type_expected(TypeId::BOOL, test_type.type_id, while_stmt.test.get_range())?;

		self.ctx.enter_scope(ScopeKind::loop_scope());

		let body_type = self.check_stmt(&mut while_stmt.body)?;

		self.ctx.exit_scope();

		self.equal_type_expected(TypeId::UNIT, body_type.type_id, while_stmt.body.get_range())?;
		Ok(TypedValue::default())
	}
}
