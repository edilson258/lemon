use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::{Checker, TyResult};
use crate::ast::{self};

impl Checker<'_> {
	pub fn check_call_expr(&mut self, call_expr: &mut ast::CallExpr) -> TyResult<TypeId> {
		let callee_id = self.check_expr(&mut call_expr.callee)?;
		// remove clone :(
		match self.get_stored_type(callee_id).clone() {
			Type::Fn(fn_type) => {
				self.call_args_match(&fn_type.args, &mut call_expr.args, false)?;
				call_expr.set_type_id(fn_type.ret);
				call_expr.set_args_type(fn_type.args);
				return Ok(fn_type.ret);
			}

			Type::ExternFn(fn_type) => {
				self.call_args_match(&fn_type.args, &mut call_expr.args, fn_type.var_packed)?;
				call_expr.set_type_id(fn_type.ret);
				call_expr.set_args_type(fn_type.args);
				return Ok(fn_type.ret);
			}
			_ => {}
		}

		Err(SyntaxErr::not_a_fn(self.display_type(callee_id), call_expr.get_range()))
	}

	fn call_args_match(
		&mut self,
		types: &[TypeId],
		exprs: &mut [ast::Expr],
		var_pack: bool,
	) -> TyResult<()> {
		if types.len() != exprs.len() && !var_pack {
			return Err(SyntaxErr::args_mismatch(types.len(), exprs.len(), exprs[0].get_range()));
		}
		for (expected, found_expr) in types.iter().zip(exprs) {
			let found = self.check_expr(found_expr)?;
			let found = self.infer_type(*expected, found)?;
			self.equal_type_expected(*expected, found, found_expr.get_range())?;
		}
		Ok(())
	}
}
