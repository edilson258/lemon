use crate::ast;
use crate::message::MessageResult;

use super::synthesis::synthesise_extren_fn_stmt;
use super::Checker;

use super::types::TypeId;

impl Checker<'_> {
	pub fn check_extern_fn_stmt(
		&mut self,
		extrn_fn: &mut ast::ExternFnStmt,
	) -> MessageResult<TypeId> {
		let fn_type = synthesise_extren_fn_stmt(extrn_fn, self.ctx, self.ctx.mod_id)?;
		let lexeme = extrn_fn.name.lexeme();

		let type_id = self.ctx.type_store.add_type(fn_type.into());

		self.ctx.add_function_value(lexeme, type_id);

		Ok(TypeId::UNIT)
	}
}
