use super::context::scope::ScopeType;
use super::diags::SyntaxErr;
use super::synthesis;
use super::types::TypeId;
use super::{Checker, TyResult};

use crate::ast;
use crate::range::Range;

impl Checker<'_> {
	pub fn check_fn_stmt(&mut self, fn_stmt: &mut ast::FnStmt) -> TyResult<TypeId> {
		let fn_type = synthesis::synthesise_fn_stmt(fn_stmt, self.ctx)?;
		let lexeme = fn_stmt.name.lexeme();
		self.check_exist_fn_name(lexeme, fn_stmt.name.get_range())?;
		let ret_id = fn_type.ret;
		let fn_arg_types = fn_type.args.clone();

		let fn_type_id = self.ctx.type_store.add_type(fn_type.into());

		self.ctx.add_fn_value(lexeme, fn_type_id, false);

		self.ctx.enter_scope(ScopeType::new_fn(ret_id));

		for (bind, bind_type_id) in fn_stmt.params.iter().zip(fn_arg_types.iter()) {
			let type_value = self.get_stored_type(*bind_type_id);
			self.ctx.add_value(bind.lexeme(), *bind_type_id, false);
		}

		let ret_found = self.check_fn_body(&mut fn_stmt.body)?;
		self.equal_type_expected(ret_id, ret_found, fn_stmt.body.get_range())?;

		self.ctx.exit_scope();
		Ok(TypeId::NOTHING)
	}

	#[inline(always)]
	pub fn check_fn_body(&mut self, stmt: &mut ast::FnBody) -> TyResult<TypeId> {
		match stmt {
			ast::FnBody::Block(block_stmt) => self.check_fn_block_stmt(block_stmt),
			ast::FnBody::Expr(expr) => self.check_ret_expr(expr),
		}
	}

	#[inline(always)]
	pub fn check_fn_block_stmt(&mut self, stmt: &mut ast::BlockStmt) -> TyResult<TypeId> {
		let mut ret_type = TypeId::NOTHING;
		for stmt in stmt.stmts.iter_mut() {
			ret_type = self.check_stmt(stmt)?;
		}
		Ok(ret_type)
	}

	#[inline(always)]
	pub fn check_ret_expr(&mut self, expr: &mut ast::Expr) -> TyResult<TypeId> {
		if !self.ctx.has_fn_scope() {
			// todo: change this error
			return Err(SyntaxErr::return_outside_fn(expr.get_range()));
		}

		let expected_ret_id = self.ctx.ret_scope_type().unwrap();
		let ret_id = self.check_expr(expr)?;
		if !self.equal_type_id(expected_ret_id, ret_id) {
			let expected = self.display_type(expected_ret_id);
			let found = self.display_type(ret_id);
			return Err(SyntaxErr::type_mismatch(expected, found, expr.get_range()));
		}
		Ok(ret_id)
	}

	pub fn check_exist_fn_name(&mut self, name: &str, range: Range) -> TyResult<()> {
		if self.ctx.contains_fn_value_in_current_scope(name) {
			return Err(SyntaxErr::redefine_fn_in_same_scope(name, range));
		}
		Ok(())
	}
}
