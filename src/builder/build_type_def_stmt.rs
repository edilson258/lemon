use crate::{
	ast::{TypeDefKind, TypeDefStmt},
	error_build,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_type_def_stmt(&mut self, type_def_stmt: &mut TypeDefStmt) {
		match &mut type_def_stmt.kind {
			TypeDefKind::Struct(struct_def_stmt) => {
				// todo: is the best way? but why :( two data structures? we relly need?
				let (mut ir_struct, field_table) = self.build_struct_def_stmt(struct_def_stmt);
				ir_struct.set_name(type_def_stmt.lexeme());
				self.ctx.define_struct_fields(type_def_stmt.lexeme().into(), field_table);
				self.ctx.struct_sizes.insert(type_def_stmt.lexeme().into(), ir_struct.size);
				self.ir.add_struct(ir_struct);
			}
			_ => error_build!("unsupported type definition kind")
				.range(type_def_stmt.get_range())
				.report(self.loader),
		}
	}
}
