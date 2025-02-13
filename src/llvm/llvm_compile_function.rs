use crate::checker::types::TypeId;
use crate::ir::{self};
use crate::report::throw_llvm_error;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};
use inkwell::values::FunctionValue;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_function(&mut self, function: &ir::Function) {
		self.env.enter_function_scope();

		let function_value = self.create_llvm_function_value(function);
		self.register_function_param(&function_value, &function.args);

		function.blocks.iter().for_each(|block| {
			self.llvm_register_block(block.llvm_name().as_str(), &function_value);
		});

		if !function.is_extern_function() {
			function.blocks.iter().for_each(|block| {
				self.llvm_compile_block(block);
			});
		}
		self.build_llvm_return_function(function);
		self.env.exit_function_scope();
	}

	fn create_llvm_function_value(&mut self, function: &ir::Function) -> FunctionValue<'ll> {
		let function_type = self.create_llvm_function_type(function);
		let linkage = None;
		self.module.add_function(function.name.as_str(), function_type, linkage)
	}

	fn create_llvm_function_type(&mut self, function: &ir::Function) -> FunctionType<'ll> {
		let args_type = self.create_llvm_param_types(function);
		// todo: ignore ret type of main function?
		let ret_type = if function.is_main() { TypeId::I32 } else { function.ret };
		let ret_type = self.type_store.resolve_borrow_type(ret_type);
		if ret_type.is_unit() || ret_type.is_void() {
			return self.ctx.void_type().fn_type(&args_type, function.is_variadic_args());
		}
		let llvm_type = self.compile_type_to_basic_type(ret_type);
		llvm_type.fn_type(&args_type, function.is_variadic_args())
	}

	fn create_llvm_param_types(&mut self, fun: &ir::Function) -> Vec<BasicMetadataTypeEnum<'ll>> {
		let mut args_type = Vec::with_capacity(fun.args.len());
		for arg in &fun.args {
			let arg_type = self.compile_type_to_basic_type(arg.kind);
			args_type.push(arg_type.into());
		}
		args_type
	}

	fn register_function_param(&mut self, function: &FunctionValue<'ll>, params: &[ir::IrBind]) {
		params.iter().enumerate().for_each(|(i, param)| {
			let param_value = match function.get_nth_param(i as u32) {
				Some(param_value) => param_value,
				None => throw_llvm_error(format!("param {} not found", param.name)),
			};
			self.env.set_value(param.name.as_str(), param_value);
		});
	}

	fn build_llvm_return_function(&mut self, function: &ir::Function) {
		let ret_type = self.type_store.resolve_borrow_type(function.ret);
		let is_main_return_void = function.is_main() && (ret_type.is_unit() || ret_type.is_void());

		if is_main_return_void {
			let sucess = self.ctx.i32_type().const_int(0, false);
			if let Err(err) = self.builder.build_return(Some(&sucess)) {
				throw_llvm_error(format!("cannot build success void return, error: {}", err));
			}
			return;
		}

		if let Err(err) = self.builder.build_return(None) {
			throw_llvm_error(format!("cannot build return, error: {}", err));
		}
	}
}
