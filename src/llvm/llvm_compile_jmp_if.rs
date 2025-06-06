use crate::{error_codegen, ir};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_jmp_if(&mut self, jump_if: &ir::JmpIfInstr) {
		let cond = self.llvm_compile_value(&jump_if.cond);
		// todo: is trow a error when cond is not bool(e.g: 1 or 0)
		let cond_value = cond.into_int_value();
		#[rustfmt::skip]
		let block_true = *self.env.get_block(&jump_if.llvm_true_label()).unwrap_or_else(||{
		  error_codegen!("cannot find true label to jmp").report(self.loader)
		});

		#[rustfmt::skip]
		let block_false = *self.env.get_block(&jump_if.llvm_false_label()).unwrap_or_else(||{
		  error_codegen!("cannot find false label to jmp").report(self.loader)
		});

		if let Err(err) = self.builder.build_conditional_branch(cond_value, block_true, block_false) {
			error_codegen!("while jmp if, reason `{}`", err).report(self.loader)
		}
	}
}
