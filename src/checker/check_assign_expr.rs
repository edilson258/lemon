use crate::ast::{self};

use super::{diags::SyntaxErr, types::TypeId, Checker, TyResult};

impl Checker<'_> {
	pub fn check_assign_expr(&mut self, assign_expr: &mut ast::AssignExpr) -> TyResult<TypeId> {
		let found = self.check_expr(&mut assign_expr.right)?;
		let found = self.infer_no_type_anotation(found)?;
		self.assign_left_expr(&mut assign_expr.left, found)
	}

	fn assign_left_expr(&mut self, expr: &mut ast::Expr, found_id: TypeId) -> TyResult<TypeId> {
		// println!("assign left expr: {:?}", expr);
		match expr {
			ast::Expr::Ident(ident) => self.assign_ident_expr(ident, found_id),
			ast::Expr::Deref(deref) => self.assign_deref_expr(deref, found_id),
			_ => todo!(),
		}
	}

	fn assign_ident_expr(&mut self, ident: &mut ast::Ident, found: TypeId) -> TyResult<TypeId> {
		let lexeme = ident.lexeme();
		if let Some(value) = self.ctx.get_value(lexeme) {
			if !value.is_mutable() {
				return Err(SyntaxErr::cannot_borrow_as_mutable_more_than_once(lexeme, ident.get_range()));
			}
			self.equal_type_expected(value.type_id, found, ident.get_range())?;
			return Ok(TypeId::UNIT);
		}
		Err(SyntaxErr::not_found_value(lexeme, ident.get_range()))
	}

	fn assign_deref_expr(&mut self, deref: &mut ast::DerefExpr, found: TypeId) -> TyResult<TypeId> {
		let expected = self.check_deref_expr(deref)?;
		self.equal_type_expected(expected, found, deref.get_range())?;
		Ok(TypeId::UNIT)
	}
}
