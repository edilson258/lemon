use super::{Checker, TypedValue};
use crate::ast;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_for_stmt(&mut self, for_stmt: &mut ast::ForStmt) -> MessageResult<TypedValue> {
		todo!()
		// let test_type = self.check_expr(&mut for_stmt.test)?;
		// self.equal_type_expected(TypeId::BOOL, test_type, for_stmt.test.get_range())?;

		// let body_type = self.check_stmt(&mut for_stmt.body)?;
		// self.equal_type_expected(TypeId::UNIT, body_type, for_stmt.body.get_range())?;

		// Ok(TypeId::UNIT)
	}
}
