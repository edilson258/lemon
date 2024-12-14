use crate::{ast::ast_type, lexer::Token};

use super::{PResult, Parser};

impl<'l> Parser<'l> {
  pub fn parse_type(&mut self) -> PResult<'l, ast_type::AstType> {
    match self.token {
      Some(Token::BoolType) => self.parse_bool_type().map(ast_type::AstType::Bool),
      Some(Token::CharType) => self.parse_char_type().map(ast_type::AstType::Char),
      Some(Token::StringType) => self.parse_string_type().map(ast_type::AstType::String),
      Some(Token::Ident) => self.parse_ident_type().map(ast_type::AstType::Ident),
      Some(Token::Fn) => self.parse_fn_type().map(ast_type::AstType::Fn),
      Some(Token::F32Type) | Some(Token::F64Type) => {
        self.parse_float_type().map(ast_type::AstType::Float)
      }
      Some(Token::I8Type)
      | Some(Token::U8Type)
      | Some(Token::I16Type)
      | Some(Token::U16Type)
      | Some(Token::I32Type)
      | Some(Token::U32Type)
      | Some(Token::I64Type)
      | Some(Token::U64Type)
      | Some(Token::IsizeType)
      | Some(Token::UsizeType) => self.parse_numb_type().map(ast_type::AstType::Numb),
      _ => Err(self.unexpected()),
    }
  }

  fn parse_float_type(&mut self) -> PResult<'l, ast_type::FloatType> {
    match self.token {
      Some(Token::F32Type) => {
        let range = self.expect(Token::F32Type)?.clone();
        Ok(ast_type::FloatType { range, bits: 32 })
      }
      Some(Token::F64Type) => {
        let range = self.expect(Token::F64Type)?.clone();
        Ok(ast_type::FloatType { range, bits: 64 })
      }
      _ => Err(self.unexpected()),
    }
  }
  fn parse_numb_type(&mut self) -> PResult<'l, ast_type::NumbType> {
    match self.token {
      Some(Token::IsizeType) => {
        let range = self.expect(Token::IsizeType)?.clone();
        Ok(ast_type::NumbType { range, bits: 0, signed: true })
      }
      Some(Token::UsizeType) => {
        let range = self.expect(Token::UsizeType)?.clone();
        Ok(ast_type::NumbType { range, bits: 0, signed: false })
      }
      Some(Token::I8Type) => {
        let range = self.expect(Token::I8Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 8, signed: true })
      }
      Some(Token::U8Type) => {
        let range = self.expect(Token::U8Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 8, signed: false })
      }
      Some(Token::I16Type) => {
        let range = self.expect(Token::I16Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 16, signed: true })
      }
      Some(Token::U16Type) => {
        let range = self.expect(Token::U16Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 16, signed: false })
      }
      Some(Token::I32Type) => {
        let range = self.expect(Token::I32Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 32, signed: true })
      }
      Some(Token::U32Type) => {
        let range = self.expect(Token::U32Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 32, signed: false })
      }
      Some(Token::I64Type) => {
        let range = self.expect(Token::I64Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 64, signed: true })
      }
      Some(Token::U64Type) => {
        let range = self.expect(Token::U64Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 64, signed: false })
      }
      Some(Token::F32Type) => {
        let range = self.expect(Token::F32Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 32, signed: true })
      }
      Some(Token::F64Type) => {
        let range = self.expect(Token::F64Type)?.clone();
        Ok(ast_type::NumbType { range, bits: 64, signed: true })
      }
      _ => Err(self.unexpected()),
    }
  }

  fn parse_bool_type(&mut self) -> PResult<'l, ast_type::BaseType> {
    let range = self.expect(Token::BoolType)?.clone();
    Ok(ast_type::BaseType { range })
  }

  fn parse_char_type(&mut self) -> PResult<'l, ast_type::BaseType> {
    let range = self.expect(Token::CharType)?.clone();
    Ok(ast_type::BaseType { range })
  }

  fn parse_string_type(&mut self) -> PResult<'l, ast_type::BaseType> {
    let range = self.expect(Token::StringType)?.clone();
    Ok(ast_type::BaseType { range })
  }
  fn parse_ident_type(&mut self) -> PResult<'l, ast_type::IdentType> {
    if !self.match_token(Token::Ident) {
      self.expect(Token::Ident)?;
    }
    let range = self.range.clone();
    let text = self.take_text_and_next()?;
    Ok(ast_type::IdentType { text, range })
  }
  fn parse_fn_type(&mut self) -> PResult<'l, ast_type::FnType> {
    let range = self.expect(Token::Fn)?.clone();
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
    Ok(ast_type::FnType { params, ret_type, range })
  }
}
