use crate::{
	ast,
	ir::ir::{self, Value},
};

use super::Builder;

impl Builder {
	pub fn build_if_expr(&mut self, expr: &ast::IfExpr) -> Value {
		let cond = self.build_expr(&expr.cond).get_register().unwrap();
		let then_b_id = self.ctx.create_block();
		let merge_b_id = self.ctx.create_block();

		let other_b_id = if expr.otherwise.is_some() { self.ctx.create_block() } else { merge_b_id };

		let instr = ir::JmpIfInstr { cond, l0: then_b_id, l1: other_b_id };

		self.add_instr(ir::Instr::JmpIf(instr));

		self.ctx.switch_to_block(then_b_id);
		self.build_stmt(&expr.then);

		if let Some(otherwise) = &expr.otherwise {
			if !otherwise.ends_with_ret() {
				let goto_instr = ir::GotoInstr { block_id: merge_b_id };
				self.add_instr(ir::Instr::Goto(goto_instr));
			}
			self.ctx.switch_to_block(other_b_id);
			self.build_stmt(otherwise);
		};
		self.ctx.switch_to_block(merge_b_id);
		// todo: we dont need to return here... maybe move if expr to stmt?
		Value::Null
	}
}
