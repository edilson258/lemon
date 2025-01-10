use crate::{
	ast,
	checker::types::TypeId,
	ir::ir::{self, Value},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_literal(&mut self, expr: &ast::Literal) -> Value {
		let value = match expr {
			ast::Literal::Number(number) => self.build_number_expr(number),
			ast::Literal::String(string) => self.build_string_expr(string),
			ast::Literal::Char(char) => self.build_char_expr(char),
			ast::Literal::Bool(bool) => self.build_bool_expr(bool),
			ast::Literal::Null(_) => todo!(),
		};
		let register = self.ctx.get_register();
		// todo: resolve type
		let instr = ir::OwnInstr { type_id: TypeId::UNIT, value, dest: register };
		let own_instr = ir::Instr::Own(instr);
		if self.ctx.is_comptime() {
			self.add_global(own_instr);
		} else {
			self.add_instr(own_instr);
		}
		Value::new_register(register)
	}

	fn build_number_expr(&mut self, number: &ast::NumberLiteral) -> Value {
		if number.as_dot() {
			let float = number.text.parse::<f64>().expect("error: float parsing");
			Value::new_float(float)
		} else {
			let number = number.text.parse::<i64>().expect("error: int parsing");
			Value::new_int(number)
		}
	}

	fn build_string_expr(&mut self, string: &ast::StringLiteral) -> Value {
		let string = string.text.clone();
		Value::new_string(string)
	}

	fn build_char_expr(&mut self, char: &ast::CharLiteral) -> Value {
		let char = char.value;
		Value::new_char(char)
	}

	fn build_bool_expr(&mut self, bool: &ast::BoolLiteral) -> Value {
		let bool = bool.value;
		Value::new_bool(bool)
	}
}
