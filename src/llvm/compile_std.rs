#![allow(unused_imports)]
use std::collections::HashMap;

use inkwell::{self, values::FunctionValue, AddressSpace};

use crate::{
	checker::context::Context,
	ir::{self, Block, Builder},
};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn generate_std_module(&self) -> HashMap<String, FunctionValue<'ll>> {
		let mut std_functions = HashMap::new();

		let printf_fn = self.create_printf();

		let println_fn = self.create_println(printf_fn);
		std_functions.insert("println".to_string(), println_fn);

		let print_fn = self.create_print(printf_fn);
		std_functions.insert("print".to_string(), print_fn);

		// let exit_fn = self.create_exit();
		// std_functions.insert("exit".to_string(), exit_fn);

		// let assert_fn = self.create_assert();
		// std_functions.insert("assert".to_string(), assert_fn);

		std_functions
	}

	fn create_printf(&self) -> FunctionValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let printf_ty = i32_type.fn_type(&[self.ctx.ptr_type(AddressSpace::default()).into()], true);
		self.module.add_function("printf", printf_ty, None)
	}

	fn create_print_wrapper(
		&self,
		printf_fn: FunctionValue<'ll>,
		wrapper_name: &str,
		format_str: &str,
	) -> FunctionValue<'ll> {
		let wrapper_ty = self.ctx.void_type().fn_type(&[self.ctx.i32_type().into()], false);
		let wrapper_fn = self.module.add_function(wrapper_name, wrapper_ty, None);
		let entry = self.ctx.append_basic_block(wrapper_fn, "entry");
		self.builder.position_at_end(entry);

		let format_ptr = self
			.builder
			.build_global_string_ptr(format_str, &format!("{}_format", wrapper_name))
			.unwrap();

		let arg = wrapper_fn.get_nth_param(0).unwrap().into_int_value();

		self
			.builder
			.build_call(
				printf_fn,
				&[format_ptr.as_pointer_value().into(), arg.into()],
				&format!("{}_call", wrapper_name),
			)
			.unwrap();

		self.builder.build_return(None).unwrap();
		wrapper_fn
	}

	fn create_println(&self, printf_fn: FunctionValue<'ll>) -> FunctionValue<'ll> {
		self.create_print_wrapper(printf_fn, "println", "%d\n")
	}

	fn create_print(&self, printf_fn: FunctionValue<'ll>) -> FunctionValue<'ll> {
		self.create_print_wrapper(printf_fn, "print", "%d")
	}
	fn create_exit(&self) -> FunctionValue<'ll> {
		let exit_ty = self.ctx.void_type().fn_type(&[self.ctx.i32_type().into()], false);
		let exit_fn = self.module.add_function("exit", exit_ty, None);

		self.module.add_function(
			"exit",
			self.ctx.void_type().fn_type(&[self.ctx.i32_type().into()], false),
			None,
		)
	}

	fn create_assert(&self) -> FunctionValue<'ll> {
		let assert_ty = self.ctx.void_type().fn_type(&[self.ctx.i8_type().into()], false);
		let assert_fn = self.module.add_function("assert", assert_ty, None);
		let entry = self.ctx.append_basic_block(assert_fn, "entry");
		self.builder.position_at_end(entry);

		let condition = assert_fn.get_nth_param(0).unwrap().into_int_value();

		let success_block = self.ctx.append_basic_block(assert_fn, "success");
		let fail_block = self.ctx.append_basic_block(assert_fn, "fail");

		self.builder.build_conditional_branch(condition, success_block, fail_block).unwrap();

		//  exit(1)
		self.builder.position_at_end(fail_block);
		let exit_fn = self.module.get_function("exit").unwrap();
		self
			.builder
			.build_call(exit_fn, &[self.ctx.i32_type().const_int(1, false).into()], "exit_call")
			.unwrap();
		self.builder.build_unreachable().unwrap();

		// Success block
		self.builder.position_at_end(success_block);
		self.builder.build_return(None).unwrap();

		assert_fn
	}

	pub fn generate_std_self(&self) -> HashMap<String, FunctionValue<'ll>> {
		let mut std_functions = HashMap::new();

		let printf_fn = self.create_printf();

		let println_fn = self.create_println(printf_fn);
		std_functions.insert("println".to_string(), println_fn);

		std_functions
	}
}
