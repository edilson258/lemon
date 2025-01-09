use crate::ast;
use crate::checker::synthesis;

use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TyResult};
impl Checker<'_> {
	pub fn check_let_stmt(&mut self, let_stmt: &mut ast::LetStmt) -> TyResult<TypeId> {
		let lexeme = let_stmt.bind.lexeme();
		let found_id = self.check_expr(&mut let_stmt.expr)?;

		let expect_id = match let_stmt.bind.ty.as_ref() {
			Some(ast_type) => synthesis::synthesise_ast_type(ast_type, false, self.ctx)?,
			None => {
				let found_id = self.infer_no_type_anotation(found_id)?;
				self.ctx.add_value(lexeme, found_id, let_stmt.mutable.is_some());
				return Ok(TypeId::NOTHING);
			}
		};
		let found_id = self.infer_type(expect_id, found_id)?;
		if !self.equal_type_id(expect_id, found_id) {
			let found = self.display_type(found_id);
			let expect = self.display_type(expect_id);
			return Err(SyntaxErr::type_mismatch(expect, found, let_stmt.get_range()));
		}

		self.ctx.add_value(lexeme, expect_id, let_stmt.mutable.is_some());
		let_stmt.set_type_id(expect_id);
		Ok(TypeId::NOTHING)
	}
}
