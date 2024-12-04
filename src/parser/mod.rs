use logos::Lexer;

use crate::ast::{self, BASE_BIN, BASE_DECIMAL, BASE_HEX, MIN_PDE};
use crate::diag::Diag;
use crate::lexer::Token;
use crate::range::Range;
mod parse_type;
// use crate::tokens::{Token, TokenType};

// --- pde utils -----

// state

pub struct Parser<'l> {
  pub lex: &'l mut Lexer<'l, Token>,
  token: Option<Token>,
  range: Range,
}
// --- parser  -----

type PResult<'l, T> = Result<T, Diag>;

impl<'l> Parser<'l> {
  pub fn new(lex: &'l mut Lexer<'l, Token>) -> Self {
    let range = Range::from_span(lex.span());
    let token = lex.next().map(|t| t.unwrap());
    Self { lex, token, range }
  }
  pub fn parse_program(&mut self) -> PResult<'l, ast::Program> {
    let mut stmts = vec![];
    while !self.is_end() {
      stmts.push(self.parse_stmt()?);
    }
    Ok(ast::Program { stmts })
  }

  fn parse_stmt(&mut self) -> PResult<'l, ast::Stmt> {
    let stmt = match self.token {
      Some(Token::Let) => self.parse_let_stmt().map(ast::Stmt::Let),
      Some(Token::Fn) => self.parse_fn_stmt().map(ast::Stmt::Fn),
      Some(Token::LBrace) => self.parse_block_stmt().map(ast::Stmt::Block),
      _ => self.parse_expr(MIN_PDE).map(ast::Stmt::Expr),
    };
    self.match_take(Token::Semi)?;
    stmt
  }

  fn parse_let_stmt(&mut self) -> PResult<'l, ast::LetStmt> {
    let range = self.expect(Token::Let)?.clone();
    let name = self.parse_binding()?;
    self.expect(Token::Assign)?; // =
    let expr = self.parse_expr(MIN_PDE)?;
    Ok(ast::LetStmt { name, expr, range })
  }

  fn parse_fn_stmt(&mut self) -> PResult<'l, ast::FnStmt> {
    let range = self.expect(Token::Fn)?.clone();
    let name = self.parse_ident()?;
    self.expect(Token::LParen)?;

    let mut params = vec![];
    while !self.match_token(Token::RParen) {
      params.push(self.parse_binding()?);
      if !self.match_token(Token::RParen) {
        self.expect(Token::Comma)?;
      }
    }

    self.expect(Token::RParen)?; // take ')'

    let mut ret_type = None;
    if self.match_token(Token::Colon) {
      self.expect(Token::Colon)?;
      ret_type = Some(self.parse_type()?);
    }
    self.expect(Token::Assign)?; // take '='
    let body = Box::new(self.parse_stmt()?);
    Ok(ast::FnStmt { name, params, ret_type, body, range })
  }

  fn parse_block_stmt(&mut self) -> PResult<'l, ast::BlockStmt> {
    let mut range = self.expect(Token::LBrace)?.clone();
    let mut stmts = vec![];
    while !self.match_token(Token::RBrace) {
      let stmt = self.parse_stmt()?;
      stmts.push(stmt);
    }
    range.merge(&self.expect(Token::RBrace)?);
    Ok(ast::BlockStmt::new(stmts, range))
  }

  fn parse_expr(&mut self, pde: u8) -> PResult<'l, ast::Expr> {
    let mut left = self.parse_primary()?;
    while let Some((operator, range)) = self.match_operator(pde) {
      let right = self.parse_expr(pde + 1)?;
      left = ast::Expr::Binary(ast::BinaryExpr {
        left: Box::new(left),
        operator,
        right: Box::new(right),
        range,
      });
    }
    Ok(left)
  }

  fn match_operator(&mut self, pde: u8) -> Option<(ast::Operator, Range)> {
    if let Some(operator) = self.token.as_ref().and_then(|t| ast::Operator::from_token(t)) {
      if operator.pde() >= pde {
        let range = self.range.to_owned();
        self.next().ok();
        return Some((operator, range));
      }
    }
    None
  }

  fn parse_primary(&mut self) -> PResult<'l, ast::Expr> {
    match self.token {
      Some(Token::Ident) => self.parse_ident().map(ast::Expr::Ident),
      Some(Token::Char) => self.parse_char().map(ast::Expr::Literal),
      Some(Token::String) => self.parse_string().map(ast::Expr::Literal),
      Some(Token::Decimal) | Some(Token::Hex) | Some(Token::Bin) => {
        self.parse_numb().map(ast::Expr::Literal)
      }
      Some(Token::Fn) => self.parse_fn_expr().map(ast::Expr::Fn),
      _ => Err(self.unexpected()),
    }
  }

  fn parse_char(&mut self) -> PResult<'l, ast::Literal> {
    self.ensure_char()?;
    let range = self.take_range();
    let text = self.take_text_and_next()?;
    let value = text.chars().nth(1).unwrap(); // skipe '
    let char = ast::CharLiteral { value, range };
    Ok(ast::Literal::Char(char))
  }

  fn parse_string(&mut self) -> PResult<'l, ast::Literal> {
    if !self.match_token(Token::String) {
      self.expect(Token::String)?;
    }
    let range = self.take_range();
    let text = self.take_text_and_next()?;
    let string = ast::StringLiteral { text, range };
    Ok(ast::Literal::String(string))
  }

  fn parse_numb(&mut self) -> PResult<'l, ast::Literal> {
    self.ensure_numb()?;
    let range = self.take_range();
    let text = self.take_text_and_next()?;
    let (base, cleaned_text) = self.detect_numb_base(&text);
    let as_dot = text.contains('.');
    let text = self.normalize_number(&cleaned_text);
    let num = ast::NumLiteral { base, as_dot, text, range };
    Ok(ast::Literal::Num(num))
  }

  fn parse_fn_expr(&mut self) -> PResult<'l, ast::FnExpr> {
    let range = self.expect(Token::Fn)?.clone();
    let mut params = vec![];
    self.expect(Token::LParen)?; // take '('
    while !self.match_token(Token::RParen) {
      params.push(self.parse_binding()?);
      if !self.match_token(Token::RParen) {
        self.expect(Token::Comma)?;
      }
    }
    self.expect(Token::RParen)?; // take ')'

    let ret_type = match self.token {
      Some(Token::Colon) => {
        self.expect(Token::Colon)?;
        Some(self.parse_type()?)
      }
      _ => None,
    };

    self.expect(Token::Assign)?; // take '='

    let body = Box::new(self.parse_stmt()?);

    Ok(ast::FnExpr { params, body, range, ret_type })
  }

  fn ensure_numb(&mut self) -> PResult<'l, ()> {
    if !matches!(self.token, Some(Token::Decimal | Token::Hex | Token::Bin)) {
      self.expect(Token::Decimal)?;
    }
    Ok(())
  }

  fn ensure_char(&mut self) -> PResult<'l, ()> {
    if !matches!(self.token, Some(Token::Char)) {
      self.expect(Token::Char)?;
    }

    // include "'"
    if self.take_text().len() > 3 {
      let diag = Diag::error("expected char literal", self.range.clone());
      return Err(diag);
    }
    Ok(())
  }

  fn detect_numb_base(&self, text: &str) -> (u8, String) {
    if text.starts_with("0x") {
      return (BASE_HEX, text.trim_start_matches("0x").to_string());
    }

    if text.starts_with("0b") {
      return (BASE_BIN, text.trim_start_matches("0b").to_string());
    }

    return (BASE_DECIMAL, text.to_string());
  }

  fn parse_binding(&mut self) -> PResult<'l, ast::Binding> {
    let ident = self.parse_ident()?;
    let mut ty = None;
    if self.match_token(Token::Colon) {
      self.expect(Token::Colon)?;
      ty = Some(self.parse_type()?);
    }
    Ok(ast::Binding { ident, ty })
  }

  fn parse_ident(&mut self) -> PResult<'l, ast::Ident> {
    if !self.match_token(Token::Ident) {
      self.expect(Token::Ident)?;
    }
    let range = self.range.clone();
    let text = self.take_text_and_next()?;
    Ok(ast::Ident { text, range })
  }

  // helpers
  //

  fn normalize_number(&self, text: &str) -> String {
    text.replace('_', "")
  }

  fn is_end(&self) -> bool {
    self.token.is_none()
  }

  fn take_text(&mut self) -> &'l str {
    self.lex.slice()
  }

  fn take_text_and_next(&mut self) -> PResult<'l, String> {
    let text = self.take_text();
    self.next()?;
    Ok(text.to_string())
  }

  fn next(&mut self) -> PResult<'l, Option<Token>> {
    let temp = self.token.take();
    self.token = self.lex.next().transpose().map_err(|_| self.unexpected())?;
    self.range = Range::from_span(self.lex.span());
    Ok(temp)
  }

  fn take_range(&mut self) -> Range {
    self.range.clone()
  }

  fn match_take(&mut self, token: Token) -> PResult<'l, Option<&Range>> {
    if self.match_token(token) {
      self.next()?;
      Ok(Some(&self.range))
    } else {
      Ok(None)
    }
  }

  fn expect(&mut self, token: Token) -> PResult<'l, Range> {
    if !self.match_token(token) {
      // todo: add error message
      let peeked = self.token.map(|t| t.to_string()).unwrap_or_else(|| "unkown".to_string());
      let diag = Diag::error(format!("expected {} but got {}", token, peeked), self.range.clone());
      return Err(diag);
    }
    let range = self.range.clone();
    self.next()?;
    Ok(range)
  }

  fn match_token(&mut self, token: Token) -> bool {
    self.token.as_ref().map(|t| *t == token).unwrap_or(false)
  }

  fn unexpected(&mut self) -> Diag {
    Diag::error("unexpected token", self.range.clone())
  }
}
