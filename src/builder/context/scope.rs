use crate::{checker::types::TypeId, ir::IrBasicValue};
use rustc_hash::FxHashMap;

pub enum ScopeKind {
	Function { return_type: TypeId },
	Implementation { receiver_name: String, receiver_type: TypeId },
	StructMember,
	CodeBlock,
}

pub struct Scope {
	kind: ScopeKind,
	local_variables: FxHashMap<String, IrBasicValue>,
	unbound_values: FxHashMap<String, IrBasicValue>,
}

impl Default for Scope {
	fn default() -> Self {
		Self::new()
	}
}

impl Scope {
	pub fn new() -> Self {
		Self::new_with_kind(ScopeKind::CodeBlock)
	}

	pub fn new_function_scope(return_type: TypeId) -> Self {
		Self::new_with_kind(ScopeKind::Function { return_type })
	}

	pub fn new_implementation_scope(receiver_name: String, receiver_type: TypeId) -> Self {
		Self::new_with_kind(ScopeKind::Implementation { receiver_name, receiver_type })
	}

	pub fn new_struct_member_scope() -> Self {
		Self::new_with_kind(ScopeKind::StructMember)
	}

	fn new_with_kind(kind: ScopeKind) -> Self {
		let local_variables = FxHashMap::default();
		let unbound_values = FxHashMap::default();
		Self { kind, local_variables, unbound_values }
	}

	pub fn return_type(&self) -> Option<TypeId> {
		if let ScopeKind::Function { return_type } = self.kind {
			Some(return_type)
		} else {
			None
		}
	}

	pub fn register_unbound_value(&mut self, value: IrBasicValue) {
		if value.is_register() {
			self.unbound_values.insert(value.value.as_str().into(), value);
		}
	}

	pub fn collect_unbound_values(&self) -> Vec<IrBasicValue> {
		self.unbound_values.values().cloned().collect()
	}

	pub fn is_implementation(&self) -> bool {
		matches!(self.kind, ScopeKind::Implementation { .. })
	}

	#[allow(dead_code)]
	pub fn is_struct_member(&self) -> bool {
		matches!(self.kind, ScopeKind::StructMember)
	}
	#[allow(dead_code)]
	pub fn is_function(&self) -> bool {
		matches!(self.kind, ScopeKind::Function { .. })
	}

	pub fn receiver_info(&self) -> Option<(String, TypeId)> {
		match &self.kind {
			ScopeKind::Implementation { receiver_name, receiver_type } => {
				Some((receiver_name.clone(), *receiver_type))
			}
			_ => None,
		}
	}

	pub fn define_local_variable(&mut self, key: String, basic_value: IrBasicValue) {
		self.local_variables.insert(key, basic_value);
	}

	pub fn lookup_local_variable(&self, name: &str) -> Option<&IrBasicValue> {
		self.local_variables.get(name)
	}
}
