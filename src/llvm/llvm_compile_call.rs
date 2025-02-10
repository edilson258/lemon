use crate::ir;

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_call(&mut self, call: &ir::CallInstr) {
		todo!("call {:?}", call);
	}
}
