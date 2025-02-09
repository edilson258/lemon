use env::Env;
use inkwell::{builder::Builder, context::Context, module::Module};
// use mem::stack::Stack;

use crate::{
	checker::types::TypeStore,
	ir::{self},
	source::Source,
};

mod env;
mod llvm_compile_add;
mod llvm_compile_block;
mod llvm_compile_call;
mod llvm_compile_cmp_eq;
mod llvm_compile_cmp_ge;
mod llvm_compile_cmp_gt;
mod llvm_compile_cmp_le;
mod llvm_compile_cmp_lt;
mod llvm_compile_function;
mod llvm_compile_instr;
mod llvm_compile_jmp;
mod llvm_compile_jmp_if;
mod llvm_compile_load;
mod llvm_compile_mul;
mod llvm_compile_ret;
mod llvm_compile_salloc;
mod llvm_compile_set;
mod llvm_compile_sub;
mod llvm_compile_type;
mod llvm_compile_value;
mod llvm_memory;
// mod allocate;
// mod allocate_struct;
// mod bind;
// mod calculate_struct_size;
// mod fill_struct_values;
// mod llvm_block;
// mod llvm_call;
// mod llvm_cmp;
// mod llvm_free;
// mod llvm_function;
// mod llvm_get_field;
// mod llvm_init_struct;
// mod llvm_instr;
// mod llvm_jump;
// mod llvm_math;
// mod llvm_mem;
// mod llvm_ownership;
// mod llvm_ret;
// mod llvm_set_field;
// mod llvm_struct;
// mod llvm_type;
// mod llvm_value;
// mod mem;
// mod store_struct_fields;

pub fn create_module_from_source<'ll>(ctx: &'ll Context, source: &Source) -> Module<'ll> {
	let module = ctx.create_module(source.file_name());
	module
}

pub struct Llvm<'ll> {
	pub ctx: &'ll Context,
	pub module: Module<'ll>,
	pub builder: Builder<'ll>,
	pub env: Env<'ll>,
	pub type_store: &'ll TypeStore,
}

impl<'ll> Llvm<'ll> {
	pub fn new(ctx: &'ll Context, module: Module<'ll>, type_store: &'ll TypeStore) -> Self {
		let builder = ctx.create_builder();
		let env = Env::new();
		Self { ctx, module, builder, type_store, env }
	}

	pub fn compile_ir(&mut self, root: &ir::IR) {
		root.functions.iter().for_each(|function| {
			self.llvm_compile_function(function);
		});
	}
}
