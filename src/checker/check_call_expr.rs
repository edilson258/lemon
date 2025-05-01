use super::{CheckResult, Checker, ExpectSome, TypedValue};
use crate::ast::{self};
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_call_expr(&mut self, c: &mut ast::CallExpr) -> CheckResult {
		let callee = self.check_expr(&mut c.callee).some(c.callee.get_range())?;
		let args = self.check_call_args(&mut c.args)?;

		let (params, ret_ty) = self.fn_signature(callee.type_id, c.get_range())?;
		self.call_args_match(&params, &args, c.get_range())?;
		self.register_multi_type(params, c.get_range());
		self.register_type(ret_ty, c.get_range());

		let owner = self.ctx.borrow.create_owner();
		let value = TypedValue::new(ret_ty, owner);
		Ok(Some(value))
		// self.make_ret_value(&args, ret_ty, c.get_range())
	}
	pub fn check_call_args(&mut self, args: &mut [ast::Expr]) -> MessageResult<Vec<TypedValue>> {
		let mut values = Vec::with_capacity(args.len());
		for argument in args {
			let arg_range = argument.get_range();
			let tv = self.check_expr(argument).some(arg_range)?;
			self.register_type(tv.type_id, arg_range);
			values.push(tv);
		}
		Ok(values)
	}
	// pub fn make_ret_value(&mut self, args: &[TypedValue], ret: TypeId, span: Range) -> CheckResult {
	// 	match self.lookup_stored_type(ret) {
	// 		Type::Borrow(b) => self.ret_borrow(args, b, ret, span),
	// 		// t if t.is_copy() => Ok(self.ret_copy(ret)),
	// 		_ => Ok(self.ret_owned(ret)),
	// 	}
	// }

	// // ---------- caso &T / &mut T --------------------------------------
	// fn ret_borrow(
	// 	&mut self,
	// 	args: &[TypedValue],
	// 	borrow: BorrowType,
	// 	ret_id: TypeId,
	// 	span: Range,
	// ) -> CheckResult {
	// 	let src_ids = self.collect_borrow_sources(args, &borrow);

	// 	if src_ids.is_empty() {
	// 		return Err(SyntaxErr::internal("borrow return: no matching argument", span));
	// 	}

	// 	let kind = if borrow.mutable { RefKind::Mutable } else { RefKind::Shared };
	// 	let union = self.ctx.borrow.union_borrow(kind, &src_ids);

	// 	Ok(TypedValue::from_borrow(&mut self.ctx.borrow, ret_id, union))
	// }

	// fn collect_borrow_sources(&self, args: &[TypedValue], borrow: &BorrowType) -> Vec<PtrId> {
	// 	args
	// 		.iter()
	// 		.filter(|tv| tv.type_id == borrow.value)
	// 		.filter_map(|tv| tv.ptr.as_ref())
	// 		.filter(|ptr| match (borrow.mutable, ptr.kind) {
	// 			(true, RefKind::Mutable) | (false, RefKind::Shared) => true,
	// 			_ => false,
	// 		})
	// 		.map(|ptr| ptr.id)
	// 		.collect()
	// }

	// fn ret_copy(&mut self, ret_id: TypeId) -> TypedValue {
	// 	let id = self.ctx.borrow.alloc(RefKind::Copy);
	// 	let ptr = self.ctx.borrow.get_ref(id);
	// 	TypedValue::new(ret_id, Some(ptr))
	// }

	// fn ret_owned(&mut self, ret_id: TypeId) -> TypedValue {
	// 	let id = self.ctx.borrow.alloc(RefKind::Owned);
	// 	let ptr = self.ctx.borrow.get_ref(id);
	// 	TypedValue::new(ret_id, Some(ptr))
	// }
}
