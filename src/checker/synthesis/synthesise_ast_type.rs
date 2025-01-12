use crate::{
	ast::{self, AstType},
	checker::{
		context::Context,
		types::{BorrowType, FnType, TypeId},
		TyResult,
	},
};

pub(crate) fn synthesise_ast_type(
	ast_type: &ast::AstType,
	external_borrow: bool,
	ctx: &mut Context,
) -> TyResult<TypeId> {
	match ast_type {
		AstType::Number(number) => synthesise_number_type(number, ctx),
		AstType::Bool(bool) => Ok(TypeId::BOOL),
		AstType::Char(char) => Ok(TypeId::CHAR),
		AstType::String(string) => Ok(TypeId::STRING),
		AstType::Fn(fn_type) => synthesise_fn_type(fn_type, ctx),
		AstType::Borrow(borrow) => synthesise_borrow_type(borrow, external_borrow, ctx),
		_ => todo!(),
	}
}

fn synthesise_number_type(number: &ast::NumberType, ctx: &mut Context) -> TyResult<TypeId> {
	if number.bits == 8 {
		return if number.signed { Ok(TypeId::I8) } else { Ok(TypeId::U8) };
	}
	if number.bits == 16 {
		return if number.signed { Ok(TypeId::I16) } else { Ok(TypeId::U16) };
	}
	if number.bits == 32 {
		return if number.signed { Ok(TypeId::I32) } else { Ok(TypeId::U32) };
	}
	if number.bits == 64 {
		return if number.signed { Ok(TypeId::I64) } else { Ok(TypeId::U64) };
	}
	unreachable!();
}

fn synthesise_fn_type(fn_type: &ast::FnType, ctx: &mut Context) -> TyResult<TypeId> {
	let mut args = Vec::with_capacity(fn_type.params.len());
	for param in fn_type.params.iter() {
		let type_id = synthesise_ast_type(param, false, ctx)?;
		args.push(type_id);
	}
	let ret = match fn_type.ret_type.as_ref() {
		Some(ty) => synthesise_ast_type(ty, false, ctx)?,
		None => TypeId::VOID,
	};
	let fn_type = FnType::new(args, ret);
	Ok(ctx.type_store.add_type(fn_type.into()))
}

fn synthesise_borrow_type(
	borrow_type: &ast::BorrowType,
	external_borrow: bool,
	ctx: &mut Context,
) -> TyResult<TypeId> {
	let value = synthesise_ast_type(&borrow_type.value, false, ctx)?;
	let borrow = BorrowType::new(value, borrow_type.mutable, external_borrow);
	Ok(ctx.type_store.add_type(borrow.into()))
}
