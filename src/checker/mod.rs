#![allow(dead_code, unused_variables)]
use crate::{
	ast,
	diag::{Diag, DiagGroup},
};
use context::Context;
use types::{Type, TypeFormatter, TypeId};
mod synthesis;

// mod check_assign_expr
mod check_binary_expr;
mod check_block_stmt;
mod check_call_expr;
mod check_const_stmt;
mod check_deref_expr;
mod check_expr;
mod check_fn_stmt;
mod check_ident_expr;
mod check_if_expr;
mod check_let_stmt;
mod check_literal;
mod check_number;
mod check_primitive_type;
mod check_ref_expr;
mod check_ret_stmt;
mod check_type;
pub mod context;
mod diags;
mod equal_type;
mod infer;
pub(crate) mod modules;
pub mod types;

type TypeResult<T> = Result<T, Diag>;

pub struct Checker<'ckr> {
	ctx: &'ckr mut Context,
	diag_group: &'ckr mut DiagGroup<'ckr>,
}

impl<'ckr> Checker<'ckr> {
	pub fn new(diag_group: &'ckr mut DiagGroup<'ckr>, ctx: &'ckr mut Context) -> Self {
		Self { ctx, diag_group }
	}

	pub fn check_program(&mut self, ast: &mut ast::Program) -> TypeResult<TypeId> {
		for stmt in ast.stmts.iter_mut() {
			self.check_stmt(stmt)?;
		}
		Ok(TypeId::NOTHING)
	}

	pub(crate) fn check_stmt(&mut self, stmt: &mut ast::Stmt) -> TypeResult<TypeId> {
		match stmt {
			ast::Stmt::Expr(expr) => self.check_expr(expr),
			ast::Stmt::Let(let_stmt) => self.check_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.check_fn_stmt(fn_stmt),
			ast::Stmt::Block(block_stmt) => self.check_block_stmt(block_stmt),
			ast::Stmt::Const(const_stmt) => self.check_const_stmt(const_stmt),
			ast::Stmt::Ret(ret_stmt) => self.check_ret_stmt(ret_stmt),
		}
	}

	fn get_stored_type(&self, type_id: TypeId) -> TypeResult<&Type> {
		Ok(self.ctx.type_store.get_type(type_id).unwrap()) // TODO: error handling
	}
	pub fn format(&self, type_id: TypeId) -> String {
		// todo: we can use Rc and make format global???
		let formatter = TypeFormatter::new(&self.ctx.type_store);
		formatter.format(type_id)
	}
}
