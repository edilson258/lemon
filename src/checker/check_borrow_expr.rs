use super::types::{BorrowType, TypeId};
use super::{Checker, TyResult};
use crate::ast::{self};
use crate::checker::diags::SyntaxErr;

impl Checker<'_> {
	pub fn check_borrow_expr(&mut self, borrow_expr: &mut ast::BorrowExpr) -> TyResult<TypeId> {
		let range = borrow_expr.get_range();
		let found_id = self.check_borrow_value(borrow_expr)?;
		// todo: allow &mut pointer?
		// if !found_id.is_known() {
		// 	let found = self.get_stored_type(found_id);
		// 	if found.is_borrow() {
		// 		// return Err(SyntaxErr::borrow_conflict(range));
		// 	}
		// }
		borrow_expr.set_type_id(found_id);
		let found_id = self.infer_no_type_anotation(found_id)?;
		let borrow_value = BorrowType::new_internal(found_id, borrow_expr.mutable.is_some());
		let borrow_id = self.ctx.type_store.add_type(borrow_value.into());
		Ok(borrow_id)
	}

	fn check_borrow_value(&mut self, borrow_expr: &mut ast::BorrowExpr) -> TyResult<TypeId> {
		let muttable = borrow_expr.mutable.is_some();
		let range = borrow_expr.get_range();

		if let ast::Expr::Ident(ref mut ident) = *borrow_expr.expr {
			if !self.ctx.can_borrow_as(ident.lexeme(), muttable) {
				return Err(SyntaxErr::cannot_borrow_as_mutable(ident.lexeme(), range));
			}

			if let Some(value) = self.ctx.get_value(ident.lexeme()) {
				if !value.is_mut & muttable {
					return Err(SyntaxErr::cannot_borrow_as_mutable(ident.lexeme(), range));
				}
				let brrow_id = self.ctx.add_borrow(value.id, muttable);
			}

			return self.check_ident_expr(ident);
		}
		self.check_expr(&mut borrow_expr.expr)
	}

	fn can_borrow_as(&self, name: &str, is_mut: bool) -> TyResult<()> {
		todo!()
	}
}
