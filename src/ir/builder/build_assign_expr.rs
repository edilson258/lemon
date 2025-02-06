use crate::{
	ast,
	ir::{ir, IrValue, Register},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_assign_expr(&mut self, binary: &ast::AssignExpr) -> Register {
		if let ast::Expr::Member(member) = &*binary.left {
			return self.build_member_assign(member, &binary.right);
		}
		let register = self.build_expr(&binary.right);
		let dest = self.build_expr(&binary.left);
		let type_id = self.get_type_id(binary.get_type_id());
		self.ir_ctx.register_struct(register, dest);
		let value = register.into();
		let instr = ir::StoreInstr { type_id, value, dest };
		self.ir_ctx.add_instr(instr.into());
		dest
	}

	fn build_member_assign(&mut self, member: &ast::MemberExpr, right: &ast::Expr) -> Register {
		let self_value = self.build_expr(&member.left);
		if let Some(struct_name) = self.ir_ctx.get_struct_register(self_value) {
			let field_name = member.method.lexeme();
			let field = self.ir_ctx.get_struct_field_register(struct_name, field_name);
			let field_type = self.get_type_id(member.method.get_type_id());
			let value = self.build_expr(right);
			// todo: maybe use value type instance instead of field type
			let instr = ir::SetFieldInstr { self_value, field, value, value_type: field_type };
			self.ir_ctx.add_instr(instr.into());
			return field;
		}
		throw_ir_build_error(format!("struct register not found for {:?}", member.left).as_str());
	}
}
