use crate::{
	ast::{TypeDefKind, TypeDefStmt},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_type_def_stmt(&mut self, type_def_stmt: &mut TypeDefStmt) {
		match &mut type_def_stmt.kind {
			TypeDefKind::Struct(struct_def_stmt) => {
				// todo: is the best way? but why :( two data structures? we relly need?
				let (mut ir_struct, field_table) = self.build_struct_def_stmt(struct_def_stmt);
				ir_struct.set_name(type_def_stmt.lexeme());
				self.ctx.set_struct_field(type_def_stmt.lexeme().into(), field_table);
				self.ctx.struct_table_size.insert(type_def_stmt.lexeme().into(), ir_struct.size);
				self.ir.add_struct(ir_struct);
			}
			_ => throw_ir_build_error("unsupported type definition kind"),
		}
	}
}
