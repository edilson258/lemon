#![allow(unused_imports)]
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::{
	ir::{self, Block},
	report::throw_llvm_error,
};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_call(&mut self, call_instr: &ir::CallInstr) {
		let llvm_fn = match self.module.get_function(&call_instr.fn_id) {
			Some(llvm_fn) => llvm_fn,
			None => throw_llvm_error(format!("fn {} not found", call_instr.fn_id)),
		};

		let mut args: Vec<_> = Vec::with_capacity(call_instr.args.len());
		for arg in &call_instr.args {
			let value = self.get_value_or_load(arg.register, arg.type_id);
			args.push(value.into());
		}

		let result = match self.builder.build_call(llvm_fn, &args, &call_instr.dest.as_string()) {
			Ok(sucess) => sucess,
			Err(err) => throw_llvm_error(format!("call error: {}", err)),
		};

		if let Some(return_value) = result.try_as_basic_value().left() {
			if self.stack.get_struct_type(call_instr.type_id).is_some() {
				self.stack.set_value(call_instr.dest, return_value);
			}
			self.stack.set_value(call_instr.dest, return_value);
		} else if !call_instr.type_id.is_unit() && !call_instr.type_id.is_void() {
			let mut type_text = String::new();
			call_instr.type_id.display_type(&mut type_text, self.type_store, true);
			throw_llvm_error(format!("call ret expected '{}', but nothing found", type_text));
		}
	}
}
