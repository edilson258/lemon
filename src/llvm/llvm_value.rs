use inkwell::values::BasicValueEnum;

use crate::ir::{self};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn lemon_value_to_llvm(&self, value: &ir::IrValue) -> BasicValueEnum<'ll> {
		match value {
			ir::IrValue::Int(int) => self.ctx.i32_type().const_int(*int as u64, false).into(),
			ir::IrValue::Float(float) => self.ctx.f64_type().const_float(*float).into(),
			ir::IrValue::Bool(bool) => {
				self.ctx.bool_type().const_int(if *bool { 1 } else { 0 }, false).into()
			}
			ir::IrValue::Char(char) => self.ctx.i8_type().const_int(*char as u64, false).into(),
			ir::IrValue::String(string) => todo!(),
			_ => todo!(),
		}
	}
}
