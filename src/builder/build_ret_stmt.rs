use crate::builder::Builder;
use crate::report::throw_ir_build_error;
use crate::{ast, ir};

impl Builder<'_> {
	pub fn build_ret_stmt(&mut self, ret_stmt: &mut ast::RetStmt) {
		self.ctx.block.as_returned();
		if let Some(expr) = &mut ret_stmt.expr {
			let mut ret_value = self.build_expr(expr);
			let ret_type = self.ctx.get_ret_type().unwrap_or_else(|| {
				throw_ir_build_error("ret type not found");
			});
			let ret_value = ret_value.with_new_type(ret_type);
			let ret_value = self.resolve_value(ret_value);

			if ret_value.is_register() {
				self.drop_local_function_values(Some(ret_value.value.as_str()));
			}

			let instr = ir::Instr::Ret(Some(ret_value));
			self.ctx.block.add_instr(instr);
			return;
		}
		self.drop_local_function_values(None);
		let instr = ir::Instr::Ret(None);
		self.ctx.block.add_instr(instr);
	}
}
