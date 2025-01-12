#![allow(unused_imports)]
use inkwell::values::FunctionValue;

use crate::{
	ir::{self, Block},
	report::throw_llvm_error,
};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_load(&mut self, instr: &ir::UnaryInstr) {
		self.stack.load(instr.value, instr.dest, &mut self.builder);
	}

	pub fn llvm_store(&mut self, instr: &ir::StoreInstr) {
		let llvm_value = self.lemon_value_to_llvm(&instr.value);
		let llvm_type = self.resolve_llvm_type(instr.type_id);
		self.stack.allocate(llvm_type, instr.dest, &mut self.builder);
		self.stack.save(llvm_value, instr.dest, &mut self.builder);
	}

	pub fn llvm_borrow(&mut self, instr: &ir::UnaryInstr) {
		let value_ptr = self.stack.get_value(instr.value);

		if !value_ptr.is_pointer_value() {
			throw_llvm_error(format!("cannot borrow non-pointer '{}'", instr.value.as_string()));
		}

		self.stack.set_value(instr.dest, *value_ptr);
	}

	pub fn llvm_borrow_mut(&mut self, instr: &ir::UnaryInstr) {
		self.llvm_borrow(instr);
	}

	pub fn llvm_own(&mut self, instr: &ir::OwnInstr) {
		let llvm_type = self.resolve_llvm_type(instr.type_id);

		self.stack.allocate(llvm_type, instr.dest, &mut self.builder);

		let value = self.stack.get_value(instr.value);

		self.stack.save(*value, instr.dest, &mut self.builder);
	}

	pub fn llvm_own_heap(&mut self, instr: &ir::OwnHeapInstr) {
		todo!()
	}
	pub fn llvm_free(&mut self, instr: &ir::FreeInstr) {
		todo!()
	}
}
