use inkwell::{basic_block::BasicBlock, values::FunctionValue};

use crate::ir::{self, Block};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn compile_block(&mut self, llvm_fn: &FunctionValue<'ll>, block: &Block) {
		let llvm_block = self.get_or_append_block(&block.block_id, llvm_fn);

		self.builder.position_at_end(llvm_block);

		for instr in &block.instrs {
			self.compile_instr(instr);
		}
	}

	pub fn compile_block_id(&self, block_id: &ir::BlockId) -> String {
		if block_id.0 == 0 {
			return "entry".to_string();
		}
		format!("block_{}", block_id.0)
	}

	pub fn get_or_append_block(
		&mut self,
		block_id: &ir::BlockId,
		llvm_fn: &FunctionValue<'ll>,
	) -> BasicBlock<'ll> {
		if let Some(&existing_block) = self.block_store.get(block_id) {
			return existing_block;
		}

		let block_name = self.compile_block_id(block_id);
		let new_block = self.ctx.append_basic_block(*llvm_fn, &block_name);
		self.block_store.insert(*block_id, new_block);

		new_block
	}
}
