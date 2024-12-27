use crate::{
	ast,
	ir::ir::{self, Value},
};

use super::Builder;

impl Builder {
	pub fn build_call_expr(&mut self, expr: &ast::CallExpr) -> Value {
		let fn_id = self.build_expr(&expr.callee).get_fn_id().expect("error: not a fn");
		let dest = self.ctx.get_register();
		let mut args = Vec::with_capacity(expr.args.len());
		for arg in expr.args.iter() {
			let value = self.build_expr(arg);
			args.push(value.get_register().unwrap());
		}
		let type_id = expr.type_id.unwrap();
		let instr = ir::CallInstr { fn_id, type_id, args, dest };
		self.add_instr(ir::Instr::Call(instr));
		Value::new_register(dest)
	}
}
