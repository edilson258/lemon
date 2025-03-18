use crate::ast::{self};
use crate::loader::ModId;
use crate::range::Range;
use crate::report::throw_error_with_range;

use super::diags::SyntaxErr;
use super::types::{ModuleType, TypeId};
use super::{Checker, TyResult};

impl Checker<'_> {
	pub fn check_import_expr(&mut self, import_expr: &mut ast::ImportExpr) -> TyResult<TypeId> {
		let filename = import_expr.get_path();
		let range = import_expr.get_range();
		let mod_id = match import_expr.mod_id {
			Some(mod_id) => mod_id,
			None => return Err(SyntaxErr::not_found_module(filename.as_str(), range)),
		};
		if let Some(type_id) = self.ctx.type_store.get_mod(mod_id) {
			return Ok(*type_id);
		}
		self.ctx.add_mod(mod_id);
		self.check_mod(mod_id, range)?;
		let module_type = ModuleType::new(mod_id);
		let type_id = self.ctx.type_store.add_type(module_type.into());
		self.ctx.type_store.add_mod(mod_id, type_id);
		Ok(type_id)
	}

	pub fn check_mod(&mut self, mod_id: ModId, range: Range) -> TyResult<TypeId> {
		let source = self.loader.get_source_unchecked(mod_id).clone();
		#[rustfmt::skip]
		let mut ast = self.loader.get_mod_result(mod_id).cloned().unwrap_or_else(|err| {
			throw_error_with_range(err, range, &source)
		});
		let temp_mod_id = self.ctx.mod_id;
		self.ctx.swap_mod(mod_id);
		for stmt in ast.stmts.iter_mut() {
			self.check_stmt(stmt)?;
		}
		self.ctx.swap_mod(temp_mod_id);
		Ok(TypeId::UNIT)
	}
}
