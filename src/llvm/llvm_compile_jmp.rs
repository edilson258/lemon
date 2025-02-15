use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_jmp(&mut self, jump: &ir::JmpInstr) {
		#[rustfmt::skip]
		let block = self.env.get_block(&jump.llvm_label()).unwrap_or_else(|| {
			throw_llvm_error(format!("cannot find a block named '{}'", jump.llvm_label()))
		});

		if let Err(err) = self.builder.build_unconditional_branch(*block) {
			throw_llvm_error(format!("while jmp, reason `{}`", err))
		}
	}
}
