use crate::{
	ast,
	ir::{ir, Register},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_member_expr(&mut self, member_expr: &ast::MemberExpr) -> Register {
		let self_reg = self.build_expr(&member_expr.left);
		let filed = member_expr.method.lexeme();
		let field = self.ir_ctx.get_struct_field_by_register(self_reg, filed);
		let dest = self.ir_ctx.new_register();
		let field_type = self.get_type_id(member_expr.get_method_type());
		let instr = ir::GetFieldInstr { self_reg, field, field_type, dest };
		self.ir_ctx.add_instr(instr.into());
		dest
	}
}
