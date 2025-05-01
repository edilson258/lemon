use crate::checker::{typed_value::TypedValue, types::TypeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
	pub mutable: bool,
	pub typed_value: TypedValue,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionValue {
	pub type_id: TypeId,
	pub comptime: bool,
}

impl Value {
	pub fn new(typed_value: TypedValue, mutable: bool) -> Self {
		Self { typed_value, mutable }
	}
}

impl FunctionValue {
	pub fn new(type_id: TypeId, comptime: bool) -> Self {
		Self { type_id, comptime }
	}
	pub fn new_comptime(type_id: TypeId) -> Self {
		Self::new(type_id, true)
	}
	pub fn new_runtime(type_id: TypeId) -> Self {
		Self::new(type_id, false)
	}
}
