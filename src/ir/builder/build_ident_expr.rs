use crate::{
	ast,
	ir::ir::{self, Value},
};

use super::Builder;

impl Builder {
	pub fn build_ident_expr(&mut self, expr: &ast::Ident) -> Value {
		let ident = expr.lexeme();
		if let Some(register) = self.ctx.get_value(ident) {
			let type_id = expr.type_id.unwrap();
			let bind = ir::Bind { register: *register, type_id };
			return Value::new_bind(bind);
		}

		if let Some(fn_id) = self.ctx.get_fn_id(ident) {
			return Value::Fn(fn_id);
		}

		panic!("unknown identifier '{}'", ident);
	}
}
