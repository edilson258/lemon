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
	pub fn llvm_function(&mut self, fn_ir: &ir::Fn) {
		match fn_ir {
			ir::Fn::Ln(ln_fn) => self.llvm_ln_fn(ln_fn),
			ir::Fn::Ex(ex_fn) => self.llvm_ex_fn(ex_fn),
		}
	}

	pub fn llvm_ex_fn(&mut self, ex_fn: &ir::ExFn) {
		let fn_type = self.llvm_fun_type(&ex_fn.args, ex_fn.ret, ex_fn.var_packed);
		let fn_name = ex_fn.fn_id.as_string();

		self.module.add_function(fn_name, fn_type, None);
	}

	pub fn llvm_ln_fn(&mut self, ln_fn: &ir::LnFn) {
		let llvm_fun_value = self.llvm_fun_value(ln_fn);

		self.map_llvm_fun_value(&llvm_fun_value, &ln_fn.args);

		ln_fn.blocks.iter().for_each(|block| {
			self.register_block(block.block_id, &llvm_fun_value);
		});

		for block in &ln_fn.blocks {
			self.llvm_instr_block(block);
		}

		self.llvm_void_ret_value(ln_fn.ret, ln_fn.is_main());
	}

	fn llvm_fun_value(&mut self, ln_fn: &ir::LnFn) -> FunctionValue<'ll> {
		let fn_name = ln_fn.fn_id.as_string();
		let ret_type = if ln_fn.is_main() { TypeId::I32 } else { ln_fn.ret };
		let fn_type = self.llvm_fun_type(&ln_fn.args, ret_type, false);
		self.module.add_function(fn_name, fn_type, None)
	}

	fn llvm_void_ret_value(&mut self, type_id: TypeId, is_main: bool) {
		if type_id.is_unit() || type_id.is_void() {
			if is_main {
				let sucess = self.ctx.i32_type().const_int(0, false);
				if let Err(err) = self.builder.build_return(Some(&sucess)) {
					throw_llvm_error(format!("void return, error: {}", err));
				}
				return;
			}

			if let Err(err) = self.builder.build_return(None) {
				throw_llvm_error(format!("void return, error: {}", err));
			}
		}
	}

	fn map_llvm_fun_value(&mut self, llvm_fun_value: &FunctionValue<'ll>, params: &[ir::Bind]) {
		for (i, param) in params.iter().enumerate() {
			let param_value = match llvm_fun_value.get_nth_param(i as u32) {
				Some(param_value) => param_value,
				None => throw_llvm_error(format!("param {} not found", param.register.as_string())),
			};
			self.stack.set_value(param.register, param_value);
		}
	}

	fn llvm_fun_type(
		&mut self,
		binds: &[ir::Bind],
		ret: TypeId,
		is_packed: bool,
	) -> FunctionType<'ll> {
		let mut param_types: Vec<_> = Vec::with_capacity(binds.len());
		for param in binds {
			let param_type = self.resolve_llvm_type(param.type_id);
			param_types.push(param_type.into());
		}
		if let TypeId::UNIT = ret {
			return self.ctx.void_type().fn_type(&param_types, is_packed);
		}
		match self.llvm_type_from_type(ret) {
			Some(ret_type) => ret_type.fn_type(&param_types, is_packed),
			None => self.ctx.void_type().fn_type(&param_types, is_packed),
		}
	}
}
