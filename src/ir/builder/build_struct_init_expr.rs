use crate::{
	ast,
	ir::{
		ir::{self},
		Register,
	},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_struct_init_expr(&mut self, struct_init: &ast::StructInitExpr) -> Register {
		let dest = self.ir_ctx.new_register();
		let struct_name = struct_init.name.lexeme();
		let struct_id = struct_name.to_string();
		let mut binds = Vec::with_capacity(struct_init.fields.len());
		for field in struct_init.fields.iter() {
			let value_register = self.build_expr(&field.value);
			let type_id = self.get_type_id(field.name.get_type_id());
			let bind = ir::Bind::new(value_register, type_id);
			let field_name = field.name.lexeme();
			let dest_register = self.ir_ctx.get_struct_field_register(struct_name, field_name);
			binds.push((dest_register, bind));
		}
		self.ir_ctx.register_struct_name(dest, struct_name);
		let type_id = self.get_type_id(struct_init.get_type_id());
		let instr = ir::StructInitInstr { struct_id, binds, dest, type_id };
		self.ir_ctx.add_instr(ir::Instr::StructInit(instr));
		dest
	}
}
