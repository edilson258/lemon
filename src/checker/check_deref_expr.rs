use crate::ast::{self};
use crate::message::MessageResult;

use super::diags::SyntaxErr;
use super::types::Type;
use super::{Checker, TypedValue};
impl Checker<'_> {
	pub fn check_deref_expr(&mut self, deref_expr: &mut ast::DerefExpr) -> MessageResult<TypedValue> {
		let dereference = self.check_expr(&mut deref_expr.expr)?;
		if dereference.type_id.is_builtin_type() {
			let dereference_type = self.display_type(dereference.type_id);
			return Err(SyntaxErr::cannot_dereference(dereference_type, deref_expr.get_range()));
		}
		if let Type::Borrow(borrow) = self.get_stored_type(dereference.type_id).clone() {
			self.register_type(borrow.value, deref_expr.get_range());
			self.ctx.ownership.mark_and_drop_if_needed(dereference.ptr)?;
			return Ok(TypedValue::new(borrow.value, dereference.ptr));
		}
		let dereference_type = self.display_type(dereference.type_id);
		Err(SyntaxErr::cannot_dereference(dereference_type, deref_expr.get_range()))
	}
}
