use inkwell::types::BasicTypeEnum;

use crate::{
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_struct(&mut self, struct_instr: &ir::Struct) {
		#[rustfmt::skip]
		let struct_name = struct_instr.name.as_ref().unwrap_or_else(|| {
		  // todo: support it
		  throw_llvm_error("cannot support unnamed struct")
		});

		let llvm_struct_type = self.ctx.opaque_struct_type(struct_name);
		let mut fields: Vec<BasicTypeEnum<'ll>> = Vec::with_capacity(struct_instr.fields.len());
		for field in struct_instr.fields.iter() {
			let llvm_type = self.compile_type_to_basic_type(*field);
			fields.push(llvm_type);
		}
		llvm_struct_type.set_body(&fields, false);
	}
}
