use super::synthesis::synthesise_struct_def;
use super::types::{StructType, TypeId};
use super::{Checker, TyResult};
use crate::ast;
use crate::checker::synthesis::synthesise_ast_type;

impl Checker<'_> {
	pub fn check_type_def_stmt(&mut self, type_def: &mut ast::TypeDefStmt) -> TyResult<TypeId> {
		match &mut type_def.kind {
			ast::TypeDefKind::Alias(alias) => self.synthesise_alias(type_def),
			ast::TypeDefKind::Struct(struct_def) => self.check_struct_def(type_def),
		}
	}

	pub fn check_struct_def(&mut self, type_def: &mut ast::TypeDefStmt) -> TyResult<TypeId> {
		let lexeme = type_def.lexeme().to_string();
		let struct_def = type_def.get_struct_def().unwrap();
		let fields = synthesise_struct_def(struct_def, self.ctx)?;
		let mut struct_type = StructType::new(lexeme.to_owned());
		struct_type.with_fields(fields);
		let type_id = self.ctx.type_store.add_type(struct_type.into());
		type_def.set_type_id(type_id);
		type_def.name.set_type_id(type_id);
		self.ctx.type_store.add_type_by_name(lexeme, type_id);
		self.ctx.add_value(type_def.lexeme(), type_id, false);
		Ok(TypeId::UNIT)
	}

	pub fn synthesise_alias(&mut self, type_def: &mut ast::TypeDefStmt) -> TyResult<TypeId> {
		let alias_def = type_def.get_alias().unwrap();
		let lexeme = type_def.lexeme().to_string();
		let type_id = synthesise_ast_type(alias_def, false, self.ctx)?;
		self.ctx.type_store.add_type_by_name(lexeme, type_id);
		Ok(TypeId::UNIT)
	}
}
