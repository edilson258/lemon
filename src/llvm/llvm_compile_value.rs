use inkwell::values::BasicValueEnum;

use crate::{checker::types::TypeId, error_codegen, ir};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_value(&mut self, ir_basic_value: &ir::IrBasicValue) -> BasicValueEnum<'ll> {
		match &ir_basic_value.value {
			ir::BasicValue::Int(value) => self.llvm_compile_int(ir_basic_value.get_type(), *value),
			ir::BasicValue::Float(value) => self.llvm_compile_float(ir_basic_value.get_type(), *value),
			ir::BasicValue::Bool(value) => self.ctx.bool_type().const_int(*value as u64, false).into(),
			ir::BasicValue::String(value) => self.llvm_compile_string(value),
			ir::BasicValue::Char(_) => error_codegen!("unsupported char value").report(self.loader),
			ir::BasicValue::None => error_codegen!("unsupported none value").report(self.loader),
			ir::BasicValue::Register(name) => self.llvm_compile_register(name),
			// ir::BasicValue::String(value) => self.ctx.const_string(value.as_bytes(), true).into(),
		}
	}

	fn llvm_compile_int(&mut self, type_id: TypeId, value: u64) -> BasicValueEnum<'ll> {
		match type_id {
			TypeId::I8 => self.ctx.i8_type().const_int(value, false).into(),
			TypeId::U8 => self.ctx.i8_type().const_int(value, true).into(),

			TypeId::I16 => self.ctx.i16_type().const_int(value, false).into(),
			TypeId::U16 => self.ctx.i16_type().const_int(value, true).into(),

			TypeId::I32 => self.ctx.i32_type().const_int(value, false).into(),
			TypeId::U32 => self.ctx.i32_type().const_int(value, true).into(),

			TypeId::I64 => self.ctx.i64_type().const_int(value, false).into(),
			TypeId::U64 => self.ctx.i64_type().const_int(value, true).into(),
			_ => {
				let display = self.type_store.get_display_type(type_id);
				error_codegen!("expected integer type, found '{}'", display).report(self.loader)
			}
		}
	}

	fn llvm_compile_float(&mut self, type_id: TypeId, value: f64) -> BasicValueEnum<'ll> {
		match type_id {
			TypeId::F32 => self.ctx.f32_type().const_float(value).into(),
			TypeId::F64 => self.ctx.f64_type().const_float(value).into(),
			_ => {
				let display = self.type_store.get_display_type(type_id);
				error_codegen!("expected float type, found '{}'", display).report(self.loader)
			}
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
		error_codegen!("register not found '{}'", name).report(self.loader);
	}
}
