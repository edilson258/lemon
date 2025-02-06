use crate::{
	ir::{self},
	report::throw_llvm_error,
};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_init_struct(&mut self, instr: &ir::StructInitInstr) {
		let struct_name = instr.struct_id.as_str();

		#[rustfmt::skip]
		let struct_type = self.module.get_struct_type(struct_name).unwrap_or_else(|| {
			throw_llvm_error(format!("struct '{}' not found", struct_name))
		});

		self.stack.set_struct_type(instr.type_id, struct_type);

		let mut values = vec![None; struct_type.count_fields() as usize];
		self.fill_struct_values(struct_name, &instr.binds, &mut values);

		let final_values = values.into_iter().flatten().collect::<Vec<_>>();

		let _ = struct_type.const_named_struct(&final_values);

		let struct_size = self.calculate_struct_size(struct_type);
		let struct_ptr = self.allocate_struct(struct_size, &instr.dest);

		final_values.iter().enumerate().for_each(|(at, value)| {
			self.store_struct_field(struct_type, struct_ptr, *value, at);
		});
		self.stack.set_register_type(instr.dest, struct_type);
		self.stack.set_value(instr.dest, struct_ptr.into());
	}
}
