use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::{Checker, TyResult};
use crate::ast::{self};

impl Checker<'_> {
	pub fn check_call_expr(&mut self, call_expr: &mut ast::CallExpr) -> TyResult<TypeId> {
		let callee_id = self.check_expr(&mut call_expr.callee)?;
		// remove clone :(
		if let Type::Fn(fn_type) = self.get_stored_type(callee_id).clone() {
			self.call_args_match(&fn_type.args, &mut call_expr.args)?;
			call_expr.set_type_id(fn_type.ret);
			call_expr.set_args_type(fn_type.args);
			return Ok(fn_type.ret);
		}
		Err(SyntaxErr::not_a_fn(self.display_type(callee_id), call_expr.get_range()))
	}

	fn call_args_match(&mut self, types: &[TypeId], exprs: &mut [ast::Expr]) -> TyResult<()> {
		if types.len() != exprs.len() {
			return Err(SyntaxErr::args_mismatch(types.len(), exprs.len(), exprs[0].get_range()));
		}

		for (expected_id, found_expr) in types.iter().zip(exprs) {
			let found_id = self.check_expr(found_expr)?;
			self.equal_type_expected(*expected_id, found_id, found_expr.get_range())?;
		}
		Ok(())
	}
}
