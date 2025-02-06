#![allow(unused_imports, dead_code)]

use std::ffi::CString;

use inkwell::{
	types::{BasicType, BasicTypeEnum},
	values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
	AddressSpace,
};
use logos::Source;

use crate::{
	ir::{self, Block, IrValue, Register},
	report::throw_llvm_error,
};

use super::Llvm;
type Ptr<'ll> = inkwell::values::PointerValue<'ll>;
type ValueEnum<'ll> = inkwell::values::BasicValueEnum<'ll>;
impl<'ll> Llvm<'ll> {
	pub fn alloc<T: BasicType<'ll>>(&mut self, llvm_t: T, dest: &str) -> Ptr<'ll> {
		match self.builder.build_alloca(llvm_t, dest) {
			Ok(value) => value,
			Err(err) => throw_llvm_error(format!("alloc error: {}", err)),
		}
	}

	pub fn load<T: BasicType<'ll>>(&mut self, t: T, ptr: Ptr<'ll>, dest: &str) -> ValueEnum<'ll> {
		match self.builder.build_load(t, ptr, dest) {
			Ok(value) => value,
			Err(err) => throw_llvm_error(format!("load error: {}", err)),
		}
	}

	pub fn store<V: BasicValue<'ll>>(&mut self, ptr: Ptr<'ll>, value: V) {
		if let Err(err) = self.builder.build_store(ptr, value) {
			throw_llvm_error(format!("store error: {}", err))
		}
	}

	fn malloc(&mut self, size: u64, dest: &str) -> Ptr<'ll> {
		let malloc_fun = self.get_malloc_fun();
		let llvm_value = self.ctx.i64_type().const_int(size, false);
		let value = match self.builder.build_call(malloc_fun, &[llvm_value.into()], dest) {
			Ok(site_value) => site_value,
			Err(err) => throw_llvm_error(format!("malloc error: {}", err)),
		};

		match value.try_as_basic_value().left() {
			Some(value) => value.into_pointer_value(),
			None => throw_llvm_error("malloc return value not found"),
		}
	}

	fn free(&mut self, ptr: Ptr<'ll>) {
		let temp = self.stack.temp_register();
		let free_fun = self.get_free_fun();
		match self.builder.build_call(free_fun, &[ptr.into()], temp.as_str()) {
			Ok(_) => {} // do nothing here?
			Err(err) => throw_llvm_error(format!("free error: {}", err)),
		};
	}

	// heap memory
	//
	pub fn get_malloc_fun(&mut self) -> FunctionValue<'ll> {
		match self.module.get_function("malloc") {
			Some(fun) => fun,
			None => self.declare_malloc_fun(),
		}
	}

	pub fn get_free_fun(&mut self) -> FunctionValue<'ll> {
		match self.module.get_function("free") {
			Some(fun) => fun,
			None => self.declare_free_fun(),
		}
	}

	fn declare_malloc_fun(&mut self) -> FunctionValue<'ll> {
		let i8_ptr = self.ctx.ptr_type(AddressSpace::default());
		let malloc_type = i8_ptr.fn_type(&[self.ctx.i64_type().into()], false);
		self.module.add_function("malloc", malloc_type, None)
	}

	fn declare_free_fun(&mut self) -> FunctionValue<'ll> {
		let ptr_type = self.ctx.ptr_type(AddressSpace::default());
		let free_type = self.ctx.void_type().fn_type(&[ptr_type.into()], false);
		self.module.add_function("free", free_type, None)
	}
}
