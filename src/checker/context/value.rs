use crate::checker::{
	ownership::{pointer::Ptr, tracker::PtrId},
	types::TypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value {
	pub type_id: TypeId,
	pub ptr: Option<Ptr>,
	pub function: bool,
	pub mutable: bool,
}

impl Value {
	pub fn new(type_id: TypeId, ptr: Option<Ptr>, mutable: bool) -> Self {
		Self { type_id, ptr, mutable, function: false }
	}
	pub fn new_ptr(type_id: TypeId, ptr: Ptr, mutable: bool) -> Self {
		Self::new(type_id, Some(ptr), mutable)
	}

	pub fn new_fn(type_id: TypeId) -> Self {
		Self { type_id, ptr: None, mutable: false, function: true }
	}

	pub fn new_mutable(type_id: TypeId, ptr: Ptr) -> Self {
		Self::new(type_id, Some(ptr), true)
	}

	pub fn new_immutable(type_id: TypeId, ptr: Ptr) -> Self {
		Self::new(type_id, Some(ptr), false)
	}

	pub fn add_ptr(&mut self, ptr: Ptr) {
		self.ptr = Some(ptr);
	}

	pub fn lookup_ptr_id(&self) -> Option<PtrId> {
		self.ptr.map(|ptr| ptr.id)
	}
	pub fn lookup_ptr_id_unchecked(&self) -> PtrId {
		self.lookup_ptr_id().unwrap()
	}
}
