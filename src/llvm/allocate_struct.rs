use inkwell::{
	values::{IntValue, PointerValue},
	AddressSpace,
};

use crate::{error_codegen, ir::Register};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn allocate_struct(&mut self, stt_size: IntValue<'ll>, dest: &Register) -> PointerValue<'ll> {
		let malloc = self.get_malloc_fun();
		let temp = self.stack.temp_register();
		#[rustfmt::skip]
		let call_value = self.builder.build_call(malloc, &[stt_size.into()], &temp).unwrap_or_else(|err| {
			throw_llvm_error(format!("failed to allocate struct memory: {}", err))
		});

		#[rustfmt::skip]
		let value = call_value.try_as_basic_value().left().unwrap_or_else(|| {
			throw_llvm_error("allocated value not found".to_string())
		});

		let ptr_value = value.into_pointer_value();
		let address = self.ctx.ptr_type(AddressSpace::default());
		let temp = self.stack.temp_register();

		#[rustfmt::skip]
		let raw_ptr = self.builder.build_pointer_cast(ptr_value, address, &temp).unwrap_or_else(|err| {
			throw_llvm_error(format!("failed to cast pointer: {}", err))
		});

		let address = self.ctx.ptr_type(AddressSpace::default());

		// todo: rethinking about this
		self.stack.set_free_ptr(*dest, raw_ptr);

		match self.builder.build_pointer_cast(raw_ptr, address, &dest.as_string()) {
			Ok(ptr) => ptr,
			Err(err) => throw_llvm_error(format!("failed to cast pointer: {}", err)),
		}
	}
}
