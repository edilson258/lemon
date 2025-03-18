use crate::checker::types::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(usize);

impl ValueId {
	pub fn as_usize(self) -> usize {
		self.0
	}

	pub fn init() -> Self {
		Self(0)
	}

	pub fn next_id(&mut self) -> Self {
		let id = self.0;
		self.0 += 1;
		Self(id)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value {
	pub id: ValueId,
	pub type_id: TypeId,
	pub is_scoped: bool,
	pub is_mut: bool,
}

impl Value {
	pub fn new(id: ValueId, type_id: TypeId, is_mut: bool, is_scoped: bool) -> Self {
		Self { id, type_id, is_mut, is_scoped }
	}

	pub fn new_mutable(id: ValueId, type_id: TypeId, is_scoped: bool) -> Self {
		Self::new(id, type_id, true, is_scoped)
	}

	pub fn new_immutable(id: ValueId, type_id: TypeId, is_scoped: bool) -> Self {
		Self::new(id, type_id, false, is_scoped)
	}

	pub fn new_scoped(id: ValueId, type_id: TypeId, is_mut: bool) -> Self {
		Self::new(id, type_id, is_mut, true)
	}

	pub fn new_external(id: ValueId, type_id: TypeId, is_mut: bool) -> Self {
		Self::new(id, type_id, is_mut, false)
	}

	pub fn get_type_id(self) -> TypeId {
		self.type_id
	}
}
