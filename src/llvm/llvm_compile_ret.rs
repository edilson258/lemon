use crate::ir;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_ret(&mut self, binary: &ir::IrBasicValue) {}
}
