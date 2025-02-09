use inkwell::values::BasicValueEnum;

use crate::{ir, report::throw_llvm_error};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_value(&mut self, ir_basic_value: &ir::IrBasicValue) -> BasicValueEnum<'ll> {
		match &ir_basic_value.value {
			ir::BasicValue::Int(value) => self.ctx.i64_type().const_int(*value, false).into(),
			ir::BasicValue::Float(value) => self.ctx.f64_type().const_float(*value).into(),
			ir::BasicValue::Bool(value) => self.ctx.bool_type().const_int(*value as u64, false).into(),
			ir::BasicValue::String(_) => throw_llvm_error("unsupported string value"),
			ir::BasicValue::Char(_) => throw_llvm_error("unsupported char value"),
			ir::BasicValue::None => throw_llvm_error("unsupported none value"),
			ir::BasicValue::Register(name) => self.llvm_compile_register(name),
		}
	}

	pub fn llvm_compile_value_and_load(&mut self, value: &ir::IrBasicValue) -> BasicValueEnum<'ll> {
		let basic_value = self.llvm_compile_value(value);
		if !basic_value.is_pointer_value() {
			return basic_value;
		};
		let ptr = basic_value.into_pointer_value();
		let basic_type = self.compile_type_to_basic_type(value.type_id);
		let dest = self.env.get_temp();
		self.load(basic_type, ptr, dest.as_str())
	}

	fn llvm_compile_register(&mut self, name: &str) -> BasicValueEnum<'ll> {
		if let Some(ptr) = self.env.get_ptr_value(name) {
			// we don't load the value, just return the pointer
			return ptr.into();
		}
		if let Some(value) = self.env.get_value(name) {
			return *value;
		}
		throw_llvm_error(format!("register not found: {:?}", name));
	}
}
