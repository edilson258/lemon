use super::Context;
use crate::checker::types::{FnType, TypeId};

pub fn create_binds(ctx: &mut Context) {
	let write = FnType::new(vec![TypeId::I32, TypeId::I8, TypeId::I32], TypeId::I32);
	let len = FnType::new(vec![TypeId::STRING], TypeId::I32);
	let read = FnType::new(vec![TypeId::I32, TypeId::I8, TypeId::I32], TypeId::I32);
	let exit = FnType::new(vec![TypeId::I32], TypeId::I32);

	let write_id = ctx.type_store.add_type(write.into());
	let read_id = ctx.type_store.add_type(read.into());
	let exit_id = ctx.type_store.add_type(exit.into());
	let len_id = ctx.type_store.add_type(len.into());

	ctx.add_fn_value("write", write_id);
	ctx.add_fn_value("read", read_id);
	ctx.add_fn_value("exit", exit_id);
	ctx.add_fn_value("len", len_id);
}
