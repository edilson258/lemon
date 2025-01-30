#![allow(dead_code)]
use inkwell::{
	basic_block::BasicBlock,
	types::StructType,
	values::{BasicValueEnum, PointerValue},
};
use rustc_hash::FxHashMap;

use crate::{
	checker::types::TypeId,
	ir::{self, Register},
	report::throw_llvm_error,
};

pub struct Stack<'ll> {
	pub values: FxHashMap<ir::Register, BasicValueEnum<'ll>>,
	pub structs: FxHashMap<String, FxHashMap<ir::Register, usize>>,
	pub struct_table_type: FxHashMap<ir::Register, StructType<'ll>>,
	pub global_values: FxHashMap<ir::Register, BasicValueEnum<'ll>>,
	pub blocks: FxHashMap<ir::BlockId, BasicBlock<'ll>>,
	pub llvm_type_store: FxHashMap<TypeId, StructType<'ll>>,
	frees: FxHashMap<Register, PointerValue<'ll>>,
	pub temp_count: usize,
}

impl<'ll> Stack<'ll> {
	pub fn new() -> Self {
		let frees = FxHashMap::default();
		let values = FxHashMap::default();
		let blocks = FxHashMap::default();
		let structs = FxHashMap::default();
		let global_values = FxHashMap::default();
		let llvm_type_store = FxHashMap::default();
		let temp_count = 0;
		let struct_table_type = FxHashMap::default();
		Self {
			frees,
			values,
			blocks,
			structs,
			global_values,
			llvm_type_store,
			temp_count,
			struct_table_type,
		}
	}

	pub fn set_register_type(&mut self, register: Register, struct_type: StructType<'ll>) {
		self.struct_table_type.insert(register, struct_type);
	}

	pub fn get_register_type(&self, register: Register) -> Option<&StructType<'ll>> {
		self.struct_table_type.get(&register)
	}

	// trance pointer
	pub fn set_free_ptr(&mut self, register: Register, pointer: PointerValue<'ll>) {
		self.frees.insert(register, pointer);
	}

	pub fn get_free_ptr(&self, register: Register) -> Option<&PointerValue<'ll>> {
		self.frees.get(&register)
	}

	pub fn take_frees(&mut self) -> FxHashMap<Register, PointerValue<'ll>> {
		let mut frees = FxHashMap::default();
		std::mem::swap(&mut frees, &mut self.frees);
		frees
	}

	pub fn reset_frees(&mut self) {
		self.frees.clear();
	}

	pub fn get_struct_type(&self, type_id: TypeId) -> Option<&StructType<'ll>> {
		self.llvm_type_store.get(&type_id)
	}

	pub fn set_struct_type(&mut self, type_id: TypeId, struct_type: StructType<'ll>) {
		self.llvm_type_store.insert(type_id, struct_type);
	}

	pub fn temp_register(&mut self) -> String {
		let reg = format!("ter{}", self.temp_count);
		self.temp_count += 1;
		reg
	}

	pub fn add_struct_field(&mut self, struct_id: String, reg: ir::Register, index: usize) {
		let fields = self.structs.entry(struct_id).or_default();
		fields.insert(reg, index);
	}

	pub fn get_struct_field(&self, struct_id: &str, reg: ir::Register) -> Option<usize> {
		let fields = self.structs.get(struct_id)?;
		fields.get(&reg).copied()
	}

	pub fn get_global_value(&self, reg: ir::Register) -> Option<&BasicValueEnum<'ll>> {
		self.global_values.get(&reg)
	}

	pub fn set_global_value(&mut self, reg: ir::Register, value: BasicValueEnum<'ll>) {
		self.global_values.insert(reg, value);
	}

	pub fn get_gloabl_count(&mut self) -> usize {
		self.global_values.len()
	}

	//blcoks
	pub fn set_block(&mut self, block_id: ir::BlockId, block: BasicBlock<'ll>) {
		self.blocks.insert(block_id, block);
	}

	pub fn get_block(&self, block_id: ir::BlockId) -> Option<&BasicBlock<'ll>> {
		self.blocks.get(&block_id)
	}

	pub fn get_ptr_value(&self, reg: ir::Register) -> PointerValue<'ll> {
		if let BasicValueEnum::PointerValue(ptr) = self.get_value(reg) {
			return *ptr;
		}
		throw_llvm_error(format!("not found pointer '{}'", reg.as_string()));
	}

	pub fn get_basic_value(&self, reg: ir::Register) -> BasicValueEnum<'ll> {
		let value = self.get_value(reg);
		if !value.is_pointer_value() {
			return *value;
		}
		throw_llvm_error(format!("not found value '{}'", reg.as_string()));
	}

	pub fn block_clear(&mut self) {
		self.blocks.clear();
	}

	pub fn has_value(&self, reg: ir::Register) -> bool {
		self.values.contains_key(&reg)
	}
	pub fn has_ptr_value(&self, reg: ir::Register) -> bool {
		self.has_value(reg) && self.get_value(reg).is_pointer_value()
	}

	pub fn set_value(&mut self, reg: ir::Register, value: BasicValueEnum<'ll>) {
		self.values.insert(reg, value);
	}

	pub fn get_value(&self, reg: ir::Register) -> &BasicValueEnum<'ll> {
		match self.values.get(&reg) {
			Some(value) => value,
			None => throw_llvm_error(format!("not found value at '{}'", reg.as_string())),
		}
	}
}
