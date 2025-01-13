#![allow(unused_imports, unused_variables, dead_code)]

use inkwell::{types::StructType, values::FunctionValue, AddressSpace};

use crate::report::throw_llvm_error;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn core_bind(&mut self) {
		self.declare_write();
		self.declare_read();
		self.declare_exit();
		self.define_len();
	}

	pub fn declare_write(&mut self) -> FunctionValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let i8_ptr_type = self.ctx.i8_type();
		let params = [
			i32_type.into(),    // file descriptor
			i8_ptr_type.into(), // buffer
			i32_type.into(),    // buffer length
		];
		let write_type = i32_type.fn_type(&params, false);
		match self.module.get_function("write") {
			Some(fun) => fun,
			None => self.module.add_function("write", write_type, None),
		}
	}

	pub fn declare_read(&mut self) -> FunctionValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let i8_ptr_type = self.ctx.i8_type();
		let params = [
			i32_type.into(),    // file descriptor
			i8_ptr_type.into(), // buffer
			i32_type.into(),    // buffer length
		];
		let read_type = i32_type.fn_type(&params, false);
		match self.module.get_function("read") {
			Some(fun) => fun,
			None => self.module.add_function("read", read_type, None),
		}
	}

	pub fn declare_exit(&mut self) -> FunctionValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let params = [i32_type.into()];
		let exit_type = i32_type.fn_type(&params, false);
		match self.module.get_function("exit") {
			Some(fun) => fun,
			None => self.module.add_function("exit", exit_type, None),
		}
	}

	// (ptr, len)
	pub fn define_string_struct(&mut self) -> StructType<'ll> {
		let i8_ptr_type = self.ctx.ptr_type(AddressSpace::default());
		let i32_type = self.ctx.i32_type();
		self
			.ctx
			.struct_type(&[i8_ptr_type.into(), i32_type.into()], false)
	}

	// strings lens
	pub fn declare_len(&mut self) -> FunctionValue<'ll> {
		let string_ptr_type = self.ctx.ptr_type(AddressSpace::default());
		let i32_type = self.ctx.i32_type();
		let len_type = i32_type.fn_type(&[string_ptr_type.into()], false);
		self.module.add_function("len", len_type, None)
	}

	pub fn define_len(&mut self) {
		let string_type = self.define_string_struct();
		let len_fn = self.declare_len();
		// let string_type = string.get_type();
		let entry = self.ctx.append_basic_block(len_fn, "entry");
		self.builder.position_at_end(entry);

		let str_ptr = len_fn.get_first_param().unwrap().into_pointer_value();

		let len_ptr = match self
			.builder
			.build_struct_gep(string_type, str_ptr, 1, "len_ptr")
		{
			Ok(value) => value,
			Err(err) => throw_llvm_error("failed to get len ptr"),
		};

		let llvm_len_type = self.ctx.i32_type();
		let len_value = match self.builder.build_load(llvm_len_type, len_ptr, "len") {
			Ok(value) => value,
			Err(err) => throw_llvm_error("failed to load len"),
		};

		match self.builder.build_return(Some(&len_value.into_int_value())) {
			Ok(_) => (),
			Err(err) => throw_llvm_error("failed to return len"),
		}
	}
}
