use crate::{ast, ir::ir};

use super::Builder;

impl Builder {
	pub fn build_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) {
		let lexeme = fn_stmt.name.lexeme();
		let ret = fn_stmt.type_id.unwrap();
		let fn_id = ir::FnId::new(lexeme);
		self.ctx.enter_scope();
		let params = self.build_fn_params(&fn_stmt.params);
		self.ctx.add_fn(lexeme);
		let fn_native = ir::FnNative::new(fn_id, params, ret);
		self.add_fn(ir::Fn::Native(fn_native));
		self.build_fn_body(&fn_stmt.body);
		self.exit_fn_scope();
	}

	fn build_fn_body(&mut self, stmt: &ast::Stmt) {
		match stmt {
			ast::Stmt::Block(block_stmt) => {
				// wee don't need to add block to context
				for stmt in block_stmt.stmts.iter() {
					self.build_stmt(stmt);
				}
			}
			_ => self.build_stmt(stmt),
		}
	}

	fn build_fn_params(&mut self, params: &[ast::Binding]) -> Vec<ir::Bind> {
		let mut binds = Vec::with_capacity(params.len());
		for param in params {
			let bind = self.build_binding(param);
			binds.push(bind);
		}
		binds
	}
}
