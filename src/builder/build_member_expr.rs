use crate::builder::Builder;
use crate::{ast, ir, throw_error};

impl Builder<'_> {
	pub fn build_member_expr(&mut self, member_expr: &mut ast::MemberExpr) -> ir::IrBasicValue {
		if !self.ctx.in_implementation_scope() {
			throw_error!("cannot find a self scope")
		}
		let range = member_expr.get_range();
		let (self_name, _) = self.ctx.receiver_info().unwrap(); // we check if the self scope exists

		let mut self_value = self.build_expr(&mut member_expr.left);
		let _self_type = self.lookup_event_type(member_expr.get_range());
		self_value = self_value.with_new_type(_self_type);

		let field_name = member_expr.method.lexeme();
		let (field_type, offset) = match self.ctx.lookup_struct_field(self_name.as_str(), field_name) {
			Some((offset, field_type)) => (offset, field_type),
			None => throw_error!("cannot find a field"),
		};

		let dest = self.create_basic_value(field_type);
		// get pointer to field
		//
		let instr = ir::GetPtrInstr::new(self_name, self_value, offset, dest.clone());
		self.append_instr(instr.into(), Some(range));
		dest
	}
}
