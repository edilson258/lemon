use crate::ir;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_set(&mut self, instr: &ir::UnInstr) {
		let ptr = self.env.get_ptr_value(instr.dest.value.as_str()).unwrap();
		let value = self.llvm_compile_value(&instr.src);
		self.store(ptr, value);
	}
}
