use inkwell::values::{BasicValueEnum, UnnamedAddress};

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn llvm_compile_string(&mut self, value: &str) -> BasicValueEnum<'ll> {
		let global_name = format!(".gstr_{}", self.env.get_temp());
		let string_type = self.ctx.i8_type().array_type(value.len() as u32);

		let global = self.module.add_global(string_type, None, &global_name);

		global.set_initializer(&self.ctx.const_string(value.as_bytes(), true));
		global.set_constant(true);
		global.set_unnamed_address(UnnamedAddress::Global);

		global.as_pointer_value().into()
	}
}
