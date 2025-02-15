use inkwell::{FloatPredicate, IntPredicate};

use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_cmp_eq(&mut self, binary: &ir::BinInstr) {
		let left = self.llvm_compile_value_and_load(&binary.left);
		let right = self.llvm_compile_value_and_load(&binary.right);
		let dest = binary.dest.value.as_str();
		if left.is_int_value() && right.is_int_value() {
			let left_int = left.into_int_value();
			let right_int = right.into_int_value();
			let value = self.cmp_int_values(left_int, right_int, IntPredicate::EQ);
			let ptr = self.env.get_ptr_value_unwrap(dest);
			return self.store(ptr, value);
		}

		if left.is_float_value() && right.is_float_value() {
			let left_float = left.into_float_value();
			let right_float = right.into_float_value();
			let value = self.cmp_float_values(left_float, right_float, FloatPredicate::OEQ);
			let ptr = self.env.get_ptr_value_unwrap(dest);
			return self.store(ptr, value);
		}

		throw_llvm_error(format!("unsupported types for comparison: {}", left.get_type()));
	}
}
