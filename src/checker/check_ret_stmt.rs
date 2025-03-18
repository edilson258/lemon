use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TyResult};
use crate::ast::{self};

impl Checker<'_> {
	pub fn check_ret_stmt(&mut self, ret_stmt: &mut ast::RetStmt) -> TyResult<TypeId> {
		if !self.ctx.has_function_scope() {
			return Err(SyntaxErr::return_outside_fn(ret_stmt.get_range()));
		}

		let ret_id = self.ctx.get_return_type().unwrap(); // we know it's a fn

		if let Some(value_expr) = &mut ret_stmt.expr {
			let found_id = self.check_expr(value_expr)?;
			let found_id = self.infer_type(ret_id, found_id)?;
			let found_type = self.get_stored_type(found_id);
			if found_type.is_local_borrow() {
				return Err(SyntaxErr::return_local_borrow(ret_stmt.get_range()));
			}
			self.equal_type_expected(ret_id, found_id, ret_stmt.get_range())?;
			ret_stmt.set_type_id(ret_id);
			return Ok(ret_id);
		}

		if !ret_id.is_void() {
			let found = self.display_type(TypeId::VOID);
			let ret = self.display_type(ret_id);
			return Err(SyntaxErr::type_mismatch(ret, found, ret_stmt.get_range()));
		}
		ret_stmt.set_type_id(ret_id);
		Ok(ret_id)
	}
}
