#![allow(dead_code, unused_variables)]

use inkwell::{types::StructType, values::BasicValueEnum};

use crate::{
	checker::types::TypeId,
	ir::{self, Register},
};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn llvm_own_struct(&mut self, instr: &ir::OwnInstr, llvm_type: StructType<'ll>) {
		// ref pointer to dest using  Bitcast... check if this is the best way to represent owner without overhead
		// let adress = self.ctx.ptr_type(AddressSpace::default());
		// let ptr = *self.stack.get_value(instr.value);
		// #[rustfmt::skip]
		// let dest_ptr = self.builder.build_bit_cast(ptr, adress, &instr.dest.as_string()).unwrap_or_else(|_| {
		// let error = format!("failed to cast {} to {}", instr.value.as_string(), instr.dest.as_string());
		// 	throw_llvm_error(error);
		// });
		// self.stack.set_value(instr.dest, dest_ptr);

		// // call free

		// let free_fn = self.get_free_fun();
		// let temp = self.stack.get_temp_reg();
		// self.builder.build_call(free_fn, &[dest_ptr.into()], &temp).unwrap();
	}

	// own
	//
	pub fn llvm_own(&mut self, instr: &ir::OwnInstr) {
		if let Some(llvm_type) = self.stack.get_struct_type(instr.type_id) {
			self.llvm_own_struct(instr, *llvm_type);
			return;
		}
		let value = self.get_value_or_load(instr.value, instr.type_id);
		if self.stack.has_value(instr.dest) {
			let ptr = self.stack.get_ptr_value(instr.dest);
			return self.store(ptr, value);
		}
		let ptr = self.alloc(self.resolve_llvm_type(instr.type_id), &instr.dest.as_string());
		self.store(ptr, value);
		self.stack.set_value(instr.dest, ptr.into());
	}

	// own heap
	//
	pub fn llvm_own_heap(&mut self, instr: &ir::OwnHeapInstr) {
		todo!()
	}

	// free
	//
	pub fn llvm_free(&mut self, instr: &ir::FreeInstr) {
		todo!()
	}

	// borrow
	//
	pub fn llvm_borrow(&mut self, instr: &ir::UnaryInstr) {
		todo!()
	}

	pub fn llvm_borrow_mut(&mut self, instr: &ir::UnaryInstr) {
		todo!()
	}

	// deref
	//
	pub fn llvm_deref(&mut self, instr: &ir::UnaryInstr) {
		todo!()
	}

	// load
	//
	pub fn llvm_load(&mut self, instr: &ir::UnaryInstr) {
		let ptr = self.stack.get_ptr_value(instr.value);
		// if let Some(type_id) = self.stack.get_struct_type(instr.type_id) {
		// 	let value = self.load(t, ptr, &instr.dest.as_string());
		// 	self.stack.set_value(instr.dest, value);
		// }
		let t = self.resolve_llvm_type(instr.type_id);
		let value = self.load(t, ptr, &instr.dest.as_string());
		self.stack.set_value(instr.dest, value);
	}

	// store
	//
	pub fn llvm_store(&mut self, instr: &ir::StoreInstr) {
		let basic_value = self.get_basic_value(&instr.value);
		if instr.type_id.is_string() || instr.type_id.is_str() {
			self.stack.set_global_value(instr.dest, basic_value);
			return;
		}
		self.alloc_and_store(instr.type_id, basic_value, instr.dest);
	}

	// load utils

	pub fn get_value_or_load(&mut self, reg: ir::Register, type_id: TypeId) -> BasicValueEnum<'ll> {
		if let Some(value) = self.stack.get_global_value(reg) {
			return *value;
		}
		if self.stack.has_ptr_value(reg) {
			let ptr = self.stack.get_ptr_value(reg);
			let t = self.resolve_llvm_type(type_id);
			let temp = self.stack.temp_register();
			return self.load(t, ptr, &temp);
		}
		self.stack.get_basic_value(reg)
	}

	pub fn alloc_and_store(&mut self, t: TypeId, value: BasicValueEnum<'ll>, dest: Register) {
		if self.stack.has_value(dest) {
			let ptr = self.stack.get_ptr_value(dest);
			return self.store(ptr, value);
		}
		let ptr = self.alloc(self.resolve_llvm_type(t), &dest.as_string());
		self.store(ptr, value);
		self.stack.set_value(dest, ptr.into());
	}
}
