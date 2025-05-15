use crate::{
	ast::StructInitExpr,
	checker::types::TypeId,
	error_build,
	ir::{self, IrBasicValue},
	range::Range,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_struct_init_expr(&mut self, struct_init_expr: &mut StructInitExpr) -> IrBasicValue {
		let struct_init_range = struct_init_expr.get_range();

		let self_type = self.lookup_event_type(struct_init_range);
		let self_name = struct_init_expr.name.lexeme();
		let self_size = *self.ctx.struct_sizes.get(self_name).unwrap_or_else(|| {
			let message = error_build!("struct `{}` does not exist", self_name);
			message.range(struct_init_range).report(self.loader);
		});

		// heap allocate memory for the struct
		// todo: we need to bitcast ptr in our ir?
		//
		let size_ir_value: IrBasicValue = self_size.into();
		let ptr_dest = self.create_basic_value(self_type);

		self.ctx.register_unbound_value(ptr_dest.clone());

		let instr = ir::UnInstr::new(ptr_dest.clone(), size_ir_value);

		self.append_instr(ir::Instr::Halloc(instr), Some(struct_init_range));

		// initialize fields
		//
		for field in struct_init_expr.fields.iter_mut() {
			let field_name = field.name.lexeme();
			let field_offset = self.ctx.lookup_struct_field(self_name, field_name).unwrap_or_else(|| {
				let message = error_build!("struct `{}` does not have field `{}`", self_name, field_name);
				message.range(field.name.get_range()).report(self.loader);
			});
			let field_value = self.build_expr(&mut field.value);
			self.build_set_field((self_name, ptr_dest.clone()), field_offset, field_value, field.range);
		}

		ptr_dest
	}

	pub fn build_set_field(
		&mut self,
		base: (&str, IrBasicValue),
		offset: (TypeId, usize),
		value: IrBasicValue,
		range: Range,
	) {
		let (self_name, self_ptr) = base;
		let (offset_type, offset) = offset;

		let field_dest = self.create_basic_value(offset_type);
		// get pointer to field
		let instr = ir::GetPtrInstr::new(self_name.into(), self_ptr, offset, field_dest.clone());

		self.append_instr(instr.into(), Some(range));

		// set field value
		let instr = ir::UnInstr::new(field_dest, value);
		self.append_instr(ir::Instr::Set(instr), Some(range));
	}
}
