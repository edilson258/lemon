use inkwell::values::FunctionValue;

use crate::ir::{self, Block};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn compile_block(&mut self, llvm_fn: &FunctionValue<'ll>, block: &Block) {
		let block_id = self.compile_block_id(&block.block_id);
		let llvm_block = self.ctx.append_basic_block(*llvm_fn, &block_id);
		self.builder.position_at_end(llvm_block);
		self.set_block(&block.block_id, llvm_block);
		for instr in &block.instrs {
			self.compile_instr(instr);
		}
	}
	pub fn compile_block_id(&mut self, block_id: &ir::BlockId) -> String {
		if block_id.0 == 0 {
			return "entry".to_string();
		}
		block_id.as_string()
	}
}
