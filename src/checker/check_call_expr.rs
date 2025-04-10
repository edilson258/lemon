use rustc_hash::FxHashSet;

use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::{Checker, TypedValue};
use crate::ast::{self};
use crate::checker::ownership::pointer::PtrKind;
use crate::message::{Message, MessageResult};
use crate::range::Range;

impl Checker<'_> {
	pub fn check_call_expr(&mut self, call_expr: &mut ast::CallExpr) -> MessageResult<TypedValue> {
		let callee = self.check_expr(&mut call_expr.callee)?;
		let call_args = self.check_call_args(&mut call_expr.args)?;
		match self.get_stored_type(callee.type_id).clone() {
			Type::Fn(fn_type) => {
				self.args_mismatch(fn_type.args.len(), call_args.len(), call_expr.get_range())?;
				self.call_args_match(&fn_type.args, &call_args)?;
				self.register_type(fn_type.ret, call_expr.get_range());
				self.register_multi_type(fn_type.args, call_expr.get_range());
				return self.create_return_value(&call_args, fn_type.ret);
			}

			Type::ExternFn(fn_type) => {
				if !fn_type.var_packed {
					self.call_args_match(&fn_type.args, &call_args)?;
				}
				self.register_type(fn_type.ret, call_expr.get_range());
				self.register_multi_type(fn_type.args, call_expr.get_range());
				return self.create_return_value(&call_args, fn_type.ret);
			}
			_ => {}
		}
		Err(SyntaxErr::not_a_fn(self.display_type(callee.type_id), call_expr.get_range()))
	}

	fn create_return_value(
		&mut self,
		call_args: &[(TypedValue, Range)],
		ret_id: TypeId,
	) -> MessageResult<TypedValue> {
		if ret_id.is_builtin_type() {
			let ptr_kind = self.ptr_kind(ret_id);
			let ptr_id = self.ctx.ownership.alloc_pointer(ptr_kind);
			return Ok(TypedValue::new(ret_id, ptr_id));
		}

		let found = self.get_stored_type(ret_id);
		if found.is_borrow() || found.is_borrow_mut() {
			let mut addresses = FxHashSet::default();
			let kind = if found.is_borrow_mut() { Some(PtrKind::MutableBorrow) } else { None }
				.unwrap_or(PtrKind::ReadOnlyBorrow);

			for (arg_value, _) in call_args {
				let arg_value_addr = self.ctx.ownership.lookup_ptr(arg_value.ptr)?;
				let ptr_kind = arg_value_addr.kind;
				if ptr_kind.is_mutable_borrow() || ptr_kind.is_read_only_borrow() {
					addresses.extend(arg_value_addr.addresses.clone());
				}
			}
			if !addresses.is_empty() {
				let ptr_id = self.ctx.ownership.alloc_pointer_with_addresses(addresses, kind);
				return Ok(TypedValue::new(ret_id, ptr_id));
			}
			let message = Message::error_ownership("cannot return a borrowed value");
			return Err(message.note_internal());
		}

		let ptr_id = self.ctx.ownership.owned_pointer();
		Ok(TypedValue::new(ret_id, ptr_id))
	}

	fn check_call_args(
		&mut self,
		exprs: &mut [ast::Expr],
	) -> MessageResult<Vec<(TypedValue, Range)>> {
		self.ctx.mark_use = true;
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
		founds: &[(TypedValue, Range)],
	) -> MessageResult<()> {
		for (expected, (found, range)) in expects.iter().zip(founds) {
			let found = self.infer_type_from_expected(*expected, found.type_id);
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
