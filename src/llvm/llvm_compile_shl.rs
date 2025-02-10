use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_shl(&mut self, binary: &ir::BinInstr) {
		let left = self.llvm_compile_value(&binary.left);
		let right = self.llvm_compile_value(&binary.right);
		let dest = binary.dest.value.as_str();
		if left.is_int_value() && right.is_int_value() {
			let left_int = left.into_int_value();
			let right_int = right.into_int_value();
			match self.builder.build_left_shift(left_int, right_int, dest) {
				Ok(result) => self.env.set_value(dest, result.into()),
				Err(_) => throw_llvm_error("build int shl"),
			};
			return;
		}
		let error = format!("unsupported 'shl' {} to {}", left.get_type(), right.get_type());
		throw_llvm_error(error);
	}
}
