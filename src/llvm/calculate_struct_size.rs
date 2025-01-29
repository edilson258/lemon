use inkwell::{types::StructType, values::IntValue, AddressSpace};

use crate::report::throw_llvm_error;

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn calculate_struct_size(&mut self, struct_type: StructType<'ll>) -> IntValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let i64_type = self.ctx.i64_type();
		let address = self.ctx.ptr_type(AddressSpace::default());
		let null_ptr = address.const_null();

		let size_register = self.stack.temp_register();
		let i32_one = i32_type.const_int(1, false);

		#[rustfmt::skip]
		let size_ptr = unsafe {
			self.builder.build_gep(struct_type, null_ptr, &[i32_one], &size_register)
			.unwrap_or_else(|_| {
					throw_llvm_error("failed to calculate struct size using gep")
				})
		};

		let size_register = self.stack.temp_register();

		#[rustfmt::skip]
		let size = self.builder.build_ptr_to_int(size_ptr, i64_type, &size_register).unwrap_or_else(|_| {
			throw_llvm_error("failed to cast pointer size")
		});

		size
	}
}
