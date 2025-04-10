use std::ffi::CString;

use inkwell::values::{BasicValueEnum, UnnamedAddress};

use crate::{
	error_codegen,
	ir::{self},
};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn get_basic_value(&mut self, value: &ir::IrValue) -> BasicValueEnum<'ll> {
		match value {
			ir::IrValue::Int(int) => self.ctx.i32_type().const_int(*int as u64, false).into(),
			ir::IrValue::Float(float) => self.ctx.f32_type().const_float(*float).into(),
			ir::IrValue::Char(char) => self.ctx.i8_type().const_int(*char as u64, false).into(),
			ir::IrValue::Reg(reg) => *self.stack.get_value(*reg),
			ir::IrValue::String(value) => self.create_string_value(value),
			ir::IrValue::Bool(bool) => {
				let bool_value = if *bool { 1 } else { 0 } as u64;
				self.ctx.bool_type().const_int(bool_value, false).into()
			}
			_ => todo!("value {:?}", value),
		}
	}

	// pub fn create_string_value(&self, value: &str) -> BasicValueEnum<'ll> {
	// 	let c_string = match CString::new(value) {
	// 		Err(_) => throw_llvm_error("transform to c_string"),
	// 		Ok(str) => str,
	// 	};
	// 	let string_value = self.ctx.const_string(c_string.as_bytes_with_nul(), false);
	// 	string_value.into()
	// }

	pub fn create_string_value(&mut self, value: &str) -> BasicValueEnum<'ll> {
		let c_string = match CString::new(value) {
			Err(_) => throw_llvm_error("faild to convert to c string"),
			Ok(cstr) => cstr,
		};

		let global_name = format!(".r_s{}", self.stack.get_gloabl_count());
		let global = self.module.add_global(
			self.ctx.i8_type().array_type(c_string.to_bytes_with_nul().len() as u32),
			None,
			&global_name,
		);
		global.set_initializer(&self.ctx.const_string(c_string.as_bytes_with_nul(), false));
		global.set_constant(true);
		global.set_unnamed_address(UnnamedAddress::Global);

		global.as_pointer_value().into()
	}
}
