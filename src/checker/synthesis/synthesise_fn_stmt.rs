use crate::{
	ast,
	checker::{
		context::Context,
		diags::SyntaxErr,
		types::{FnType, TypeId},
		TyResult,
	},
};

use super::synthesise_ast_type;

pub fn synthesise_fn_stmt(fn_stmt: &mut ast::FnStmt, ctx: &mut Context) -> TyResult<FnType> {
	let params = synthesise_fn_binds(&mut fn_stmt.params, ctx)?;
	let ret = match fn_stmt.ret_type.as_ref() {
		Some(ty) => synthesise_ast_type(ty, false, ctx)?,
		None => TypeId::VOID,
	};
	Ok(FnType::new(params, ret))
}

pub fn synthesise_fn_binds(binds: &mut [ast::Binding], ctx: &mut Context) -> TyResult<Vec<TypeId>> {
	let types = binds
		.iter_mut()
		.map(|param| synthesise_binding(param, ctx))
		.collect::<Result<Vec<_>, _>>()?;

	Ok(types)
}

pub fn synthesise_binding(binding: &mut ast::Binding, ctx: &mut Context) -> TyResult<TypeId> {
	if let Some(ty) = &mut binding.ty {
		let type_id = synthesise_ast_type(ty, true, ctx)?;
		binding.set_type_id(type_id);
		return Ok(type_id);
	};
	let diag = SyntaxErr::required_type_notation(binding.get_range());
	Err(diag)
}
