use super::synthesis;
use super::types::TypeId;
use super::{diags::SyntaxErr, Checker};
use crate::ast;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_const_del_stmt(
		&mut self,
		const_del: &mut ast::ConstDelStmt,
	) -> MessageResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(const_del.range));
		}
		let found_id = self.check_expr(&mut const_del.expr)?;

		let lexeme = const_del.name.ident.text.clone();

		if self.ctx.type_store.is_module(found_id) {
			if const_del.name.ty.is_some() {
				return Err(SyntaxErr::type_annotation_not_allowed_for_module(const_del.get_range()));
			}
			self.ctx.type_store.add_mod_name(found_id, lexeme.as_str());
		}

		let expect_id = match const_del.name.ty.as_ref() {
			Some(ast_type) => synthesis::synthesise_ast_type(ast_type, false, self.ctx)?,
			None => {
				let found_id = self.infer_default_type(found_id);
				self.ctx.add_owned_value(lexeme.as_str(), found_id, false);
				self.register_type(found_id, const_del.get_range());
				return Ok(TypeId::UNIT);
			}
		};
		self.register_type(expect_id, const_del.get_range());
		self.ctx.add_owned_value(lexeme.as_str(), expect_id, false);
		Ok(TypeId::UNIT)
	}

	pub fn check_const_bind(&mut self, const_bind: &mut ast::Binding) -> MessageResult<TypeId> {
		todo!()
	}
}
