use super::{
	borrow::BorrowChecker,
	events::Event,
	types::{TypeId, TypeStore},
};
use crate::loader::ModId;
use module::Module;
use rustc_hash::FxHashMap;
use scope::{Scope, ScopeKind};
use value::{FunctionValue, Value};

mod module;
pub mod scope;
pub mod value;

#[derive(Debug)]
pub struct Context {
	pub scopes: Vec<Scope>,
	pub event: Event,
	pub type_store: TypeStore,
	pub borrow: BorrowChecker,
	pub mods: FxHashMap<ModId, Module>,
	pub mod_id: ModId,
	pub mark_use: bool,
}

impl Context {
	pub fn new() -> Self {
		Self::new_with_defaults(ModId::new(0))
	}

	fn new_with_defaults(mod_id: ModId) -> Self {
		let event = Event::new();
		let type_store = TypeStore::default();
		let mods = FxHashMap::default();
		let borrow = BorrowChecker::new();
		Self {
			scopes: vec![Scope::default()],
			event,
			borrow,
			type_store,
			mods,
			mod_id,
			mark_use: false,
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
	}

	pub fn exit_scope(&mut self) {
		let Some(scope) = self.scopes.pop() else {
			return;
		};
		self.borrow.release_all_from_scope(&scope);
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
	pub fn add_value(&mut self, name: &str, value: Value) {
		self.get_scope_mut().add_variable(name.to_string(), value);
	}

	pub fn add_type_definition(&mut self, name: String, type_id: TypeId) {
		self.type_store.add_type_definition(name, type_id);
	}

	pub fn lookup_type_definition(&self, name: &str) -> Option<&TypeId> {
		self.type_store.lookup_type_definition(name)
	}

	pub fn lookup_variable_value(&self, name: &str) -> Option<&Value> {
		self.scopes.iter().rev().find_map(|scope| scope.lookup_variable(name))
	}
	pub fn lookup_variable_value_mut(&mut self, name: &str) -> Option<&mut Value> {
		self.scopes.iter_mut().rev().find_map(|scope| scope.lookup_variable_mut(name))
	}

	pub fn add_function_value(&mut self, name: &str, type_id: TypeId) {
		let value = FunctionValue::new_runtime(type_id);
		self.get_scope_mut().add_function(name.to_string(), value);
	}

	pub fn lookup_function_value(&self, name: &str) -> Option<&FunctionValue> {
		self.scopes.iter().rev().find_map(|scope| scope.lookup_function(name))
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}
