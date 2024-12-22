use crate::ast::{self};

use super::diags::TypeCheckError;
use super::types::TypeId;
use super::{Checker, TypeResult};

impl Checker<'_> {
	pub fn check_ident_expr(&mut self, ident: &ast::Ident) -> TypeResult<TypeId> {
		let value = self.ctx.get_value(ident.lexeme());
		if value.is_none() {
			return Err(TypeCheckError::not_found_value(ident.lexeme(), ident.get_range()));
		}
		Ok(value.unwrap().type_id)
	}
}
