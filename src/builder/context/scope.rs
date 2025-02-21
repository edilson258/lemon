use rustc_hash::{FxHashMap, FxHashSet};

use crate::{checker::types::TypeId, ir::IrBasicValue};

pub enum ScopeKind {
	Function { ret_type: TypeId },
	Impl { self_name: String, self_type: TypeId, refs: FxHashSet<String> },
	Block,
}

pub struct Scope {
	pub kind: ScopeKind,
	pub locals: FxHashMap<String, IrBasicValue>,
	pub dont_load: FxHashMap<String, bool>,
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
		Scope { dont_load, locals, kind: ScopeKind::Block }
	}

	pub fn new_function(ret_type: TypeId) -> Self {
		let dont_load = FxHashMap::default();
		let locals = FxHashMap::default();
		Scope { dont_load, locals, kind: ScopeKind::Function { ret_type } }
	}

	pub fn new_impl(self_name: String, self_type: TypeId) -> Self {
		let dont_load = FxHashMap::default();
		let locals = FxHashMap::default();
		let refs = FxHashSet::default();
		Scope { dont_load, locals, kind: ScopeKind::Impl { self_name, self_type, refs } }
	}

	pub fn add_self_ref(&mut self, ref_name: String) {
		if let ScopeKind::Impl { refs, .. } = &mut self.kind {
			refs.insert(ref_name);
		}
	}

	pub fn get_ret_type(&self) -> Option<TypeId> {
		match self.kind {
			ScopeKind::Function { ret_type } => Some(ret_type),
			_ => None,
		}
	}

	pub fn is_impl_scope(&self) -> bool {
		matches!(&self.kind, ScopeKind::Impl { .. })
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
