use crate::{error_codegen, ir};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_shl(&mut self, binary: &ir::BinInstr) {
		let left = self.llvm_compile_value(&binary.left);
		let right = self.llvm_compile_value(&binary.right);
		let dest = binary.dest.value.as_str();
		let temp = &self.env.get_temp();
		if left.is_int_value() && right.is_int_value() {
			let left_int = left.into_int_value();
			let right_int = right.into_int_value();
			let value = match self.builder.build_left_shift(left_int, right_int, temp) {
				Ok(result) => result,
				Err(_) => error_codegen!("build int shl").report(self.loader),
			};

			let ptr = self.env.get_ptr_value_unwrap(dest);
			return self.store(ptr, value);
		}
		let message = error_codegen!("unsupported 'shl' {} to {}", left.get_type(), right.get_type());
		message.report(self.loader);
	}
}
