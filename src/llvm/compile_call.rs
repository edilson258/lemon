#![allow(unused_imports)]
use inkwell::values::FunctionValue;

use crate::{
	ir::{self, Block},
	report::throw_llvm_error,
};

use super::Llvm;

impl Llvm<'_> {
	pub fn compile_call(&mut self, call_instr: &ir::CallInstr) {
		let fn_name = call_instr.fn_id.as_string();
		let llvm_fn = match self.module.get_function(fn_name) {
			Some(llvm_fn) => llvm_fn,
			None => throw_llvm_error(format!("fn {} not found", fn_name)),
		};
		let mut args: Vec<_> = Vec::with_capacity(call_instr.args.len());
		for arg in &call_instr.args {
			let value = self.load_value(*arg);
			args.push(value.into());
		}
		let result = match self.builder.build_call(llvm_fn, &args, &call_instr.dest.as_string()) {
			Ok(sucess) => sucess,
			Err(err) => throw_llvm_error(format!("call error: {}", err)),
		};

		if let Some(return_value) = result.try_as_basic_value().left() {
			self.insert_value(call_instr.dest, return_value);
		} else if !call_instr.type_id.is_nothing() {
			throw_llvm_error(format!("call return type {:?} not found", call_instr.type_id));
		}
	}
}
