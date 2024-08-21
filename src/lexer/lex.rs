use super::token::{Token, TokenType};

use crate::{
  diagnostics::report::report_and_exit,
  utils::{match_number, range::Range},
};

pub struct Lexer<'l> {
  raw: &'l str,
  file_name: &'l str,
  cursor: usize,
  pos_cursor: usize,
}

impl<'l> Lexer<'l> {
  pub fn new(raw: &'l str, file_name: &'l str) -> Self {
    Self { raw, cursor: 0, pos_cursor: 0, file_name }
  }

  pub fn lex_all(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    while !self.is_end() {
      let current = self.lex_next();
      tokens.push(current);
    }
    tokens
  }

  pub fn lex_peek(&mut self) -> Token {
    // Implement as needed
    unimplemented!()
  }

  pub fn lex_next(&mut self) -> Token {
    self.skip_whitespace();
    if self.is_end() {
      let range = self.create_range();
      return Token::create_eof(range);
    }
    match self.peek() {
      '0'..='9' => self.read_number(),
      '"' => self.read_string(),
      'a'..='z' | 'A'..='Z' | '_' | '$' => self.read_identifier(),
      '-' => self.read_check_ahead("-=", TokenType::MinusAssign, TokenType::Minus),
      '+' => self.read_check_ahead("+=", TokenType::PlusAssign, TokenType::Plus),
      '*' => self.read_check_ahead("*=", TokenType::StarAssign, TokenType::Star),
      '/' => self.read_check_ahead("/=", TokenType::SlashAssign, TokenType::Slash),
      '=' => self.read_check_ahead("==", TokenType::Equal, TokenType::Assign),
      '!' => self.read_check_ahead("!=", TokenType::NotEqual, TokenType::Bang),
      '<' => self.read_check_ahead("<=", TokenType::LessThanOrEqual, TokenType::LessThan),
      '>' => self.read_check_ahead(">=", TokenType::GreaterThanOrEqual, TokenType::GreaterThan),
      '(' => self.read_simple_token(TokenType::OpenParen, "("),
      ')' => self.read_simple_token(TokenType::CloseParen, ")"),
      '{' => self.read_simple_token(TokenType::OpenBrace, "{"),
      '}' => self.read_simple_token(TokenType::CloseBrace, "}"),
      '[' => self.read_simple_token(TokenType::OpenBracket, "["),
      ']' => self.read_simple_token(TokenType::CloseBracket, "]"),
      ';' => self.read_simple_token(TokenType::Semicolon, ";"),
      ',' => self.read_simple_token(TokenType::Comma, ","),
      '.' => self.read_simple_token(TokenType::Dot, "."),
      ':' => self.read_simple_token(TokenType::Colon, ":"),
      _ => {
        let range = self.create_range();
        report_and_exit("unknown token type", &range, self.raw, self.file_name);
      }
    }
  }

  fn read_simple_token(&mut self, kind: TokenType, text: &str) -> Token {
    self.consume_expect(text);
    Token::new(kind, None, self.create_range())
  }

  fn read_check_ahead(&mut self, text: &str, match_type: TokenType, no_match_type: TokenType) -> Token {
    if self.starts_with(text) {
      self.consume_expect(text);
      Token::new(match_type, None, self.create_range())
    } else {
      self.advance();
      Token::new(no_match_type, None, self.create_range())
    }
  }

  // Utilities
  fn read_number(&mut self) -> Token {
    let number = self.read_while(match_number);
    Token::create_number(number, self.create_range())
  }

  fn read_string(&mut self) -> Token {
    self.consume_expect("\"");
    let text = self.read_while(|c| c != '"');
    self.consume_expect_with_error("\"", "unterminated string");
    Token::create_string(text, self.create_range())
  }

  fn read_identifier(&mut self) -> Token {
    let text = self.read_while(|c| c.is_ascii_alphabetic() || c == '_' || c == '$' || c.is_ascii_digit());
    Token::create_identifier(text, self.create_range())
  }

  fn read_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> String {
    let start = self.cursor;
    while !self.is_end() && predicate(self.peek()) {
      self.advance();
    }
    self.raw[start..self.cursor].to_string()
  }

  fn advance(&mut self) {
    if let Some(c) = self.raw[self.cursor..].chars().next() {
      self.cursor += c.len_utf8();
    }
  }

  fn create_range(&mut self) -> Range {
    let start = self.pos_cursor;
    let end = self.cursor;
    self.pos_cursor = self.cursor;
    Range { start, end }
  }

  fn consume_expect(&mut self, text: &str) {
    if self.starts_with(text) {
      self.advance_by(text.len());
    } else {
      let found = self.peek_many(text.len());
      self.advance_by(text.len());
      let text_error = format!("expected '{}', found '{}'", text, found);
      report_and_exit(&text_error, &self.create_range(), self.raw, self.file_name);
    }
  }

  fn consume_expect_with_error(&mut self, text: &str, message: &str) {
    if self.starts_with(text) {
      self.advance_by(text.len());
    } else {
      report_and_exit(message, &self.create_range(), self.raw, self.file_name);
    }
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.raw.len()
  }

  fn peek(&self) -> char {
    self.raw[self.cursor..].chars().next().unwrap_or('\0') // TODO: handle errors
  }

  fn peek_many(&self, count: usize) -> String {
    self.raw[self.cursor..].chars().take(count).collect()
  }

  fn advance_by(&mut self, count: usize) {
    for _ in 0..count {
      self.advance();
    }
  }

  fn starts_with(&self, s: &str) -> bool {
    self.raw[self.cursor..].starts_with(s)
  }

  fn skip_whitespace(&mut self) {
    while !self.is_end() && self.peek().is_whitespace() {
      self.advance();
      self.pos_cursor = self.cursor;
    }
  }
}
