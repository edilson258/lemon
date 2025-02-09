use crate::{ir, report::throw_llvm_error};
use inkwell::values::FunctionValue;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_block(&mut self, block: &ir::IrBlock) {
		let llvm_block = match self.env.get_block(block.llvm_name().as_str()) {
			Some(block) => block,
			None => throw_llvm_error(format!("block {} not found", block.llvm_name())),
		};
		self.builder.position_at_end(*llvm_block);

		block.instrs.iter().for_each(|instr| {
			self.llvm_compile_instr(instr);
		});
	}
	pub fn llvm_register_block(&mut self, block_name: &str, function: &FunctionValue<'ll>) {
		let llvm_block = self.ctx.append_basic_block(*function, block_name);
		self.env.set_block(block_name, llvm_block);
	}
}
