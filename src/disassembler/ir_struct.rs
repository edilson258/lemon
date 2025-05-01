use crate::ir;

use super::Disassembler;

impl Disassembler<'_> {
	pub fn disassemble_struct(&self, ir_struct: &ir::Struct, output: &mut String) {
		let struct_name = ir_struct.name.clone().unwrap_or("Unknown".into());

		if ir_struct.fields.len() <= 5 {
			output.push_str(&format!("type {} = {{ ", struct_name));
			for (index, field) in ir_struct.fields.iter().enumerate() {
				let type_text = self.type_store.lookup_display_ir_type(*field);
				output.push_str(&type_text);
				if index < ir_struct.fields.len() - 1 {
					output.push_str(", ");
				}
			}
			output.push_str(&format!(" }}, size={}\n", ir_struct.size));
		} else {
			let max_wrapper = 3;
			output.push_str(&format!("type {} = {{\n", struct_name));
			for (index, field) in ir_struct.fields.iter().enumerate() {
				let type_text = self.type_store.lookup_display_ir_type(*field);
				output.push_str(&type_text);
				if index < ir_struct.fields.len() - 1 {
					output.push_str(", ");
				}
				// add new line between 3 fields
				if index % max_wrapper == max_wrapper - 1 && index != ir_struct.fields.len() - 1 {
					output.push('\n');
				}
			}
			output.push_str(&format!("\n}}, size={}\n", ir_struct.size));
		}
	}
}
