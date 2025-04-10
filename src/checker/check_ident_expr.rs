use crate::ast::{self, Ident};
use crate::error_type;
use crate::message::MessageResult;
use crate::report::report_message_without_module;

use super::diags::SyntaxErr;
use super::types::{ModuleType, StructType, Type};
use super::{Checker, TypedValue};

impl Checker<'_> {
	pub fn check_ident_expr(&mut self, ident: &mut ast::Ident) -> MessageResult<TypedValue> {
		if self.ctx.is_accessor_scope() {
			return self.self_acessor(ident);
		}

		let name = ident.lexeme();
		let range = ident.get_range();

		if let Some(value) = self.ctx.lookup_variable_value(name).copied() {
			if self.ctx.mark_use {
				self.ctx.mark_use = false;
				self.ctx.ownership.mark_and_drop_if_needed(value.ptr)?;
			}
			self.register_type(value.type_id, range);
			return Ok(TypedValue::new(value.type_id, value.ptr));
		}

		if let Some(fn_type_id) = self.ctx.lookup_function_value(name).map(|value| value.type_id) {
			self.register_type(fn_type_id, range);
			return Ok(TypedValue::new(fn_type_id, usize::MAX));
		}

		Err(SyntaxErr::not_found_value(name, range))
	}

	pub fn self_acessor(&mut self, ident: &mut Ident) -> MessageResult<TypedValue> {
		let lexeme = ident.lexeme();
		let self_type = self.ctx.get_accessor_scope_type().expect("error: accessor scope not found");
		// todo: don;t clone type
		let self_type = self.get_stored_type_without_borrow(self_type).clone();
		if let Type::Struct(struct_type) = self_type {
			return self.struct_acessor(ident, &struct_type);
		}

		if let Type::Mod(mod_type) = self_type {
			return self.mod_acessor(ident, &mod_type);
		}

		todo!("error: self type not found: {:?}", self_type)
	}

	fn mod_acessor(&self, ident: &mut Ident, mod_type: &ModuleType) -> MessageResult<TypedValue> {
		let mod_id = mod_type.mod_id;
		let module = match self.ctx.get_module(mod_id) {
			Some(module) => module,
			None => {
				let message = error_type!("module not found: {}", mod_id);
				report_message_without_module(&message);
			}
		};
		let lexeme = ident.lexeme();
		if let Some(value_type) = module.get_value(lexeme) {
			return Ok(TypedValue::new(*value_type, usize::MAX));
		}
		if let Some(fn_type) = module.get_function(lexeme) {
			return Ok(TypedValue::new(*fn_type, usize::MAX));
		}
		Err(SyntaxErr::not_found_pub_item(lexeme.into(), ident.get_range()))
	}

	fn struct_acessor(
		&mut self,
		ident: &mut Ident,
		struct_type: &StructType,
	) -> MessageResult<TypedValue> {
		let lexeme = ident.lexeme();
		let range = ident.get_range();
		if self.ctx.is_associated_scope() {
			if let Some(field_id) = struct_type.get_associate(lexeme) {
				self.register_type(*field_id, range);
				return Ok(TypedValue::new(*field_id, usize::MAX));
			}
			let name = lexeme.to_owned();
			let self_type: Type = struct_type.clone().into();
			let found = self.display_type_value(&self_type);
			return Err(SyntaxErr::not_found_associate_field(name, found, range));
		}

		if let Some(field) = struct_type.get_field(lexeme) {
			self.register_type(field.type_id, range);
			return Ok(TypedValue::new(field.type_id, field.ptr_id));
		}

		if let Some(method) = struct_type.get_fn(lexeme) {
			self.register_type(*method, range);
			return Ok(TypedValue::new(*method, usize::MAX));
		}
		let name = lexeme.to_owned();
		let self_type: Type = struct_type.clone().into();
		let found = self.display_type_value(&self_type);
		Err(SyntaxErr::not_found_method_named(name, found, range))
	}
}
