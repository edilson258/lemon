use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_jmp(&mut self, jump: &ir::JmpInstr) {
		#[rustfmt::skip]
		let block = self.env.get_block(&jump.label).unwrap_or_else(|| {
			throw_llvm_error(format!("cannot find a block named '{}'", jump.label))
		});

		if let Err(err) = self.builder.build_unconditional_branch(*block) {
			throw_llvm_error(format!("while jmp, reason `{}`", err))
		}
	}
}
