use std::mem;

use super::ir::{self};
use crate::{
	ast,
	checker::types::{self, TypeId},
	report::throw_ir_build_error,
};
mod build_binary_expr;
mod build_block_stmt;
mod build_call_expr;
mod build_const_del_stmt;
mod build_const_fn_stmt;
mod build_deref_expr;
mod build_expr;
mod build_fn_stmt;
mod build_ident_expr;
mod build_if_expr;
mod build_import_expr;
mod build_let_stmt;
mod build_literal;
mod build_ref_expr;
mod build_ret_stmt;
pub(crate) mod context;
mod drop_values;

pub struct Builder<'br> {
	#[allow(dead_code)]
	pub type_store: &'br types::TypeStore,
	pub ctx: context::Context,
	pub root: ir::Root,
}

impl<'br> Builder<'br> {
	pub fn new(type_store: &'br types::TypeStore) -> Self {
		let ctx = context::Context::new();
		Self { ctx, root: ir::Root::new(), type_store }
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
		if self.ctx.is_fn_comptime() {
			self.root.add_fn_global(fn_ir);
		} else {
			self.root.add_fn(fn_ir);
		}
	}

	pub fn add_blocks(&mut self, blocks: Vec<ir::Block>) {
		if self.ctx.is_fn_comptime() {
			self.root.add_global_blocks(blocks);
		} else {
			self.root.add_blocks(blocks);
		}
	}

	pub fn exit_fn_scope(&mut self) {
		self.drop_values();
		let blocks = self.ctx.exit_fn_scope();
		self.add_blocks(blocks);
	}

	pub fn build(&mut self, program: &ast::Program) -> ir::Root {
		for stmt in program.stmts.iter() {
			self.build_stmt(stmt);
		}
		self.root.set_size(self.ctx.register);
		mem::take(&mut self.root)
	}

	pub fn get_type_id(&self, id: Option<TypeId>) -> TypeId {
		match id {
			Some(id) => id,
			None => throw_ir_build_error("type_id not found"),
		}
	}

	fn build_stmt(&mut self, stmt: &ast::Stmt) {
		match stmt {
			ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.build_fn_stmt(fn_stmt),
			ast::Stmt::Block(block_stmt) => self.build_block_stmt(block_stmt),
			ast::Stmt::Expr(expr) => {
				self.build_expr(expr);
			}
			ast::Stmt::ConstDel(const_del) => self.build_const_del_stmt(const_del),
			ast::Stmt::ConstFn(const_fn) => self.build_const_fn_stmt(const_fn),
			ast::Stmt::Ret(ret_stmt) => self.build_ret_stmt(ret_stmt),
		}
	}
}
