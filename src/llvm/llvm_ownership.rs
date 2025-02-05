#![allow(dead_code, unused_variables)]

use inkwell::{types::StructType, values::BasicValueEnum};

use crate::{
	checker::types::TypeId,
	ir::{self, Register},
	report::throw_llvm_error,
};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn llvm_own_struct(&mut self, instr: &ir::OwnInstr, llvm_type: StructType<'ll>) {
		let value = *self.stack.get_value(instr.value);
		if value.is_pointer_value() {
			self.stack.set_register_type(instr.dest, llvm_type);
			return self.stack.set_value(instr.dest, value);
		}

		let value_type = value.get_type();
		if value_type.is_struct_type() {
			let struct_type = value_type.into_struct_type();
			let struct_size = self.calculate_struct_size(struct_type);
			let ptr = self.allocate_struct(struct_size, &instr.dest);
			match self.builder.build_store(ptr, value) {
				#[rustfmt::skip]
				Ok(sucess) => sucess.set_alignment(4).unwrap_or_else(|err| {
					throw_llvm_error(format!("store error: {}", err))
				}),
				Err(err) => throw_llvm_error(format!("store error: {}", err)),
			}
			self.stack.set_register_type(instr.dest, llvm_type);
			self.stack.set_value(instr.dest, ptr.into());
		}
	}

	pub fn llvm_own(&mut self, instr: &ir::OwnInstr) {
		if let Some(llvm_type) = self.stack.get_struct_type(instr.type_id) {
			return self.llvm_own_struct(instr, *llvm_type);
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
			return self.stack.set_global_value(instr.dest, basic_value);
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
