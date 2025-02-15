use inkwell::{
	values::{FloatValue, IntValue},
	FloatPredicate, IntPredicate,
};

use crate::report::throw_llvm_error;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn cmp_int_values(
		&mut self,
		lhs: IntValue<'ll>,
		rhs: IntValue<'ll>,
		operator: IntPredicate,
	) -> IntValue<'ll> {
		let dest = &self.env.get_temp();
		match self.builder.build_int_compare(operator, lhs, rhs, dest) {
			Ok(value) => value,
			Err(err) => throw_llvm_error(format!("compare values, reason: {}", err)),
		}
	}

	pub fn cmp_float_values(
		&mut self,
		lhs: FloatValue<'ll>,
		rhs: FloatValue<'ll>,
		operator: FloatPredicate,
	) -> IntValue<'ll> {
		let dest = &self.env.get_temp();
		match self.builder.build_float_compare(operator, lhs, rhs, dest) {
			Ok(value) => value,
			Err(err) => throw_llvm_error(format!("compare values, reason: {}", err)),
		}
	}
}
