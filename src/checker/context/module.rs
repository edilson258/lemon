use rustc_hash::FxHashMap;

use crate::{checker::types::TypeId, loader::ModId};

#[derive(Debug)]
pub struct Module {
	pub mod_id: ModId,
	pub is_entry: bool,
	// types or values
	pub values: FxHashMap<String, TypeId>,
	// functions
	pub functions: FxHashMap<String, TypeId>,
}

impl Module {
	pub fn new(mod_id: ModId) -> Self {
		let values = FxHashMap::default();
		let functions = FxHashMap::default();
		Self { mod_id, values, functions, is_entry: false }
	}

	pub fn with_entry(mod_id: ModId) -> Self {
		let values = FxHashMap::default();
		let functions = FxHashMap::default();
		Self { mod_id, values, functions, is_entry: true }
	}

	pub fn add_value(&mut self, name: String, type_id: TypeId) {
		self.values.insert(name, type_id);
	}

	pub fn add_function(&mut self, name: String, type_id: TypeId) {
		self.functions.insert(name, type_id);
	}

	pub fn get_value(&self, name: &str) -> Option<&TypeId> {
		self.values.get(name)
	}

	pub fn get_function(&self, name: &str) -> Option<&TypeId> {
		self.functions.get(name)
	}
}
