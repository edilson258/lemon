use rustc_hash::FxHashMap;

use crate::{checker::types::TypeId, ir::IrBasicValue};

pub struct Scope {
	pub ret_type: Option<TypeId>,
	pub locals: FxHashMap<String, IrBasicValue>,
	pub dont_load: FxHashMap<String, bool>,
}

impl Default for Scope {
	fn default() -> Self {
		Self::new(None)
	}
}

impl Scope {
	pub fn new(ret_type: Option<TypeId>) -> Self {
		Scope { ret_type, locals: FxHashMap::default(), dont_load: FxHashMap::default() }
	}

	pub fn get_ret_type(&self) -> Option<&TypeId> {
		self.ret_type.as_ref()
	}

	pub fn add_dont_load(&mut self, key: impl Into<String>) {
		self.dont_load.insert(key.into(), true);
	}

	pub fn is_dont_load(&self, key: &str) -> bool {
		self.dont_load.contains_key(key)
	}

	pub fn add_local(&mut self, key: String, basic_value: IrBasicValue) {
		self.locals.insert(key, basic_value);
	}

	pub fn get_local(&self, name: &str) -> Option<&IrBasicValue> {
		self.locals.get(name)
	}

	// pub fn get_local_mut(&mut self, name: &str) -> Option<&mut IrBasicValue> {
	// 	self.locals.get_mut(name)
	// }

	// pub fn get_local_type(&self, name: &str) -> Option<&TypeId> {
	// 	self.get_local(name).map(|local| &local.type_id)
	// }
}
