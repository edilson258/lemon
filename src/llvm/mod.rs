use inkwell::{builder::Builder, context::Context, module::Module};
use mem::stack::Stack;

use crate::{
	checker::types::TypeStore,
	ir::{self},
	source::Source,
};

mod allocate_struct;
mod bind;
mod calculate_struct_size;
mod fill_struct_values;
mod llvm_block;
mod llvm_call;
mod llvm_cmp;
mod llvm_free;
mod llvm_function;
mod llvm_get_field;
mod llvm_init_struct;
mod llvm_instr;
mod llvm_jump;
mod llvm_math;
mod llvm_mem;
mod llvm_ownership;
mod llvm_ret;
mod llvm_set_field;
mod llvm_struct;
mod llvm_type;
mod llvm_value;
mod mem;
mod store_struct_fields;

pub fn create_module_from_source<'ll>(ctx: &'ll Context, source: &Source) -> Module<'ll> {
	let module = ctx.create_module(source.file_name());
	module
}

pub struct Llvm<'ll> {
	pub ctx: &'ll Context,
	pub module: Module<'ll>,
	pub builder: Builder<'ll>,
	pub stack: Stack<'ll>,
	pub type_store: &'ll TypeStore,
}

impl<'ll> Llvm<'ll> {
	pub fn new(ctx: &'ll Context, module: Module<'ll>, type_store: &'ll TypeStore) -> Self {
		let builder = ctx.create_builder();
		let stack = mem::stack::Stack::new();
		Self { ctx, stack, module, builder, type_store }
	}

	pub fn compile(&mut self, root: &ir::Root) {
		for struct_ir in root.structs.iter() {
			self.llvm_struct(struct_ir);
		}

		for fn_ir in root.fns.iter() {
			self.llvm_function(fn_ir);
			self.stack.block_clear();
		}
	}
}
