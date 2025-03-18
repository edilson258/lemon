use crate::ast::{self, MemberExpr};

use super::{
	diags::SyntaxErr,
	types::{Type, TypeId},
	Checker, TyResult,
};

impl Checker<'_> {
	pub fn check_assign_expr(&mut self, assign_expr: &mut ast::AssignExpr) -> TyResult<TypeId> {
		let found = self.check_expr(&mut assign_expr.right)?;
		if self.ctx.type_store.is_module(found) {
			return Err(SyntaxErr::cannot_reassign_module(assign_expr.get_range()));
		}
		let expected = self.assign_left_expr(&mut assign_expr.left, found)?;
		assign_expr.set_type_id(expected);
		Ok(TypeId::UNIT)
	}

	fn assign_left_expr(&mut self, expr: &mut ast::Expr, found_id: TypeId) -> TyResult<TypeId> {
		match expr {
			ast::Expr::Ident(ident) => self.assign_ident_expr(ident, found_id),
			ast::Expr::Deref(deref) => self.assign_deref_expr(deref, found_id),
			ast::Expr::Member(member) => self.assign_member_expr(member, found_id),
			_ => Err(SyntaxErr::left_hand_cannot_be_assigned(expr.get_range())),
		}
	}

	fn assign_ident_expr(&mut self, ident: &mut ast::Ident, found: TypeId) -> TyResult<TypeId> {
		let lexeme = ident.lexeme();
		if let Some(value) = self.ctx.get_value(lexeme) {
			if !value.is_mut {
				return Err(SyntaxErr::cannot_assign_immutable(lexeme, ident.get_range()));
			}
			let found = self.infer_type(value.type_id, found)?;
			self.equal_type_expected(value.type_id, found, ident.get_range())?;
			return Ok(value.type_id);
		}
		Err(SyntaxErr::not_found_value(lexeme, ident.get_range()))
	}

	fn assign_deref_expr(&mut self, deref: &mut ast::DerefExpr, found: TypeId) -> TyResult<TypeId> {
		let expected = self.check_deref_expr(deref)?;

		let (name, mutable) = self.try_mutate_expr(&deref.expr)?;
		if !mutable {
			return Err(SyntaxErr::cannot_assign_immutable(&name, deref.get_range()));
		}

		let found = self.infer_type(expected, found)?;
		self.equal_type_expected(expected, found, deref.get_range())?;
		Ok(expected)
	}

	fn assign_member_expr(&mut self, member: &mut MemberExpr, found: TypeId) -> TyResult<TypeId> {
		let self_type = self.check_expr(&mut member.left)?;
		let self_type = self.get_stored_type(self_type);
		if let Type::Struct(struct_type) = self_type {
			let lexeme = member.method.lexeme();
			let field = struct_type.get_field(lexeme);
			if let Some(field) = field {
				let (name, mutable) = self.try_mutate_expr(&member.left)?;
				if !mutable {
					return Err(SyntaxErr::cannot_assign_immutable(&name, member.get_range()));
				}
				let found = self.infer_type(field.type_id, found)?;
				member.method.set_type_id(found);
				self.equal_type_expected(field.type_id, found, member.get_range())?;
				return Ok(field.type_id);
			}

			let method = member.method.lexeme().to_owned();
			let found = self.display_type_value(self_type);
			return Err(SyntaxErr::not_found_method_named(method, found, member.get_range()));
		}
		todo!("assign member expr {:?}", self_type)
	}

	pub fn try_mutate_expr(&self, expr: &ast::Expr) -> TyResult<(String, bool)> {
		match expr {
			ast::Expr::Ident(ident) => self.try_mutate_ident_expr(ident),
			ast::Expr::Member(member) => self.try_mutate_expr(&member.left),
			ast::Expr::Deref(deref) => self.try_mutate_expr(&deref.expr),
			ast::Expr::Assign(assign) => self.try_mutate_expr(&assign.right),
			_ => todo!("code {:?}", expr),
		}
	}

	fn try_mutate_ident_expr(&self, ident: &ast::Ident) -> TyResult<(String, bool)> {
		let lexeme = ident.lexeme();
		if let Some(value) = self.ctx.get_value(lexeme) {
			return Ok((lexeme.to_owned(), value.is_mut));
		}
		Err(SyntaxErr::not_found_value(lexeme, ident.get_range()))
	}
}
