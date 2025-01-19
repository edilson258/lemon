use super::Context;
use crate::checker::types::{FnType, TypeId};

pub fn create_binds(ctx: &mut Context) {
	let is_float = FnType::new(vec![TypeId::ANY], TypeId::BOOL);
	let is_int = FnType::new(vec![TypeId::ANY], TypeId::BOOL);
	let is_str = FnType::new(vec![TypeId::ANY], TypeId::BOOL);
	let is_char = FnType::new(vec![TypeId::ANY], TypeId::BOOL);

	let is_float_id = ctx.type_store.add_type(is_float.into());
	let is_int_id = ctx.type_store.add_type(is_int.into());
	let is_str_id = ctx.type_store.add_type(is_str.into());
	let is_char_id = ctx.type_store.add_type(is_char.into());

	ctx.add_fn_value("is_float", is_float_id);
	ctx.add_fn_value("is_int", is_int_id);
	ctx.add_fn_value("is_str", is_str_id);
	ctx.add_fn_value("is_char", is_char_id);
}
