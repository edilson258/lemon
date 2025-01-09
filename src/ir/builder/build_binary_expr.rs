use crate::{
	ast,
	ir::ir::{self, Value},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_binary_expr(&mut self, binary: &ast::BinaryExpr) -> Value {
		let lhs = self.build_expr(&binary.left).get_register().unwrap();
		let rhs = self.build_expr(&binary.right).get_register().unwrap();
		let dest = self.ctx.get_register();
		let type_id = self.get_type_id(binary.get_type_id());
		let binary_instr = ir::BinaryInstr::new(type_id, lhs, rhs, dest);
		// let instr = match binary.operator {
		// 	ast::Operator::ADD => ir::Instr::Add(binary_instr),
		// 	ast::Operator::SUB => ir::Instr::Sub(binary_instr),
		// 	ast::Operator::MUL => ir::Instr::Mul(binary_instr),
		// 	ast::Operator::DIV => ir::Instr::Div(binary_instr),
		// 	ast::Operator::MOD => ir::Instr::Mod(binary_instr),
		// 	ast::Operator::RANGE => ir::Instr::CmpGt(binary_instr),
		// 	ast::Operator::EQ => ir::Instr::CmpEq(binary_instr),
		// 	ast::Operator::NOTEQ => ir::Instr::CmpLt(binary_instr),
		// 	ast::Operator::LE => ir::Instr::CmpLe(binary_instr),
		// 	ast::Operator::GE => ir::Instr::CmpGe(binary_instr),
		// 	ast::Operator::LT => ir::Instr::CmpLt(binary_instr),
		// 	ast::Operator::GT => ir::Instr::CmpGt(binary_instr),
		// 	_ => todo!(),
		// };
		// self.add_instr(instr);
		Value::new_register(dest)
	}
}
