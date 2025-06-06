use crate::{
	ast,
	checker::{
		context::Context,
		diags::SyntaxErr,
		events::EventId,
		types::{FnType, InferType, TypeId},
	},
	loader::ModId,
	message::MessageResult,
};

use super::synthesise_ast_type;

pub fn synthesise_fn_stmt(
	fn_stmt: &mut ast::FnStmt,
	ctx: &mut Context,
	mod_id: ModId,
) -> MessageResult<FnType> {
	let generics = synthesise_generics(&mut fn_stmt.generics, ctx)?;

	let params = synthesise_fn_binds(&mut fn_stmt.params, ctx, mod_id)?;
	let ret = match fn_stmt.ret_type.as_ref() {
		Some(ty) => synthesise_ast_type(ty, ctx)?,
		None => TypeId::VOID,
	};
	let event_id = EventId::new(mod_id, fn_stmt.get_range());
	ctx.event.add_type(event_id, ret);

	let mut fn_type = FnType::new(params, ret);

	fn_type.extend_generics(generics);
	Ok(fn_type)
}

pub fn synthesise_generics(
	generics: &mut [ast::Generic],
	ctx: &mut Context,
) -> MessageResult<Vec<TypeId>> {
	let types = generics.iter_mut().map(|param| synthesise_generic(param, ctx));
	types.collect::<Result<Vec<_>, _>>()
}

pub fn synthesise_generic(generic: &mut ast::Generic, ctx: &mut Context) -> MessageResult<TypeId> {
	let mut bound_type = None;
	if let Some(bound) = &mut generic.bound {
		bound_type = Some(synthesise_ast_type(bound, ctx)?);
	};
	let infered = InferType { id: generic.lexeme(), extend: bound_type };
	Ok(ctx.type_store.add_infer_type(infered))
}

#[inline]
pub fn synthesise_fn_binds(
	binds: &mut [ast::Binding],
	ctx: &mut Context,
	mod_id: ModId,
) -> MessageResult<Vec<TypeId>> {
	let types = binds.iter_mut().map(|param| synthesise_binding(param, ctx, mod_id));
	types.collect::<Result<Vec<_>, _>>()
}

pub fn synthesise_binding(
	binding: &mut ast::Binding,
	ctx: &mut Context,
	mod_id: ModId,
) -> MessageResult<TypeId> {
	if let Some(ty) = &mut binding.ty {
		let type_id = synthesise_ast_type(ty, ctx)?;
		let event_id = EventId::new(mod_id, binding.get_range());
		ctx.event.add_type(event_id, type_id);
		return Ok(type_id);
	};
	let diag = SyntaxErr::required_type_notation(binding.get_range());
	Err(diag)
}
