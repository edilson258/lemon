use crate::{
	checker::types::TypeId,
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;
impl Llvm<'_> {
	pub fn llvm_jmp_if(&mut self, jmp: &ir::JmpIfInstr) {
		let cond = self.get_value_or_load(jmp.cond, TypeId::BOOL);
		if !cond.is_int_value() || cond.into_int_value().get_type().get_bit_width() != 1 {
			throw_llvm_error("jmp if condition must be i1");
		}
		let cond_value = cond.into_int_value();
		#[rustfmt::skip]
		let block_true =  self.stack.get_block(jmp.l0).unwrap_or_else(||{
		throw_llvm_error("jmp if block l0 not found")
		});
		#[rustfmt::skip]
		let block_false = self.stack.get_block(jmp.l1).unwrap_or_else(||{
			throw_llvm_error("jmp if block l1 not found")
		});

		if let Err(err) = self.builder.build_conditional_branch(cond_value, *block_true, *block_false) {
			throw_llvm_error(format!("jmp if error: {}", err))
		}
	}

	pub fn llvm_goto(&mut self, goto: &ir::GotoInstr) {
		let block = self.stack.get_block(goto.block_id).unwrap_or_else(|| {
			throw_llvm_error(format!("failed to find '{}'", goto.block_id.as_string()))
		});

		if let Err(err) = self.builder.build_unconditional_branch(*block) {
			throw_llvm_error(format!("goto error: {}", err));
		}
	}
}
