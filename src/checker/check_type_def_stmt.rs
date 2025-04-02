use super::synthesis::synthesise_struct_def;
use super::types::{StructType, TypeId};
use super::Checker;
use crate::ast;
use crate::checker::synthesis::synthesise_ast_type;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_type_def_stmt(&mut self, type_def: &mut ast::TypeDefStmt) -> MessageResult<TypeId> {
		match &mut type_def.kind {
			ast::TypeDefKind::Alias(alias) => self.synthesise_alias(type_def),
			ast::TypeDefKind::Struct(struct_def) => self.check_struct_def(type_def),
		}
	}

	pub fn check_struct_def(&mut self, type_def: &mut ast::TypeDefStmt) -> MessageResult<TypeId> {
		let lexeme = type_def.lexeme().to_string();
		let range = type_def.get_range();
		let name_range = type_def.name.get_range();
		let struct_def = type_def.get_struct_def().unwrap();
		let fields = synthesise_struct_def(struct_def, self.ctx, self.ctx.mod_id)?;
		let mut struct_type = StructType::new(lexeme.to_owned());
		struct_type.with_fields(fields);
		let type_id = self.ctx.type_store.add_type(struct_type.into());
		self.register_type(type_id, range);
		self.register_type(type_id, name_range);
		self.ctx.type_store.add_type_definition(lexeme, type_id);
		Ok(TypeId::UNIT)
	}

	pub fn synthesise_alias(&mut self, type_def: &mut ast::TypeDefStmt) -> MessageResult<TypeId> {
		let alias_def = type_def.get_alias().unwrap();
		let lexeme = type_def.lexeme().to_string();
		let type_id = synthesise_ast_type(alias_def, false, self.ctx)?;
		self.ctx.type_store.add_type_definition(lexeme, type_id);
		Ok(TypeId::UNIT)
	}
}
