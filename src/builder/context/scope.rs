use rustc_hash::FxHashMap;

use crate::checker::types::TypeId;

pub struct Local {
	pub name: String,
	pub type_id: TypeId,
}

pub struct Scope {
	pub locals: FxHashMap<String, Local>,
}

impl Default for Scope {
	fn default() -> Self {
		Self::new()
	}
}

impl Scope {
	pub fn new() -> Scope {
		Scope { locals: FxHashMap::default() }
	}

	pub fn add_local(&mut self, name: String, type_id: TypeId) {
		let local_key = name.to_string();
		let local = Local { name, type_id };
		self.locals.insert(local_key, local);
	}

	pub fn get_local(&self, name: &str) -> Option<&Local> {
		self.locals.get(name)
	}

	pub fn get_local_mut(&mut self, name: &str) -> Option<&mut Local> {
		self.locals.get_mut(name)
	}

	pub fn get_local_type(&self, name: &str) -> Option<&TypeId> {
		self.get_local(name).map(|local| &local.type_id)
	}
}
