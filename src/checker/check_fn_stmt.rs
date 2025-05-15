use super::context::scope::ScopeKind;
use super::context::value::Value;
use super::diags::SyntaxErr;
use super::typed_value::TypedValue;
use super::types::TypeId;
use super::{synthesis, CheckResult, Checker, ExpectSome};

use crate::ast;
use crate::message::Message;
use crate::range::Range;

impl Checker<'_> {
	pub fn check_fn_stmt(&mut self, fn_stmt: &mut ast::FnStmt) -> CheckResult {
		let mod_id = self.ctx.mod_id;
		let range = fn_stmt.get_range();
		let fn_type = synthesis::synthesise_fn_stmt(fn_stmt, self.ctx, mod_id)?;
		let lexeme = fn_stmt.name.lexeme();
		self.check_exist_function_name(lexeme, range)?;

		let return_id = fn_type.ret;
		let fn_args_types = fn_type.args.clone();
		let fn_type_id = self.ctx.type_store.add_type(fn_type.into());

		self.register_function_type(lexeme, fn_type_id, range)?;
		self.register_type(return_id, range);
		self.ctx.enter_scope(ScopeKind::function(return_id));

		self.register_fn_parameters(&mut fn_stmt.params, &fn_args_types)?;

		let ret_value = self.check_fn_body(&mut fn_stmt.body)?;

		self.ctx.exit_scope();

		if fn_stmt.is_pub {
			self.ctx.add_pub_function(lexeme.into(), fn_type_id);
		}

		Ok(None)
	}

	#[rustfmt::skip]
	fn register_fn_parameters(&mut self, params: &mut [ast::Binding], types: &[TypeId]) -> Result<(), Message> {
		for (param, type_id) in params.iter_mut().zip(types.iter()) {
		let type_value = self.lookup_stored_type(*type_id);
			let mutable = type_value.is_borrow_mut();
			let param_name = param.lexeme();

			let owner_id = self.ctx.borrow.create_owner();
			let typed_value = TypedValue::new(*type_id, owner_id);
			let value = Value::new(typed_value, mutable);
			self.ctx.add_value(param_name, value);
		}
		Ok(())
}

	#[inline(always)]
	pub fn check_fn_body(&mut self, stmt: &mut ast::FnBody) -> CheckResult {
		match stmt {
			ast::FnBody::Block(block_stmt) => self.check_fn_block_stmt(block_stmt),
			ast::FnBody::Expr(expr) => self.check_inplicit_return(expr),
		}
	}

	pub fn check_fn_block_stmt(&mut self, stmt: &mut ast::BlockStmt) -> CheckResult {
		let mut last_ret = None;
		for stmt in &mut stmt.stmts {
			last_ret = self.check_stmt(stmt)?;
		}
		Ok(last_ret)
	}

	pub fn check_inplicit_return(&mut self, expr: &mut ast::Expr) -> CheckResult {
		if !self.ctx.has_function_scope() {
			return Err(SyntaxErr::return_outside_fn(expr.get_range()));
		}

		let expected_ret_id = self.ctx.get_return_type().unwrap();
		let ret_value = self.check_expr(expr).some(expr.get_range())?;

		// self.ctx.borrow.validate_return_refs(&ret_value)?;

		if !self.equal_type_id(expected_ret_id, ret_value.type_id) {
			let expected = self.display_type(expected_ret_id);
			let found = self.display_type(ret_value.type_id);
			return Err(SyntaxErr::type_mismatch(expected, found, expr.get_range()));
		}

		Ok(Some(ret_value))
	}

	#[inline(always)]
	pub fn check_exist_function_name(&mut self, name: &str, range: Range) -> Result<(), Message> {
		if self.ctx.get_scope().has_function(name) {
			return Err(SyntaxErr::redefine_fn_in_same_scope(name, range));
		}
		Ok(())
	}

	#[rustfmt::skip]
	fn register_function_type(&mut self,name: &str, type_id: TypeId, range: Range) -> Result<(), Message> {
		if self.ctx.has_implementation_scope() {
			let self_type_id = self.ctx.get_scope().get_self_scope_type().unwrap();
			let self_type = self.lookup_stored_mut_type(self_type_id);
			let struct_type = self_type.get_struct_type().unwrap();
			if struct_type.has_function(name) {
				return Err(SyntaxErr::redefine_fn_in_same_scope(name, range));
			}
			struct_type.add_function(name.to_string(), type_id);
			struct_type.add_associate(name.to_string(), type_id);
			return Ok(());
		}
		self.ctx.add_function_value(name, type_id);
		Ok(())
	}
}
