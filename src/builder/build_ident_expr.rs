use crate::{ast, error_build, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_ident_expr(&mut self, ident: &mut ast::Ident) -> IrBasicValue {
		let range = ident.get_range();
		if let Some(local) = self.ctx.lookup_local_variable(&ident.text) {
			return local.clone();
		}
		let message = error_build!("not found local variablue `{}` not found", ident.text);
		message.mod_id(self.mod_id_unchecked()).range(range).report(self.loader);
	}
}
