use rustc_hash::FxHashMap;

use crate::{checker::types::TypeId, ir::IrBasicValue};

pub enum ScopeKind {
	Function {
		ret_type: TypeId,
	},
	Impl {
		self_name: String,
		self_type: TypeId,
	},
	Member {
		// self_value: Option<IrBasicValue>
	},
	Block,
}

pub struct Scope {
	pub kind: ScopeKind,
	pub locals: FxHashMap<String, IrBasicValue>,
	pub dont_load: FxHashMap<String, bool>,
	pub free_values: FxHashMap<String, IrBasicValue>,
}

impl Default for Scope {
	fn default() -> Self {
		Self::new()
	}
}

impl Scope {
	pub fn new() -> Self {
		let dont_load = FxHashMap::default();
		let locals = FxHashMap::default();
		let free_values = FxHashMap::default();
		Scope { dont_load, locals, free_values, kind: ScopeKind::Block }
	}

	pub fn new_function(ret_type: TypeId) -> Self {
		let dont_load = FxHashMap::default();
		let locals = FxHashMap::default();
		let free_values = FxHashMap::default();
		Scope { dont_load, locals, free_values, kind: ScopeKind::Function { ret_type } }
	}

	pub fn new_impl(self_name: String, self_type: TypeId) -> Self {
		let dont_load = FxHashMap::default();
		let locals = FxHashMap::default();
		let free_values = FxHashMap::default();
		Scope { dont_load, locals, free_values, kind: ScopeKind::Impl { self_name, self_type } }
	}

	pub fn new_member() -> Self {
		let dont_load = FxHashMap::default();
		let locals = FxHashMap::default();
		let free_values = FxHashMap::default();
		Scope { dont_load, locals, free_values, kind: ScopeKind::Member {} }
	}

	pub fn get_ret_type(&self) -> Option<TypeId> {
		match self.kind {
			ScopeKind::Function { ret_type } => Some(ret_type),
			_ => None,
		}
	}

	pub fn add_free_value(&mut self, value: IrBasicValue) {
		self.free_values.insert(value.value.as_str().into(), value);
	}

	pub fn get_free_values(&self) -> Vec<IrBasicValue> {
		self.free_values.values().cloned().collect()
	}

	pub fn is_impl_scope(&self) -> bool {
		matches!(&self.kind, ScopeKind::Impl { .. })
	}

	pub fn is_member_scope(&self) -> bool {
		matches!(&self.kind, ScopeKind::Member {})
	}

	pub fn get_self_info(&self) -> Option<(String, TypeId)> {
		match &self.kind {
			ScopeKind::Impl { self_name, self_type, .. } => Some((self_name.clone(), *self_type)),
			_ => None,
		}
	}

	pub fn is_function_scope(&self) -> bool {
		matches!(&self.kind, ScopeKind::Function { .. })
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
