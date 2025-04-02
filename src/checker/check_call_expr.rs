use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::Checker;
use crate::ast::{self};
use crate::message::MessageResult;
use crate::range::Range;

impl Checker<'_> {
	pub fn check_call_expr(&mut self, call_expr: &mut ast::CallExpr) -> MessageResult<TypeId> {
		let callee = self.check_expr(&mut call_expr.callee)?;
		let call_args = self.check_call_args(&mut call_expr.args)?;
		match self.get_stored_type(callee).clone() {
			Type::Fn(fn_type) => {
				self.args_mismatch(fn_type.args.len(), call_args.len(), call_expr.get_range())?;
				self.call_args_match(&fn_type.args, &call_args)?;
				self.register_type(fn_type.ret, call_expr.get_range());
				self.register_multi_type(fn_type.args, call_expr.get_range());
				return Ok(fn_type.ret);
			}

			Type::ExternFn(fn_type) => {
				if !fn_type.var_packed {
					self.call_args_match(&fn_type.args, &call_args)?;
				}
				self.register_type(fn_type.ret, call_expr.get_range());
				self.register_multi_type(fn_type.args, call_expr.get_range());
				return Ok(fn_type.ret);
			}
			_ => {}
		}
		Err(SyntaxErr::not_a_fn(self.display_type(callee), call_expr.get_range()))
	}

	fn check_call_args(&mut self, exprs: &mut [ast::Expr]) -> MessageResult<Vec<(TypeId, Range)>> {
		let mut results = Vec::with_capacity(exprs.len());
		for expr in exprs {
			let found = self.check_expr(expr)?;
			results.push((found, expr.get_range()));
		}
		Ok(results)
	}

	fn call_args_match(
		&mut self,
		expects: &[TypeId],
		founds: &[(TypeId, Range)],
	) -> MessageResult<()> {
		for (expected, (found, range)) in expects.iter().zip(founds) {
			let found = self.infer_type_from_expected(*expected, *found);
			self.equal_type_expected(*expected, found, *range)?;
		}
		Ok(())
	}

	fn args_mismatch(&self, left: usize, found: usize, range: Range) -> MessageResult<()> {
		if left != found {
			return Err(SyntaxErr::args_mismatch(left, found, range));
		}
		Ok(())
	}
}
