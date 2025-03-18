use crate::checker::types::TypeId;
use rustc_hash::FxHashMap;

use super::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeKind {
	Function { ret_type: TypeId },
	ConstantFunction { ret_type: TypeId },
	Loop,
	Block,
	Global,
	Implementation { self_type: TypeId },
	Accessor { self_type: TypeId, is_associated: bool },
}

impl ScopeKind {
	pub fn function(ret_type: TypeId) -> Self {
		Self::Function { ret_type }
	}

	pub fn constant_function(ret_type: TypeId) -> Self {
		Self::ConstantFunction { ret_type }
	}

	pub fn loop_scope() -> Self {
		Self::Loop
	}

	pub fn block_scope() -> Self {
		Self::Block
	}

	pub fn global_scope() -> Self {
		Self::Global
	}

	pub fn implementation(self_type: TypeId) -> Self {
		Self::Implementation { self_type }
	}

	pub fn accessor(self_type: TypeId, is_associated: bool) -> Self {
		Self::Accessor { self_type, is_associated }
	}

	pub fn get_return_type(&self) -> Option<TypeId> {
		match self {
			Self::Function { ret_type } | Self::ConstantFunction { ret_type } => Some(*ret_type),
			_ => None,
		}
	}

	pub fn get_self_type(&self) -> Option<TypeId> {
		match self {
			Self::Implementation { self_type } | Self::Accessor { self_type, .. } => Some(*self_type),
			_ => None,
		}
	}

	pub fn get_accessor_type(&self) -> Option<TypeId> {
		match self {
			Self::Accessor { self_type, .. } => Some(*self_type),
			_ => None,
		}
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
	pub variables: FxHashMap<String, Value>,
	pub functions: FxHashMap<String, Value>,
	// pub borrow_tracker: BorrowTracker,
	pub kind: ScopeKind,
}

impl Scope {
	pub fn new(kind: ScopeKind) -> Self {
		Self {
			variables: FxHashMap::default(),
			functions: FxHashMap::default(),
			// borrow_tracker: BorrowTracker::default(),
			kind,
		}
	}

	// pub fn request_borrow(&mut self, value_id: ValueId, mutable: bool) -> Option<BorrowId> {
	// 	self.borrow_tracker.request_borrow(value_id, mutable)
	// }

	// pub fn get_borrow(&self, borrow_id: BorrowId) -> Option<&BorrowRecord> {
	// 	self.borrow_tracker.get_borrow(borrow_id)
	// }

	// pub fn return_borrow(&mut self, borrow_id: BorrowId) -> bool {
	// 	self.borrow_tracker.return_borrow(borrow_id)
	// }

	// pub fn can_borrow(&self, value_id: ValueId, mutable: bool) -> bool {
	// 	self.borrow_tracker.can_borrow(value_id, mutable)
	// }

	pub fn add_variable(&mut self, name: String, value: Value) {
		self.variables.insert(name, value);
	}

	pub fn add_function(&mut self, name: String, value: Value) {
		self.functions.insert(name, value);
	}

	pub fn get_variable(&self, name: &str) -> Option<&Value> {
		self.variables.get(name)
	}

	pub fn get_function(&self, name: &str) -> Option<&Value> {
		self.functions.get(name)
	}

	pub fn has_variable(&self, name: &str) -> bool {
		self.variables.contains_key(name)
	}

	pub fn has_function(&self, name: &str) -> bool {
		self.functions.contains_key(name)
	}

	pub fn is_implementation_scope(&self) -> bool {
		matches!(self.kind, ScopeKind::Implementation { .. })
	}

	pub fn is_function_scope(&self) -> bool {
		matches!(self.kind, ScopeKind::Function { .. } | ScopeKind::ConstantFunction { .. })
	}

	pub fn is_loop_scope(&self) -> bool {
		matches!(self.kind, ScopeKind::Loop)
	}

	pub fn is_block_scope(&self) -> bool {
		matches!(self.kind, ScopeKind::Block)
	}

	pub fn is_global_scope(&self) -> bool {
		matches!(self.kind, ScopeKind::Global)
	}

	pub fn is_accessor_scope(&self) -> bool {
		matches!(self.kind, ScopeKind::Accessor { .. })
	}

	// getters

	pub fn get_accessor_type(&self) -> Option<TypeId> {
		self.kind.get_accessor_type()
	}

	pub fn get_return_type(&self) -> Option<TypeId> {
		self.kind.get_return_type()
	}

	pub fn get_self_scope_type(&self) -> Option<TypeId> {
		self.kind.get_self_type()
	}
}

impl Default for Scope {
	fn default() -> Self {
		Self::new(ScopeKind::global_scope())
	}
}
