use crate::ast;

use super::{synthesis, CheckResult, Checker, TypedValue};
impl Checker<'_> {
	pub fn check_literal(&mut self, lit: &ast::Literal) -> CheckResult {
		let type_id = synthesis::synthesise_literal(lit, self.ctx)?;
		let ptr = self.ctx.borrow.create_owner();
		Ok(Some(TypedValue::new(type_id, ptr)))
	}
}
