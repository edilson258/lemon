use super::context::scope::ScopeKind;
use super::diags::SyntaxErr;
use super::{CheckResult, Checker};
use crate::ast;

impl Checker<'_> {
	pub fn check_associate_expr(&mut self, associate: &mut ast::AssociateExpr) -> CheckResult {
		let self_name = associate.self_name.lexeme();
		let self_type_id = match self.ctx.type_store.lookup_type_definition(self_name) {
			Some(type_id) => *type_id,
			None => return Err(SyntaxErr::not_found_type(self_name, associate.self_name.get_range())),
		};
		self.ctx.enter_scope(ScopeKind::accessor(self_type_id, true));
		let ret_type = self.check_ident_expr(&mut associate.method)?;
		self.ctx.exit_scope();
		self.register_type(self_type_id, associate.get_range());
		Ok(ret_type)
	}
}
