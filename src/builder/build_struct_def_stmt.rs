use crate::{ast::StructType, ir::Struct};

use super::{context::StructFieldMap, Builder};

impl Builder<'_> {
	pub fn build_struct_def_stmt(
		&mut self,
		struct_type: &mut StructType,
	) -> (Struct, StructFieldMap) {
		let mut field_table = StructFieldMap::default();
		let mut ir_struct = Struct::with_capacity(struct_type.fields.len());
		for field in struct_type.fields.iter() {
			let field_name = field.ident.lexeme();
			let field_type = self.build_type(field.get_type_id(), field.get_range());
			let position = ir_struct.add_field(field_type);
			field_table.insert(field_name.into(), (field_type, position));
		}
		(ir_struct, field_table)
	}
}
