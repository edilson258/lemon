use inkwell::{
	types::StructType,
	values::{BasicValueEnum, PointerValue},
};

use crate::report::throw_llvm_error;

use super::Llvm;
impl<'ll> Llvm<'ll> {
	// pub fn store_struct_fields(
	// 	&mut self,
	// 	values: &[BasicValueEnum<'ll>],
	// 	ptr: PointerValue<'ll>,
	// 	struct_type: StructType<'ll>,
	// ) {

	// }

	pub fn store_struct_field(
		&mut self,
		ll_type: StructType<'ll>,
		ptr: PointerValue<'ll>,
		value: BasicValueEnum<'ll>,
		at: usize,
	) {
		let field_ptr = self.build_field_pointer(ll_type, ptr, at);
		match self.builder.build_store(field_ptr, value) {
			Ok(inst) => inst.set_alignment(4).unwrap_or_else(|err| {
				throw_llvm_error(format!("failed to set alignment for field: {}", err))
			}),
			Err(_) => throw_llvm_error(format!("failed to store field {}", at)),
		}
	}
}
