#![allow(unused_imports)]
use inkwell::{values::FunctionValue, FloatPredicate, IntPredicate};

use crate::{
	ir::{self, Block},
	report::throw_llvm_error,
};

use super::Llvm;

impl Llvm<'_> {
	pub fn compile_cmp_gt(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);
		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};
		let dest = self.allocate_register(llvm_type, binary.dest);

		if lhs.is_int_value() && rhs.is_int_value() {
			let operator = IntPredicate::SGT;
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result =
				match self.builder.build_int_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_gt register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let operator = FloatPredicate::OGT;
			let result =
				match self.builder.build_float_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_gt register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		throw_llvm_error("cmp_gt register");
	}

	pub fn compile_cmp_eq(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);
		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};
		let dest = self.allocate_register(llvm_type, binary.dest);

		if lhs.is_int_value() && rhs.is_int_value() {
			let operator = IntPredicate::EQ;
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result =
				match self.builder.build_int_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_eq register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let operator = FloatPredicate::OEQ;
			let result =
				match self.builder.build_float_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_eq register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		throw_llvm_error("cmp_eq register");
	}

	pub fn compile_cmp_lt(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);
		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};
		let dest = self.allocate_register(llvm_type, binary.dest);

		if lhs.is_int_value() && rhs.is_int_value() {
			let operator = IntPredicate::SLT;
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result =
				match self.builder.build_int_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_lt register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let operator = FloatPredicate::OLT;
			let result =
				match self.builder.build_float_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_lt register"),
				};
			self.insert_value(binary.dest, result.into());
		}
	}

	pub fn compile_cmp_le(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);
		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};
		let dest = self.allocate_register(llvm_type, binary.dest);

		if lhs.is_int_value() && rhs.is_int_value() {
			let operator = IntPredicate::SLE;
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result =
				match self.builder.build_int_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_le register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let operator = FloatPredicate::OLE;
			let result =
				match self.builder.build_float_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_le register"),
				};
			self.insert_value(binary.dest, result.into());
		}
	}

	pub fn compile_cmp_ge(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.load_value(binary.lhs);
		let rhs = self.load_value(binary.rhs);
		let llvm_type = match self.compile_type_id(binary.type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error("found `UNIT` type in instr..."),
		};
		let dest = self.allocate_register(llvm_type, binary.dest);

		if lhs.is_int_value() && rhs.is_int_value() {
			let operator = IntPredicate::SGE;
			let lhs = lhs.into_int_value();
			let rhs = rhs.into_int_value();
			let result =
				match self.builder.build_int_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_ge register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs = lhs.into_float_value();
			let rhs = rhs.into_float_value();
			let operator = FloatPredicate::OGE;
			let result =
				match self.builder.build_float_compare(operator, lhs, rhs, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(err) => throw_llvm_error("cmp_ge register"),
				};
			self.insert_value(binary.dest, result.into());
		}

		throw_llvm_error("cmp_ge register");
	}
}
