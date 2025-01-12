mod display_type;
mod store;
mod type_id;
pub use store::*;
pub use type_id::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	Void,
	Bool,
	Str,
	String,
	Char,
	// Number
	NumRange(NumRange),
	Number(Number),
	Borrow(BorrowType),
	Const(ConstType),
	Fn(FnType),

	// internal
	Unit,
}

impl Type {
	pub fn is_numeric(&self) -> bool {
		matches!(self, Type::Number(_) | Type::NumRange(_))
	}

	pub fn is_float(&self) -> bool {
		matches!(self, Type::Number(Number::F32) | Type::Number(Number::F64))
	}
	pub fn is_infer(&self) -> bool {
		matches!(self, Type::NumRange(_))
	}
	pub fn is_borrow(&self) -> bool {
		matches!(self, Type::Borrow(_))
	}

	pub fn is_borrow_mut(&self) -> bool {
		matches!(self, Type::Borrow(BorrowType { mutable: true, .. }))
	}
	pub fn is_local_borrow(&self) -> bool {
		matches!(self, Type::Borrow(BorrowType { external: false, .. }))
	}
	pub fn is_external_borrow(&self) -> bool {
		matches!(self, Type::Borrow(BorrowType { external: true, .. }))
	}
	pub fn is_const(&self) -> bool {
		matches!(self, Type::Const(_))
	}
	// gen type_id
	pub fn get_type_id(&self) -> Option<TypeId> {
		match self {
			Type::Void => Some(TypeId::VOID),
			Type::Bool => Some(TypeId::BOOL),
			Type::Str => Some(TypeId::STR),
			Type::String => Some(TypeId::STRING),
			Type::Char => Some(TypeId::CHAR),
			Type::Number(number) => Some(TypeId::from(number)),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumRange {
	pub bits: u8, // bits of the number
	pub is_float: bool,
}

impl NumRange {
	pub fn new(bits: u8, is_float: bool) -> Self {
		assert!(bits <= 64); // don't support more than 64 bits
		Self { bits, is_float }
	}

	pub fn as_float(&self) -> Number {
		match self.bits {
			32 => Number::F32,
			64 => Number::F64,
			_ => unreachable!(),
		}
	}

	pub fn as_number(&self) -> Number {
		if self.is_float {
			return self.as_float();
		}
		match self.bits {
			0..=32 => Number::I32,
			64 => Number::I64,
			_ => unreachable!(),
		}
	}
	pub fn infer_with_type_id(&self, expected: TypeId) -> Option<TypeId> {
		if self.is_float != expected.is_float() {
			return None;
		};
		let number = match expected {
			TypeId::I8 if self.bits <= 8 => TypeId::I8,
			TypeId::I16 if self.bits <= 16 => TypeId::I16,
			TypeId::I32 if self.bits <= 32 => TypeId::I32,
			TypeId::I64 if self.bits <= 64 => TypeId::I64,
			TypeId::F32 if self.bits == 32 => TypeId::F32,
			TypeId::F64 if self.bits == 64 => TypeId::F64,
			_ => return None,
		};
		Some(number)
	}
	pub fn as_infer_number(&self, expected: &Number) -> Option<Number> {
		if self.is_float != expected.is_float() {
			return None;
		};
		let number = match expected {
			Number::I8 if self.bits <= 8 => Number::I8,
			Number::I16 if self.bits <= 16 => Number::I16,
			Number::I32 if self.bits <= 32 => Number::I32,
			Number::I64 if self.bits <= 64 => Number::I64,
			Number::F32 if self.bits == 32 => Number::F32,
			Number::F64 if self.bits == 64 => Number::F64,
			_ => return None,
		};
		Some(number)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Number {
	// isize
	I8,
	I16,
	I32,
	I64,
	Isize,
	// usize
	Usize,
	U8,
	U16,
	U32,
	U64,
	// float
	F32,
	F64,
}

impl Number {
	pub fn is_isize(&self) -> bool {
		matches!(self, Number::I8 | Number::I16 | Number::I32 | Number::I64)
	}
	pub fn is_usize(&self) -> bool {
		matches!(self, Number::Usize | Number::U8 | Number::U16 | Number::U32 | Number::U64)
	}
	pub fn is_float(&self) -> bool {
		matches!(self, Number::F32 | Number::F64)
	}

	pub fn as_type(&self) -> Type {
		match self {
			Number::I8 => Type::Number(Number::I8),
			Number::I16 => Type::Number(Number::I16),
			Number::I32 => Type::Number(Number::I32),
			Number::I64 => Type::Number(Number::I64),
			Number::Isize => Type::Number(Number::Isize),
			Number::Usize => Type::Number(Number::Usize),
			Number::U8 => Type::Number(Number::U8),
			Number::U16 => Type::Number(Number::U16),
			Number::U32 => Type::Number(Number::U32),
			Number::U64 => Type::Number(Number::U64),
			Number::F32 => Type::Number(Number::F32),
			Number::F64 => Type::Number(Number::F64),
		}
	}
}

#[derive(Debug, Clone, Eq, Hash)]
#[allow(renamed_and_removed_lints)]
#[allow(clippy::derive_hash_xor_eq)]
pub struct BorrowType {
	pub value: TypeId,
	pub mutable: bool,
	pub external: bool,
}
impl PartialEq for BorrowType {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value && self.mutable == other.mutable
	}
}

impl BorrowType {
	pub fn new(value: TypeId, mutable: bool, external: bool) -> Self {
		Self { value, mutable, external }
	}

	pub fn change_value(&mut self, value: TypeId) {
		self.value = value;
	}

	pub fn new_external(value: TypeId, mutable: bool) -> Self {
		Self::new(value, mutable, true)
	}

	pub fn new_internal(value: TypeId, mutable: bool) -> Self {
		Self::new(value, mutable, false)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstType {
	pub value: TypeId,
	pub kind: ConstKind,
}

impl ConstType {
	pub fn new(value: TypeId, kind: ConstKind) -> Self {
		Self { value, kind }
	}

	pub fn new_fn(value: TypeId) -> Self {
		Self::new(value, ConstKind::Fn)
	}

	pub fn new_del(value: TypeId) -> Self {
		Self::new(value, ConstKind::Del)
	}

	pub fn is_fn(&self) -> bool {
		matches!(self.kind, ConstKind::Fn)
	}
	pub fn is_del(&self) -> bool {
		matches!(self.kind, ConstKind::Del)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstKind {
	Fn,
	Del,
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

impl From<FnType> for Type {
	fn from(value: FnType) -> Self {
		Type::Fn(value)
	}
}

impl From<NumRange> for Type {
	fn from(value: NumRange) -> Self {
		Type::NumRange(value)
	}
}
impl From<Number> for Type {
	fn from(value: Number) -> Self {
		Type::Number(value)
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

impl From<BorrowType> for Type {
	fn from(value: BorrowType) -> Self {
		Type::Borrow(value)
	}
}
