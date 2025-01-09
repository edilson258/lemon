use crate::ast::{self};

use super::{diags::SyntaxErr, types::TypeId, Checker, TyResult};

impl Checker<'_> {
	pub fn check_assign_expr(&mut self, assign_expr: &mut ast::AssignExpr) -> TyResult<TypeId> {
		let found_id = self.check_expr(&mut assign_expr.right)?;
		self.assign_left_expr(&mut assign_expr.left, found_id)
	}

	fn assign_left_expr(&mut self, expr: &mut ast::Expr, found_id: TypeId) -> TyResult<TypeId> {
		println!("assign left expr: {:?}", expr);
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
				return Err(SyntaxErr::cannot_assign_immutable(lexeme, ident.get_range()));
			}
			let found = self.infer_type(value.type_id, found)?;
			self.equal_type_expected(value.type_id, found, ident.get_range())?;
			return Ok(TypeId::NOTHING);
		}
		Err(SyntaxErr::not_found_value(lexeme, ident.get_range()))
	}

	fn assign_deref_expr(&mut self, deref: &mut ast::DerefExpr, found: TypeId) -> TyResult<TypeId> {
		let deref_id = self.check_deref_expr(deref)?;
		self.equal_type_expected(deref_id, found, deref.get_range())?;
		Ok(TypeId::NOTHING)
	}
}
