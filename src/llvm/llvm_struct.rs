use inkwell::types::BasicTypeEnum;

use crate::ir::{self};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn llvm_struct(&mut self, struct_instr: &ir::StructInstr) {
		let llvm_struct_type = self.ctx.opaque_struct_type(&struct_instr.struct_id);
		let mut fields: Vec<BasicTypeEnum<'ll>> = Vec::with_capacity(struct_instr.fields.len());
		for (index, field) in struct_instr.fields.iter().enumerate() {
			self.stack.add_struct_field(struct_instr.struct_id.clone(), field.register, index);
			let llvm_type = self.resolve_llvm_type(field.type_id);
			fields.push(llvm_type);
		}
		llvm_struct_type.set_body(&fields, false);
	}
}
