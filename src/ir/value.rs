use crate::{checker::types::TypeId, throw_error};
use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub enum BasicValue {
	Int(u64),
	Float(f64),
	String(String),
	Char(char),
	Bool(bool),
	Register(String), // register name
	None,
}

impl BasicValue {
	pub fn is_register(&self) -> bool {
		matches!(self, Self::Register(_))
	}

	pub fn try_get_register(&self) -> Option<&str> {
		if let Self::Register(register) = self {
			Some(register)
		} else {
			None
		}
	}

	pub fn as_str(&self) -> &str {
		match self {
			Self::Register(register) => register,
			_ => throw_error!("expected register"),
		}
	}

	pub fn is_raw_value(&self) -> bool {
		matches!(self, Self::Int(_) | Self::Float(_) | Self::String(_) | Self::Char(_) | Self::Bool(_))
	}
}

#[derive(Debug, Clone)]
pub struct IrBasicValue {
	pub value: BasicValue,
	pub type_id: TypeId,
}

impl IrBasicValue {
	pub fn new(value: BasicValue, type_id: TypeId) -> Self {
		Self { value, type_id }
	}

	pub fn with_new_type(&mut self, type_id: TypeId) -> Self {
		Self { value: mem::take(&mut self.value), type_id }
	}

	pub fn get_type(&self) -> TypeId {
		self.type_id
	}

	pub fn get_value(&self) -> &BasicValue {
		&self.value
	}

	pub fn is_none(&self) -> bool {
		matches!(self.value, BasicValue::None)
	}

	pub fn is_register(&self) -> bool {
		self.value.is_register()
	}

	pub fn is_raw_value(&self) -> bool {
		self.value.is_raw_value()
	}
}

macro_rules! impl_from {
	($type:ty, $variant:ident, $type_id:ident) => {
		impl From<$type> for IrBasicValue {
			fn from(value: $type) -> Self {
				Self::new(BasicValue::$variant(value.into()), TypeId::$type_id)
			}
		}
	};
}

impl From<usize> for IrBasicValue {
	fn from(value: usize) -> Self {
		// todo: review this
		Self::new(BasicValue::Int(value as u64), TypeId::I32)
	}
}

impl From<i64> for IrBasicValue {
	fn from(value: i64) -> Self {
		Self::new(BasicValue::Int(value as u64), TypeId::I64)
	}
}

impl_from!(u64, Int, I64);
impl_from!(f64, Float, F64);
impl_from!(String, String, STR);
impl_from!(char, Char, CHAR);
impl_from!(bool, Bool, BOOL);

impl Default for BasicValue {
	fn default() -> Self {
		Self::None
	}
}

impl Default for IrBasicValue {
	fn default() -> Self {
		Self::new(BasicValue::None, TypeId::UNIT)
	}
}
