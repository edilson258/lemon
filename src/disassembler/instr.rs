use crate::ir::{self, BinInstr, UnInstr};

use super::Disassembler;

impl<'ir> Disassembler<'ir> {
	pub fn disassemble_instr(&self, instr: &'ir ir::Instr, output: &mut String) {
		match instr {
			ir::Instr::Add(instr) => self.disassemble_bin_inst("add", instr, output),
			ir::Instr::Sub(instr) => self.disassemble_bin_inst("sub", instr, output),
			ir::Instr::Mul(instr) => self.disassemble_bin_inst("mul", instr, output),
			ir::Instr::Div(instr) => self.disassemble_bin_inst("div", instr, output),
			ir::Instr::Mod(instr) => self.disassemble_bin_inst("mod", instr, output),

			ir::Instr::CmpEq(instr) => self.disassemble_bin_inst("cmp_eq", instr, output),
			ir::Instr::CmpNe(instr) => self.disassemble_bin_inst("cmp_ne", instr, output),
			ir::Instr::CmpLt(instr) => self.disassemble_bin_inst("cmp_lt", instr, output),
			ir::Instr::CmpGt(instr) => self.disassemble_bin_inst("cmp_gt", instr, output),
			ir::Instr::CmpLe(instr) => self.disassemble_bin_inst("cmp_le", instr, output),
			ir::Instr::CmpGe(instr) => self.disassemble_bin_inst("cmp_ge", instr, output),

			ir::Instr::And(bin_instr) => self.disassemble_bin_inst("and", bin_instr, output),
			ir::Instr::Or(bin_instr) => self.disassemble_bin_inst("or", bin_instr, output),
			ir::Instr::Shl(bin_instr) => self.disassemble_bin_inst("shl", bin_instr, output),
			ir::Instr::Shr(bin_instr) => self.disassemble_bin_inst("shr", bin_instr, output),

			ir::Instr::Not(instr) => self.disassemble_bin_inst("not", instr, output),
			ir::Instr::Neg(instr) => self.disassemble_bin_inst("neg", instr, output),

			ir::Instr::Load(un_instr) => self.disassemble_un_instr("load", un_instr, output),
			ir::Instr::Set(un_instr) => self.disassemble_un_instr("set", un_instr, output),

			ir::Instr::Mov(un_instr) => self.disassemble_mov_instr(un_instr, output),
			ir::Instr::Drop(instr) => self.disassemble_drop_instr(instr, output),
			ir::Instr::Ret(instr) => self.disassemble_ret_instr(instr, output),
			ir::Instr::Jmp(instr) => self.disassemble_jmp_instr(instr, output),
			ir::Instr::JmpIf(instr) => self.disassemble_jmp_if_instr(instr, output),
			ir::Instr::Call(instr) => self.disassemble_call_inst(instr, output),
			ir::Instr::Salloc(instr) => self.disassemble_salloc_instr(instr, output),
			ir::Instr::Halloc(un_instr) => self.disassemble_halloc_instr(un_instr, output),
		}
	}

	pub fn disassemble_salloc_instr(&self, instr: &'ir ir::SallocInstr, output: &mut String) {
		let dest = self.disassemble_basic_value(&instr.dest);
		let size = self.type_store.get_display_ir_type(instr.size);
		output.push_str(&format!("{} = salloc {}", dest, size));
	}
	pub fn disassemble_halloc_instr(&self, instr: &'ir ir::UnInstr, output: &mut String) {
		let dest = self.disassemble_basic_value(&instr.dest);
		let size = self.disassemble_basic_value(&instr.src);
		output.push_str(&format!("{} = halloc {}", dest, size));
	}
	pub fn disassemble_mov_instr(&self, instr: &'ir ir::UnInstr, output: &mut String) {
		let dest = self.disassemble_basic_value(&instr.dest);
		let src = self.disassemble_value(&instr.src);
		output.push_str(&format!("mov {}, {}", dest, src));
	}
	pub fn disassemble_drop_instr(&self, instr: &'ir ir::IrBasicValue, output: &mut String) {
		let dest = self.disassemble_value(instr);
		output.push_str(&format!("drop {}", dest));
	}
	pub fn disassemble_ret_instr(&self, instr: &'ir ir::IrBasicValue, output: &mut String) {
		let dest = self.disassemble_value(instr);
		output.push_str(&format!("ret {}", dest));
	}

	pub fn disassemble_un_instr(&self, name: &'ir str, instr: &'ir UnInstr, output: &mut String) {
		let dest = self.disassemble_basic_value(&instr.dest);
		let src = self.disassemble_value(&instr.src);
		output.push_str(&format!("{} {}, {}", name, dest, src));
	}

	pub fn disassemble_bin_inst(&self, name: &'ir str, instr: &'ir BinInstr, output: &mut String) {
		let dest = self.disassemble_basic_value(&instr.dest);
		let left = self.disassemble_value(&instr.left);
		let right = self.disassemble_value(&instr.right);
		output.push_str(&format!("{} = {} {}, {}", dest, name, left, right));
	}

	pub fn disassemble_jmp_instr(&self, instr: &'ir ir::JmpInstr, output: &mut String) {
		output.push_str(&format!("jmp {}", instr.label));
	}

	pub fn disassemble_jmp_if_instr(&self, instr: &'ir ir::JmpIfInstr, output: &mut String) {
		let cond = self.disassemble_value(&instr.cond);
		output.push_str(&format!("jmp_if {}, {}, {}", cond, instr.true_label, instr.false_label));
	}

	pub fn disassemble_call_inst(&self, instr: &'ir ir::CallInstr, output: &mut String) {
		let mut args = Vec::with_capacity(instr.args.len());
		for arg in &instr.args {
			args.push(self.disassemble_value(arg));
		}
		let dest = self.disassemble_basic_value(&instr.dest);
		let type_name = self.type_store.get_display_ir_type(instr.ret_id);

		let fmt = format!("{} = call {} {}({})", dest, type_name, instr.callee, args.join(", "));
		output.push_str(&fmt);
	}
}
