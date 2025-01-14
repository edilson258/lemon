use inkwell::types::BasicTypeEnum;

use crate::{checker::types::TypeId, ir, report::throw_llvm_error};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_add(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);
		let dest_str = binary.dest.as_string();
		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let result = match self.builder.build_int_add(lhs_int, rhs_int, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("build int add"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let result = match self.builder.build_float_add(lhs_float, rhs_float, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("build float add"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		let error = format!("unsupported 'add' {} to {}", lhs.get_type(), rhs.get_type());
		throw_llvm_error(error);
	}

	pub fn llvm_sub(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);

		let dest_str = binary.dest.as_string();

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let result = match self.builder.build_int_sub(lhs_int, rhs_int, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("sub register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}
		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let result = match self.builder.build_float_sub(lhs_float, rhs_float, &dest_str) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("sub register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}

		let error = format!("unsupported 'sub' {} to {}", lhs.get_type(), rhs.get_type());
		throw_llvm_error(error);
	}

	pub fn llvm_mul(&mut self, binary: &ir::BinaryInstr) {
		let lhs = self.stack.get_value(binary.lhs);
		let rhs = self.stack.get_value(binary.rhs);
		let llvm_type = self.resolve_llvm_type(binary.type_id);

		if lhs.is_int_value() && rhs.is_int_value() {
			let lhs_int = lhs.into_int_value();
			let rhs_int = rhs.into_int_value();
			let result = match self.builder.build_int_mul(lhs_int, rhs_int, &binary.dest.as_string()) {
				Ok(result) => result,
				Err(_) => throw_llvm_error("mul register"),
			};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
			return;
		}
		if lhs.is_float_value() && rhs.is_float_value() {
			let lhs_float = lhs.into_float_value();
			let rhs_float = rhs.into_float_value();
			let result =
				match self.builder.build_float_mul(lhs_float, rhs_float, &binary.dest.as_string()) {
					Ok(result) => result,
					Err(_) => throw_llvm_error("mul register"),
				};
			self.stack.allocate(llvm_type, binary.dest, &mut self.builder);
			self.stack.save(result.into(), binary.dest, &mut self.builder);
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
