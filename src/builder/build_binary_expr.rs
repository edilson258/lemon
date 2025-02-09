use crate::{
	ast::{self, OperatorKind},
	ir::{self, BinInstr, IrBasicValue},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_binary_expr(&mut self, binary_expr: &mut ast::BinaryExpr) -> IrBasicValue {
		let lhs = self.build_expr(&mut binary_expr.left);
		let rhs = self.build_expr(&mut binary_expr.right);

		let type_id = self.build_type(binary_expr.get_type_id(), binary_expr.get_range());

		let dest = self.ctx.new_register(type_id);

		let instr = BinInstr::new(dest.clone(), lhs, rhs);

		let instr = match binary_expr.operator.kind {
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
			OperatorKind::AND => ir::Instr::And(instr),
			OperatorKind::OR => ir::Instr::Or(instr),
			OperatorKind::SHL => ir::Instr::Shl(instr),
			OperatorKind::SHR => ir::Instr::Shr(instr),
			_ => todo!("code {:?}", binary_expr.operator.kind),
		};
		self.ctx.block.add_instr(instr);

		dest
	}
}
