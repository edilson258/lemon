use crate::ast;
use crate::checker::synthesis;

use super::context::value::Value;
use super::diags::SyntaxErr;
use super::{CheckResult, Checker, ExpectSome};
impl Checker<'_> {
	pub fn check_let_stmt(&mut self, let_stmt: &mut ast::LetStmt) -> CheckResult {
		let lexeme = let_stmt.bind.lexeme();
		let mutable = let_stmt.mutable.is_some();
		let range = let_stmt.get_range();

		let mut found = self.check_expr(&mut let_stmt.expr).some(range)?;

		let expected_id = match let_stmt.bind.ty.as_ref() {
			Some(ast_type) => synthesis::synthesise_ast_type(ast_type, false, self.ctx)?,
			None => self.infer_default_type(found.type_id),
		};

		let final_id = self.infer_type_from_expected(expected_id, found.type_id);

		if !self.equal_type_id(expected_id, final_id) {
			let found_ty = self.display_type(final_id);
			let expected_ty = self.display_type(expected_id);
			return Err(SyntaxErr::type_mismatch(expected_ty, found_ty, range));
		}
		self.register_type(final_id, range);
		found.infer_type(final_id);
		self.ctx.add_value(lexeme, Value::new(found, mutable));

		Ok(None)
	}
}
