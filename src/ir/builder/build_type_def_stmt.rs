use crate::{
	ast,
	ir::ir::{self},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_type_def_stmt(&mut self, type_def: &ast::TypeDefStmt) {
		let name = &type_def.name;
		match &type_def.kind {
			ast::TypeDefKind::Struct(strtuct_type) => self.build_struct_type(strtuct_type, name),
			_ => todo!(),
		}
	}

	fn build_struct_type(&mut self, type_def: &ast::StructType, ident: &ast::Ident) {
		let mut fields = Vec::with_capacity(type_def.fields.len());
		for field in type_def.fields.iter() {
			let field_bind = self.build_field_type(field);
			self.ir_ctx.add_struct_field(ident.lexeme(), field.lexeme(), field_bind.register);
			fields.push(field_bind);
		}
		let dest = self.ir_ctx.new_register();
		// let type_id = self.get_type_id(ident.get_type_id());
		let struct_id = ident.lexeme().to_owned();
		let instr = ir::StructInstr { struct_id, fields };
		self.add_struct(instr);
		self.ir_ctx.add_ir_value(ident.lexeme());
	}

	fn build_field_type(&mut self, type_def: &ast::FieldType) -> ir::Bind {
		let register = self.ir_ctx.new_register();
		let type_id = self.get_type_id(type_def.get_type_id());
		ir::Bind { register, type_id }
	}
}
