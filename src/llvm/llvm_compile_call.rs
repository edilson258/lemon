use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_call(&mut self, call: &ir::CallInstr) {
		let llvm_callee = match self.module.get_function(&call.callee) {
			Some(llvm_callee) => llvm_callee,
			None => throw_llvm_error(format!("function '{}' not found", call.callee)),
		};

		let args: Vec<_> = call.args.iter().map(|arg| self.llvm_compile_value(arg).into()).collect();

		let temp = self.env.get_temp();
		let call_result = match self.builder.build_call(llvm_callee, &args, temp.as_str()) {
			Ok(result) => result,
			Err(err) => throw_llvm_error(format!("call '{}'", err)),
		};

		if let Some(return_value) = call_result.try_as_basic_value().left() {
			let dest = call.dest.value.as_str();
			// todo: ???
			// if call.ret_id.is_unit() || call.ret_id.is_void() {
			// 	// let type_name = self.type_store.get_display_ir_type(call.ret_id);
			// }
			let ptr = self.env.get_ptr_value_unwrap(dest);
			self.store(ptr, return_value);
			// self.env.set_value(dest, return_value);
		}
	}
}
