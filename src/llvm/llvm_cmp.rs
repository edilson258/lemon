#![allow(unused_imports)]
use inkwell::{values::FunctionValue, FloatPredicate, IntPredicate};

use crate::{
	ir::{self, Block},
	report::throw_llvm_error,
};

use super::Llvm;
impl Llvm<'_> {
	pub fn llvm_cmp_gt(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);
		let dest_str = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SGT;
			let result = match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_gt register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OGT;
			let result = match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest_str)
			{
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_gt register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}
		throw_llvm_error("unsupported operand types for 'cmp_gt' instr");
	}

	pub fn llvm_cmp_eq(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);
		let dest_str = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::EQ;
			let result = match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_eq register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OEQ;
			let result = match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest_str)
			{
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_eq register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}
		throw_llvm_error("unsupported operand types for 'cmp_eq' instr");
	}

	pub fn llvm_cmp_lt(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);
		let dest_str = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SLT;
			let result = match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_lt register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OLT;
			let result = match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest_str)
			{
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_lt register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}
		throw_llvm_error("unsupported operand types for 'cmp_lt' instr");
	}

	pub fn llvm_cmp_le(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);
		let dest_str = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SLE;
			let result = match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_le register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OLE;
			let result = match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest_str)
			{
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_le register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}
		throw_llvm_error("unsupported operand types for 'cmp_le' instr");
	}

	pub fn llvm_cmp_ge(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);
		let dest_str = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SGE;
			let result = match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_ge register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OGE;
			let result = match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest_str)
			{
				Ok(result) => result,
				Err(_) => throw_llvm_error("cmp_ge register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}
		throw_llvm_error("unsupported operand types for 'cmp_ge' instr");
	}
}
