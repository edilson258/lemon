use crate::{
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;
impl Llvm<'_> {
	pub fn llvm_ret(&mut self, ret: &ir::RetInstr) {
		if ret.type_id.is_void() || ret.value.is_none() {
			#[rustfmt::skip]
			self.builder.build_return(None).unwrap_or_else(|err| {
				 throw_llvm_error(format!("return error: {}", err))
			});

			return;
		}

		let ret_value = ret.value.unwrap_or_else(|| throw_llvm_error("return value not found"));
		let value = self.get_value_or_load(ret_value, ret.type_id);
		#[rustfmt::skip]
		self.builder.build_return(Some(&value)).unwrap_or_else(|err| {
			throw_llvm_error(format!("return error: {}", err))
		});
	}
}
