use crate::{
	ast::{self},
	checker::{context::Context, types::FieldType, TyResult},
};

use super::synthesise_ast_type;

pub fn synthesise_struct_def(
	struct_def: &mut ast::StructType,
	ctx: &mut Context,
) -> TyResult<Vec<FieldType>> {
	let fields = struct_def.fields.iter_mut().map(|param| synthesise_field(param, ctx));
	fields.collect::<Result<Vec<_>, _>>()
}

pub fn synthesise_field(field: &mut ast::FieldType, ctx: &mut Context) -> TyResult<FieldType> {
	let name = field.lexeme().to_owned();
	let type_id = synthesise_ast_type(&field.ast_type, false, ctx)?;
	field.set_type_id(type_id);
	Ok(FieldType::new(name, type_id))
}
