use crate::ast;
use crate::error_type;
use crate::report::report_message_without_module;

use super::diags::SyntaxErr;
use super::types::{ModuleType, StructType, Type};
use super::{CheckResult, Checker, TypedValue};

impl Checker<'_> {
	pub fn check_ident_expr(&mut self, ident: &mut ast::Ident) -> CheckResult {
		let name = ident.lexeme();
		let range = ident.get_range();

		if self.ctx.is_accessor_scope() {
			return self.check_self_access(ident);
		}

		if let Some(value) = self.ctx.lookup_variable_value(name).cloned() {
			self.register_type(value.typed_value.type_id, range);
			return Ok(Some(value.typed_value));
		}

		if let Some(fn_type_id) = self.ctx.lookup_function_value(name).map(|v| v.type_id) {
			self.register_type(fn_type_id, range);
			// todo: copy or owned?
			let raw_copy = self.ctx.borrow.create_raw_copy();
			return Ok(Some(TypedValue::new(fn_type_id, raw_copy)));
		}

		Err(SyntaxErr::not_found_value(name, range))
	}

	fn check_self_access(&mut self, ident: &mut ast::Ident) -> CheckResult {
		let self_type_id = self.ctx.get_accessor_scope_type().expect("accessor scope not found");
		let self_type = self.lookup_stored_type_without_borrow(self_type_id).clone();

		match self_type {
			Type::Struct(s) => self.check_struct_access(ident, &s),
			Type::Mod(m) => self.check_module_access(ident, &m),
			other => todo!("unexpected self type in accessor: {:?}", other),
		}
	}

	fn check_module_access(&self, ident: &mut ast::Ident, mod_type: &ModuleType) -> CheckResult {
		let mod_id = mod_type.mod_id;
		let range = ident.get_range();
		let name = ident.lexeme();

		let module = match self.ctx.get_module(mod_id) {
			Some(m) => m,
			// return Err(SyntaxErr::module_not_found(mod_id, range)); //
			None => report_message_without_module(&error_type!("module not found: {}", mod_id)),
		};

		if module.get_value(name).is_some() || module.get_function(name).is_some() {
			return Ok(None);
		}

		Err(SyntaxErr::not_found_pub_item(name.into(), range))
	}

	fn check_struct_access(
		&mut self,
		ident: &mut ast::Ident,
		struct_type: &StructType,
	) -> CheckResult {
		let name = ident.lexeme();
		let range = ident.get_range();

		if self.ctx.is_associated_scope() {
			if let Some(type_id) = struct_type.get_associate(name) {
				self.register_type(*type_id, range);
				return Ok(None);
			}

			let self_type = Type::from(struct_type.clone());
			let found = self.display_type_value(&self_type);
			return Err(SyntaxErr::not_found_associate_field(name.to_owned(), found, range));
		}

		if let Some(field) = struct_type.get_field(name) {
			self.register_type(field.type_id, range);
			return Ok(Some(TypedValue::new(field.type_id, field.ptr_id)));
		}

		if let Some(method_id) = struct_type.get_fn(name) {
			self.register_type(*method_id, range);
			return Ok(None);
		}

		let self_type = Type::from(struct_type.clone());
		let found = self.display_type_value(&self_type);
		Err(SyntaxErr::not_found_method_named(name.to_owned(), found, range))
	}
}
