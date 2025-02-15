use crate::ir;

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_set(&mut self, instr: &ir::UnInstr) {
		let ptr = self.env.get_ptr_value_unwrap(instr.dest.value.as_str());
		let value = self.llvm_compile_value(&instr.src);
		self.store(ptr, value);
	}
}
