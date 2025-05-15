use crate::builder::Builder;
use crate::{ast, ir, throw_error};

impl Builder<'_> {
	pub fn build_ret_stmt(&mut self, r: &mut ast::RetStmt) {
		self.ctx.current_block.mark_as_returned();
		if let Some(expr) = &mut r.expr {
			let mut ret_value = self.build_expr(expr);
			let ret_type = self.ctx.function_return_type().unwrap_or_else(|| {
				throw_error!("ret type not found");
			});
			let ret_value = ret_value.with_new_type(ret_type);
			let ret_value = self.ensure_loaded(ret_value, expr.get_range());

			if ret_value.is_register() {
				self.drop_local_function_values(Some(ret_value.value.as_str()));
			}

			let instr = ir::Instr::Ret(Some(ret_value));
			self.append_instr(instr, Some(r.get_range()));
			// self.drop_local_function_values(None);
			// let instr = ir::Instr::Ret(None);
			// self.append_instr(instr, Some(r.get_range()));
		}
	}
}
