use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeId(pub(crate) usize);

impl TypeId {
	pub const NOTHING: TypeId = TypeId(0);
	pub const BOOL: TypeId = TypeId(1);
	pub const STR: TypeId = TypeId(2);
	pub const STRING: TypeId = TypeId(3);
	pub const CHAR: TypeId = TypeId(4);
	// int
	pub const I8: TypeId = TypeId(5);
	pub const I16: TypeId = TypeId(6);
	pub const I32: TypeId = TypeId(7);
	pub const I64: TypeId = TypeId(8);
	pub const INT: TypeId = TypeId(9);
	// usize
	pub const U8: TypeId = TypeId(10);
	pub const U16: TypeId = TypeId(11);
	pub const U32: TypeId = TypeId(12);
	pub const U64: TypeId = TypeId(13);
	pub const USIZE: TypeId = TypeId(14);
	// float
	pub const FLOAT32: TypeId = TypeId(15);
	pub const FLOAT64: TypeId = TypeId(16);

	// internal
	// pub const INFERI32: TypeId = TypeId(19);
	// pub const INFERF32: TypeId = TypeId(20);

	pub const LENGTH: usize = 17; // internal

	// methods
	pub fn as_usize(&self) -> usize {
		self.0
	}
}
