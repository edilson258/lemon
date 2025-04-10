use crate::ast::{self};
use crate::message::MessageResult;

use super::{synthesis, Checker, TypedValue};
impl Checker<'_> {
	pub fn check_literal(&mut self, lit: &ast::Literal) -> MessageResult<TypedValue> {
		let type_id = synthesis::synthesise_literal(lit, self.ctx)?;
		let ptr = self.ctx.ownership.copied_pointer();
		Ok(TypedValue::new(type_id, ptr))
	}
}
