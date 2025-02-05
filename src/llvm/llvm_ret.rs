use crate::{
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;
impl Llvm<'_> {
	pub fn llvm_ret(&mut self, ret: &ir::RetInstr) {
		if ret.type_id.is_void() || ret.value.is_none() {
			self.free_end_of_scope();
			#[rustfmt::skip]
			self.builder.build_return(None).unwrap_or_else(|err| {
				 throw_llvm_error(format!("return error: {}", err))
			});
			return;
		}

		let ret_value = ret.value.unwrap_or_else(|| throw_llvm_error("return value not found"));
		if let Some(struct_type) = self.stack.get_struct_type(ret.type_id) {
			let ptr = self.stack.get_ptr_value(ret_value);
			let value = match self.builder.build_load(*struct_type, ptr, &ret_value.as_string()) {
				Ok(sucess) => sucess,
				Err(err) => throw_llvm_error(format!("return error: {}", err)),
			};
			self.free_end_of_scope();
			#[rustfmt::skip]
			self.builder.build_return(Some(&value)).unwrap_or_else(|err| {
				throw_llvm_error(format!("return error: {}", err))
			});
			return;
		}

		let value = self.get_value_or_load(ret_value, ret.type_id);

		self.free_end_of_scope();
		#[rustfmt::skip]
		self.builder.build_return(Some(&value)).unwrap_or_else(|err| {
			throw_llvm_error(format!("return error: {}", err))
		});
	}
}
