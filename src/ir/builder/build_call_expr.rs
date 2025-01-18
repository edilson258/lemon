use std::process::id;

use crate::{
	ast,
	checker::types::TypeId,
	ir::{
		ir::{self, IrValue},
		Bind, FnId, Register,
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
	fn build_args(&mut self, args: &[ast::Expr], args_type: &[TypeId]) -> Vec<Bind> {
		let mut binds = Vec::with_capacity(args.len());
		for (index, arg) in args.iter().enumerate() {
			let reg = self.build_expr(arg);
			let arg_type = match args_type.get(index) {
				Some(type_id) => type_id,
				None => self.ir_ctx.get_type(reg).expect("type not found"),
			};

			binds.push(Bind::new(reg, *arg_type));
		}
		binds
	}
}
