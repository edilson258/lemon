use super::types::TypeId;
use super::{Checker, TypedValue};
use crate::ast;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_if_stmt(&mut self, if_expr: &mut ast::IfStmt) -> MessageResult<TypedValue> {
		let cond_type = self.check_expr(&mut if_expr.cond)?;
		self.equal_type_expected(TypeId::BOOL, cond_type.type_id, if_expr.cond.get_range())?;

		let then_typed = self.check_stmt(&mut if_expr.then)?;

		if let Some(otherwise) = &mut if_expr.otherwise {
			let otherwise_typed = self.check_stmt(otherwise)?;

			let then_typed = self.unify_types(then_typed.type_id, otherwise_typed.type_id)?;

			// return Ok(otherwise_typed );
		}

		// TODO: unify types of then and otherwise branches

		Ok(then_typed)
	}
}
