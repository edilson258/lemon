use super::context::scope::ScopeType;
use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TyResult};
use crate::ast;

impl Checker<'_> {
	pub fn check_associate_expr(&mut self, associate: &mut ast::AssociateExpr) -> TyResult<TypeId> {
		let self_name = associate.self_name.lexeme();
		let self_type_id = match self.ctx.type_store.get_type_by_name(self_name) {
			Some(type_id) => *type_id,
			None => return Err(SyntaxErr::not_found_type(self_name, associate.self_name.get_range())),
		};
		self.ctx.enter_scope(ScopeType::new_accessor_associate(self_type_id));
		let ret_type = self.check_ident_expr(&mut associate.method)?;
		self.ctx.exit_scope();
		associate.set_self_type(self_type_id);
		Ok(ret_type)
	}
}
