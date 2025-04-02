use super::types::{BorrowType, TypeId};
use super::Checker;
use crate::ast::{self};
use crate::checker::diags::SyntaxErr;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_borrow_expr(&mut self, borrow_expr: &mut ast::BorrowExpr) -> MessageResult<TypeId> {
		let range = borrow_expr.get_range();
		let found_id = self.check_borrow_value(borrow_expr)?;
		// why register before infer?
		self.register_type(found_id, range);
		let found_id = self.infer_default_type(found_id);
		let borrow_value = BorrowType::new_internal(found_id, borrow_expr.mutable.is_some());
		let borrow_id = self.ctx.type_store.add_type(borrow_value.into());
		Ok(borrow_id)
	}

	fn check_borrow_value(&mut self, borrow_expr: &mut ast::BorrowExpr) -> MessageResult<TypeId> {
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
				value.lookup_ptr_id_unchecked()
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
			value.add_ptr(successor_ptr);
			return Ok(value.type_id);
		}
		self.check_expr(&mut borrow_expr.expr)
	}
}
