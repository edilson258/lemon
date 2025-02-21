use crate::{ir::IrBasicValue, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_drop(&mut self, ptr_value: &IrBasicValue) {
		let free_function = self.get_free_function();

		let ptr = self.env.get_ptr_value(ptr_value.value.as_str()).unwrap_or_else(|| {
			throw_llvm_error(format!("not found pointer value for {}", ptr_value.value.as_str()))
		});

		let temp = self.env.get_temp();
		#[rustfmt::skip]
		self.builder.build_call(free_function, &[ptr.into()], &temp)
			.unwrap_or_else(|err| throw_llvm_error(format!("failed to free pointer: {}", err)));
	}
}
