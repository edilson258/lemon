use crate::{error_codegen, ir::IrBasicValue};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_drop(&mut self, ptr_value: &IrBasicValue) {
		let free_function = self.get_free_function();

		let ptr = self.env.get_ptr_value(ptr_value.value.as_str()).unwrap_or_else(|| {
			let message = error_codegen!("not found pointer value for {}", ptr_value.value.as_str());
			message.report(self.loader);
		});

		let temp = self.env.get_temp();
		#[rustfmt::skip]
		self.builder.build_call(free_function, &[ptr.into()], &temp)
			.unwrap_or_else(|err| {
			  let message = error_codegen!("failed to free pointer: {}", err);
				message.report(self.loader);
			});
	}
}
