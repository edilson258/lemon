use super::types::{TypeId, TypeStore};
use crate::loader::ModId;
use flow::Flow;
use module::Module;
use ownership::OwnershipId;
use rustc_hash::FxHashMap;
use scope::{Scope, ScopeKind};
use store::Store;
use value::{Value, ValueId};

mod bind;
mod flow;
mod module;
mod ownership;
pub mod scope;
pub mod store;
pub mod value;

#[derive(Debug)]
pub struct Context {
	pub scopes: Vec<Scope>,
	pub flow: Flow,
	pub type_store: TypeStore,
	pub store: Store,
	pub value_id: ValueId,
	pub store_id: usize,
	pub mods: FxHashMap<ModId, Module>,
	pub mod_id: ModId,
}

impl Context {
	pub fn new() -> Self {
		Self::new_with_defaults(ModId::new(0))
	}

	fn new_with_defaults(mod_id: ModId) -> Self {
		Self {
			scopes: vec![Scope::default()],
			flow: Flow::new(),
			type_store: TypeStore::default(),
			store: Store::new(),
			value_id: ValueId::init(),
			store_id: 0,
			mods: FxHashMap::default(),
			mod_id,
		}
	}

	// ======= scope methods =======
	pub fn get_scope(&self) -> &Scope {
		self.scopes.last().unwrap()
	}

	pub fn get_scope_mut(&mut self) -> &mut Scope {
		self.scopes.last_mut().unwrap()
	}

	pub fn enter_scope(&mut self, scope_kind: ScopeKind) {
		self.scopes.push(Scope::new(scope_kind));
		self.store_id += 1;
	}

	pub fn exit_scope(&mut self) {
		self.scopes.pop();
	}

	pub fn is_global_scope(&self) -> bool {
		self.get_scope().is_global_scope()
	}

	pub fn has_function_scope(&self) -> bool {
		self.scopes.iter().rev().any(Scope::is_function_scope)
	}

	pub fn has_loop_scope(&self) -> bool {
		self.scopes.iter().rev().any(Scope::is_loop_scope)
	}

	pub fn has_block_scope(&self) -> bool {
		self.scopes.iter().rev().any(Scope::is_block_scope)
	}

	pub fn has_global_scope(&self) -> bool {
		self.scopes.iter().rev().any(Scope::is_global_scope)
	}

	pub fn has_implementation_scope(&self) -> bool {
		self.scopes.iter().rev().any(Scope::is_implementation_scope)
	}

	pub fn is_accessor_scope(&self) -> bool {
		self.get_scope().is_accessor_scope()
	}

	pub fn is_associated_scope(&self) -> bool {
		match self.get_scope().kind {
			ScopeKind::Accessor { self_type, is_associated } => is_associated,
			_ => false,
		}
	}

	pub fn get_self_scope_type(&self) -> Option<TypeId> {
		self.scopes.iter().rev().find_map(Scope::get_self_scope_type)
	}

	pub fn get_accessor_scope_type(&self) -> Option<TypeId> {
		self.get_scope().get_accessor_type()
	}

	pub fn get_return_type(&self) -> Option<TypeId> {
		self.scopes.iter().rev().find_map(Scope::get_return_type)
	}

	// ======= module methods =======
	pub fn get_module(&self, mod_id: ModId) -> Option<&Module> {
		self.mods.get(&mod_id)
	}

	pub fn get_module_mut(&mut self, mod_id: ModId) -> Option<&mut Module> {
		self.mods.get_mut(&mod_id)
	}

	pub fn swap_mod(&mut self, mod_id: ModId) {
		self.mod_id = mod_id;
	}

	pub fn add_mod(&mut self, mod_id: ModId) {
		self.mods.insert(mod_id, Module::new(mod_id));
		// self.swap_mod(mod_id);
	}

	pub fn add_entry_mod(&mut self, mod_id: ModId) {
		self.mods.insert(mod_id, Module::with_entry(mod_id));
	}

	pub fn is_entry_module(&self, mod_id: ModId) -> bool {
		self.mods.get(&mod_id).is_some_and(|module| module.is_entry)
	}

	pub fn get_current_mod(&self) -> Option<&Module> {
		self.mods.get(&self.mod_id)
	}

	pub fn add_pub_value(&mut self, name: String, type_id: TypeId) {
		if let Some(module) = self.mods.get_mut(&self.mod_id) {
			module.add_value(name, type_id);
		}
	}

	pub fn add_pub_function(&mut self, name: String, type_id: TypeId) {
		if let Some(module) = self.mods.get_mut(&self.mod_id) {
			module.add_function(name, type_id);
		}
	}

	// ======= value methods =======
	pub fn add_value(&mut self, name: &str, type_id: TypeId, is_mut: bool) -> ValueId {
		let value = Value::new_scoped(self.value_id, type_id, is_mut);
		self.store.add_value_type(self.store_id, name.to_string(), type_id);
		self.get_scope_mut().add_variable(name.to_string(), value);
		self.value_id.next_id()
	}

	pub fn get_value(&self, name: &str) -> Option<&Value> {
		self.scopes.iter().rev().find_map(|scope| scope.get_variable(name))
	}

	pub fn add_fn_value(&mut self, name: &str, type_id: TypeId) -> ValueId {
		let value = Value::new_scoped(self.value_id, type_id, false);
		self.store.add_value_type(self.store_id, name.to_string(), type_id);
		self.get_scope_mut().add_function(name.to_string(), value);
		self.value_id.next_id()
	}

	pub fn get_fn_value(&self, name: &str) -> Option<&Value> {
		self.scopes.iter().rev().find_map(|scope| scope.get_function(name))
	}

	pub fn contains_fn_value_in_current_scope(&self, name: &str) -> bool {
		self.get_scope().has_function(name)
	}

	// ====== borrow methods =======
	pub fn add_borrow(&mut self, value_id: ValueId, is_mut: bool) -> Option<OwnershipId> {
		todo!()
		// if self.get_scope().can_borrow_as(value_id, is_mut) {
		// 	Some(self.get_scope_mut().add_borrow_value(value_id, is_mut))
		// } else {
		// 	None
		// }
	}

	pub fn release_borrow(&mut self, borrow_id: OwnershipId) {
		// self.get_scope_mut().drop_borrows(borrow_id);
	}

	#[rustfmt::skip]
	pub fn can_borrow_as(&self, name: &str, is_mut: bool) -> bool {
		todo!()
		// self.get_value(name).map(|value|
		//   // we should throw an error?
		// 	self.get_scope().can_borrow_as(value.id, is_mut)).unwrap_or(true)
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}
