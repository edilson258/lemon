use crate::{ir::GetPtrInstr, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_getptr(&mut self, getptr_instr: &GetPtrInstr) {
		let offset = getptr_instr.offset;

		let dest = getptr_instr.dest.value.as_str();
		let i32_type = self.ctx.i32_type();
		let offset = i32_type.const_int(offset as u64, false);

		let self_value = self.llvm_compile_value(&getptr_instr.self_base);

		let self_ptr = self_value.into_pointer_value();

		let self_name = getptr_instr.self_name.as_str();
		let self_type = match self.ctx.get_struct_type(self_name) {
			Some(value) => value,
			None => throw_llvm_error(format!("failed to get self type: {}", self_name)),
		};

		let field_ptr = unsafe {
			match self.builder.build_gep(self_type, self_ptr, &[i32_type.const_zero(), offset], dest) {
				Ok(value) => value,
				Err(e) => throw_llvm_error(format!("failed grep: {}", e)),
			}
		};

		self.env.set_value(dest, field_ptr.into());
	}
}
