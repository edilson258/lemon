use super::{diags::SyntaxErr, Checker};
use super::{synthesis, CheckResult, ExpectSome};
use crate::ast;

impl Checker<'_> {
	pub fn check_const_del_stmt(&mut self, c: &mut ast::ConstDelStmt) -> CheckResult {
		let range = c.get_range();
		// todo: allow const del in fn scope and block scope
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(range));
		}
		let mut found = self.check_expr(&mut c.expr).some(range)?;
		let lexeme = c.name.ident.text.clone();
		if found.module {
			if c.name.ty.is_some() {
				return Err(SyntaxErr::type_annotation_not_allowed_for_module(range));
			}
			self.ctx.type_store.add_mod_name(found.type_id, &lexeme);
			return Ok(Some(found));
		}

		let expected_type = match c.name.ty.as_ref() {
			Some(ast_type) => synthesis::synthesise_ast_type(ast_type, self.ctx)?,
			None => {
				found.infer_type(self.infer_default_type(found.type_id));
				found.type_id
			}
		};
		self.register_type(expected_type, range);
		Ok(Some(found))
	}
}
