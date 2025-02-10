use crate::ir::{self};

use super::Disassembler;

impl<'ir> Disassembler<'ir> {
	pub fn disassemble_value(&self, ir_value: &'ir ir::IrBasicValue) -> String {
		let basic_type = self.type_store.get_display_ir_type(ir_value.type_id);
		let value = self.disassemble_basic_value(ir_value);
		format!("{} {}", basic_type, value)
	}

	pub fn disassemble_basic_value(&self, ir_value: &'ir ir::IrBasicValue) -> String {
		match &ir_value.value {
			ir::BasicValue::Register(value) => value.to_string(),
			ir::BasicValue::String(value) => value.to_string(),
			ir::BasicValue::Int(value) => format!("{}", value),
			ir::BasicValue::Float(value) => format!("{}", value),
			ir::BasicValue::Char(value) => format!("{}", value),
			ir::BasicValue::Bool(value) => format!("{}", value),
			ir::BasicValue::None => todo!("none"),
		}
	}
}
