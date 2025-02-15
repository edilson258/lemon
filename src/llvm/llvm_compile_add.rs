use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_add(&mut self, binary: &ir::BinInstr) {
		let left = self.llvm_compile_value(&binary.left);
		let right = self.llvm_compile_value(&binary.right);
		let dest = binary.dest.value.as_str();
		let temp = &self.env.get_temp();
		if left.is_int_value() && right.is_int_value() {
			let left_int = left.into_int_value();
			let right_int = right.into_int_value();
			let value = match self.builder.build_int_add(left_int, right_int, temp) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("build int add"),
			};

			let ptr = self.env.get_ptr_value_unwrap(dest);
			return self.store(ptr, value);
		}

		if left.is_float_value() && right.is_float_value() {
			let left_float = left.into_float_value();
			let right_float = right.into_float_value();
			let value = match self.builder.build_float_add(left_float, right_float, temp) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("build float add"),
			};

			let ptr = self.env.get_ptr_value_unwrap(dest);
			return self.store(ptr, value);
		}
		let error = format!("unsupported 'add' {} to {}", left.get_type(), right.get_type());
		throw_llvm_error(error);
	}
}
