use crate::{
	ast::{self, OperatorKind},
	checker::types::TypeId,
	ir::{ir, BinaryInstr, IrValue, Register},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_binary_expr(&mut self, binary: &ast::BinaryExpr) -> Register {
		let lhs = self.build_expr(&binary.left);
		let rhs = self.build_expr(&binary.right);
		let dest = self.ir_ctx.new_register();
		let type_id = self.get_type_id(binary.get_type_id());
		let instr = BinaryInstr::new(type_id, lhs, rhs, dest);
		let instr = match binary.operator.kind {
			OperatorKind::ADD => ir::Instr::Add(instr),
			OperatorKind::SUB => ir::Instr::Sub(instr),
			OperatorKind::MUL => ir::Instr::Mul(instr),
			OperatorKind::DIV => ir::Instr::Div(instr),
			OperatorKind::MOD => ir::Instr::Mod(instr),
			OperatorKind::RANGE => ir::Instr::CmpGt(instr),
			OperatorKind::EQ => ir::Instr::CmpEq(instr),
			OperatorKind::NOTEQ => ir::Instr::CmpLt(instr),
			OperatorKind::LE => ir::Instr::CmpLe(instr),
			OperatorKind::GE => ir::Instr::CmpGe(instr),
			OperatorKind::LT => ir::Instr::CmpLt(instr),
			OperatorKind::GT => ir::Instr::CmpGt(instr),
			// OperatorKind::AND => ir::Instr::And(instr),
			// OperatorKind::OR => ir::Instr::Or(instr),
			// OperatorKind::XOR => ir::Instr::Xor(instr),
			// OperatorKind::BOR => ir::Instr::Bor(instr),
			// OperatorKind::SHL => ir::Instr::Shl(instr),
			// OperatorKind::SHR => ir::Instr::Shr(instr),
			// OperatorKind::POW => ir::Instr::Pow(instr),
			// OperatorKind::PIPE => ir::Instr::Pipe(instr),
			_ => todo!("code {:?}", binary.operator.kind),
		};
		self.ir_ctx.add_instr(instr);
		dest
	}
}
