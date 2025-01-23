#![allow(unused_imports, dead_code, unused_variables)]
use std::mem;

use super::ir::{self};
use crate::{
	ast,
	checker::types::{self, TypeId},
	report::throw_ir_build_error,
};

use ircontext::IrContext;
mod build_assign_expr;
mod build_binary_expr;
mod build_block_stmt;
mod build_borrow_expr;
mod build_call_expr;
mod build_const_del_stmt;
mod build_const_fn_stmt;
mod build_deref_expr;
mod build_expr;
mod build_extern_fn;
mod build_fn_stmt;
mod build_for_stmt;
mod build_ident_expr;
mod build_if_expr;
mod build_let_stmt;
mod build_literal;
mod build_ret_stmt;
mod build_type_def_stmt;
mod build_while_stmt;
pub mod ircontext;

pub struct Builder<'br> {
	pub type_store: &'br types::TypeStore,
	pub ir_ctx: IrContext,
	pub root: ir::Root,
}

impl<'br> Builder<'br> {
	pub fn new(type_store: &'br types::TypeStore) -> Self {
		let ir_ctx = IrContext::new();
		Self { ir_ctx, root: ir::Root::new(), type_store }
	}

	pub fn add_fn(&mut self, fn_ir: ir::Fn) {
		self.root.add_fn(fn_ir);
	}

	pub fn add_struct(&mut self, struct_ir: ir::StructInstr) {
		self.root.add_struct(struct_ir);
	}

	pub fn add_blocks(&mut self, blocks: Vec<ir::Block>) {
		self.root.add_blocks(blocks);
	}

	pub fn exit_fn_scope(&mut self) {}

	pub fn build(&mut self, program: &mut ast::Program) -> ir::Root {
		for stmt in program.stmts.iter_mut() {
			self.build_stmt(stmt);
		}
		mem::take(&mut self.root)
	}

	pub fn get_type_id(&self, id: Option<TypeId>) -> TypeId {
		match id {
			Some(id) => id,
			None => throw_ir_build_error("type_id not found"),
		}
	}

	fn end_fn_scope(&mut self) {
		let blocks = self.ir_ctx.reset_fn_scope();
		self.root.add_blocks(blocks);
		self.ir_ctx.exit_scope();
		self.ir_ctx.set_ret_type(None)
	}

	fn build_stmt(&mut self, stmt: &ast::Stmt) {
		match stmt {
			ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.build_fn_stmt(fn_stmt),
			ast::Stmt::Block(block_stmt) => self.build_block_stmt(block_stmt),
			ast::Stmt::While(while_stmt) => self.build_while_stmt(while_stmt),
			ast::Stmt::For(for_stmt) => self.build_for_stmt(for_stmt),
			ast::Stmt::ConstDel(const_del) => self.build_const_del_stmt(const_del),
			ast::Stmt::ConstFn(const_fn) => self.build_const_fn_stmt(const_fn),
			ast::Stmt::Ret(ret_stmt) => self.build_ret_stmt(ret_stmt),
			ast::Stmt::ExternFn(extern_fn) => self.build_extern_fn(extern_fn),
			ast::Stmt::TypeDef(type_def) => self.build_type_def_stmt(type_def),
			ast::Stmt::Expr(expr) => {
				self.build_expr(expr);
			}
			_ => todo!("code {:?}", stmt),
		}
	}
}
