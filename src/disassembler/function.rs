use crate::ir;

use super::Disassembler;

impl<'ir> Disassembler<'ir> {
	pub fn disassemble_function(&self, function: &'ir ir::Function, output: &mut String) {
		if function.is_extern_function() {
			output.push_str("extern ");
		}
		output.push_str(&format!("fn {}(", function.name));
		for (arg_position, arg) in function.args.iter().enumerate() {
			self.disassemble_args(arg, output);
			if arg_position != function.args.len() - 1 {
				output.push_str(", ");
			}
		}

		if function.is_variadic_args() {
			output.push_str("...");
		}

		let type_name = self.type_store.lookup_display_ir_type(function.ret);
		output.push_str(&format!("): {} = ", type_name));

		if function.is_extern_function() {
			output.push_str("{}\n");
			return;
		}

		output.push_str("{\n");
		for (index, block) in function.blocks.iter().enumerate() {
			self.disassemble_block(block, output);
			if index != function.blocks.len() - 1 {
				output.push('\n');
			}
		}

		output.push_str("\n}\n");
	}

	pub fn disassemble_args(&self, bind: &'ir ir::IrBasicValue, output: &mut String) {
		output.push_str(bind.value.as_str());
		let type_name = self.type_store.lookup_display_ir_type(bind.get_type());
		output.push_str(&format!(": {}", type_name));
	}
}
