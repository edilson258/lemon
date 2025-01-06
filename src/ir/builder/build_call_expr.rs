use crate::{
	ast,
	checker::types::TypeId,
	ir::ir::{self, Value},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_call_expr(&mut self, expr: &ast::CallExpr) -> Value {
		let fn_id = match self.build_expr(&expr.callee) {
			Value::Fn(fn_id) => fn_id,
			_ => throw_ir_build_error(format!("not found '{:?}'", expr.callee)),
		};
		let dest = self.ctx.get_register();
		let args = self.load_args(&expr.args, &expr.args_type);
		let type_id = self.get_type_id(expr.type_id);
		let instr = ir::CallInstr { fn_id, type_id, args, dest };
		self.add_instr(ir::Instr::Call(instr));
		Value::new_register(dest)
	}

	fn load_args(&mut self, args: &[ast::Expr], args_type: &[TypeId]) -> Vec<ir::Register> {
		let mut registers = Vec::with_capacity(args.len());
		for (arg, arg_type) in args.iter().zip(args_type) {
			let value = self.build_expr(arg);
			let instr = ir::UnaryInstr {
				type_id: *arg_type,
				value: value.get_register().unwrap(),
				dest: self.ctx.get_register(),
			};
			registers.push(instr.dest);
			self.add_instr(ir::Instr::Load(instr));
		}
		registers
	}
}
