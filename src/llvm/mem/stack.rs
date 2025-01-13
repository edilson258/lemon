use std::collections::HashMap;

use inkwell::{
	basic_block::BasicBlock,
	builder::Builder,
	types::{BasicType, BasicTypeEnum},
	values::{self, BasicValueEnum},
};

use crate::{
	ir::{self},
	report::throw_llvm_error,
};

pub struct Stack<'ll> {
	pub values: HashMap<ir::Register, values::BasicValueEnum<'ll>>,
	pub blocks: HashMap<ir::BlockId, BasicBlock<'ll>>,
}

impl<'ll> Stack<'ll> {
	pub fn new() -> Self {
		Self { values: HashMap::new(), blocks: HashMap::new() }
	}

	//blcoks
	pub fn set_block(&mut self, block_id: ir::BlockId, block: BasicBlock<'ll>) {
		self.blocks.insert(block_id, block);
	}

	pub fn get_block(&self, block_id: ir::BlockId) -> Option<&BasicBlock<'ll>> {
		self.blocks.get(&block_id)
	}

	pub fn block_clear(&mut self) {
		self.blocks.clear();
	}

	pub fn set_value(&mut self, reg: ir::Register, value: values::BasicValueEnum<'ll>) {
		self.values.insert(reg, value);
	}

	pub fn get_value(&self, reg: ir::Register) -> &values::BasicValueEnum<'ll> {
		match self.values.get(&reg) {
			Some(value) => value,
			None => throw_llvm_error(format!("stack not found '{}'", reg.as_string())),
		}
	}

	pub fn load(&mut self, ptr: ir::Register, dest: ir::Register, builder: &mut Builder<'ll>) {
		let value = self.get_value(ptr);

		if !value.is_pointer_value() {
			throw_llvm_error(format!("Cannot load from non-pointer value '{}'", ptr.as_string()));
		}

		let llvm_ptr = value.into_pointer_value();
		let llvm_type = llvm_ptr.get_type().as_basic_type_enum(); // Verifica o tipo do valor referenciado

		// Garante que o destino tenha o mesmo tipo
		let loaded_value = match builder.build_load(llvm_type, llvm_ptr, &dest.as_string()) {
			Ok(value) => value,
			Err(err) => throw_llvm_error(format!("stack load error: {}", err)),
		};

		self.set_value(dest, loaded_value);
	}

	pub fn allocate(
		&mut self,
		llvm_type: BasicTypeEnum<'ll>,
		dest: ir::Register,
		builder: &mut Builder<'ll>,
	) {
		let value = match builder.build_alloca(llvm_type, &dest.as_string()) {
			Ok(value) => value,
			Err(err) => throw_llvm_error(format!("stack allocate error: {}", err)),
		};
		self.set_value(dest, value.into());
	}

	pub fn save(
		&mut self,
		value: BasicValueEnum<'ll>,
		dest: ir::Register,
		builder: &mut Builder<'ll>,
	) {
		let ptr = self.get_value(dest);
		if !ptr.is_pointer_value() {
			throw_llvm_error(format!("stack not found pointer '{}'", dest.as_string()));
		}
		let llvm_ptr = ptr.into_pointer_value();
		match builder.build_store(llvm_ptr, value) {
			Ok(_) => {}
			Err(err) => throw_llvm_error(format!("stack store error: {}", err)),
		}
		// is ok?
		self.set_value(dest, value);
	}
}
