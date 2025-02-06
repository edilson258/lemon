use super::Disassembler;
use crate::ir;

impl<'ir> Disassembler<'ir> {
	pub fn disassemble_block(&self, block: &'ir ir::IrBlock, output: &mut String) {
		let block_name = block.format_label();
		let block_fmt = &format!("  {}: ", block_name);
		output.push_str(block_fmt);
		for (index, instr) in block.instrs.iter().enumerate() {
			if index != 0 {
				output.push_str(&format!("{:width$}", "", width = block_fmt.len()));
			}

			self.disassemble_instr(instr, output);

			if index != block.instrs.len() - 1 {
				output.push('\n');
			}
		}
	}
}
