use crate::{
	ast,
	ir::{
		get_std_fn_id,
		ir::{self, Value},
	},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_ident_expr(&mut self, expr: &ast::Ident) -> Value {
		let ident = expr.lexeme();
		// todo: remove this
		if let Some(fn_id) = get_std_fn_id(ident) {
			return Value::Fn(fn_id);
		}

		if let Some(register) = self.ctx.get_value(ident) {
			let type_id = self.get_type_id(expr.type_id);
			let bind = ir::Bind { register: *register, type_id };
			let value = Value::new_bind(bind);
			// todo: this is the best way to acess type_id and register?
			self.can_free_value(value.get_register().unwrap(), value.get_type_id().unwrap());
			return value;
		}

		if let Some(fn_id) = self.ctx.get_fn_id(ident) {
			return Value::Fn(fn_id);
		}

		throw_ir_build_error(format!("'{}' not found", ident));
	}
}
