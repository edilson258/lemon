use crate::ast;

use super::{diags::TypeCheckError, Checker, TypeResult};

use super::types::TypeId;
impl Checker<'_> {
	pub fn check_primitive_type(&mut self, ast_type: &ast::AstType) -> TypeResult<TypeId> {
		match ast_type {
			ast::AstType::Bool(bool) => Ok(TypeId::BOOL),
			ast::AstType::Char(char) => Ok(TypeId::CHAR),
			ast::AstType::String(string) => Ok(TypeId::STRING),
			ast::AstType::Float(float) => self.check_float_type(float),
			ast::AstType::Number(number) => self.check_number_type(number),
			_ => todo!(),
		}
	}

	pub fn check_float_type(&mut self, ast_type: &ast::FloatType) -> TypeResult<TypeId> {
		let bits = ast_type.bits;
		match bits {
			32 => Ok(TypeId::FLOAT32),
			64 => Ok(TypeId::FLOAT64),
			_ => Err(TypeCheckError::number_too_large(ast_type.get_range())),
		}
	}
	pub fn check_number_type(&mut self, ast_type: &ast::NumberType) -> TypeResult<TypeId> {
		let signed = ast_type.signed;
		let bits = ast_type.bits;
		match (signed, bits) {
			(true, 8) => Ok(TypeId::I8),
			(true, 16) => Ok(TypeId::I16),
			(true, 32) => Ok(TypeId::I32),
			(true, 64) => Ok(TypeId::I64),
			// (true, 128) => Ok(TypeId::I128),
			(false, 8) => Ok(TypeId::U8),
			(false, 16) => Ok(TypeId::U16),
			(false, 32) => Ok(TypeId::U32),
			(false, 64) => Ok(TypeId::U64),
			// (false, 128) => Ok(TypeId::UINT128),
			_ => Err(TypeCheckError::number_too_large(ast_type.get_range())),
		}
	}
}
