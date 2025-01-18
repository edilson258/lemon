use crate::{
	ast,
	ir::ir::{self},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_while_stmt(&mut self, while_stmt: &ast::WhileStmt) {
		let test_label = self.ir_ctx.new_block();
		let body_label = self.ir_ctx.new_block();
		let exit_label = body_label.next_block(); // get next but not push to stack

		self.ir_ctx.add_instr(ir::GotoInstr { block_id: test_label }.into());

		self.ir_ctx.switch_to_block(test_label);

		let cond = self.build_expr(&while_stmt.test);
		let jmp_instr = ir::JmpIfInstr { cond, l0: body_label, l1: exit_label };
		self.ir_ctx.add_instr(jmp_instr.into());

		self.ir_ctx.switch_to_block(body_label);
		self.build_loop_body_stmt(&while_stmt.body);

		self.ir_ctx.add_instr(ir::GotoInstr { block_id: test_label }.into());

		let exit_label = self.ir_ctx.new_block(); // push to stack a exit label
		self.ir_ctx.switch_to_block(exit_label);
	}

	pub fn build_loop_body_stmt(&mut self, stmt: &ast::Stmt) {
		match stmt {
			ast::Stmt::Block(block) => block.stmts.iter().for_each(|stmt| self.build_stmt(stmt)),
			_ => self.build_stmt(stmt),
		}
	}
}
