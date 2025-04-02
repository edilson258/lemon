use crate::{error_codegen, ir};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_sub(&mut self, binary: &ir::BinInstr) {
		let left = self.llvm_compile_value(&binary.left);
		let right = self.llvm_compile_value(&binary.right);
		let dest = binary.dest.value.as_str();
		let temp = &self.env.get_temp();
		if left.is_int_value() && right.is_int_value() {
			let left_int = left.into_int_value();
			let right_int = right.into_int_value();
			let value = match self.builder.build_int_sub(left_int, right_int, temp) {
				Ok(result) => result,
				Err(_) => error_codegen!("build int add").report(self.loader),
			};

			let ptr = self.env.get_ptr_value_unwrap(dest);
			return self.store(ptr, value);
		}

		if left.is_float_value() && right.is_float_value() {
			let left_float = left.into_float_value();
			let right_float = right.into_float_value();
			let value = match self.builder.build_float_sub(left_float, right_float, temp) {
				Ok(result) => result,
				Err(_) => error_codegen!("build float add").report(self.loader),
			};

			let ptr = self.env.get_ptr_value_unwrap(dest);
			return self.store(ptr, value);
		}
		let message = error_codegen!("unsupported 'add' {} to {}", left.get_type(), right.get_type());
		message.report(self.loader);
	}
}
