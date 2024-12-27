use crate::ast::{self};

use super::diags::TypeCheckError;
use super::types::TypeId;
use super::{Checker, TypeResult};

impl Checker<'_> {
	pub fn check_ident_expr(&mut self, ident: &mut ast::Ident) -> TypeResult<TypeId> {
		let value = match self.ctx.get_value(ident.lexeme()) {
			Some(value) => value,
			None => return Err(TypeCheckError::not_found_value(ident.lexeme(), ident.get_range())),
		};
		ident.set_type_id(value.type_id);
		Ok(value.type_id)
	}
}
