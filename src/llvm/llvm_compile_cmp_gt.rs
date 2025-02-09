use inkwell::{FloatPredicate, IntPredicate};

use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_cmp_gt(&mut self, binary: &ir::BinInstr) {
		let left = self.llvm_compile_value_and_load(&binary.left);
		let right = self.llvm_compile_value_and_load(&binary.right);
		let dest = binary.dest.value.as_str();
		if left.is_int_value() && right.is_int_value() {
			let left_int = left.into_int_value();
			let right_int = right.into_int_value();
			let operator = IntPredicate::SGT;
			match self.builder.build_int_compare(operator, left_int, right_int, dest) {
				Ok(value) => self.env.set_value(dest, value.into()),
				Err(err) => throw_llvm_error(format!("cmp_gt register: {}", err)),
			}
			return;
		}

		if left.is_float_value() && right.is_float_value() {
			let left_float = left.into_float_value();
			let right_float = right.into_float_value();
			let operator = FloatPredicate::OGT;
			match self.builder.build_float_compare(operator, left_float, right_float, dest) {
				Ok(value) => self.env.set_value(dest, value.into()),
				Err(err) => throw_llvm_error(format!("cmp_gt register: {}", err)),
			}
			return;
		}

		throw_llvm_error(format!("cannot cmp_gt '{}' to '{}'", left.get_type(), right.get_type()));
	}
}
