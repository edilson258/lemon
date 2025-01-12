use crate::{ast, checker::types::TypeId, ir::ir};

use super::Builder;

impl Builder<'_> {
	pub fn build_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) {
		let ret_id = self.get_type_id(fn_stmt.get_ret_id());
		let fn_id = ir::FnId::new(fn_stmt.name.lexeme());
		self.ir_ctx.enter_scope();
		let binds = self.build_fn_binds(fn_stmt);
		let fn_native = ir::Fn::new(fn_id, binds, ret_id);
		self.root.add_fn(fn_native);
		self.ir_ctx.add_fn(fn_stmt.name.lexeme());
		self.build_fn_body(&fn_stmt.body, ret_id);
		self.end_fn_scope();
	}

	fn build_fn_binds(&mut self, fn_stmt: &ast::FnStmt) -> Vec<ir::Bind> {
		let mut binds = Vec::with_capacity(fn_stmt.params.len());
		for param in &fn_stmt.params {
			let register = self.ir_ctx.new_register();
			let type_id = self.get_type_id(param.type_id);
			self.ir_ctx.add_value(param.lexeme(), register);
			binds.push(ir::Bind { register, type_id });
		}
		binds
	}

	fn build_fn_body(&mut self, stmt: &ast::FnBody, ret_id: TypeId) {
		match stmt {
			ast::FnBody::Block(block_stmt) => self.build_fn_block_stmt(block_stmt),
			ast::FnBody::Expr(expr) => self.build_ret_expr(expr, ret_id),
		}
	}

	fn build_ret_expr(&mut self, expr: &ast::Expr, ret_id: TypeId) {
		let value = Some(self.build_expr(expr));
		let instr = ir::RetInstr { value, type_id: ret_id };
		self.ir_ctx.add_instr(instr.into());
	}

	fn build_fn_block_stmt(&mut self, block_stmt: &ast::BlockStmt) {
		for stmt in block_stmt.stmts.iter() {
			self.build_stmt(stmt);
		}
	}
}
