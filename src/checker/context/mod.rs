use borrow::{BorrowId, BorrowStore};
use scope::{Scope, ScopeType};
use store::Store;
use value::{Value, ValueId};

use super::types::{TypeId, TypeStore};

mod borrow;
pub mod scope;
pub mod store;
pub mod value;

#[derive(Debug)]
pub struct Context {
	pub scopes: Vec<Scope>,
	pub borrow_store: BorrowStore,
	pub type_store: TypeStore,
	pub store: Store,
	pub value_id: ValueId,
	pub store_id: usize,
}

impl Context {
	pub fn new() -> Self {
		let scopes = Vec::from_iter(vec![Scope::default()]);
		let borrow_store = BorrowStore::default();
		let type_store = TypeStore::default();
		let store = Store::new();
		let value_id = ValueId::init();
		let store_id = 0;
		Self { scopes, borrow_store, type_store, store, value_id, store_id }
	}

	pub fn get_type_store(&self) -> &TypeStore {
		&self.type_store
	}

	pub fn get_scope(&self) -> &Scope {
		self.scopes.last().unwrap()
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

	pub fn has_loop_scope(&self) -> bool {
		self.scopes.iter().rev().any(|scope| scope.is_loop_scope())
	}

	pub fn get_fn_scope_ret_type(&self) -> Option<TypeId> {
		self.scopes.iter().rev().find_map(|scope| scope.ret_scope())
	}
	// values
	pub fn add_value(&mut self, name: &str, type_id: TypeId, is_mut: bool) -> ValueId {
		let value = Value::new_scoped(self.value_id, type_id, is_mut);
		self.store.add_value(self.store_id, name.to_string(), type_id);
		self.get_scope_mut().add_value(name.to_string(), value);
		self.value_id.next_id()
	}

	pub fn get_value(&self, name: &str) -> Option<&Value> {
		self.scopes.iter().rev().find_map(|scope| scope.get_value(name))
	}

	// borrows
	//
	//
	pub fn add_borrow(&mut self, value_id: ValueId, is_mut: bool) -> Option<BorrowId> {
		if self.borrow_store.conflicts_with_borrow(value_id, is_mut) {
			return None;
		}
		// let scope_id = ScopeId(self.scopes.len() - 1);
		Some(self.borrow_store.add_borrow(value_id, is_mut))
	}

	pub fn release_borrow(&mut self, borrow_id: BorrowId) {
		self.borrow_store.drop_borrows(borrow_id)
	}

	pub fn conflicts_with_borrow(&self, value_id: ValueId, is_mut: bool) -> bool {
		self.borrow_store.conflicts_with_borrow(value_id, is_mut)
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}
