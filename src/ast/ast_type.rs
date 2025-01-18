use crate::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstType {
	Number(NumberType),
	Float(FloatType),
	Bool(BaseType),
	String(BaseType),
	Str(BaseType),
	Char(BaseType),
	Void(BaseType),
	Ident(IdentType),
	Fn(FnType),
	Borrow(BorrowType),
}
impl AstType {
	pub fn get_range(&self) -> Range {
		match self {
			AstType::Number(number) => number.get_range(),
			AstType::Float(float) => float.get_range(),
			AstType::Bool(bool) => bool.get_range(),
			AstType::String(string) => string.get_range(),
			AstType::Char(char) => char.get_range(),
			AstType::Ident(ident) => ident.get_range(),
			AstType::Fn(fn_type) => fn_type.get_range(),
			AstType::Borrow(borrow) => borrow.get_range(),
			AstType::Str(str) => str.get_range(),
			AstType::Void(void) => void.get_range(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseType {
	pub range: Range,
}

impl BaseType {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberType {
	pub range: Range,
	pub bits: u8, // 0 = depends on arch
	pub signed: bool,
}

impl NumberType {
	pub fn display(&self) -> String {
		match (self.bits, self.signed) {
			(8, true) => "i8".to_owned(),
			(8, false) => "u8".to_owned(),
			(16, true) => "i16".to_owned(),
			(16, false) => "u16".to_owned(),
			(32, true) => "i32".to_owned(),
			(32, false) => "u32".to_owned(),
			(64, true) => "i64".to_owned(),
			(64, false) => "u64".to_owned(),
			(_, true) if self.bits < 8 => "isize".to_owned(),
			(_, false) if self.bits < 8 => "usize".to_owned(),
			_ => panic!("error: unsupported number type"),
		}
	}
}

impl NumberType {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowType {
	pub range: Range,
	pub mutable: bool,
	pub value: Box<AstType>,
}

impl BorrowType {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.value.get_range())
	}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerefType {
	pub range: Range,
	pub value: Box<AstType>,
}

impl DerefType {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.value.get_range())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatType {
	pub range: Range,
	pub bits: u8, // only 32 and 64 are supported
}

impl FloatType {
	pub fn display(&self) -> String {
		match self.bits {
			32 => "f32".to_owned(),
			64 => "f64".to_owned(),
			_ => panic!("error: unsupported float type"),
		}
	}

	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentType {
	pub range: Range,
	pub text: String,
}

impl IdentType {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn lexeme(&self) -> &str {
		&self.text
	}
}

// fn(params_types...): ret_type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnType {
	pub params: Vec<AstType>,
	pub ret_type: Option<Box<AstType>>,
	pub range: Range, // fn range
}

impl FnType {
	pub fn get_range(&self) -> Range {
		if let Some(ret_type) = &self.ret_type {
			return self.range.merged_with(&ret_type.get_range());
		}
		self.range.clone()
	}
}
