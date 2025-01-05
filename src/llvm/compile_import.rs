#![allow(unused_imports)]
use std::collections::HashMap;

use inkwell::{self, values::FunctionValue, AddressSpace};

use crate::{
	checker::context::Context,
	ir::{self, Block, Builder},
	report::throw_llvm_error,
};

use super::Llvm;

impl Llvm<'_> {
	pub fn compile_import(&mut self, import: &ir::ImportInstr) {
		let module = import.module.as_str();

		if let Some(llvm_module) = self.c_fns.get(module) {
			self.ported.insert(module.to_string());
			// register module
			self.module.add_function(module, llvm_module.get_type(), None);
		} else {
			throw_llvm_error(format!("module '{}' not found", module));
		}
	}
}
