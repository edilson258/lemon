use super::ir::{self};
use crate::ast;
mod build_binary_expr;
mod build_block_stmt;
mod build_call_expr;
mod build_const_stmt;
mod build_deref_expr;
mod build_expr;
mod build_fn_stmt;
mod build_ident_expr;
mod build_if_expr;
mod build_let_stmt;
mod build_literal;
mod build_ref_expr;
mod build_ret_stmt;
pub(crate) mod context;

pub struct Builder {
	// pub type_store: &'br types::TypeStore,
	pub ctx: context::Context,
	pub root: ir::Root,
}

impl Builder {
	pub fn new() -> Self {
		let ctx = context::Context::new();
		Self { ctx, root: ir::Root::new() }
	}

	pub fn add_global(&mut self, instr: ir::Instr) {
		self.root.add_global(instr);
	}

	pub fn add_instr(&mut self, instr: ir::Instr) {
		if self.ctx.is_comptime() {
			self.root.add_global(instr);
		} else {
			self.ctx.add_instr(instr);
		}
	}

	pub fn add_fn(&mut self, fn_ir: ir::Fn) {
		self.root.fns.push(fn_ir);
	}

	pub fn exit_fn_scope(&mut self) {
		let blocks = self.ctx.exit_fn_scope();
		self.root.add_blocks(blocks);
	}

	pub fn build(&mut self, program: &ast::Program) -> ir::Root {
		for stmt in program.stmts.iter() {
			self.build_stmt(stmt);
		}
		self.root.clone()
	}

	fn build_stmt(&mut self, stmt: &ast::Stmt) {
		match stmt {
			ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.build_fn_stmt(fn_stmt),
			ast::Stmt::Block(block_stmt) => self.build_block_stmt(block_stmt),
			ast::Stmt::Expr(expr) => {
				self.build_expr(expr);
			}
			ast::Stmt::Const(const_stmt) => self.build_const_stmt(const_stmt),
			ast::Stmt::Ret(ret_stmt) => self.build_ret_stmt(ret_stmt),
		}
	}
}

impl Default for Builder {
	fn default() -> Self {
		Self::new()
	}
}
