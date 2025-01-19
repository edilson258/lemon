use crate::ast::{self};

use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TyResult};

impl Checker<'_> {
	pub fn check_ident_expr(&mut self, ident: &mut ast::Ident) -> TyResult<TypeId> {
		if let Some(value) = self.ctx.get_value(ident.lexeme()) {
			ident.set_type_id(value.type_id);
			return Ok(value.type_id);
		}
		self.ctx.type_store.create_monomo_fn(ident.lexeme().to_string());
		if let Some(fn_value) = self.ctx.get_fn_value(ident.lexeme()) {
			ident.set_type_id(fn_value.type_id);
			return Ok(fn_value.type_id);
		}
		self.ctx.type_store.end_monomo_fn();
		Err(SyntaxErr::not_found_value(ident.lexeme(), ident.get_range()))
	}
}
