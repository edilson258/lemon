use crate::checker::{types::TypeId, TyResult};

use super::synthesise_ast_type;

pub fn synthesise_struct_type() -> TyResult<TypeId> {
	let str = 21;
	todo!()
	// let fields = synthesise_struct_fields(struct_type, ctx)?;
	// let struct_id = ctx.type_store.add_type(ExternFnType::new(fields, TypeId::UNIT, false));
	// Ok(struct_id)
}

fn synthesise_struct_fields() -> TyResult<Vec<TypeId>> {}
