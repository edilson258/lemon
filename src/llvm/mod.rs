#![allow(dead_code, unused_variables)]
use std::collections::{HashMap, HashSet};

use inkwell::{
	basic_block::BasicBlock,
	builder::Builder,
	context::Context,
	module::Module,
	types::{BasicType, BasicTypeEnum},
	values::{BasicValueEnum, FunctionValue, PointerValue},
};

use crate::{
	checker::types::{Type, TypeId, TypeStore},
	ir::{self, Register},
	report::throw_llvm_error,
	source::Source,
};
mod compile_block;
mod compile_call;
mod compile_comparison;
mod compile_fn_ir;
mod compile_import;
mod compile_instr;
mod compile_math;
mod compile_std;

pub fn create_module_from_source<'ll>(ctx: &'ll Context, source: &Source) -> Module<'ll> {
	let module = ctx.create_module(source.file_name());
	module
}

pub struct Llvm<'ll> {
	pub ctx: &'ll Context,
	pub module: Module<'ll>,
	pub builder: Builder<'ll>,
	pub type_store: &'ll TypeStore,
	pub value_store: HashMap<ir::Register, BasicValueEnum<'ll>>,
	pub block_store: HashMap<ir::BlockId, BasicBlock<'ll>>,
	pub c_fns: HashMap<String, FunctionValue<'ll>>,
	pub ported: HashSet<String>,
}

impl<'ll> Llvm<'ll> {
	pub fn new(ctx: &'ll Context, module: Module<'ll>, type_store: &'ll TypeStore) -> Self {
		let builder = ctx.create_builder();
		let value_store = HashMap::new();
		let block_store = HashMap::new();
		let c_fns = HashMap::new(); // std functions
		let ported = HashSet::new(); // ported std  functions
		Self { ctx, module, builder, type_store, value_store, block_store, c_fns, ported }
	}
	pub fn compile_root_ir(&mut self, root: &ir::Root) {
		if self.c_fns.is_empty() {
			let c_fns = self.generate_std_module();
			self.c_fns = c_fns;
		}
		for fn_ir in root.fns.iter() {
			self.compile_fn_ir(fn_ir);
			self.block_store.clear(); // clear block store
		}
	}

	fn load_value(&mut self, reg: Register) -> BasicValueEnum<'ll> {
		match self.value_store.get(&reg) {
			Some(value) => *value,
			None => throw_llvm_error(format!("{} not found", reg.as_string())),
		}
	}
	fn insert_value(&mut self, reg: Register, value: BasicValueEnum<'ll>) {
		self.value_store.insert(reg, value);
	}

	fn allocate_register<T: BasicType<'ll>>(&mut self, base: T, dest: Register) -> PointerValue<'ll> {
		let ptr = match self.builder.build_alloca(base, &dest.as_string()) {
			Ok(ptr) => ptr,
			Err(err) => throw_llvm_error(format!("allocate register {}", dest.as_string())),
		};
		// self.ptr_store.insert(dest, ptr);
		ptr
	}

	// fn get_value_from_register(&mut self, reg: Register) -> BasicValueEnum<'ll> {
	// 	let ptr = *self.get_register(reg);
	// 	let ptr_type = ptr.as_basic_value_enum().get_type();
	// 	match self.builder.build_load(ptr_type, ptr, &reg.as_string()) {
	// 		Ok(value) => value,
	// 		Err(err) => throw_llvm_error(format!("load error: {:?}", err)),
	// 	}
	// }

	// fn store_value_to_ptr<V: BasicValue<'ll>>(&self, value: V, ptr: PointerValue<'ll>) {
	// 	match self.builder.build_store(ptr, value) {
	// 		Ok(_) => {}
	// 		Err(err) => throw_llvm_error(format!("borrow error: {:?}", err)),
	// 	};
	// }

	pub fn set_block(&mut self, block_id: &ir::BlockId, block: BasicBlock<'ll>) {
		self.block_store.insert(*block_id, block);
	}

	pub fn get_block(&self, block_id: &ir::BlockId) -> &BasicBlock<'ll> {
		match self.block_store.get(block_id) {
			Some(block) => block,
			None => throw_llvm_error(format!("{:?} block not found", block_id)),
		}
	}

	fn compile_type_id(&mut self, type_id: TypeId) -> Option<BasicTypeEnum<'ll>> {
		match type_id {
			TypeId::I8 => Some(self.ctx.i8_type().into()),
			TypeId::I16 => Some(self.ctx.i16_type().into()),
			TypeId::I32 => Some(self.ctx.i32_type().into()),
			TypeId::I64 => Some(self.ctx.i64_type().into()),
			TypeId::INT => Some(self.ctx.i64_type().into()),
			// todo: check if this is correct
			// usize
			TypeId::U8 => Some(self.ctx.i8_type().into()),
			TypeId::U16 => Some(self.ctx.i16_type().into()),
			TypeId::U32 => Some(self.ctx.i32_type().into()),
			TypeId::U64 => Some(self.ctx.i64_type().into()),
			TypeId::USIZE => Some(self.ctx.i64_type().into()),
			// todo: check if this is correct
			TypeId::FLOAT32 => Some(self.ctx.f32_type().into()),
			TypeId::FLOAT64 => Some(self.ctx.f64_type().into()),
			TypeId::BOOL => Some(self.ctx.bool_type().into()),
			TypeId::CHAR => Some(self.ctx.i8_type().into()),
			TypeId::STRING => Some(self.ctx.i8_type().into()),
			TypeId::NOTHING => None, // void
			_ => match self.type_store.get_type(type_id).unwrap() {
				Type::ConstFn(const_type) => self.compile_type_id(const_type.value),
				Type::Par { target } => self.compile_type_id(*target),
				_ => throw_llvm_error(format!("type {:?} not found", type_id)),
			},
		}
	}
}
