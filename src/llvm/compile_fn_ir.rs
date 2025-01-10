use inkwell::types::{BasicType, FunctionType};

use crate::{
	checker::types::TypeId,
	ir::{self, FnNative},
	report::throw_llvm_error,
};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn compile_fn_ir(&mut self, fn_ir: &ir::Fn) {
		match fn_ir {
			ir::Fn::Native(fn_ir) => self.compile_native_fn_ir(fn_ir),
			ir::Fn::Comptime(fn_ir) => self.compile_comptime_fn_ir(fn_ir),
			ir::Fn::Extern(fn_ir) => self.compile_extern_fn_ir(fn_ir),
		}
	}

	fn compile_native_fn_ir(&mut self, fn_native: &ir::FnNative) {
		todo!()
		// let fn_name = fn_native.fn_id.as_string();
		// let fn_type = self.create_fn_type(fn_native);
		// let llvm_fn = self.module.add_function(fn_name, fn_type, None);
		// for (i, param) in fn_native.params.iter().enumerate() {
		// 	let param_value = llvm_fn.get_nth_param(i as u32).unwrap();
		// 	self.insert_value(param.register, param_value);
		// }
		// for block in &fn_native.blocks {
		// 	self.compile_block(&llvm_fn, block);
		// }
		// if fn_native.ret.is_UNIT() {
		// 	// return 0 if UNIT
		// 	let zero = self.ctx.i32_type().const_zero();
		// 	match self.builder.build_return(Some(&zero)) {
		// 		Ok(_) => {}
		// 		Err(err) => throw_llvm_error(format!("when build a void return, error: {}", err)),
		// 	}
	}

	fn create_fn_type(&mut self, fn_native: &FnNative) -> FunctionType<'ll> {
		let mut param_types: Vec<_> = Vec::with_capacity(fn_native.params.len());
		for param in &fn_native.params {
			let param_type = match self.compile_type_id(param.type_id) {
				Some(param_type) => param_type,
				None => throw_llvm_error("error: found `UNIT` type in param, isnt't allowed"),
			};
			param_types.push(param_type.into());
		}

		match fn_native.ret {
			TypeId::UNIT => {
				let void_type = self.ctx.void_type();
				void_type.fn_type(&param_types, false)
			}
			_ => {
				let ret_type = self.compile_type_id(fn_native.ret).unwrap();
				ret_type.fn_type(&param_types, false)
			}
		}
	}

	fn compile_comptime_fn_ir(&mut self, fn_ir: &ir::FnComptime) {
		throw_llvm_error("compile comptime fn not implemented")
	}

	fn compile_extern_fn_ir(&mut self, fn_ir: &ir::FnExtern) {
		throw_llvm_error("compile extern fn not implemented")
	}
}
