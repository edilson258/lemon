use crate::{ast, ir::ir};

use super::Builder;

impl Builder {
	pub fn build_const_fn_stmt(&mut self, const_fn: &ast::ConstFnStmt) {
		self.ctx.enter_fn_comptime();
		let lexeme = const_fn.name.lexeme();
		let ret = const_fn.get_ret_id().unwrap();
		let fn_id = ir::FnId::new(lexeme);
		self.ctx.enter_scope();
		let params = self.build_fn_params(&const_fn.params);
		self.ctx.add_fn(lexeme);
		let fn_native = ir::FnNative::new(fn_id, params, ret);
		self.add_fn(ir::Fn::Native(fn_native));
		self.build_fn_body(&const_fn.body);
		self.exit_fn_scope();
		self.ctx.exit_fn_comptime();
	}
}
