use crate::{checker::types::TypeId, throw_error};
use std::{fmt::Display, mem};

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
	pub base_type: Option<TypeId>,
	pub param: bool,
}

impl IrBasicValue {
	pub fn new(value: BasicValue, type_id: TypeId) -> Self {
		if value.is_register() {
			return Self { value, type_id, base_type: Some(type_id), param: false };
		}
		Self { value, type_id, base_type: None, param: false }
	}

	pub fn new_register(value: BasicValue, type_id: TypeId, base_type: Option<TypeId>) -> Self {
		let base_type = Some(base_type.unwrap_or(type_id));
		Self { value, type_id, base_type, param: false }
	}

	pub fn with_new_type(&mut self, type_id: TypeId) -> Self {
		Self { value: mem::take(&mut self.value), type_id, ..self.clone() }
	}

	pub fn get_type(&self) -> TypeId {
		self.type_id
	}

	pub fn as_param(self) -> Self {
		Self { value: self.value, type_id: self.type_id, base_type: self.base_type, param: true }
	}
	pub fn needs_load(&self) -> bool {
		if self.param {
			return false;
		}
		self.value.is_register() && self.base_type.is_some()
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

	pub fn as_string(&self) -> &str {
		self.value.as_str()
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
impl_from!(f32, Float, F32);
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

impl Display for BasicValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Int(value) => write!(f, "{}", value),
			Self::Float(value) => write!(f, "{}", value),
			Self::String(value) => write!(f, "\"{}\"", value),
			Self::Char(value) => write!(f, "{}", value),
			Self::Bool(value) => write!(f, "{}", value),
			Self::Register(value) => write!(f, "{}", value),
			Self::None => write!(f, "None"),
		}
	}
}

impl Display for IrBasicValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.value {
			BasicValue::Int(value) => write!(f, "{}", value),
			BasicValue::Float(value) => write!(f, "{}", value),
			BasicValue::String(value) => write!(f, "\"{}\"", value),
			BasicValue::Char(value) => write!(f, "{}", value),
			BasicValue::Bool(value) => write!(f, "{}", value),
			BasicValue::Register(value) => {
				if self.base_type.is_some() {
					let base = self.base_type.unwrap();
					if base == self.type_id {
						write!(f, "{}, t_id: {}", value, self.type_id.as_usize())
					} else {
						write!(f, "{}, t_id: {}, b_id: {}", value, self.type_id.as_usize(), base.as_usize())
					}
				} else {
					write!(f, "{}, t_id: {}", value, self.type_id.as_usize())
				}
			}
			BasicValue::None => write!(f, "None"),
		}
	}
}
