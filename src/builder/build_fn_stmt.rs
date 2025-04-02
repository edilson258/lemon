use crate::{ast, ir};

use super::Builder;

impl Builder<'_> {
	pub fn build_fn_stmt(&mut self, fn_stmt: &mut ast::FnStmt) {
		let range = fn_stmt.get_range();
		let ret_type = self.lookup_event_type(range);
		self.ctx.push_function_scope(ret_type);
		let name = fn_stmt.name.lexeme().to_owned();
		let args: Vec<_> = fn_stmt.params.iter_mut().map(|arg| self.build_bind(arg)).collect();
		let ret = self.lookup_event_type(range);
		let comptime = false;
		let func = ir::Function::new(name, comptime, args, ret);
		self.build_fn_body(&mut fn_stmt.body);
		self.ctx.pop_scope();
		self.push_function_with_blocks(func);
	}

	pub fn build_bind(&mut self, bind: &mut ast::Binding) -> ir::IrBasicValue {
		let lexeme = bind.lexeme().to_owned();
		let kind = self.lookup_event_type(bind.get_range());
		let value = self.ctx.create_register(kind);
		// fix: remove this
		//
		if !kind.is_str() {
			self.ctx.mark_skip_loading(value.get_value().as_str());
		}

		self.ctx.define_local_variable(lexeme, value.clone());
		value
	}

	pub fn build_fn_body(&mut self, body: &mut ast::FnBody) {
		if let ast::FnBody::Expr(expr) = body {
			let ret_value = self.build_expr(expr);
			let result = self.ctx.current_block.append_instr(ir::Instr::Ret(Some(ret_value)));
			result.unwrap_or_else(|message| {
				message.mod_id(self.mod_id_unchecked()).range(expr.get_range()).report(self.loader)
			});
			if !self.ctx.current_block.has_returned {
				self.drop_local_function_values(None);
			}
		}
		if let ast::FnBody::Block(block) = body {
			for stmt in block.stmts.iter_mut() {
				self.build_stmt(stmt);
			}
			if !self.ctx.current_block.has_returned {
				self.drop_local_function_values(None);
			}
		}
	}
}
