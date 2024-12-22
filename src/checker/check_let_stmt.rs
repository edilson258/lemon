use crate::ast;

use super::types::TypeId;
use super::{Checker, TypeResult};
impl Checker<'_> {
	pub fn check_let_stmt(&mut self, let_stmt: &ast::LetStmt) -> TypeResult<TypeId> {
		let lexeme = let_stmt.lexeme();
		let expect_id = match let_stmt.name.ty.as_ref() {
			Some(expect) => Some(self.check_type(expect)?),
			None => None,
		};
		let found_id = self.check_expr(&let_stmt.expr)?;
		match expect_id {
			Some(expect_id) => {
				self.equal_type_id(expect_id, found_id, let_stmt.name.get_range())?;
				self.ctx.add_value(lexeme, expect_id, let_stmt.mutable.is_some());
				Ok(TypeId::NOTHING)
			}
			None => {
				let found_id = self.infer_no_type_anotation(found_id)?;
				self.ctx.add_value(lexeme, found_id, let_stmt.mutable.is_some());
				Ok(TypeId::NOTHING)
			}
		}
	}
}
