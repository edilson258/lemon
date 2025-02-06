use crate::ir;

use super::Disassembler;

impl<'ir> Disassembler<'ir> {
	pub fn disassemble_function(&self, func: &'ir ir::Function, output: &mut String) {
		output.push_str(&format!("fn {}(", func.name));
		for arg in &func.args {
			self.disassemble_bind(arg, output);
			output.push_str(", ");
		}

		let type_name = self.type_store.get_display_type(func.ret);
		output.push_str(&format!("): {} = ", type_name));

		output.push_str("{\n");
		for block in &func.blocks {
			self.disassemble_block(block, output);
		}

		output.push_str("\n}\n");
	}

	pub fn disassemble_bind(&self, bind: &'ir ir::IrBind, output: &mut String) {
		output.push_str(bind.name.as_str());
		let type_name = self.type_store.get_display_type(bind.kind);
		output.push_str(&format!(": {}", type_name));
	}
}
