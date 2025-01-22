use crate::ast::{self};

use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::{Checker, TyResult};

impl Checker<'_> {
	pub fn check_ident_expr(&mut self, ident: &mut ast::Ident) -> TyResult<TypeId> {
		if self.ctx.has_accessor_scope() {
			return self.self_acessor(ident);
		}

		if let Some(value) = self.ctx.get_value(ident.lexeme()) {
			ident.set_type_id(value.type_id);
			return Ok(value.type_id);
		}

		self.ctx.type_store.create_monomo_fn(ident.lexeme().to_string());
		if let Some(fn_value) = self.ctx.get_fn_value(ident.lexeme()) {
			ident.set_type_id(fn_value.type_id);
			return Ok(fn_value.type_id);
		}
		self.ctx.type_store.end_monomo_fn();
		println!("not found value: {}", ident.lexeme());
		Err(SyntaxErr::not_found_value(ident.lexeme(), ident.get_range()))
	}

	pub fn self_acessor(&mut self, ident: &mut ast::Ident) -> TyResult<TypeId> {
		let lexeme = ident.lexeme();
		let self_type = self.ctx.accessor_scope_type().expect("error: accessor scope not found");

		let is_associate = self.ctx.is_acessor_associate_scope();
		let self_type = self.get_stored_type(self_type);

		if let Type::Struct(struct_type) = self_type {
			let found = self.display_type_value(self_type);
			let name = ident.lexeme().to_owned();

			if is_associate {
				if let Some(field_id) = struct_type.get_associate(lexeme) {
					ident.set_type_id(*field_id);
					return Ok(*field_id);
				}
				return Err(SyntaxErr::not_found_associate_field(name, found, ident.get_range()));
			}

			if let Some(field) = struct_type.get_field(lexeme) {
				ident.set_type_id(field.type_id);
				return Ok(field.type_id);
			}

			return Err(SyntaxErr::not_found_method_named(name, found, ident.get_range()));
		}
		todo!("error: self type not found: {:?}", self_type)
	}
}
