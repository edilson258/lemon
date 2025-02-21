use crate::builder::Builder;
use crate::report::throw_ir_build_error;
use crate::{ast, ir};

impl Builder<'_> {
	pub fn build_member_expr(&mut self, member_expr: &mut ast::MemberExpr) -> ir::IrBasicValue {
		if !self.ctx.is_impl_scope() {
			throw_ir_build_error("cannot find a self scope")
		}

		let (self_name, _) = self.ctx.get_self_info().unwrap(); // we check if the self scope exists

		let mut self_value = self.build_expr(&mut member_expr.left);
		let _self_type = self.build_type(member_expr.get_left_type(), member_expr.get_range());
		self_value = self_value.with_new_type(_self_type);

		let field_name = member_expr.method.lexeme();
		let (field_type, offset) = match self.ctx.get_struct_field(self_name.as_str(), field_name) {
			Some((offset, field_type)) => (offset, field_type),
			None => throw_ir_build_error("cannot find a field"),
		};

		let dest = self.ctx.new_register(field_type);
		// get pointer to field
		//
		let instr = ir::GetPtrInstr::new(self_name, self_value, offset, dest.clone());
		self.ctx.block.add_instr(instr.into());

		dest
	}
}
