use super::context::scope::ScopeKind;
use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{synthesis, Checker};

use crate::ast;
use crate::message::MessageResult;
use crate::range::Range;

impl Checker<'_> {
	pub fn check_fn_stmt(&mut self, fn_stmt: &mut ast::FnStmt) -> MessageResult<TypeId> {
		let mod_id = self.ctx.mod_id;
		let range = fn_stmt.get_range();
		let fn_type = synthesis::synthesise_fn_stmt(fn_stmt, self.ctx, mod_id)?;
		let lexeme = fn_stmt.name.lexeme();
		self.check_exist_fn_name(lexeme, range)?;
		let ret_id = fn_type.ret;
		let fn_arg_types = fn_type.args.clone();

		let fn_type_id = self.ctx.type_store.add_type(fn_type.into());

		self.register_fn_type(lexeme, fn_type_id, range)?;

		self.register_type(ret_id, range);
		self.ctx.enter_scope(ScopeKind::function(ret_id));

		for (bind, bind_type_id) in fn_stmt.params.iter().zip(fn_arg_types.iter()) {
			let type_value = self.get_stored_type(*bind_type_id);
			let lexeme = bind.lexeme();
			let mutable = type_value.is_borrow_mut();
			self.ctx.add_owned_value(lexeme, *bind_type_id, mutable);
		}

		let _ = self.check_fn_body(&mut fn_stmt.body)?;
		// self.equal_type_expected(ret_id, ret_found, fn_stmt.body.get_range())?;

		self.ctx.exit_scope();
		if fn_stmt.is_pub {
			let lexeme = fn_stmt.name.lexeme();
			self.ctx.add_pub_function(lexeme.into(), fn_type_id);
		}

		Ok(TypeId::UNIT)
	}

	#[inline(always)]
	fn register_fn_type(
		&mut self,
		fn_name: &str,
		type_id: TypeId,
		range: Range,
	) -> MessageResult<()> {
		if self.ctx.has_implementation_scope() {
			let self_type_id = self.ctx.get_scope().get_self_scope_type().unwrap();
			let self_type = self.get_stored_mut_type(self_type_id);
			let struct_type = self_type.get_struct_type().unwrap(); // fix... suport enums etc
			if struct_type.has_fn(fn_name) {
				return Err(SyntaxErr::redefine_fn_in_same_scope(fn_name, range));
			}
			// todo: check ir is associated like no self is passed
			struct_type.add_fn(fn_name.to_string(), type_id);
			struct_type.add_associate(fn_name.to_string(), type_id);
			return Ok(());
		}
		self.ctx.add_function_value(fn_name, type_id);
		Ok(())
	}

	#[inline(always)]
	pub fn check_fn_body(&mut self, stmt: &mut ast::FnBody) -> MessageResult<TypeId> {
		match stmt {
			ast::FnBody::Block(block_stmt) => self.check_fn_block_stmt(block_stmt),
			ast::FnBody::Expr(expr) => self.check_ret_expr(expr),
		}
	}

	#[inline(always)]
	pub fn check_fn_block_stmt(&mut self, stmt: &mut ast::BlockStmt) -> MessageResult<TypeId> {
		let mut ret_type = TypeId::UNIT;
		for stmt in stmt.stmts.iter_mut() {
			ret_type = self.check_stmt(stmt)?;
		}
		Ok(ret_type)
	}

	#[inline(always)]
	pub fn check_ret_expr(&mut self, expr: &mut ast::Expr) -> MessageResult<TypeId> {
		if !self.ctx.has_function_scope() {
			// todo: change this error
			return Err(SyntaxErr::return_outside_fn(expr.get_range()));
		}

		let expected_ret_id = self.ctx.get_return_type().unwrap();
		let ret_id = self.check_expr(expr)?;
		if !self.equal_type_id(expected_ret_id, ret_id) {
			let expected = self.display_type(expected_ret_id);
			let found = self.display_type(ret_id);
			return Err(SyntaxErr::type_mismatch(expected, found, expr.get_range()));
		}
		Ok(ret_id)
	}

	pub fn check_exist_fn_name(&mut self, name: &str, range: Range) -> MessageResult<()> {
		if self.ctx.contains_fn_value_in_current_scope(name) {
			return Err(SyntaxErr::redefine_fn_in_same_scope(name, range));
		}
		Ok(())
	}
}
