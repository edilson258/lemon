use crate::{
	ast,
	checker::{
		context::Context,
		types::{NumRange, TypeId},
		TyResult,
	},
};

pub(crate) fn synthesise_literal(literal: &ast::Literal, ctx: &mut Context) -> TyResult<TypeId> {
	match literal {
		ast::Literal::Number(number) => synthesise_number_literal(number, ctx),
		ast::Literal::String(string) => Ok(TypeId::STRING),
		ast::Literal::Bool(bool) => Ok(TypeId::BOOL),
		ast::Literal::Char(char) => Ok(TypeId::CHAR),
		ast::Literal::Null(null) => todo!(),
	}
}

fn synthesise_number_literal(number: &ast::NumberLiteral, ctx: &mut Context) -> TyResult<TypeId> {
	if number.as_dot() {
		let bits = synthesise_float_bits(number.text.as_str()).unwrap();
		let float_type = NumRange::new(bits, true);
		let type_id = ctx.type_store.add_type(float_type.into());
		return Ok(type_id);
	}
	let bits = synthesise_number_bits(&number.text).unwrap();
	let number = NumRange::new(bits, false);
	let type_id = ctx.type_store.add_type(number.into());
	Ok(type_id)
}

// todo: improve this...
fn synthesise_float_bits(text: &str) -> Option<u8> {
	match text.parse::<f32>() {
		Ok(num) if !num.is_nan() && !num.is_infinite() => return Some(32),
		_ => {}
	}
	match text.parse::<f64>() {
		Ok(num) if !num.is_nan() && !num.is_infinite() => return Some(64),
		_ => {}
	}
	None
}
// todo:  improve
fn synthesise_number_bits(text: &str) -> Option<u8> {
	let value: u128 = match text.parse::<u128>() {
		Ok(value) => value,
		Err(_) => return None,
	};
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
