use super::{diags::TypeCheckError, Checker, TypeResult};
use crate::ast;

use super::types::{Type, TypeId};

impl Checker<'_> {
	pub fn check_const_del_stmt(&mut self, const_del: &mut ast::ConstDelStmt) -> TypeResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(TypeCheckError::const_outside_global_scope(const_del.range.clone()));
		}

		let lexeme = const_del.name.lexeme();

		if let Some(found_id) = self.ctx.get_value(lexeme) {
			return Err(TypeCheckError::const_redefinition(const_del.range.clone()));
		}

		let found_id = self.check_expr(&mut const_del.expr)?;

		let expected_id = self.check_const_bind(&mut const_del.name)?;
		//  todo: call equal type after we have the type of the const?
		self.equal_type_id(expected_id, found_id, const_del.get_range())?;

		let const_type = Type::new_const_del(expected_id);

		let const_id = self.ctx.type_store.add_type(const_type);
		const_del.set_type_id(const_id);
		self.ctx.add_value(const_del.name.lexeme(), const_id, false);

		Ok(TypeId::NOTHING)
	}

	pub fn check_const_bind(&mut self, const_bind: &mut ast::Binding) -> TypeResult<TypeId> {
		match &const_bind.ty {
			Some(ty) => self.check_type(ty),
			None => Err(TypeCheckError::const_required_type_notation(const_bind.get_range())),
		}
	}
}
