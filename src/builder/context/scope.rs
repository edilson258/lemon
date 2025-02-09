use rustc_hash::FxHashMap;

use crate::{
	checker::types::TypeId,
	ir::{BasicValue, IrBasicValue},
};

pub struct Local {
	pub name: String,
	pub type_id: TypeId,
}

pub struct Scope {
	pub locals: FxHashMap<String, IrBasicValue>,
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

	pub fn add_local(&mut self, key: String, basic_value: IrBasicValue) {
		self.locals.insert(key, basic_value);
	}

	pub fn get_local(&self, name: &str) -> Option<&IrBasicValue> {
		self.locals.get(name)
	}

	pub fn get_local_mut(&mut self, name: &str) -> Option<&mut IrBasicValue> {
		self.locals.get_mut(name)
	}

	pub fn get_local_type(&self, name: &str) -> Option<&TypeId> {
		self.get_local(name).map(|local| &local.type_id)
	}
}
