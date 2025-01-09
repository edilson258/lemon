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

pub(crate) fn synthesise_fn_stmt(fn_stmt: &mut ast::FnStmt, ctx: &mut Context) -> TyResult<FnType> {
	let params = synthesise_fn_params(&mut fn_stmt.params, ctx)?;
	let ret = match fn_stmt.ret_type.as_ref() {
		Some(ty) => synthesise_ast_type(ty, false, ctx)?,
		None => TypeId::VOID,
	};
	Ok(FnType::new(params, ret))
}

fn synthesise_fn_params(params: &mut [ast::Binding], ctx: &mut Context) -> TyResult<Vec<TypeId>> {
	let mut types = Vec::with_capacity(params.len());
	for param in params.iter_mut() {
		let type_id = synthesise_binding(param, ctx)?;
		types.push(type_id);
	}
	Ok(types)
}

fn synthesise_binding(binding: &mut ast::Binding, ctx: &mut Context) -> TyResult<TypeId> {
	if let Some(ty) = &mut binding.ty {
		return synthesise_ast_type(ty, true, ctx);
	};
	let diag = SyntaxErr::required_type_notation(binding.get_range());
	Err(diag)
}
