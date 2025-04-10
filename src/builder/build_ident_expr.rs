use crate::{ast, error_build, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_ident_expr(&mut self, ident: &mut ast::Ident) -> IrBasicValue {
		let range = ident.get_range();
		if let Some(local) = self.ctx.lookup_local_variable(&ident.text) {
			if self.ctx.in_struct_member_scope() {
				return local.clone();
			}
			return self.resolve_value(local.clone(), range);
		}
		let message = error_build!("not found local variablue `{}` not found", ident.text);
		message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader);
		// let kind = self.build_type(ident.get_type_id(), ident.get_range());
		// self.ctx.new_register(kind)
	}
}
