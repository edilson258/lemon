use crate::ast;

use super::types::{FnType, RefType, Type, TypeId};
use super::{Checker, TypeResult};

impl Checker<'_> {
	#[rustfmt::skip]
	pub fn check_type(&mut self, ast_type: &ast::AstType) -> TypeResult<TypeId> {
		match ast_type {
			ast::AstType::Fn(fn_type) => self.check_fn_type(fn_type),
			ast::AstType::Ident(ident) => self.check_ident_type(ident),
			ast::AstType::Ref(ref_type) => self.check_ref_type(ref_type),

			// primitive types
			ast::AstType::Bool { .. }     | ast::AstType::Char { .. }
			| ast::AstType::String { .. } | ast::AstType::Float { .. }
			| ast::AstType::Number { .. } => self.check_primitive_type(ast_type),
		}
	}

	pub fn check_fn_type(&mut self, ast_type: &ast::FnType) -> TypeResult<TypeId> {
		let mut params = Vec::with_capacity(ast_type.params.len());
		for param in ast_type.params.iter() {
			let type_id = self.check_type(param)?;
			params.push(type_id);
		}
		let mut ret_type = TypeId::NOTHING;
		if let Some(ty) = ast_type.return_type.as_ref() {
			ret_type = self.check_type(ty)?
		}
		let ty = FnType::new(params, ret_type);

		Ok(self.ctx.type_store.add_type(Type::Fn(ty)))
	}

	pub fn check_ident_type(&mut self, ast_type: &ast::IdentType) -> TypeResult<TypeId> {
		todo!()
	}

	pub fn check_ref_type(&mut self, ast_type: &ast::RefType) -> TypeResult<TypeId> {
		let type_id = self.check_type(&ast_type.value)?;
		let type_value = Type::Ref(RefType::new(ast_type.mutable, type_id));
		Ok(self.ctx.type_store.add_type(type_value))
	}
}
