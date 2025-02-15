use crate::ir;

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_salloc(&mut self, instr: &ir::SallocInstr) {
		let basic_type = self.compile_type_to_basic_type(instr.size);
		let ptr = self.alloc(basic_type, instr.dest.value.as_str());
		self.env.set_value(instr.dest.value.as_str(), ptr.into());
	}
}
