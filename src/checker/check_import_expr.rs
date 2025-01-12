use crate::ast::{self};

use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TyResult};

impl Checker<'_> {
	pub fn check_import_expr(&mut self, import_expr: &mut ast::ImportExpr) -> TyResult<TypeId> {
		Err(SyntaxErr::not_found_module(import_expr.get_path().as_str(), import_expr.get_range()))
	}
}
