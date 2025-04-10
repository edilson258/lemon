#![allow(dead_code)]
use inkwell::types::BasicType;
use inkwell::values::{BasicValue, FunctionValue};
use inkwell::AddressSpace;

use crate::error_codegen;

use super::Llvm;

type Ptr<'ll> = inkwell::values::PointerValue<'ll>;
type ValueEnum<'ll> = inkwell::values::BasicValueEnum<'ll>;

impl<'ll> Llvm<'ll> {
	#[rustfmt::skip]
	pub fn alloc<T: BasicType<'ll>>(&mut self, llvm_t: T, dest: &str) -> Ptr<'ll> {
		 self.builder.build_alloca(llvm_t, dest)
			.unwrap_or_else(|err| {
			let message = error_codegen!("cannot allocate at stack, reason `{}`", err);
			message.report(self.loader);
		})
	}

	pub fn load<T: BasicType<'ll>>(&mut self, t: T, ptr: Ptr<'ll>, dest: &str) -> ValueEnum<'ll> {
		match self.builder.build_load(t, ptr, dest) {
			Ok(value) => value,
			Err(err) => error_codegen!("cannot load from stack, reason `{}`", err).report(self.loader),
		}
	}

	pub fn store<V: BasicValue<'ll>>(&mut self, ptr: Ptr<'ll>, value: V) {
		if let Err(err) = self.builder.build_store(ptr, value) {
			error_codegen!("cannot store to stack/heap, reason `{}`", err).report(self.loader);
		}
	}

	fn malloc(&mut self, size: u64, dest: &str) -> Ptr<'ll> {
		let function_value = self.get_malloc_function();
		let llvm_value = self.ctx.i64_type().const_int(size, false);
		let value = match self.builder.build_call(function_value, &[llvm_value.into()], dest) {
			Ok(site_value) => site_value,
			Err(err) => error_codegen!("cannot allocate at heap, reason `{}`", err).report(self.loader),
		};

		match value.try_as_basic_value().left() {
			Some(value) => value.into_pointer_value(),
			None => {
				let message = error_codegen!("cannot allocate at heap, reason `cannot convert to pointer`");
				message.report(self.loader);
			}
		}
	}

	fn free(&mut self, ptr: Ptr<'ll>) {
		let function_value = self.get_free_function();
		let params = [ptr.into()];
		#[rustfmt::skip]
		self.builder.build_call(function_value, &params, "r_droped")
			.unwrap_or_else(|err| error_codegen!("cannot drop at heap, reason `{}`", err).report(self.loader));
	}

	// heap memory
	//
	pub fn get_malloc_function(&mut self) -> FunctionValue<'ll> {
		match self.module.get_function("malloc") {
			Some(fun) => fun,
			None => self.declare_malloc_function(),
		}
	}

	pub fn get_free_function(&mut self) -> FunctionValue<'ll> {
		match self.module.get_function("free") {
			Some(fun) => fun,
			None => self.declare_free_function(),
		}
	}

	fn declare_malloc_function(&mut self) -> FunctionValue<'ll> {
		let i8_ptr = self.ctx.ptr_type(AddressSpace::default());
		let malloc_type = i8_ptr.fn_type(&[self.ctx.i64_type().into()], false);
		self.module.add_function("malloc", malloc_type, None)
	}

	fn declare_free_function(&mut self) -> FunctionValue<'ll> {
		let ptr_type = self.ctx.ptr_type(AddressSpace::default());
		let free_type = self.ctx.void_type().fn_type(&[ptr_type.into()], false);
		self.module.add_function("free", free_type, None)
	}
}
