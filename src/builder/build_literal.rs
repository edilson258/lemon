use crate::{ast, checker::types::TypeId, ir::IrValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_literal(&mut self, literal: &mut ast::Literal) -> IrValue {
		match literal {
			ast::Literal::Number(str) => IrValue::new(str.text.clone(), TypeId::I64),
			_ => todo!(),
		}
	}
}
