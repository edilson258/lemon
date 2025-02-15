use crate::builder::Builder;
use crate::report::throw_ir_build_error;
use crate::{ast, ir};

impl Builder<'_> {
	pub fn build_ret_stmt(&mut self, ret_stmt: &mut ast::RetStmt) {
		self.ctx.block.as_returned();
		if let Some(expr) = &mut ret_stmt.expr {
			let mut ret = self.build_expr(expr);
			let ret_type = *self.ctx.get_ret_type().unwrap_or_else(|| {
				throw_ir_build_error("ret type not found");
			});
			let ret = ret.with_new_type(ret_type);
			let ret = self.resolve_value(ret);
			let instr = ir::Instr::Ret(Some(ret));
			self.ctx.block.add_instr(instr);
			return;
		}
		let instr = ir::Instr::Ret(None);
		self.ctx.block.add_instr(instr);
	}
}
