use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_ret(&mut self, ret_value: &ir::IrBasicValue) {
		if ret_value.type_id.is_void() || ret_value.type_id.is_unit() {
			#[rustfmt::skip]
  		self.builder.build_return(None).unwrap_or_else(|err| {
  			 throw_llvm_error(format!("while return void, resson '{}'", err))
  		});
			return;
		}

		let value = self.llvm_compile_value(ret_value);
		#[rustfmt::skip]
		self.builder.build_return(Some(&value))
			.unwrap_or_else(|err| throw_llvm_error(format!("while return '{}', resson '{}'", value.get_type(), err)));
	}
}
