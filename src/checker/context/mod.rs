// use bind::create_binds;
use super::types::{monomorphic::MonomorphicStore, TypeId, TypeStore};
use borrow::BorrowId;
use flow::Flow;
use module::{Module, ModuleId};
use scope::{Scope, ScopeType};
use store::Store;
use value::{Value, ValueId};

mod bind;
mod borrow;
mod flow;
mod module;
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
	pub modules: Vec<Module>,
}

impl Context {
	pub fn new() -> Self {
		let scopes = Vec::from_iter(vec![Scope::default()]);
		let type_store = TypeStore::default();
		let store = Store::new();
		let flow = Flow::new();
		let value_id = ValueId::init();
		let store_id = 0;
		let modules = Vec::from_iter(vec![Module::new(ModuleId::new(0))]);
		Self { scopes, flow, type_store, store, value_id, store_id, modules }
	}

	pub fn get_monomorphic_store(&mut self) -> &mut MonomorphicStore {
		&mut self.type_store.monomorphic_store
	}

	pub fn get_type_store(&self) -> &TypeStore {
		&self.type_store
	}

	pub fn get_scope(&self) -> &Scope {
		self.scopes.last().unwrap()
	}

	pub fn get_module(&self, module_id: ModuleId) -> &Module {
		self.modules.get(module_id.as_usize()).unwrap()
	}

	pub fn add_pub_value(&mut self, name: String, type_id: TypeId) {
		todo!()
	}

	pub fn add_pub_fn(&mut self, name: String, type_id: TypeId) {
		todo!()
	}

	pub fn get_scope_mut(&mut self) -> &mut Scope {
		self.scopes.last_mut().unwrap()
	}

	pub fn enter_scope(&mut self, scope_type: ScopeType) {
		self.scopes.push(Scope::new(scope_type));
		self.store_id += 1;
	}

	pub fn exit_scope(&mut self) {
		self.scopes.pop();
	}

	pub fn is_global_scope(&self) -> bool {
		self.get_scope().is_global_scope()
	}

	pub fn has_fn_scope(&self) -> bool {
		self.scopes.iter().rev().any(|scope| scope.is_fn_scope())
	}

	pub fn self_scope_type(&self) -> Option<TypeId> {
		self.scopes.iter().rev().find_map(|scope| scope.self_scope())
	}

	pub fn has_accessor_scope(&self) -> bool {
		self.get_scope().is_accessor_scope()
	}
	pub fn is_acessor_associate_scope(&self) -> bool {
		self.get_scope().is_accessor_associate_scope()
	}

	pub fn accessor_scope_type(&self) -> Option<TypeId> {
		self.get_scope().accessor_type()
	}

	pub fn has_impl_scope(&self) -> bool {
		self.scopes.iter().rev().any(|scope| scope.is_impl_scope())
	}

	pub fn has_loop_scope(&self) -> bool {
		self.scopes.iter().rev().any(|scope| scope.is_loop_scope())
	}

	pub fn ret_scope_type(&self) -> Option<TypeId> {
		self.scopes.iter().rev().find_map(|scope| scope.ret_scope())
	}
	// values
	pub fn add_value(&mut self, name: &str, type_id: TypeId, is_mut: bool) -> ValueId {
		let value = Value::new_scoped(self.value_id, type_id, is_mut);
		self.store.add_value_type(self.store_id, name.to_string(), type_id);
		self.get_scope_mut().add_value(name.to_string(), value);
		self.value_id.next_id()
	}

	pub fn get_value(&self, name: &str) -> Option<&Value> {
		self.scopes.iter().rev().find_map(|scope| scope.get_value(name))
	}
	// fns
	pub fn add_fn_value(&mut self, name: &str, type_id: TypeId) -> ValueId {
		let value = Value::new_scoped(self.value_id, type_id, false);
		self.store.add_value_type(self.store_id, name.to_string(), type_id);
		self.get_scope_mut().add_fn_value(name.to_string(), value);
		self.value_id.next_id()
	}

	pub fn get_fn_value(&self, name: &str) -> Option<&Value> {
		self.scopes.iter().rev().find_map(|scope| scope.get_fn_value(name))
	}

	pub fn contains_fn_value_in_current_scope(&self, name: &str) -> bool {
		self.get_scope().has_fn_value(name)
	}

	// borrows
	//
	//
	pub fn add_borrow(&mut self, value_id: ValueId, is_mut: bool) -> Option<BorrowId> {
		if !self.get_scope().can_borrow_as(value_id, is_mut) {
			return None;
		}
		Some(self.get_scope_mut().add_borrow_value(value_id, is_mut))
	}

	pub fn release_borrow(&mut self, borrow_id: BorrowId) {
		self.get_scope_mut().drop_borrows(borrow_id)
	}
	pub fn can_borrow_as(&self, name: &str, is_mut: bool) -> bool {
		if let Some(value_id) = self.get_value(name).map(|value| value.id) {
			return self.get_scope().can_borrow_as(value_id, is_mut);
		}
		// thorow error?
		true
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}
