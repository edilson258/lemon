use crate::{
	ir::{self},
	throw_error,
};

use super::Disassembler;

impl<'ir> Disassembler<'ir> {
	pub fn disassemble_value(&self, ir_value: &'ir ir::IrBasicValue) -> String {
		let basic_type = self.type_store.lookup_display_ir_type(ir_value.type_id);
		let value = self.disassemble_basic_value(ir_value);
		format!("{} {}", basic_type, value)
	}

	pub fn disassemble_basic_value(&self, ir_value: &'ir ir::IrBasicValue) -> String {
		match &ir_value.value {
			// .replace('\t', "\\t").replace('\r', "\\r")
			ir::BasicValue::String(value) => format!("\"{}\"", value.replace('\n', "\\n")),
			ir::BasicValue::Register(value) => value.to_string(),
			ir::BasicValue::Int(value) => format!("{}", value),
			ir::BasicValue::Float(value) => format!("{}", value),
			ir::BasicValue::Char(value) => format!("'{}'", value),
			ir::BasicValue::Bool(value) => format!("{}", value),
			ir::BasicValue::None => {
				throw_error!("internal 'none' found in ir, please report bug.")
			}
		}
	}
}
