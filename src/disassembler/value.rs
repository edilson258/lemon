use crate::ir::{self};

use super::Disassembler;

impl<'ir> Disassembler<'ir> {
	pub fn disassemble_value(&self, ir_value: &'ir ir::IrValue) -> String {
		// todo: dispaly ir_value  type
		let type_text = self.type_store.get_display_ir_type(ir_value.kind);
		format!("{} {}", type_text, ir_value.value)
	}

	pub fn disassemble_value_size(&self, ir_value: &'ir ir::IrValue) -> String {
		ir_value.value.to_string()
	}
}
