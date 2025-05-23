use crate::ast::{self};
use crate::loader::ModId;
use crate::range::Range;

use super::diags::SyntaxErr;
use super::typed_value::TypedValue;
use super::types::ModuleType;
use super::{CheckResult, Checker};

impl Checker<'_> {
	pub fn check_import_expr(&mut self, import_expr: &mut ast::ImportExpr) -> CheckResult {
		let filename = import_expr.get_path();
		let range = import_expr.get_range();
		let owner_id = self.ctx.borrow.create_owner();
		let mod_id = match import_expr.mod_id {
			Some(mod_id) => mod_id,
			None => return Err(SyntaxErr::not_found_module(filename.as_str(), range)),
		};
		if let Some(type_id) = self.ctx.type_store.lookup_mod(mod_id) {
			return Ok(Some(TypedValue::new_module(*type_id, owner_id)));
		}
		self.ctx.add_mod(mod_id);
		self.check_mod(mod_id, range)?;
		let module_type = ModuleType::new(mod_id);
		let type_id = self.ctx.type_store.add_type(module_type.into());
		self.ctx.type_store.add_mod(mod_id, type_id);
		Ok(Some(TypedValue::new_module(type_id, owner_id)))
	}

	pub fn check_mod(&mut self, mod_id: ModId, range: Range) -> CheckResult {
		let source = self.loader.lookup_source_unchecked(mod_id).clone();
		#[rustfmt::skip]
		let mut ast = self.loader.lookup_mod_result(mod_id).cloned().unwrap_or_else(|message| {
		  message.report(self.loader);
		});
		let temp_mod_id = self.ctx.mod_id;
		self.ctx.swap_mod(mod_id);
		for stmt in ast.stmts.iter_mut() {
			if let Err(message) = self.check_stmt(stmt) {
				message.mod_id(mod_id).report(self.loader);
			}
		}
		self.ctx.swap_mod(temp_mod_id);
		Ok(None)
	}
}
