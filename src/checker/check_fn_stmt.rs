use super::context::scope::ScopeType;
use super::types::{FnType, Type, TypeId};
use super::{diags::TypeCheckError, Checker, TypeResult};

use crate::ast;

impl Checker<'_> {
	pub fn check_fn_stmt(&mut self, fn_stmt: &mut ast::FnStmt) -> TypeResult<TypeId> {
		let lexeme = fn_stmt.name.lexeme();
		let params_processed = self.check_fn_params(&mut fn_stmt.params)?;

		let ret_id = self.check_fn_return_type(&fn_stmt.ret_type)?;

		let params = params_processed.iter().map(|(_, _, type_id)| *type_id).collect();

		let fn_type = Type::Fn(FnType::new(params, ret_id));

		let fn_id = self.ctx.type_store.add_type(fn_type);

		let value_id = self.ctx.add_value(lexeme, fn_id, false);
		self.ctx.enter_scope(ScopeType::new_fn(ret_id));

		for (lexeme, type_id, _) in params_processed {
			self.ctx.add_value(lexeme, type_id, false);
		}
		let ret_found = self.check_fn_body(&mut fn_stmt.body)?;

		if !self.ctx.flow.is_paths_return() {
			return Err(TypeCheckError::not_all_paths_return(fn_stmt.body.last_stmt_range()));
		}

		self.equal_type_id(ret_id, ret_found, fn_stmt.body.get_range())?;
		fn_stmt.set_ret_id(ret_id);
		self.ctx.exit_scope();
		Ok(TypeId::NOTHING)
	}

	#[inline(always)]
	pub fn check_fn_params<'a>(
		&mut self,
		params: &'a mut [ast::Binding],
	) -> TypeResult<Vec<(&'a str, TypeId, TypeId)>> {
		let mut cache = Vec::with_capacity(params.len());
		for param in params.iter_mut() {
			let type_id = self.check_fn_param(param)?;
			let par_id = self.ctx.type_store.add_type(Type::Par { target: type_id });
			param.set_type_id(par_id);
			cache.push((param.lexeme(), par_id, type_id));
			// params.push(type_id);
		}
		Ok(cache)
	}

	#[inline(always)]
	pub fn check_fn_param(&mut self, param: &mut ast::Binding) -> TypeResult<TypeId> {
		match &param.ty {
			Some(ty) => self.check_type(ty),
			None => Err(TypeCheckError::required_type_notation(param.get_range())),
		}
	}

	#[inline(always)]
	pub fn check_fn_return_type(&mut self, ret_type: &Option<ast::AstType>) -> TypeResult<TypeId> {
		match ret_type {
			Some(ty) => self.check_type(ty),
			_ => Ok(TypeId::NOTHING),
		}
	}

	#[inline(always)]
	pub fn check_fn_body(&mut self, stmt: &mut ast::Stmt) -> TypeResult<TypeId> {
		match stmt {
			ast::Stmt::Block(block) => self.check_fn_block_stmt(block),
			_ => self.check_stmt(stmt),
		}
	}

	#[inline(always)]
	pub fn check_fn_block_stmt(&mut self, stmt: &mut ast::BlockStmt) -> TypeResult<TypeId> {
		let mut ret_type = TypeId::NOTHING;
		for stmt in stmt.stmts.iter_mut() {
			self.ctx.flow.set_paths_return(stmt.ends_with_ret());
			ret_type = self.check_stmt(stmt)?;
		}
		Ok(ret_type)
	}
}
