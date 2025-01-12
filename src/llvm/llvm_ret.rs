use inkwell::values::BasicValue;

use crate::{
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;
impl Llvm<'_> {
	pub fn llvm_ret(&mut self, ret: &ir::RetInstr) {
		if ret.type_id.is_void() || ret.value.is_none() {
			match self.builder.build_return(None) {
				Ok(_) => {}
				Err(err) => throw_llvm_error(format!("return error: {}", err)),
			}
			return;
		}
		match ret.value {
			Some(value) => {
				let value = self.stack.get_value(value);
				let basic_value = value.as_basic_value_enum();
				match self.builder.build_return(Some(&basic_value)) {
					Ok(_) => {}
					Err(err) => throw_llvm_error(format!("return error: {}", err)),
				}
			}
			None => throw_llvm_error("return value not found"),
		}
	}
}
