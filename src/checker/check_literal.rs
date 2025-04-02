use crate::ast::{self};
use crate::message::MessageResult;

use super::types::TypeId;
use super::{synthesis, Checker};
impl Checker<'_> {
	pub fn check_literal(&mut self, lit: &ast::Literal) -> MessageResult<TypeId> {
		synthesis::synthesise_literal(lit, self.ctx)
	}
}
