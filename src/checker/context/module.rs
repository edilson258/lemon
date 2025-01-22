use rustc_hash::FxHashMap;

use crate::checker::types::TypeId;

#[derive(Debug)]
pub struct ModuleId(pub(crate) u64);

impl ModuleId {
	pub fn new(id: u64) -> Self {
		Self(id)
	}
	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}

	pub fn as_string(&self) -> String {
		format!("m{}", self.0)
	}
}
#[derive(Debug)]
pub struct Module {
	pub id: ModuleId,
	// types or values
	pub values: FxHashMap<String, TypeId>,
	// functions
	pub fns: FxHashMap<String, TypeId>,
}

impl Module {
	pub fn new(id: ModuleId) -> Self {
		Self { id, values: FxHashMap::default(), fns: FxHashMap::default() }
	}

	pub fn add_value(&mut self, name: String, type_id: TypeId) {
		self.values.insert(name, type_id);
	}

	pub fn add_fn(&mut self, name: String, type_id: TypeId) {
		self.fns.insert(name, type_id);
	}

	pub fn get_value(&self, name: &str) -> Option<&TypeId> {
		self.values.get(name)
	}

	pub fn get_fn(&self, name: &str) -> Option<&TypeId> {
		self.fns.get(name)
	}
}
