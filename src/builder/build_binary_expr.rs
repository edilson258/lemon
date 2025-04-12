use crate::{
	ast::{self, OperatorKind},
	ir::{self, BinInstr, IrBasicValue},
	range::Range,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_binary_expr(&mut self, binary_expr: &mut ast::BinaryExpr) -> IrBasicValue {
		let range = binary_expr.get_range();
		let lhs = self.build_expr(&mut binary_expr.left);
		let rhs = self.build_expr(&mut binary_expr.right);

		let type_id = self.lookup_event_type(range);
		let operator_id = self.lookup_event_type(binary_expr.operator.get_range());
		let dest = self.ctx.create_register(operator_id);
		let alloc_instr = ir::SallocInstr::new(dest.clone(), operator_id);
		let result = self.ctx.current_block.append_instr(alloc_instr.into());
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
		});
		let lhs = self.resolve_value(lhs, range).with_new_type(type_id);
		let rhs = self.resolve_value(rhs, range).with_new_type(type_id);

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
		let result = self.ctx.current_block.append_instr(instr);
		result.unwrap_or_else(|message| {
			message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
		});
		self.resolve_value(dest, range)
	}
	pub fn resolve_value(&mut self, value: IrBasicValue, range: Range) -> IrBasicValue {
		if value.is_register() && !self.ctx.should_skip_loading(value.value.as_str()) {
			let register = self.ctx.create_register(value.get_type());
			let instr = ir::UnInstr::new(register.clone(), value.clone());
			let result = self.ctx.current_block.append_instr(ir::Instr::Load(instr));
			result.unwrap_or_else(|message| {
				message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader)
			});
			// don't load the value again
			self.ctx.mark_skip_loading(register.value.as_str());
			return register;
		}
		value
	}
}
