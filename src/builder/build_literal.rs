use crate::{ast, ir::IrBasicValue, throw_error};

use super::Builder;

impl Builder<'_> {
	pub fn build_literal(&mut self, literal: &mut ast::Literal) -> IrBasicValue {
		match literal {
			ast::Literal::Number(str) => self.build_number_literal(str),
			ast::Literal::String(str) => str.text.clone().into(),
			ast::Literal::Char(char) => char.value.into(),
			ast::Literal::Bool(bool) => bool.value.into(),
			ast::Literal::Null(_) => todo!(),
		}
	}

	pub fn build_number_literal(&mut self, literal: &mut ast::NumberLiteral) -> IrBasicValue {
		if literal.as_dot() {
			#[rustfmt::skip]
  		let value = literal.text.parse::<f64>().unwrap_or_else(|_| {
  			throw_error!("failed to parse float literal")
  		});
			return value.into();
		}
		#[rustfmt::skip]
		let value = literal.text.parse::<i64>().unwrap_or_else(|_| {
		  throw_error!("failed to parse number literal")
		});
		value.into()
	}
}
