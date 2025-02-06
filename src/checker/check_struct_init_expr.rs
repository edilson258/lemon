use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TyResult};
use crate::ast;

impl Checker<'_> {
	pub fn check_struct_init_expr(&mut self, init: &mut ast::StructInitExpr) -> TyResult<TypeId> {
		let lexeme = init.name.lexeme();
		let found_id = self.ctx.type_store.get_type_by_name(lexeme).copied();

		if found_id.is_none() {
			return Err(SyntaxErr::not_found_type(lexeme, init.name.get_range()));
		}

		let found_id = found_id.unwrap();

		init.set_type_id(found_id);
		// remove clone :(
		let mut found_type = self.get_stored_type(found_id).clone();

		if !found_type.is_struct() {
			return Err(SyntaxErr::expect_instaced_type(
				self.display_type(found_id),
				init.name.get_range(),
			));
		}

		let found_struct_type = found_type.get_struct_type().unwrap();
		// check args length
		if found_struct_type.fields.len() != init.fields.len() {
			let found_len = found_struct_type.fields.len();
			let init_len = init.fields.len();
			return Err(SyntaxErr::args_mismatch(found_len, init_len, init.name.get_range()));
		}

		for field_expr in init.fields.iter_mut() {
			let value = self.check_expr(&mut field_expr.value)?;
			let lexeme = field_expr.name.lexeme();
			if !found_struct_type.has_field(lexeme) {
				return Err(SyntaxErr::not_found_field(lexeme, field_expr.name.get_range()));
			}

			let field_type = found_struct_type.get_field(lexeme).unwrap();

			// if !field_type.is_mut {
			// 	return Err(SyntaxErr::cannot_assign_immutable(lexeme, field_expr.name.get_range()));
			// }
			//
			let expect = field_type.type_id;
			let found = self.infer_type(expect, value)?;
			self.equal_type_expected(expect, found, field_expr.name.get_range())?;
			field_expr.name.set_type_id(field_type.type_id);
		}

		Ok(found_id)
		// check fields
		// Err(SyntaxErr::not_found_type(lexeme, init.name.get_range()))
	}
}
