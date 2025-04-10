use super::types::BorrowType;
use super::{Checker, TypedValue};
use crate::ast::{self};
use crate::checker::diags::SyntaxErr;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_borrow_expr(
		&mut self,
		borrow_expr: &mut ast::BorrowExpr,
	) -> MessageResult<TypedValue> {
		let range = borrow_expr.get_range();
		let mut found = self.check_borrow_value(borrow_expr)?;
		found.change_type(self.infer_default_type(found.type_id));
		self.register_type(found.type_id, range);
		let borrow_value = BorrowType::new_internal(found.type_id, borrow_expr.mutable.is_some());
		let borrow_id = self.ctx.type_store.add_type(borrow_value.into());
		Ok(TypedValue::new(borrow_id, found.ptr))
	}

	fn check_borrow_value(&mut self, borrow_expr: &mut ast::BorrowExpr) -> MessageResult<TypedValue> {
		let is_mutable = borrow_expr.mutable.is_some();
		let range = borrow_expr.get_range();

		if let ast::Expr::Ident(ref mut ident) = *borrow_expr.expr {
			let name = ident.lexeme();

			let ptr_id = {
				// 1. search value in mutable scope
				let Some(value) = self.ctx.lookup_variable_value(name) else {
					return Err(SyntaxErr::not_found_value(name, ident.get_range()));
				};

				// 2. check if can borrow mutable
				if is_mutable && !value.mutable {
					return Err(SyntaxErr::cannot_borrow_as_mutable(name, ident.get_range()));
				}
				value.ptr
			};

			// 3. create borrow via ownership
			let result = match is_mutable {
				true => self.ctx.ownership.mutable_borrow(ptr_id),
				false => self.ctx.ownership.readonly_borrow(ptr_id),
			};

			let (borrow_ptr, successor_ptr) = match result {
				Ok(pair) => pair,
				Err(message) => return Err(message.range(range)),
			};

			let Some(value) = self.ctx.lookup_variable_value_mut(name) else {
				return Err(SyntaxErr::not_found_value(name, range));
			};
			// 4. update pointer of variable for tracking
			value.add_ptr(successor_ptr.id);

			// self.ctx.ownership.register_use(borrow_ptr.id);
			// return Ok(TypedValue::new(value.type_id, value.ptr));
			return Ok(TypedValue::new(value.type_id, value.ptr));
		}
		self.check_expr(&mut borrow_expr.expr)
	}
}
