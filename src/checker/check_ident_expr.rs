use crate::ast::{self, Ident};
use crate::report::throw_error_with_range;

use super::diags::SyntaxErr;
use super::types::{ModuleType, StructType, Type, TypeId};
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
		Err(SyntaxErr::not_found_value(ident.lexeme(), ident.get_range()))
	}

	pub fn self_acessor(&mut self, ident: &mut Ident) -> TyResult<TypeId> {
		let lexeme = ident.lexeme();
		let self_type = self.ctx.accessor_scope_type().expect("error: accessor scope not found");

		let self_type = self.get_stored_type_without_borrow(self_type);

		if let Type::Struct(struct_type) = self_type {
			return self.struct_acessor(ident, struct_type);
		}

		if let Type::Mod(mod_type) = self_type {
			return self.mod_acessor(ident, mod_type);
		}

		todo!("error: self type not found: {:?}", self_type)
	}

	fn mod_acessor(&self, ident: &mut Ident, mod_type: &ModuleType) -> TyResult<TypeId> {
		let mod_id = mod_type.module_id;

		let module = match self.ctx.get_module(mod_id) {
			Some(module) => module,
			None => {
				// todo: improve this...
				let source = self.loader.get_source_unwrap(mod_id);
				let message = format!("error: module not found: {}", mod_id);
				throw_error_with_range(message, ident.get_range(), source);
			}
		};
		let lexeme = ident.lexeme();
		if let Some(value_type) = module.get_value(lexeme) {
			return Ok(*value_type);
		}
		if let Some(fn_type) = module.get_function(lexeme) {
			return Ok(*fn_type);
		}
		Err(SyntaxErr::not_found_pub_item(lexeme.into(), ident.get_range()))
	}

	fn struct_acessor(&self, ident: &mut Ident, _type: &StructType) -> TyResult<TypeId> {
		let lexeme = ident.lexeme();
		if self.ctx.is_acessor_associate_scope() {
			if let Some(field_id) = _type.get_associate(lexeme) {
				ident.set_type_id(*field_id);
				return Ok(*field_id);
			}
			let name = lexeme.to_owned();
			let self_type: Type = _type.clone().into();
			let found = self.display_type_value(&self_type);
			return Err(SyntaxErr::not_found_associate_field(name, found, ident.get_range()));
		}

		if let Some(field) = _type.get_field(lexeme) {
			ident.set_type_id(field.type_id);
			return Ok(field.type_id);
		}

		if let Some(method) = _type.get_fn(lexeme) {
			ident.set_type_id(*method);
			return Ok(*method);
		}
		let name = lexeme.to_owned();
		let self_type: Type = _type.clone().into();
		let found = self.display_type_value(&self_type);
		Err(SyntaxErr::not_found_method_named(name, found, ident.get_range()))
	}
}
