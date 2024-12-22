use super::diags::TypeCheckError;
use super::types::{FnType, Type, TypeId};
use super::{Checker, TypeResult};
use crate::ast::{self};
use crate::range::Range;

impl Checker<'_> {
	pub fn check_call_expr(&mut self, call_expr: &ast::CallExpr) -> TypeResult<TypeId> {
		let callee_id = self.check_expr(&call_expr.callee)?;

		let fn_type = self.unwrap_fn_type(callee_id, call_expr.get_range())?;

		self.call_args_match(fn_type.args, &call_expr.args)?;
		//  just fuck fix ... sorry :(
		Ok(fn_type.ret)
	}

	fn call_args_match(&mut self, expects: Vec<TypeId>, founds: &[ast::Expr]) -> TypeResult<()> {
		for (expected_id, found_expr) in expects.iter().zip(founds) {
			let found_id = self.check_expr(found_expr)?;
			self.equal_type_id(*expected_id, found_id, found_expr.get_range())?;
		}
		Ok(())
	}

	fn unwrap_fn_type(&self, found_id: TypeId, range: Range) -> TypeResult<FnType> {
		let found = self.get_stored_type(found_id)?;
		match found {
			Type::Fn(fn_type) => Ok(fn_type.clone()),
			_ => Err(TypeCheckError::not_a_fn(self.format(found_id), range)),
		}
	}
}
