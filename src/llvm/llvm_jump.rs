use crate::{
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;
impl Llvm<'_> {
	pub fn llvm_jmp_if(&mut self, jmp: &ir::JmpIfInstr) {
		let cond = self.stack.get_value(jmp.cond);
		if !cond.is_int_value() || cond.into_int_value().get_type().get_bit_width() != 1 {
			throw_llvm_error("jmp if condition must be i1");
		}
		let cond_value = cond.into_int_value();
		let block_true = match self.stack.get_block(jmp.l0) {
			Some(block) => block,
			None => throw_llvm_error("jmp if block l0 not found"),
		};

		let block_false = match self.stack.get_block(jmp.l1) {
			Some(block) => block,
			None => throw_llvm_error("jmp if block l1 not found"),
		};

		match self.builder.build_conditional_branch(cond_value, *block_true, *block_false) {
			Ok(_) => {}
			Err(err) => throw_llvm_error(format!("jmp if error: {}", err)),
		}
	}
}
