use super::types::{Type, TypeId};
use super::{diags::TypeCheckError, Checker, TypeResult};
use crate::{
	ast::{self},
	diag::Diag,
};

impl Checker<'_> {
	pub fn check_number(&mut self, number: &ast::NumberLiteral) -> TypeResult<TypeId> {
		if number.as_dot() {
			return self.check_float_bits(number);
		}
		self.check_int_bits(number)
	}

	pub fn check_int_bits(&mut self, number: &ast::NumberLiteral) -> TypeResult<TypeId> {
		let bit_size = self.parse_radix_to_bit_size(number)?;
		let ty = match bit_size {
			8 => Type::InferInt { bits: 8 },
			16 => Type::InferInt { bits: 16 },
			32 => Type::InferInt { bits: 32 },
			64 => Type::InferInt { bits: 64 },
			// 128 => Type::InferInt { bits: 128 },
			_ => return Err(TypeCheckError::number_too_large(number.get_range())),
		};
		Ok(self.ctx.type_store.add_type(ty))
	}

	pub fn check_float_bits(&mut self, number: &ast::NumberLiteral) -> TypeResult<TypeId> {
		if number.text.parse::<f32>().is_ok() {
			return Ok(TypeId::FLOAT32);
		}

		if number.text.parse::<f64>().is_ok() {
			return Ok(TypeId::FLOAT64);
		}

		Err(TypeCheckError::number_too_large(number.get_range()))
	}

	pub fn parse_radix_to_bit_size(&self, num: &ast::NumberLiteral) -> Result<u8, Diag> {
		if let Ok(value) = u128::from_str_radix(&num.text, num.base as u32) {
			let bit_range = (128 - value.leading_zeros()) as u8;
			return match bit_range {
				0..=8 => Ok(8),
				9..=16 => Ok(16),
				17..=32 => Ok(32),
				33..=64 => Ok(64),
				// 65..=128 => Ok(128),
				_ => Err(TypeCheckError::number_too_large(num.get_range())),
			};
		}
		Err(TypeCheckError::number_too_large(num.get_range()))
	}
}
