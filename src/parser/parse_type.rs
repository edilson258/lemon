use crate::{ast, lexer::Token, message::MessageResult};

use super::Parser;

impl Parser<'_> {
	#[rustfmt::skip]
	pub fn parse_type(&mut self) -> MessageResult< ast::AstType> {
		match self.token {
			Some(Token::BoolType)   =>   self.parse_bool_type().map(ast::AstType::Bool),
			Some(Token::VoidType)   =>   self.parse_void_type().map(ast::AstType::Void),
			Some(Token::CharType)   =>   self.parse_char_type().map(ast::AstType::Char),
			Some(Token::StringType) => self.parse_string_type().map(ast::AstType::String),
			Some(Token::StrType)    =>  self.parse_str_type().map(ast::AstType::Str),
			Some(Token::Ident)      => self.parse_ident_type().map(ast::AstType::Ident),
			Some(Token::Fn)         => self.parse_fn_type().map(ast::AstType::Fn),
			Some(Token::And)        => self.parse_borrow_type().map(ast::AstType::Borrow),

			Some(Token::F32Type)
			| Some(Token::F64Type) => self.parse_float_type().map(ast::AstType::Float),

			Some(Token::I8Type)      | Some(Token::U8Type)
			| Some(Token::I16Type)   | Some(Token::U16Type)
			| Some(Token::I32Type)   | Some(Token::U32Type)
			| Some(Token::I64Type)   | Some(Token::U64Type)
			| Some(Token::IsizeType) | Some(Token::UsizeType) => self.parse_numb_type().map(ast::AstType::Number),
			_ => Err(self.unexpected_token()),
		}
	}

	// &T or &mut T
	fn parse_borrow_type(&mut self) -> MessageResult<ast::BorrowType> {
		let range = self.expect(Token::And)?;
		let mut mutable = false;
		if self.match_token(Token::Mut) {
			mutable = true;
			self.expect(Token::Mut)?;
		}
		let value = Box::new(self.parse_type()?);
		Ok(ast::BorrowType { range, mutable, value })
	}

	// fn parse_deref_type(&mut self) -> MessageResult< ast::DerefType> {
	// 	let range = self.expect(Token::Star)?;
	// 	let value = Box::new(self.parse_type()?);
	// 	Ok(ast::DerefType { range, value })
	// }

	fn parse_float_type(&mut self) -> MessageResult<ast::FloatType> {
		match self.token {
			Some(Token::F32Type) => {
				let range = self.expect(Token::F32Type)?;
				Ok(ast::FloatType { range, bits: 32 })
			}
			Some(Token::F64Type) => {
				let range = self.expect(Token::F64Type)?;
				Ok(ast::FloatType { range, bits: 64 })
			}
			_ => Err(self.unexpected_token()),
		}
	}
	fn parse_numb_type(&mut self) -> MessageResult<ast::NumberType> {
		match self.token {
			Some(Token::IsizeType) => {
				let range = self.expect(Token::IsizeType)?;
				Ok(ast::NumberType { range, bits: 0, signed: true })
			}
			Some(Token::UsizeType) => {
				let range = self.expect(Token::UsizeType)?;
				Ok(ast::NumberType { range, bits: 0, signed: false })
			}
			Some(Token::I8Type) => {
				let range = self.expect(Token::I8Type)?;
				Ok(ast::NumberType { range, bits: 8, signed: true })
			}
			Some(Token::U8Type) => {
				let range = self.expect(Token::U8Type)?;
				Ok(ast::NumberType { range, bits: 8, signed: false })
			}
			Some(Token::I16Type) => {
				let range = self.expect(Token::I16Type)?;
				Ok(ast::NumberType { range, bits: 16, signed: true })
			}
			Some(Token::U16Type) => {
				let range = self.expect(Token::U16Type)?;
				Ok(ast::NumberType { range, bits: 16, signed: false })
			}
			Some(Token::I32Type) => {
				let range = self.expect(Token::I32Type)?;
				Ok(ast::NumberType { range, bits: 32, signed: true })
			}
			Some(Token::U32Type) => {
				let range = self.expect(Token::U32Type)?;
				Ok(ast::NumberType { range, bits: 32, signed: false })
			}
			Some(Token::I64Type) => {
				let range = self.expect(Token::I64Type)?;
				Ok(ast::NumberType { range, bits: 64, signed: true })
			}
			Some(Token::U64Type) => {
				let range = self.expect(Token::U64Type)?;
				Ok(ast::NumberType { range, bits: 64, signed: false })
			}
			Some(Token::F32Type) => {
				let range = self.expect(Token::F32Type)?;
				Ok(ast::NumberType { range, bits: 32, signed: true })
			}
			Some(Token::F64Type) => {
				let range = self.expect(Token::F64Type)?;
				Ok(ast::NumberType { range, bits: 64, signed: true })
			}
			_ => Err(self.unexpected_token()),
		}
	}

	fn parse_bool_type(&mut self) -> MessageResult<ast::BaseType> {
		let range = self.expect(Token::BoolType)?;
		Ok(ast::BaseType { range })
	}

	fn parse_void_type(&mut self) -> MessageResult<ast::BaseType> {
		let range = self.expect(Token::VoidType)?;
		Ok(ast::BaseType { range })
	}

	fn parse_char_type(&mut self) -> MessageResult<ast::BaseType> {
		let range = self.expect(Token::CharType)?;
		Ok(ast::BaseType { range })
	}

	fn parse_string_type(&mut self) -> MessageResult<ast::BaseType> {
		let range = self.expect(Token::StringType)?;
		Ok(ast::BaseType { range })
	}

	fn parse_str_type(&mut self) -> MessageResult<ast::BaseType> {
		let range = self.expect(Token::StrType)?;
		Ok(ast::BaseType { range })
	}

	fn parse_ident_type(&mut self) -> MessageResult<ast::IdentType> {
		if !self.match_token(Token::Ident) {
			self.expect(Token::Ident)?;
		}
		let range = self.range;
		let text = self.take_text_and_next()?;
		Ok(ast::IdentType { text, range })
	}
	fn parse_fn_type(&mut self) -> MessageResult<ast::FnType> {
		let range = self.expect(Token::Fn)?;
		let mut params = vec![];
		self.expect(Token::LParen)?;
		while !self.match_token(Token::RParen) {
			params.push(self.parse_type()?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}
		self.expect(Token::RParen)?;
		let mut ret_type = None;
		if self.match_token(Token::Arrow) {
			self.expect(Token::Arrow)?;
			ret_type = Some(Box::new(self.parse_type()?));
		}
		Ok(ast::FnType { params, ret_type, range })
	}
}
