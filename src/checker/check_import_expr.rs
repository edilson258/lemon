use crate::ast::{self};
use crate::loader::ModuleId;

use super::diags::SyntaxErr;
use super::types::{ModuleType, TypeId};
use super::{Checker, TyResult};

impl Checker<'_> {
	pub fn check_import_expr(&mut self, import_expr: &mut ast::ImportExpr) -> TyResult<TypeId> {
		let filename = import_expr.get_path();
		let range = import_expr.get_range();

		let current_module = self.loader.current_module;

		let module_id = match import_expr.module_id {
			Some(module_id) => module_id,
			None => return Err(SyntaxErr::not_found_module(filename.as_str(), range)),
		};
		if let Some(type_id) = self.ctx.type_store.get_module_cache(module_id) {
			return Ok(*type_id);
		}
		self.ctx.add_module(module_id);
		self.check_module(module_id)?;

		self.loader.swap_module(current_module);
		self.ctx.swap_module(current_module);

		// todo: using path for module name? hummm
		let module_type = ModuleType::new(module_id);
		let type_id = self.ctx.type_store.add_type(module_type.into());
		self.ctx.type_store.add_module_cache(module_id, type_id);
		Ok(type_id)
	}

	pub fn check_module(&mut self, module_id: ModuleId) -> TyResult<TypeId> {
		let module = self.loader.get_ast(module_id);
		let mut ast = self.loader.get_ast(module_id).clone();
		for stmt in ast.stmts.iter_mut() {
			self.check_stmt(stmt)?;
		}
		Ok(TypeId::UNIT)
	}
}
