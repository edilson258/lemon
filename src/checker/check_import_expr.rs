use crate::ast::{self};

use super::diags::TypeCheckError;
use super::types::TypeId;
use super::{Checker, TypeResult};

const PRINTLN: &str = "std/io/println";

impl Checker<'_> {
	pub fn check_import_expr(&mut self, import_expr: &mut ast::ImportExpr) -> TypeResult<TypeId> {
		if import_expr.get_path() == PRINTLN {
			self.ctx.add_value("println", TypeId::PRINTLN, false);
			return Ok(TypeId::PRINTLN);
		}
		Err(TypeCheckError::not_found_module(import_expr.get_path().as_str(), import_expr.get_range()))
	}
}
