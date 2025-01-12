use inkwell::values::FunctionValue;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn declare_write(&mut self) -> FunctionValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let i8_ptr_type = self.ctx.i8_type();
		let params = [
			i32_type.into(),    // file descriptor
			i8_ptr_type.into(), // buffer
			i32_type.into(),    // buffer length
		];
		let write_type = i32_type.fn_type(&params, false);
		match self.module.get_function("write") {
			Some(fun) => fun,
			None => self.module.add_function("write", write_type, None),
		}
	}

	pub fn declare_read(&mut self) -> FunctionValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let i8_ptr_type = self.ctx.i8_type();
		let params = [
			i32_type.into(),    // file descriptor
			i8_ptr_type.into(), // buffer
			i32_type.into(),    // buffer length
		];
		let read_type = i32_type.fn_type(&params, false);
		match self.module.get_function("read") {
			Some(fun) => fun,
			None => self.module.add_function("read", read_type, None),
		}
	}

	pub fn declare_exit(&mut self) -> FunctionValue<'ll> {
		let i32_type = self.ctx.i32_type();
		let params = [i32_type.into()];
		let exit_type = i32_type.fn_type(&params, false);
		match self.module.get_function("exit") {
			Some(fun) => fun,
			None => self.module.add_function("exit", exit_type, None),
		}
	}
}
