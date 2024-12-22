use std::collections::HashMap;

use crate::checker::types::TypeId;

use super::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeType {
	Fn { ret_type: TypeId },
	Loop,
	Block,
}

impl ScopeType {
	pub fn new_fn(ret_type: TypeId) -> Self {
		Self::Fn { ret_type }
	}
	pub fn new_loop() -> Self {
		Self::Loop
	}
	pub fn new_block() -> Self {
		Self::Block
	}

	pub fn ret_scope(&self) -> Option<TypeId> {
		match self {
			Self::Fn { ret_type } => Some(*ret_type),
			_ => None,
		}
	}

	pub fn is_fn(&self) -> bool {
		matches!(self, Self::Fn { .. })
	}

	pub fn is_loop(&self) -> bool {
		matches!(self, Self::Loop)
	}

	pub fn is_block(&self) -> bool {
		matches!(self, Self::Block)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);
impl ScopeId {
	pub fn as_usize(&self) -> usize {
		self.0
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
	pub values: HashMap<String, Value>,
	pub scope_type: ScopeType,
}

impl Scope {
	pub fn new(scope_type: ScopeType) -> Self {
		Self { values: HashMap::new(), scope_type }
	}

	pub fn add_value(&mut self, name: String, value: Value) {
		self.values.insert(name, value);
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
}

impl Default for Scope {
	fn default() -> Self {
		Self::new(ScopeType::Block)
	}
}
