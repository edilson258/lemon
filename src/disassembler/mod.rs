use crate::{checker::types::TypeStore, ir};
mod block;
mod function;
mod instr;
mod ir_struct;
mod value;

pub struct Disassembler<'ir> {
	pub type_store: &'ir TypeStore,
}

impl<'ir> Disassembler<'ir> {
	pub fn new(type_store: &'ir TypeStore) -> Self {
		Self { type_store }
	}

	pub fn disassemble_program(&self, program: &'ir ir::IR, output: &mut String) {
		program.structs.iter().for_each(|struct_def| {
			self.disassemble_struct(struct_def, output);
			output.push('\n');
		});

		program.functions.iter().for_each(|func| {
			self.disassemble_function(func, output);
			output.push('\n');
		});
	}
}
