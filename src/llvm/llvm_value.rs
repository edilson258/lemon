use inkwell::values::BasicValueEnum;

use crate::ir::{self};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn ln_value_to_llvm(&self, value: &ir::IrValue) -> BasicValueEnum<'ll> {
		match value {
			ir::IrValue::Int(int) => self.ctx.i32_type().const_int(*int as u64, false).into(),
			ir::IrValue::Float(float) => self.ctx.f64_type().const_float(*float).into(),
			ir::IrValue::Bool(bool) => {
				let bool_value = if *bool { 1 } else { 0 } as u64;
				self.ctx.bool_type().const_int(bool_value, false).into()
			}
			ir::IrValue::Char(char) => self.ctx.i8_type().const_int(*char as u64, false).into(),
			ir::IrValue::String(_) => todo!(),
			_ => todo!(),
		}
	}
}
