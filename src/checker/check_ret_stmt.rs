use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TypedValue};
use crate::ast::{self};
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_ret_stmt(&mut self, ret_stmt: &mut ast::RetStmt) -> MessageResult<TypedValue> {
		if !self.ctx.has_function_scope() {
			return Err(SyntaxErr::return_outside_fn(ret_stmt.get_range()));
		}

		let ret_id = self.ctx.get_return_type().unwrap(); // we know it's a fn
		let range = ret_stmt.get_range();
		if let Some(value_expr) = &mut ret_stmt.expr {
			let mut found = self.check_expr(value_expr)?;
			found.change_type(self.infer_type_from_expected(ret_id, found.type_id));
			let found_type = self.get_stored_type(found.type_id);
			if found_type.is_local_borrow() {
				return Err(SyntaxErr::return_local_borrow(ret_stmt.get_range()));
			}
			self.equal_type_expected(ret_id, found.type_id, ret_stmt.get_range())?;
			self.register_type(ret_id, range);
			return Ok(found);
		}

		if !ret_id.is_void_type() {
			let found = self.display_type(TypeId::VOID);
			let ret = self.display_type(ret_id);
			return Err(SyntaxErr::type_mismatch(ret, found, ret_stmt.get_range()));
		}
		self.register_type(ret_id, range);
		Ok(TypedValue::new(ret_id, usize::MAX))
	}
}
