use super::{context::scope::ScopeType, diags::TypeCheckError, types::FnType, Checker, TypeResult};
use crate::ast;

use super::types::{Type, TypeId};

impl Checker<'_> {
	pub fn check_const_fn_stmt(&mut self, const_fn: &mut ast::ConstFnStmt) -> TypeResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(TypeCheckError::const_outside_global_scope(const_fn.range.clone()));
		}

		let lexeme = const_fn.name.lexeme();

		if let Some(found_id) = self.ctx.get_value(lexeme) {
			return Err(TypeCheckError::const_redefinition(const_fn.range.clone()));
		}

		let params_processed = self.check_fn_params(&mut const_fn.params)?;

		let ret_id = self.check_fn_return_type(&const_fn.ret_type)?;

		let params = params_processed.iter().map(|(_, _, type_id)| *type_id).collect();

		let fn_id = self.ctx.type_store.add_type(Type::Fn(FnType::new(params, ret_id)));

		let const_type = Type::new_const_fn(fn_id);

		let const_id = self.ctx.type_store.add_type(const_type);

		let value_id = self.ctx.add_value(lexeme, const_id, false);
		self.ctx.enter_scope(ScopeType::new_const_fn(ret_id));
		for (lexeme, type_id, _) in params_processed {
			self.ctx.add_value(lexeme, type_id, false);
		}
		let ret_found = self.check_fn_body(&mut const_fn.body)?;

		if !self.ctx.flow.is_paths_return() {
			return Err(TypeCheckError::not_all_paths_return(const_fn.body.last_stmt_range()));
		}

		self.equal_type_id(ret_id, ret_found, const_fn.body.get_range())?;
		const_fn.set_ret_id(ret_id);
		self.ctx.exit_scope();
		Ok(TypeId::NOTHING)
	}
}
