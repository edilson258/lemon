use crate::{error_codegen, ir};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_jmp(&mut self, jump: &ir::JmpInstr) {
		#[rustfmt::skip]
		let block = self.env.get_block(&jump.llvm_label()).unwrap_or_else(|| {
		error_codegen!("cannot find a block named '{}'", jump.llvm_label()).report(self.loader)
		});

		if let Err(err) = self.builder.build_unconditional_branch(*block) {
			error_codegen!("while jmp, reason `{}`", err).report(self.loader)
		}
	}
}
