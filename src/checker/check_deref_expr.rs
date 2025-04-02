use crate::ast::{self};
use crate::message::MessageResult;

use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::Checker;
impl Checker<'_> {
	pub fn check_deref_expr(&mut self, deref_expr: &mut ast::DerefExpr) -> MessageResult<TypeId> {
		let ref_id = self.check_expr(&mut deref_expr.expr)?;
		if ref_id.is_known() {
			let ref_type = self.display_type(ref_id);
			return Err(SyntaxErr::cannot_dereference(ref_type, deref_expr.get_range()));
		}
		let ret_type = self.get_stored_type(ref_id).clone();
		if let Type::Borrow(borrow) = ret_type {
			self.register_type(borrow.value, deref_expr.get_range());
			return Ok(borrow.value);
		}
		let ref_type = self.display_type(ref_id);
		Err(SyntaxErr::cannot_dereference(ref_type, deref_expr.get_range()))
	}
}
