use super::context::scope::ScopeType;
use super::types::{FnType, Type, TypeId};
use super::{diags::TypeCheckError, Checker, TypeResult};

use crate::ast;

impl Checker<'_> {
	pub fn check_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) -> TypeResult<TypeId> {
		let lexeme = fn_stmt.lexeme();
		let mut params: Vec<TypeId> = Vec::with_capacity(fn_stmt.params.len());
		let mut cache = Vec::with_capacity(fn_stmt.params.len());
		for param in fn_stmt.params.iter() {
			let type_id = self.check_fn_param(param)?;
			cache.push((param.lexeme(), param.get_range(), type_id));
			params.push(type_id);
		}

		let ret_id = self.check_fn_return_type(&fn_stmt.return_type)?;
		let fn_type = Type::Fn(FnType::new(params, ret_id));

		let fn_id = self.ctx.type_store.add_type(fn_type);

		let value_id = self.ctx.add_value(lexeme, fn_id, false);

		self.ctx.enter_scope(ScopeType::new_fn(ret_id));

		for (lexeme, range, type_id) in cache {
			let par_id = self.ctx.type_store.add_type(Type::Par { target: type_id });
			let value_id = self.ctx.add_value_external(lexeme, par_id, false);
			self.check_borrow(value_id, par_id, range)?;
		}

		let ret_found = self.check_fn_body(&fn_stmt.body)?;
		self.equal_type_id(ret_id, ret_found, fn_stmt.body.get_range())?;
		self.ctx.exit_scope();
		Ok(TypeId::NOTHING)
	}

	pub fn check_fn_param(&mut self, param: &ast::Binding) -> TypeResult<TypeId> {
		match &param.ty {
			Some(ty) => self.check_type(ty),
			None => Err(TypeCheckError::required_type_notation(param.get_range())),
		}
	}

	fn check_fn_return_type(&mut self, ret_type: &Option<ast::AstType>) -> TypeResult<TypeId> {
		match ret_type {
			Some(ty) => self.check_type(ty),
			_ => Ok(TypeId::NOTHING),
		}
	}

	fn check_fn_body(&mut self, stmt: &ast::Stmt) -> TypeResult<TypeId> {
		match stmt {
			ast::Stmt::Block(block) => self.check_fn_block_stmt(block),
			_ => self.check_stmt(stmt),
		}
	}

	fn check_fn_block_stmt(&mut self, stmt: &ast::BlockStmt) -> TypeResult<TypeId> {
		for stmt in stmt.stmts.iter() {
			let ret_type = self.check_stmt(stmt)?;
			if ret_type != TypeId::NOTHING {
				return Ok(ret_type);
			}
		}
		Ok(TypeId::NOTHING)
	}
}
