use super::diags::SyntaxErr;
use super::{CheckResult, Checker, ExpectSome, TypedValue};
use crate::ast;

impl Checker<'_> {
	pub fn check_struct_init_expr(&mut self, init: &mut ast::StructInitExpr) -> CheckResult {
		let lexeme = init.name.lexeme();
		let range = init.get_range();
		let found_id = self.ctx.type_store.lookup_type_definition(lexeme).copied();

		if found_id.is_none() {
			return Err(SyntaxErr::not_found_type(lexeme, init.name.get_range()));
		}

		let found_id = found_id.unwrap();

		self.register_type(found_id, range);

		// remove clone :(
		let mut found_type = self.lookup_stored_type(found_id).clone();

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
			let value_range = field_expr.value.get_range();
			let value = self.check_expr(&mut field_expr.value).some(value_range)?;
			let lexeme = field_expr.name.lexeme();
			let range = field_expr.name.get_range();
			if !found_struct_type.has_field(lexeme) {
				return Err(SyntaxErr::not_found_field(lexeme, range));
			}

			let field_type = found_struct_type.get_field(lexeme).unwrap();

			// if !field_type.is_mut {
			// 	return Err(SyntaxErr::cannot_assign_immutable(lexeme, range));
			// }
			//
			let expect = field_type.type_id;
			let found = self.infer_type_from_expected(expect, value.type_id);
			self.equal_type_expected(expect, found, range)?;
			self.register_type(found, range);
		}
		let ptr = self.ctx.borrow.create_owner();
		Ok(Some(TypedValue::new(found_id, ptr)))
	}
}
