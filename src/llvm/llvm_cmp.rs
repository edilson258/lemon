#![allow(unused_imports)]
use inkwell::{values::FunctionValue, FloatPredicate, IntPredicate};

use crate::{
	checker::types::TypeId,
	ir::{self, Block},
	report::throw_llvm_error,
};

use super::Llvm;
impl Llvm<'_> {
	pub fn llvm_cmp_gt(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);
		let dest = binary.dest.as_string();
		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SGT;
			match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(err) => throw_llvm_error(format!("cmp_gt register: {}", err)),
			}
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OGT;
			match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(err) => throw_llvm_error(format!("cmp_gt register: {}", err)),
			}
			return;
		}
		throw_llvm_error(format!("cannot cmp_gt '{}' to '{}'", lhs.get_type(), rhs.get_type()));
	}

	pub fn llvm_cmp_eq(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);
		let dest = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::EQ;
			match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(_) => throw_llvm_error("cmp_eq register"),
			};
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OEQ;
			match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(_) => throw_llvm_error("cmp_eq register"),
			};
			return;
		}

		throw_llvm_error(format!("cannot cmp_eq '{}' to '{}'", lhs.get_type(), rhs.get_type()));
	}

	pub fn llvm_cmp_lt(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);
		let dest = binary.dest.as_string();
		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SLT;
			match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(_) => throw_llvm_error("cmp_lt register"),
			};
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OLT;
			match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest) {
				Ok(result) => self.alloc_and_store(binary.type_id, result.into(), binary.dest),
				Err(_) => throw_llvm_error("cmp_lt register"),
			};
			return;
		}
		throw_llvm_error(format!("cannot cmp_lt '{}' to '{}'", lhs.get_type(), rhs.get_type()));
	}

	pub fn llvm_cmp_le(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);
		let dest = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SLE;
			match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(_) => throw_llvm_error("cmp_le register"),
			};
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OLE;
			match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(_) => throw_llvm_error("cmp_le register"),
			};
			return;
		}
		throw_llvm_error(format!("cannot cmp_le '{}' to '{}'", lhs.get_type(), rhs.get_type()));
	}

	pub fn llvm_cmp_ge(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);
		let dest = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let operator = IntPredicate::SGE;
			match self.builder.build_int_compare(operator, lhs_int, rhs_int, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(_) => throw_llvm_error("cmp_ge register"),
			};
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let operator = FloatPredicate::OGE;
			match self.builder.build_float_compare(operator, lhs_float, rhs_float, &dest) {
				Ok(value) => self.stack.set_value(binary.dest, value.into()),
				Err(_) => throw_llvm_error("cmp_ge register"),
			};
			return;
		}
		throw_llvm_error(format!("cannot cmp_ge '{}' to '{}'", lhs.get_type(), rhs.get_type()));
	}
}
