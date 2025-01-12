use inkwell::{
	types::{BasicType, FunctionType},
	values::FunctionValue,
};

use crate::{
	checker::types::TypeId,
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_function(&mut self, ln_fn: &ir::Fn) {
		let llvm_fun_value = self.create_llvm_fun_value(ln_fn);

		self.map_llvm_function_value(&llvm_fun_value, &ln_fn.params);

		for block in &ln_fn.blocks {
			self.llvm_block(&llvm_fun_value, block);
		}
		self.create_llvm_void_ret_value(&ln_fn.ret);
	}

	fn create_llvm_fun_value(&mut self, ln_fn: &ir::Fn) -> FunctionValue<'ll> {
		let fn_name = ln_fn.fn_id.as_string();
		let fn_type = self.create_llvm_function_type(ln_fn);
		self.module.add_function(fn_name, fn_type, None)
	}

	fn create_llvm_void_ret_value(&mut self, type_id: &TypeId) {
		if type_id.is_unit() || type_id.is_void() {
			match self.builder.build_return(None) {
				Ok(_) => {}
				Err(err) => throw_llvm_error(format!("void return, error: {}", err)),
			}
		}
	}

	fn map_llvm_function_value(&mut self, llvm_fun_value: &FunctionValue<'ll>, params: &[ir::Bind]) {
		for (i, param) in params.iter().enumerate() {
			let param_value = match llvm_fun_value.get_nth_param(i as u32) {
				Some(param_value) => param_value,
				None => throw_llvm_error(format!("param {} not found", param.register.as_string())),
			};
			self.stack.set_value(param.register, param_value);
		}
	}

	fn create_llvm_function_type(&mut self, ln_fn: &ir::Fn) -> FunctionType<'ll> {
		let mut param_types: Vec<_> = Vec::with_capacity(ln_fn.params.len());
		for param in &ln_fn.params {
			let param_type = self.resolve_llvm_type(param.type_id);
			param_types.push(param_type.into());
		}
		if let TypeId::UNIT = ln_fn.ret {
			return self.ctx.void_type().fn_type(&param_types, false);
		}
		match self.llvm_type_from_type(ln_fn.ret) {
			Some(ret_type) => ret_type.fn_type(&param_types, false),
			None => self.ctx.void_type().fn_type(&param_types, false),
		}
	}
}
