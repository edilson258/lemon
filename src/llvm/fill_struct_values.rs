use inkwell::values::BasicValueEnum;

use crate::{
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;
type Binds = Vec<(ir::Register, ir::Bind)>;
type Fields<'ll> = Vec<Option<BasicValueEnum<'ll>>>;

impl<'ll> Llvm<'ll> {
	pub fn fill_struct_values(&mut self, name: &str, binds: &Binds, fields: &mut Fields<'ll>) {
		for (ref_register, bind_value) in binds {
			#[rustfmt::skip]
			let position = self.stack.get_struct_field(name, *ref_register).unwrap_or_else(|| {
				let error = format!("field '{}' not found", ref_register.as_string());
				throw_llvm_error(error)
			});

			let value = self.get_value_or_load(bind_value.register, bind_value.type_id);
			fields[position] = Some(value);
		}
	}
}
