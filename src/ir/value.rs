use std::mem;

use crate::{checker::types::TypeId, report::throw_ir_build_error};

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
		matches!(self, BasicValue::Register(_))
	}

	pub fn try_get_register(&self) -> Option<&str> {
		match self {
			BasicValue::Register(register) => Some(register),
			_ => None,
		}
	}
	pub fn as_str(&self) -> &str {
		match self {
			BasicValue::Register(register) => register,
			_ => throw_ir_build_error("try get register from non register value"),
		}
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

	pub fn is_int(&self) -> bool {
		matches!(self.value, BasicValue::Int(_))
	}

	pub fn is_float(&self) -> bool {
		matches!(self.value, BasicValue::Float(_))
	}

	pub fn is_string(&self) -> bool {
		matches!(self.value, BasicValue::String(_))
	}

	pub fn is_bool(&self) -> bool {
		matches!(self.value, BasicValue::Bool(_))
	}

	pub fn is_char(&self) -> bool {
		matches!(self.value, BasicValue::Char(_))
	}
	pub fn is_register(&self) -> bool {
		matches!(self.value, BasicValue::Register(_))
	}
}

impl From<i64> for IrBasicValue {
	fn from(value: i64) -> Self {
		Self::new(BasicValue::Int(value as u64), TypeId::I32)
	}
}
impl From<f64> for IrBasicValue {
	fn from(value: f64) -> Self {
		Self::new(BasicValue::Float(value), TypeId::F64)
	}
}
impl From<String> for IrBasicValue {
	fn from(value: String) -> Self {
		Self::new(BasicValue::String(value), TypeId::STR)
	}
}
impl From<char> for IrBasicValue {
	fn from(value: char) -> Self {
		Self::new(BasicValue::Char(value), TypeId::CHAR)
	}
}

impl From<bool> for IrBasicValue {
	fn from(value: bool) -> Self {
		Self::new(BasicValue::Bool(value), TypeId::BOOL)
	}
}

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
