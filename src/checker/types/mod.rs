mod formatter;
mod store;
mod type_id;
pub use formatter::*;
pub use store::*;
pub use type_id::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	None,
	Bool,
	Str,
	String,
	Char,
	Par { target: TypeId },
	InferInt { bits: u8 },
	Fn(FnType),
	Int(IntType),
	Usize(UsizeType),
	Float(FloatType),
	Ref(RefType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnType {
	pub args: Vec<TypeId>,
	pub ret: TypeId,
}

impl FnType {
	pub fn new(args: Vec<TypeId>, ret: TypeId) -> Self {
		Self { args, ret }
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefType {
	pub mutable: bool,
	pub value: TypeId,
}

impl RefType {
	pub fn new(mutable: bool, value: TypeId) -> Self {
		Self { mutable, value }
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntType {
	I8,
	I16,
	I32,
	I64,
	Int,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsizeType {
	U8,
	U16,
	U32,
	U64,
	Usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatType {
	F32,
	F64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InferType {
	pub bits: usize,
}

impl InferType {
	pub fn new(bits: usize) -> Self {
		Self { bits }
	}
}

impl Default for InferType {
	fn default() -> Self {
		Self { bits: 64 }
	}
}
