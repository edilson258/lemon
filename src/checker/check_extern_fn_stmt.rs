use crate::ast;

use super::synthesis::synthesise_extren_fn_stmt;
use super::{Checker, TyResult};

use super::types::TypeId;

impl Checker<'_> {
	pub fn check_extern_fn_stmt(&mut self, extrn_fn: &mut ast::ExternFnStmt) -> TyResult<TypeId> {
		let fn_type = synthesise_extren_fn_stmt(extrn_fn, self.ctx)?;
		let lexeme = extrn_fn.name.lexeme();

		let type_id = self.ctx.type_store.add_type(fn_type.into());

		self.ctx.add_fn_value(lexeme, type_id);

		Ok(TypeId::UNIT)
	}
}
