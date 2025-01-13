use crate::{
	ast,
	checker::types::TypeId,
	ir::{
		ir::{self, IrValue},
		FnId, Register,
	},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_call_expr(&mut self, expr: &ast::CallExpr) -> Register {
		let fn_id = self.build_callee(&expr.callee);
		let args = self.build_args(&expr.args, &expr.args_type);
		let dest = self.ir_ctx.new_register();
		let type_id = self.get_type_id(expr.get_type_id());
		let instr = ir::CallInstr { type_id, fn_id, args, dest };
		self.ir_ctx.add_instr(ir::Instr::Call(instr));
		dest
	}

	#[inline(always)]
	fn build_callee(&mut self, expr: &ast::Expr) -> FnId {
		match expr {
			ast::Expr::Ident(ident) => ir::FnId::new(ident.lexeme()),
			_ => todo!(),
		}
	}

	#[inline(always)]
	fn build_args(&mut self, args: &[ast::Expr], args_type: &[TypeId]) -> Vec<Register> {
		let registers = args
			.iter()
			.map(|arg| self.build_expr(arg))
			.collect::<Vec<_>>();
		registers
	}
}
