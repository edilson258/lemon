use inkwell::types::BasicTypeEnum;

use crate::{checker::types::TypeId, ir, report::throw_llvm_error};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_add(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);
		let dest = binary.dest.as_string();
		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			match self.builder.build_int_add(lhs_int, rhs_int, &dest) {
				Ok(result) => self.stack.set_value(binary.dest, result.into()),
				Err(_) => throw_llvm_error("build int add"),
			};
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			match self.builder.build_float_add(lhs_float, rhs_float, &dest) {
				Ok(result) => self.stack.set_value(binary.dest, result.into()),
				Err(_) => throw_llvm_error("build float add"),
			};
			return;
		}
		let error = format!("unsupported 'add' {} to {}", lhs.get_type(), rhs.get_type());
		throw_llvm_error(error);
	}

	pub fn llvm_sub(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);

		let dest = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();

			match self.builder.build_int_sub(lhs_int, rhs_int, &dest) {
				Ok(result) => self.stack.set_value(binary.dest, result.into()),
				Err(_) => throw_llvm_error("sub register"),
			};
			return;
		}
		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			match self.builder.build_float_sub(lhs_float, rhs_float, &dest) {
				Ok(result) => self.stack.set_value(binary.dest, result.into()),
				Err(_) => throw_llvm_error("sub register"),
			};
			return;
		}

		let error = format!("unsupported 'sub' {} to {}", lhs.get_type(), rhs.get_type());
		throw_llvm_error(error);
	}

	pub fn llvm_mul(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.get_value_or_load(binary.lhs, binary.type_id);
		let rhs = self.get_value_or_load(binary.rhs, binary.type_id);
		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let dest = binary.dest.as_string();
			match self.builder.build_int_mul(lhs_int, rhs_int, &dest) {
				Ok(result) => self.stack.set_value(binary.dest, result.into()),
				Err(_) => throw_llvm_error("mul register"),
			};
			return;
		}
		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let dest = binary.dest.as_string();
			match self.builder.build_float_mul(lhs_float, rhs_float, &dest) {
				Ok(result) => self.stack.set_value(binary.dest, result.into()),
				Err(_) => throw_llvm_error("mul register"),
			};
			return;
		}

		let error = format!("unsupported 'mul' {} to {}", lhs.get_type(), rhs.get_type());
		throw_llvm_error(error);
	}
	pub fn resolve_llvm_type(&self, type_id: TypeId) -> BasicTypeEnum<'ll> {
		match self.llvm_type_from_type(type_id) {
			Some(llvm_type) => llvm_type,
			None => throw_llvm_error(format!("type {:?} not found", type_id)),
		}
	}
}
