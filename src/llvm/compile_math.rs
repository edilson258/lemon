use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl Llvm<'_> {
	// math instr
	pub fn compile_add(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);
		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};
		let dest = self.allocate_register(llvm_type, binary.dest);
		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result = match self.builder.build_int_add(lhs, rhs, &binary.dest.as_string()) {
				Ok(result) => result,
				Err(err) => throw_llvm_error("add register"),
			};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let result = match self.builder.build_float_add(lhs, rhs, &binary.dest.as_string()) {
				Ok(result) => result,
				Err(err) => throw_llvm_error("add register"),
			};
			self.insert_value(binary.dest, result.into());
		}
	}

	pub fn compile_sub(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);

		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};

		let dest = self.allocate_register(llvm_type, binary.dest);

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result = match self.builder.build_int_sub(lhs, rhs, &binary.dest.as_string()) {
				Ok(result) => result,
				Err(err) => throw_llvm_error("sub register"),
			};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let result = match self.builder.build_float_sub(lhs, rhs, &binary.dest.as_string()) {
				Ok(result) => result,
				Err(err) => throw_llvm_error("sub register"),
			};
			self.insert_value(binary.dest, result.into());
		}
	}

	pub fn compile_mul(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);
		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};
		let dest = self.allocate_register(llvm_type, binary.dest);
		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result = match self.builder.build_int_mul(lhs, rhs, &binary.dest.as_string()) {
				Ok(result) => result,
				Err(err) => throw_llvm_error("mul register"),
			};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let result = match self.builder.build_float_mul(lhs, rhs, &binary.dest.as_string()) {
				Ok(result) => result,
				Err(err) => throw_llvm_error("mul register"),
			};
			self.insert_value(binary.dest, result.into());
		}
	}
}
