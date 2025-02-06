use crate::{
	ast,
	checker::types::TypeId,
	ir::{ir::IrValue, Instr, Register, StoreInstr, UnaryInstr},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_ident_expr(&mut self, expr: &ast::Ident) -> Register {
		let lexeme = expr.lexeme();
		if let Some(register) = self.ir_ctx.get_value(lexeme) {
			return *register;
		}
		let dest = self.ir_ctx.new_register();
		if let Some(fn_id) = self.ir_ctx.get_fn_id(lexeme) {
			let value = IrValue::Value(fn_id);
			let instr = StoreInstr { type_id: TypeId::UNIT, value, dest };
			self.ir_ctx.add_instr(instr.into());
			return dest;
		}
		throw_ir_build_error(format!("'{}' not found", lexeme));
	}
}
