use crate::{
	ast,
	checker::types::TypeId,
	ir::{
		ir::{self, IrValue},
		Register,
	},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_literal(&mut self, expr: &ast::Literal) -> Register {
		match expr {
			ast::Literal::Number(number) => self.build_number_expr(number),
			ast::Literal::String(string) => self.build_string_expr(string),
			ast::Literal::Char(char) => self.build_char_expr(char),
			ast::Literal::Bool(bool) => self.build_bool_expr(bool),
			ast::Literal::Null(_) => todo!(),
		}
	}
	fn build_number_expr(&mut self, number: &ast::NumberLiteral) -> Register {
		let value = if number.as_dot() {
			let float = number.text.parse::<f64>().expect("error: float parsing");
			IrValue::new_float(float)
		} else {
			let number = number.text.parse::<i64>().expect("error: int parsing");
			IrValue::new_int(number)
		};
		let type_id = if number.as_dot() { TypeId::F32 } else { TypeId::I32 };
		let dest = self.ir_ctx.new_register();
		let instr = ir::StoreInstr { value, type_id, dest };
		self.ir_ctx.add_instr(instr.into());
		dest
	}
	fn build_string_expr(&mut self, string: &ast::StringLiteral) -> Register {
		let value = IrValue::new_string(&string.text);
		let type_id = TypeId::STR;
		let dest = self.ir_ctx.new_register();
		let instr = ir::StoreInstr { value, type_id, dest };
		self.ir_ctx.add_instr(instr.into());
		dest
	}
	fn build_char_expr(&mut self, char: &ast::CharLiteral) -> Register {
		let value = IrValue::new_char(char.value);
		let type_id = TypeId::CHAR;
		let dest = self.ir_ctx.new_register();
		let instr = ir::StoreInstr { value, type_id, dest };
		self.ir_ctx.add_instr(instr.into());
		dest
	}
	fn build_bool_expr(&mut self, bool: &ast::BoolLiteral) -> Register {
		let value = IrValue::new_bool(bool.value);
		let type_id = TypeId::BOOL;
		let dest = self.ir_ctx.new_register();
		let instr = ir::StoreInstr { value, type_id, dest };
		self.ir_ctx.add_instr(instr.into());
		dest
	}
}
