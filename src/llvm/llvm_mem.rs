#![allow(unused_imports)]
use std::ffi::CString;

use inkwell::values::FunctionValue;
use logos::Source;

use crate::{
	ir::{self, Block, IrValue, Register},
	report::throw_llvm_error,
};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_load(&mut self, instr: &ir::UnaryInstr) {
		self.stack.load(instr.value, instr.dest, &mut self.builder);
	}

	pub fn llvm_store(&mut self, instr: &ir::StoreInstr) {
		if let IrValue::String(value) = &instr.value {
			self.store_string(value, instr.dest);
			return;
		}
		let llvm_value = self.ln_value_to_llvm(&instr.value);
		let llvm_type = self.resolve_llvm_type(instr.type_id);
		self
			.stack
			.allocate(llvm_type, instr.dest, &mut self.builder);
		self.stack.save(llvm_value, instr.dest, &mut self.builder);
	}

	pub fn store_string(&mut self, value: &str, dest: Register) {
		let c_string = match CString::new(value) {
			Err(_) => throw_llvm_error("transform to c_string"),
			Ok(str) => str,
		};
		let string_value = self.ctx.const_string(c_string.as_bytes_with_nul(), false);
		let global = self
			.module
			.add_global(string_value.get_type(), None, &dest.as_string());
		global.set_initializer(&string_value);

		let global_ptr_str = global.as_pointer_value();

		self.stack.set_value(dest, global_ptr_str.into());
	}

	pub fn llvm_borrow(&mut self, instr: &ir::UnaryInstr) {
		let value_ptr = self.stack.get_value(instr.value);

		if !value_ptr.is_pointer_value() {
			throw_llvm_error(format!("cannot borrow non-pointer '{}'", instr.value.as_string()));
		}

		self.stack.set_value(instr.dest, *value_ptr);
	}

	// pub fn llvm_borrow_mut(&mut self, instr: &ir::UnaryInstr) {
	// 	self.llvm_borrow(instr);
	// }

	pub fn llvm_own(&mut self, instr: &ir::OwnInstr) {
		if instr.type_id.is_string() {
			self.own_string(instr);
			return;
		}
		let llvm_type = self.resolve_llvm_type(instr.type_id);

		self
			.stack
			.allocate(llvm_type, instr.dest, &mut self.builder);

		let value = self.stack.get_value(instr.value);

		self.stack.save(*value, instr.dest, &mut self.builder);
	}

	pub fn own_string(&mut self, instr: &ir::OwnInstr) {
		let value = self.stack.get_value(instr.value);
		self.stack.set_value(instr.dest, *value);
	}

	pub fn llvm_own_heap(&mut self, _instr: &ir::OwnHeapInstr) {
		todo!()
	}
	pub fn llvm_free(&mut self, _instr: &ir::FreeInstr) {
		todo!()
	}
}
