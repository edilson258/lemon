use crate::error_codegen;

use super::Llvm;

impl Llvm<'_> {
	pub fn free_end_of_scope(&mut self) {
		let pointers = self.stack.take_frees();
		let free_fun = self.get_free_fun();

		for (_, pointer) in pointers {
			let temp = self.stack.temp_register();
			#[rustfmt::skip]
			self.builder.build_call(free_fun, &[pointer.into()], &temp)
				.unwrap_or_else(|err| throw_llvm_error(format!("failed to free pointer: {}", err)));
		}
	}
}
