use crate::{
	ast,
	checker::{
		context::Context,
		types::{ExternFnType, TypeId},
		TyResult,
	},
};

use super::{synthesise_ast_type, synthesise_fn_binds};

pub fn synthesise_extren_fn_stmt(
	fn_stmt: &mut ast::ExternFnStmt,
	ctx: &mut Context,
) -> TyResult<ExternFnType> {
	let params = synthesise_fn_binds(&mut fn_stmt.params, ctx)?;
	let ret = match fn_stmt.ret_type.as_ref() {
		Some(ty) => synthesise_ast_type(ty, false, ctx)?,
		None => TypeId::VOID,
	};

	fn_stmt.set_ret_id(ret);

	Ok(ExternFnType::new(params, ret, fn_stmt.var_packed.is_some()))
}
