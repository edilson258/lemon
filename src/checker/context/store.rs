use rustc_hash::FxHashMap;

use crate::checker::types::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoreId(pub usize, pub String); // (scope, name)

#[derive(Debug)]
pub struct Store {
	pub values: FxHashMap<StoreId, TypeId>,
}

impl Store {
	pub fn new() -> Self {
		Self { values: FxHashMap::default() }
	}

	pub fn add_value_type(&mut self, scope: usize, name: String, type_id: TypeId) {
		let id = StoreId(scope, name);
		self.values.insert(id, type_id);
	}

	pub fn get_value_type(&self, scope: usize, name: String) -> Option<&TypeId> {
		let id = StoreId(scope, name);
		self.values.get(&id)
	}
}

impl Default for Store {
	fn default() -> Self {
		Self::new()
	}
}
