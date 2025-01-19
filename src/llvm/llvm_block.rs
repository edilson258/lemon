use crate::{
	ir::{self, Block, BlockId},
	report::throw_llvm_error,
};
use inkwell::values::FunctionValue;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_instr_block(&mut self, block: &Block) {
		let llvm_block = match self.stack.get_block(block.block_id) {
			Some(block) => block,
			None => throw_llvm_error(format!("block {} not found", block.block_id.0)),
		};
		self.builder.position_at_end(*llvm_block);
		for instr in &block.instrs {
			self.llvm_instr(instr);
		}
	}

	pub fn llvm_block_name(&self, block_id: &ir::BlockId) -> String {
		if block_id.0 == 0 {
			return "entry".to_string();
		}
		block_id.as_string()
	}

	pub fn register_block(&mut self, block_id: BlockId, fun: &FunctionValue<'ll>) {
		let block_name = self.llvm_block_name(&block_id);
		let llvm_block = self.ctx.append_basic_block(*fun, &block_name);
		self.stack.set_block(block_id, llvm_block);
	}
}
