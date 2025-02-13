use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::{Checker, TyResult};
use crate::ast::{self};
use crate::range::Range;

impl Checker<'_> {
	pub fn check_call_expr(&mut self, call_expr: &mut ast::CallExpr) -> TyResult<TypeId> {
		let callee = self.check_expr(&mut call_expr.callee)?;
		let call_args = self.check_call_args(&mut call_expr.args)?;
		let infered = self.infer_generic(callee, &call_args)?;
		match infered {
			Type::Fn(fn_type) => {
				self.args_mismatch(fn_type.args.len(), call_args.len(), call_expr.get_range())?;
				self.call_args_match(&fn_type.args, &call_args)?;
				self.monomorphic_call(fn_type.clone().into())?;
				call_expr.set_ret_type_id(fn_type.ret);
				call_expr.set_args_type(fn_type.args);
				return Ok(fn_type.ret);
			}

			Type::ExternFn(fn_type) => {
				if !fn_type.var_packed {
					self.call_args_match(&fn_type.args, &call_args)?;
				}
				self.monomorphic_call(fn_type.clone().into())?;
				call_expr.set_ret_type_id(fn_type.ret);
				call_expr.set_args_type(fn_type.args);
				return Ok(fn_type.ret);
			}
			_ => {}
		}
		Err(SyntaxErr::not_a_fn(self.display_type(callee), call_expr.get_range()))
	}

	fn monomorphic_call(&mut self, callee: Type) -> TyResult<()> {
		match callee {
			Type::Fn(fn_type) => {
				if !fn_type.generics.is_empty() {
					self.ctx.type_store.add_monomo_fn(fn_type.clone());
				}
			}
			Type::ExternFn(fn_type) => {
				// self.ctx.type_store.add_monomo_extern_fn(fn_type.clone());
			}
			_ => {}
		}
		Ok(())
	}

	fn check_call_args(&mut self, exprs: &mut [ast::Expr]) -> TyResult<Vec<(TypeId, Range)>> {
		let mut results = Vec::with_capacity(exprs.len());
		for expr in exprs {
			let found = self.check_expr(expr)?;
			results.push((found, expr.get_range()));
		}
		Ok(results)
	}

	fn call_args_match(&mut self, expects: &[TypeId], founds: &[(TypeId, Range)]) -> TyResult<()> {
		for (expected, (found, range)) in expects.iter().zip(founds) {
			let found = self.infer_type(*expected, *found)?;
			self.equal_type_expected(*expected, found, range.clone())?;
		}
		Ok(())
	}

	fn args_mismatch(&self, left: usize, found: usize, range: Range) -> TyResult<()> {
		if left != found {
			return Err(SyntaxErr::args_mismatch(left, found, range));
		}
		Ok(())
	}
}
