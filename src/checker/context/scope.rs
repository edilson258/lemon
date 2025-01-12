use std::collections::HashMap;

use crate::checker::types::TypeId;

use super::{
	borrow::{Borrow, BorrowId, BorrowStore},
	value::{Value, ValueId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeType {
	Fn { ret_type: TypeId },
	ConstFn { ret_type: TypeId },
	Loop,
	Block,
	Global,
}

impl ScopeType {
	pub fn new_fn(ret_type: TypeId) -> Self {
		Self::Fn { ret_type }
	}
	pub fn new_const_fn(ret_type: TypeId) -> Self {
		Self::ConstFn { ret_type }
	}
	pub fn new_loop() -> Self {
		Self::Loop
	}
	pub fn new_block() -> Self {
		Self::Block
	}

	pub fn new_global() -> Self {
		Self::Global
	}

	pub fn ret_scope(&self) -> Option<TypeId> {
		match self {
			Self::Fn { ret_type } => Some(*ret_type),
			Self::ConstFn { ret_type } => Some(*ret_type),
			_ => None,
		}
	}

	pub fn is_fn(&self) -> bool {
		matches!(self, Self::Fn { .. } | Self::ConstFn { .. })
	}

	pub fn is_const_fn(&self) -> bool {
		matches!(self, Self::ConstFn { .. })
	}

	pub fn is_loop(&self) -> bool {
		matches!(self, Self::Loop)
	}

	pub fn is_block(&self) -> bool {
		matches!(self, Self::Block)
	}
	pub fn is_global(&self) -> bool {
		matches!(self, Self::Global)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);
impl ScopeId {
	pub fn as_usize(&self) -> usize {
		self.0
	}
}

#[derive(Debug, Clone)]
pub struct Scope {
	pub values: HashMap<String, Value>,
	pub fn_values: HashMap<String, Value>,
	pub borrow_store: BorrowStore,
	pub scope_type: ScopeType,
}

impl Scope {
	pub fn new(scope_type: ScopeType) -> Self {
		let values = HashMap::new();
		let fn_values = HashMap::new();
		let borrow_store = BorrowStore::default();
		Self { values, fn_values, scope_type, borrow_store }
	}

	pub fn add_borrow_value(&mut self, vaiue_id: ValueId, is_mut: bool) -> BorrowId {
		self.borrow_store.add_borrow(vaiue_id, is_mut)
	}

	pub fn get_borrow_value(&self, borrow_id: BorrowId) -> Option<&Borrow> {
		self.borrow_store.get_borrow(borrow_id)
	}

	pub fn drop_borrows(&mut self, borrow_id: BorrowId) {
		self.borrow_store.drop_borrows(borrow_id)
	}

	pub fn can_borrow_as(&self, value_id: ValueId, is_mut: bool) -> bool {
		self.borrow_store.can_borrow_as(value_id, is_mut)
	}

	pub fn has_value(&self, name: &str) -> bool {
		self.values.contains_key(name)
	}

	pub fn has_fn_value(&self, name: &str) -> bool {
		self.fn_values.contains_key(name)
	}

	pub fn add_value(&mut self, name: String, value: Value) {
		self.values.insert(name, value);
	}

	pub fn add_fn_value(&mut self, name: String, value: Value) {
		self.fn_values.insert(name, value);
	}

	pub fn get_fn_value(&self, name: &str) -> Option<&Value> {
		self.fn_values.get(name)
	}

	pub fn get_value(&self, name: &str) -> Option<&Value> {
		self.values.get(name)
	}

	pub fn ret_scope(&self) -> Option<TypeId> {
		self.scope_type.ret_scope()
	}

	pub fn is_fn_scope(&self) -> bool {
		self.scope_type.is_fn()
	}

	pub fn is_loop_scope(&self) -> bool {
		self.scope_type.is_loop()
	}

	pub fn is_block_scope(&self) -> bool {
		self.scope_type.is_block()
	}

	pub fn is_global_scope(&self) -> bool {
		self.scope_type.is_global()
	}
}

impl Default for Scope {
	fn default() -> Self {
		Self::new(ScopeType::new_global())
	}
}
