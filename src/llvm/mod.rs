#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use inkwell::{
	basic_block::BasicBlock, builder::Builder, context::Context, module::Module, types::BasicTypeEnum,
};

use crate::{
	checker::types::{TypeId, TypeStore},
	ir::{self},
	report::throw_llvm_error,
	source::Source,
};
mod bind;
mod llvm_block;
mod llvm_call;
mod llvm_cmp;
mod llvm_function;
mod llvm_instr;
mod llvm_math;
mod llvm_mem;
mod llvm_ret;
mod llvm_value;
mod mem;
pub fn create_module_from_source<'ll>(ctx: &'ll Context, source: &Source) -> Module<'ll> {
	let module = ctx.create_module(source.file_name());
	module
}

pub struct Llvm<'ll> {
	pub ctx: &'ll Context,
	pub stack: mem::stack::Stack<'ll>,
	pub module: Module<'ll>,
	pub builder: Builder<'ll>,
	pub type_store: &'ll TypeStore,
	pub block_store: HashMap<ir::BlockId, BasicBlock<'ll>>,
}

impl<'ll> Llvm<'ll> {
	pub fn new(ctx: &'ll Context, module: Module<'ll>, type_store: &'ll TypeStore) -> Self {
		let builder = ctx.create_builder();
		let block_store = HashMap::new();
		let stack = mem::stack::Stack::new();
		Self { ctx, stack, module, builder, type_store, block_store }
	}
	pub fn compile(&mut self, root: &ir::Root) {
		for fn_ir in root.fns.iter() {
			self.llvm_function(fn_ir);
			self.block_store.clear(); // clear block store
		}
	}

	// types
	//
	pub fn llvm_type_from_type(&self, type_id: TypeId) -> Option<BasicTypeEnum<'ll>> {
		let found = self.type_store.resolve_borrow_type(type_id);
		match found {
			TypeId::I8 => Some(self.ctx.i8_type().into()),
			TypeId::I16 => Some(self.ctx.i16_type().into()),
			TypeId::I32 => Some(self.ctx.i32_type().into()),
			TypeId::I64 => Some(self.ctx.i64_type().into()),
			TypeId::U8 => Some(self.ctx.i8_type().into()),
			TypeId::U16 => Some(self.ctx.i16_type().into()),
			TypeId::U32 => Some(self.ctx.i32_type().into()),
			TypeId::U64 => Some(self.ctx.i64_type().into()),
			TypeId::F32 => Some(self.ctx.f32_type().into()),
			TypeId::F64 => Some(self.ctx.f64_type().into()),
			TypeId::BOOL => Some(self.ctx.bool_type().into()),
			TypeId::CHAR => Some(self.ctx.i8_type().into()),
			TypeId::STRING => Some(self.ctx.i8_type().into()),
			TypeId::UNIT | TypeId::VOID => None, // void
			found => {
				let text = self.type_store.get_display_type(found);
				throw_llvm_error(format!("type '{}' not found", text))
			}
		}
	}
}
