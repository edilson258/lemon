use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{CheckResult, Checker, ExpectSome};
use crate::ast;

impl Checker<'_> {
	// todo: move ownership to checker
	pub fn check_ret_stmt(&mut self, ret_stmt: &mut ast::RetStmt) -> CheckResult {
		if !self.ctx.has_function_scope() {
			return Err(SyntaxErr::return_outside_fn(ret_stmt.get_range()));
		}

		let ret_id = self.ctx.get_return_type().unwrap();
		let range = ret_stmt.get_range();

		if let Some(value_expr) = &mut ret_stmt.expr {
			let mut found = self.check_expr(value_expr).some(range)?;
			found.infer_type(self.infer_type_from_expected(ret_id, found.type_id));

			if self.ctx.borrow.can_return_value(&found) {
				return Err(SyntaxErr::cannot_return_local_reference(range));
			}

			self.equal_type_expected(ret_id, found.type_id, range)?;
			self.register_type(ret_id, range);

			return Ok(Some(found));
		}

		// Retorno sem valor → precisa ser void
		if !ret_id.is_void_type() {
			let found = self.display_type(TypeId::VOID);
			let ret = self.display_type(ret_id);
			return Err(SyntaxErr::type_mismatch(ret, found, range));
		}

		self.register_type(ret_id, range);
		Ok(None)
	}
}
