use crate::{ast, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_ident_expr(&mut self, ident: &mut ast::Ident) -> IrBasicValue {
		if let Some(local) = self.ctx.get_local(&ident.text) {
			return local.clone();
		}
		let kind = self.build_type(ident.get_type_id(), ident.get_range());
		self.ctx.new_register(kind)
	}
}
