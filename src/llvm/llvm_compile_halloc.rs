use inkwell::AddressSpace;

use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_halloc(&mut self, instr: &ir::UnInstr) {
		let size = self.llvm_compile_value(&instr.src);
		let dest = instr.dest.value.as_str();
		let malloc = self.get_malloc_function();
		let temp = self.env.get_temp();

		#[rustfmt::skip]
		let call_site_value = self.builder.build_call(malloc, &[size.into()], &temp).unwrap_or_else(|err| {
			throw_llvm_error(format!("failed to allocate struct memory: {}", err))
		});

		#[rustfmt::skip]
		let address_value = call_site_value.try_as_basic_value().left().unwrap_or_else(|| {
			throw_llvm_error("allocated value not found".to_string())
		});

		let address_ptr_value = address_value.into_pointer_value();
		let address = self.ctx.ptr_type(AddressSpace::default());
		let temp = self.env.get_temp();

		#[rustfmt::skip]
		let raw_ptr = self.builder.build_pointer_cast(address_ptr_value, address, &temp).unwrap_or_else(|err| {
			throw_llvm_error(format!("failed to cast pointer: {}", err))
		});

		let address = self.ctx.ptr_type(AddressSpace::default());

		let ptr = match self.builder.build_pointer_cast(raw_ptr, address, dest) {
			Ok(ptr) => ptr.into(),
			Err(err) => throw_llvm_error(format!("failed to cast pointer: {}", err)),
		};

		self.env.set_value(dest, ptr);
	}
}
