use crate::ast;
use crate::checker::synthesis;
use crate::message::MessageResult;

use super::context::value::Value;
use super::diags::SyntaxErr;
use super::{Checker, TypedValue};
impl Checker<'_> {
	pub fn check_let_stmt(&mut self, let_stmt: &mut ast::LetStmt) -> MessageResult<TypedValue> {
		let lexeme = let_stmt.bind.lexeme();
		let found = self.check_expr(&mut let_stmt.expr)?;

		let expect_id = match let_stmt.bind.ty.as_ref() {
			Some(ast_type) => synthesis::synthesise_ast_type(ast_type, false, self.ctx)?,
			None => self.infer_default_type(found.type_id),
		};

		let final_id = self.infer_type_from_expected(expect_id, found.type_id);
		if !self.equal_type_id(expect_id, final_id) {
			let found = self.display_type(final_id);
			let expect = self.display_type(expect_id);
			return Err(SyntaxErr::type_mismatch(expect, found, let_stmt.expr.get_range()));
		}
		self.register_type(final_id, let_stmt.get_range());

		// let found_type = self.get_stored_type(final_id);
		// if found_type.is_borrow() || found_type.is_borrow_mut() {
		// 	self.ctx.ownership.mark_and_drop_if_needed(found.ptr)?;
		// 	let value = Value::new_ptr(found.type_id, found.ptr, let_stmt.mutable.is_some());
		// 	self.ctx.add_value(lexeme, value);
		// 	return Ok(found);
		// }

		let kind = self.ptr_kind(final_id);
		let ptr = self.ctx.ownership.alloc_pointer(kind);
		let value = Value::new_ptr(final_id, ptr, let_stmt.mutable.is_some());
		self.ctx.add_value(lexeme, value);
		Ok(TypedValue::new(final_id, ptr))
	}
}
