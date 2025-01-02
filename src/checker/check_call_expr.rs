use super::diags::TypeCheckError;
use super::types::{FnType, Type, TypeId};
use super::{Checker, TypeResult};
use crate::ast::{self};

// todo: we really need to clone fn_type?
impl Checker<'_> {
	pub fn check_call_expr(&mut self, call_expr: &mut ast::CallExpr) -> TypeResult<TypeId> {
		let callee_id = self.check_expr(&mut call_expr.callee)?;
		match self.get_stored_type(callee_id)? {
			Type::Fn(fn_type) => self.check_call_fn(fn_type.clone(), call_expr),
			Type::ConstFn(fn_type) => self.call_const_fn(fn_type.value, call_expr),
			_ => Err(TypeCheckError::not_a_fn(self.format(callee_id), call_expr.get_range())),
		}
	}

	fn check_call_fn(
		&mut self,
		fn_type: FnType,
		call_expr: &mut ast::CallExpr,
	) -> TypeResult<TypeId> {
		self.call_args_match(&fn_type.args, &mut call_expr.args)?;
		call_expr.set_type_id(fn_type.ret);
		Ok(fn_type.ret)
	}

	fn call_const_fn(&mut self, fn_id: TypeId, call_expr: &mut ast::CallExpr) -> TypeResult<TypeId> {
		if let Type::Fn(fn_type) = self.get_stored_type(fn_id)? {
			return self.check_call_fn(fn_type.clone(), call_expr);
		}
		Err(TypeCheckError::not_a_fn(self.format(fn_id), call_expr.get_range()))
	}

	#[inline(always)]
	fn call_args_match(&mut self, expects: &[TypeId], founds: &mut [ast::Expr]) -> TypeResult<()> {
		for (expected_id, found_expr) in expects.iter().zip(founds) {
			let found_id = self.check_expr(found_expr)?;
			self.equal_type_id(*expected_id, found_id, found_expr.get_range())?;
		}
		Ok(())
	}
}
