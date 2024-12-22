use crate::{
	ast,
	checker::{context::Context, types::typeid::TypeId, TypeResult},
};

pub(crate) fn synthesis_literal(literal: &ast::Literal, ctx: &mut Context) -> TypeResult {
	match literal {
		ast::Literal::Num(num) => synthesis_num_literal(num, ctx),
		ast::Literal::String(string) => Ok(TypeId::STRING),
		ast::Literal::Bool(bool) => Ok(TypeId::BOOL),
		ast::Literal::Char(char) => Ok(TypeId::CHAR),
		ast::Literal::Null(null) => todo!(),
	}
}

fn synthesis_num_literal(num: &ast::NumberLiteral, ctx: &mut Context) -> Type {
	if num.as_dot() {
		let bits = synthesis_float_bits(num, ctx).unwrap();
		return TypeId::FLOAT;
	}
	let bits = synthesis_num_bits(num, ctx).unwrap();
	let numb = NumbValue::new_signed(Some(bits));
	Type::Numb(numb)
}

fn synthesis_num_bits(num: &ast::NumberLiteral, ctx: &mut Context) -> Option<u8> {
	// todo:  we need o check if the number is valid
	let value = u128::from_str_radix(&num.text, num.base as u32).unwrap();
	let bit_range = (128 - value.leading_zeros()) as u8;
	let bits = match bit_range {
		0..=8 => 8,
		9..=16 => 16,
		17..=32 => 32,
		33..=64 => 64,
		65..=128 => 128,
		_ => unreachable!(),
	};
	Some(bits)
}

fn synthesis_float_bits(num: &ast::NumberLiteral, ctx: &mut Context) -> Option<u8> {
	// todo: improve this...
	match num.text.parse::<f32>() {
		Ok(num) if !num.is_nan() && !num.is_infinite() => return Some(32),
		_ => {}
	}
	match num.text.parse::<f64>() {
		Ok(num) if !num.is_nan() && !num.is_infinite() => return Some(64),
		_ => {}
	}
	unreachable!()
}
