use super::{diags::TypeCheckError, types::ConstType, Checker, TypeResult};
use crate::ast;

use super::types::{Type, TypeId};

impl Checker<'_> {
	pub fn check_const_stmt(&mut self, const_stmt: &mut ast::ConstStmt) -> TypeResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(TypeCheckError::const_outside_global_scope(const_stmt.range.clone()));
		}

		let lexeme = const_stmt.name.lexeme();

		if let Some(found_id) = self.ctx.get_value(lexeme) {
			return Err(TypeCheckError::const_redefinition(const_stmt.range.clone()));
		}

		let found_id = self.check_expr(&mut const_stmt.expr)?;

		let expected_id = self.check_const_bind(&mut const_stmt.name)?;
		//  todo: call equal type after we have the type of the const?
		self.equal_type_id(expected_id, found_id, const_stmt.get_range())?;

		let const_type = Type::Const(ConstType::new(expected_id));

		let const_id = self.ctx.type_store.add_type(const_type);
		const_stmt.set_type_id(const_id);
		self.ctx.add_value(const_stmt.name.lexeme(), const_id, false);

		Ok(TypeId::NOTHING)
	}

	pub fn check_const_bind(&mut self, const_bind: &mut ast::Binding) -> TypeResult<TypeId> {
		match &const_bind.ty {
			Some(ty) => self.check_type(ty),
			None => Err(TypeCheckError::const_required_type_notation(const_bind.get_range())),
		}
	}
}
