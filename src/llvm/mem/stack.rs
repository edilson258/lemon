#![allow(dead_code)]

use std::collections::HashMap;

use inkwell::{
	basic_block::BasicBlock,
	values::{BasicValueEnum, PointerValue},
};

use crate::{
	ir::{self},
	report::throw_llvm_error,
};

pub struct Stack<'ll> {
	pub values: HashMap<ir::Register, BasicValueEnum<'ll>>,
	pub global_values: HashMap<ir::Register, BasicValueEnum<'ll>>,
	pub blocks: HashMap<ir::BlockId, BasicBlock<'ll>>,
	pub temp_reg_count: usize,
}

impl<'ll> Stack<'ll> {
	pub fn new() -> Self {
		Self {
			values: HashMap::new(),
			blocks: HashMap::new(),
			global_values: HashMap::new(),
			temp_reg_count: 0,
		}
	}

	pub fn get_temp_reg(&mut self) -> String {
		let reg = format!("t{}", self.temp_reg_count);
		self.temp_reg_count += 1;
		reg
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
			None => throw_llvm_error(format!("not found '{}'", reg.as_string())),
		}
	}
}
