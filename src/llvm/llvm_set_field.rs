use super::Llvm;
use crate::ir;

impl Llvm<'_> {
	pub fn llvm_set_field(&mut self, set_field: &ir::SetFieldInstr) {
		let self_value = set_field.self_value;
		let field = set_field.field;
		// let field_type = get_field_instr.field_type;

		let struct_ptr = self.stack.get_ptr_value(self_value);
		let struct_type = self.get_struct_type(self_value);
		let struct_name = self.get_struct_name(&struct_type, self_value);
		let at = self.get_field_index(struct_name.as_str(), field);

		let value = self.get_value_or_load(set_field.value, set_field.value_type);

		self.store_struct_field(struct_type, struct_ptr, value, at);
	}
}
