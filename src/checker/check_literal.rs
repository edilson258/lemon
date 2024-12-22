use crate::ast::{self};

use super::types::TypeId;
use super::{Checker, TypeResult};

impl Checker<'_> {
	pub fn check_literal(&mut self, lit: &ast::Literal) -> TypeResult<TypeId> {
		match lit {
			ast::Literal::Number(number) => self.check_number(number),
			ast::Literal::String(string) => Ok(TypeId::STRING),
			ast::Literal::Bool(bool) => Ok(TypeId::BOOL),
			ast::Literal::Char(char) => Ok(TypeId::CHAR),
			ast::Literal::Null(null) => todo!(),
		}
	}
}
