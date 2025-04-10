use crate::{error_codegen, ir};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_ret(&mut self, ret_value: &Option<ir::IrBasicValue>) {
		if ret_value.is_none() {
			match self.builder.build_return(None) {
				Ok(_) => {}
				Err(err) => error_codegen!("while return void, resson '{}'", err).report(self.loader),
			}
			return;
		}
		let ret_value = ret_value.as_ref().unwrap();
		if ret_value.type_id.is_empty_type() {
			self.builder.build_return(None).unwrap_or_else(|err| {
				error_codegen!("while return void, resson '{}'", err).report(self.loader)
			});
			return;
		}

		let value = self.llvm_compile_value(ret_value);
		self.builder.build_return(Some(&value)).unwrap_or_else(|err| {
			let message = error_codegen!("while return, resson '{}'", err);
			message.report(self.loader);
		});
	}
}
