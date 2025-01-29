use inkwell::{
	types::StructType,
	values::{BasicValueEnum, PointerValue},
};

use crate::report::throw_llvm_error;

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn store_struct_fields(
		&mut self,
		values: &[BasicValueEnum<'ll>],
		ptr: PointerValue<'ll>,
		struct_type: StructType<'ll>,
	) {
		let i32_type = self.ctx.i32_type();
		let zero = i32_type.const_zero();
		for (idx, value) in values.iter().enumerate() {
			let position = i32_type.const_int(idx as u64, false);
			let temp = self.stack.temp_register();
			let field_ptr = unsafe {
				match self.builder.build_gep(struct_type, ptr, &[zero, position], &temp) {
					Ok(ptr) => ptr,
					Err(_) => throw_llvm_error(format!("failed to create GEP for field {}", idx)),
				}
			};

			match self.builder.build_store(field_ptr, *value) {
				Ok(inst) => inst.set_alignment(4).unwrap_or_else(|err| {
					throw_llvm_error(format!("failed to set alignment for field: {}", err))
				}),
				Err(_) => throw_llvm_error(format!("failed to store field {}", idx)),
			}
		}
	}
}
