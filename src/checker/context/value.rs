use crate::checker::{ownership::tracker::PtrId, types::TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value {
	pub type_id: TypeId,
	pub ptr: PtrId,
	pub mutable: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionValue {
	pub type_id: TypeId,
	pub comptime: bool,
}

impl Value {
	pub fn new(type_id: TypeId, ptr: PtrId, mutable: bool) -> Self {
		Self { type_id, ptr, mutable }
	}
	pub fn new_ptr(type_id: TypeId, ptr: PtrId, mutable: bool) -> Self {
		Self::new(type_id, ptr, mutable)
	}
	pub fn new_mutable(type_id: TypeId, ptr: PtrId) -> Self {
		Self::new(type_id, ptr, true)
	}
	pub fn new_immutable(type_id: TypeId, ptr: PtrId) -> Self {
		Self::new(type_id, ptr, false)
	}

	pub fn add_ptr(&mut self, ptr: PtrId) {
		self.ptr = ptr;
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
