use inkwell::{
	basic_block::BasicBlock,
	values::{BasicValueEnum, PointerValue},
};
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Scope<'ll> {
	block_store: FxHashMap<String, BasicBlock<'ll>>,
	value_store: FxHashMap<String, BasicValueEnum<'ll>>,
	temp_count: u32,
}

impl<'ll> Scope<'ll> {
	pub fn new() -> Self {
		let block_store = FxHashMap::default();
		let value_store = FxHashMap::default();
		Self { block_store, value_store, temp_count: 0 }
	}

	pub fn get_temp(&mut self) -> String {
		let name = format!("temp_{}", self.temp_count);
		self.temp_count += 1;
		name
	}

	pub fn get_block(&self, name: &str) -> Option<&BasicBlock<'ll>> {
		self.block_store.get(name)
	}

	pub fn set_block(&mut self, name: &str, block: BasicBlock<'ll>) {
		self.block_store.insert(name.to_string(), block);
	}

	pub fn get_value(&self, name: &str) -> Option<&BasicValueEnum<'ll>> {
		self.value_store.get(name)
	}

	pub fn set_value(&mut self, name: &str, value: BasicValueEnum<'ll>) {
		self.value_store.insert(name.to_string(), value);
	}

	pub fn get_ptr_value(&self, name: &str) -> Option<PointerValue<'ll>> {
		let value = self.get_value(name)?;
		if value.is_pointer_value() {
			return Some(value.into_pointer_value());
		}
		None
	}
}
