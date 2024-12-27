use crate::{
	ast,
	ir::ir::{self, Value},
};

use super::Builder;

impl Builder {
	pub fn build_deref_expr(&mut self, deref_exper: &ast::DerefExpr) -> Value {
		let expr = self.build_expr(&deref_exper.expr);
		let value = expr.get_register().unwrap();
		let type_id = deref_exper.type_id.unwrap();
		let dest = self.ctx.get_register();
		let instr = ir::UnaryInstr { type_id, value, dest };
		self.add_instr(ir::Instr::Load(instr));
		Value::new_register(dest)
	}
}
