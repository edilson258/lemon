use crate::{
	ast,
	checker::{
		context::Context,
		events::EventId,
		types::{ExternFnType, TypeId},
	},
	loader::ModId,
	message::MessageResult,
};

use super::{synthesise_ast_type, synthesise_fn_binds};

pub fn synthesise_extren_fn_stmt(
	fn_stmt: &mut ast::ExternFnStmt,
	ctx: &mut Context,
	mod_id: ModId,
) -> MessageResult<ExternFnType> {
	let params = synthesise_fn_binds(&mut fn_stmt.params, ctx, mod_id)?;
	let ret = match fn_stmt.ret_type.as_ref() {
		Some(ty) => synthesise_ast_type(ty, ctx)?,
		None => TypeId::VOID,
	};
	let event_id = EventId::new(mod_id, fn_stmt.get_range());
	ctx.event.add_type(event_id, ret);

	Ok(ExternFnType::new(params, ret, fn_stmt.var_packed.is_some()))
}
