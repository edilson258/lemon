use crate::{ast, ir::IrBasicValue, report::throw_ir_build_error};

use super::Builder;

impl Builder<'_> {
	pub fn build_ident_expr(&mut self, ident: &mut ast::Ident) -> IrBasicValue {
		if let Some(local) = self.ctx.get_local(&ident.text) {
			if self.ctx.is_member_scope() {
				return local.clone();
			}
			return self.resolve_value(local.clone());
		}
		throw_ir_build_error(format!("local value `{}` not found", ident.text));
		// let kind = self.build_type(ident.get_type_id(), ident.get_range());
		// self.ctx.new_register(kind)
	}
}
