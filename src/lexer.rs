#![allow(dead_code)]

use crate::tokens::{Token, TokenType};
use crate::utils::{match_number, range::Range, report::report_and_exit, source::Source};

pub struct Lexer {
  cursor: usize,
  pos_cursor: usize,
  source: Source,
  cached: Option<Token>,
}

impl Lexer {
  pub fn new(source: Source) -> Self {
    Self { cursor: 0, pos_cursor: 0, source, cached: None }
  }
  pub fn peek_token(&mut self) -> Token {
    if self.cached.is_none() {
      self.cached = Some(self.read_next_token());
    }
    self.cached.clone().unwrap()
  }

  pub fn next_token(&mut self) -> Token {
    if let Some(token) = self.cached.take() {
      return token;
    }
    self.read_next_token()
  }

  fn read_next_token(&mut self) -> Token {
    self.skip_whitespace();
    if self.is_end() {
      return Token::new_eof(self.create_range());
    }
    match self.peek() {
      '0'..='9' => self.read_number(),
      '"' => self.read_string(),
      '\'' => self.single_quote(),
      'a'..='z' | 'A'..='Z' | '_' | '$' => self.read_identifier(),
      '-' => self.read_check_ahead("-=", TokenType::MinusEq, TokenType::Minus),
      '+' => self.read_check_ahead("+=", TokenType::PlusEq, TokenType::Plus),
      '*' => self.read_check_ahead("*=", TokenType::StarEq, TokenType::Star),
      '/' => self.read_check_ahead("/=", TokenType::SlashEq, TokenType::Slash),
      '=' => {
        if self.peek_many(2).as_str() == "=>" {
          self.read_check_ahead("=>", TokenType::Arrow, TokenType::Assign)
        } else {
          self.read_check_ahead("==", TokenType::Eq, TokenType::Assign)
        }
      }
      '!' => self.read_check_ahead("!=", TokenType::NotEq, TokenType::Bang),
      '<' => self.read_check_ahead("<=", TokenType::LessEq, TokenType::Less),
      '>' => self.read_check_ahead(">=", TokenType::GreaterEq, TokenType::Greater),
      ':' => self.read_check_ahead("::", TokenType::DoubleColon, TokenType::Colon),
      '(' => self.read_simple_token(TokenType::LParen, "("),
      ')' => self.read_simple_token(TokenType::RParen, ")"),
      '{' => self.read_simple_token(TokenType::LBrace, "{"),
      '}' => self.read_simple_token(TokenType::RBrace, "}"),
      '[' => self.read_simple_token(TokenType::LBracket, "["),
      ']' => self.read_simple_token(TokenType::RBracket, "]"),
      '|' => match self.peek_many(2).as_str() {
        "|>" => self.read_simple_token(TokenType::Pipe, "|>"),
        "||" => self.read_simple_token(TokenType::Or, "||"),
        _ => self.handle_unknown_token("|"),
      },
      ';' => self.read_simple_token(TokenType::Semi, ";"),
      ',' => self.read_simple_token(TokenType::Comma, ","),
      '.' => self.read_simple_token(TokenType::Dot, "."),
      _ => self.handle_unknown_token(self.peek().to_string().as_str()),
    }
  }

  fn handle_unknown_token(&mut self, text: &str) -> ! {
    self.advance();
    report_and_exit(&format!("unknown token `{}`.", text), &self.create_range(), &self.source);
  }

  fn read_number(&mut self) -> Token {
    let number = self.read_while(match_number);
    Token::new_number(number, self.create_range())
  }

  fn read_string(&mut self) -> Token {
    self.consume_expect("\"");
    let text = self.read_while(|ch| ch != '"' && ch != '\n');
    self.consume_expect_with_error("\"", "unterminated string.");
    Token::new_string(text, self.create_range())
  }

  fn single_quote(&mut self) -> Token {
    self.consume_expect("'");
    let text = self.read_while(|ch| ch != '\'' && ch != '\n');
    self.consume_expect_with_error("'", "unterminated string.");
    Token::new_string(text, self.create_range())
  }

  fn read_identifier(&mut self) -> Token {
    let text = self.read_while(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '$' || ch.is_ascii_digit());
    Token::new_identifier(text.as_str(), self.create_range())
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

  fn read_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> String {
    let start = self.cursor;
    while !self.is_end() && predicate(self.peek()) {
      self.advance();
    }
    self.source.raw[start..self.cursor].to_string()
  }

  pub fn is_end(&self) -> bool {
    self.cursor >= self.source.len()
  }

  pub fn peek(&self) -> char {
    self.source.raw[self.cursor..].chars().next().unwrap_or('\0')
  }

  pub fn peek_many(&self, count: usize) -> String {
    self.source.raw[self.cursor..].chars().take(count).collect()
  }

  pub fn advance(&mut self) {
    if let Some(c) = self.source.raw[self.cursor..].chars().next() {
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
      self.report_mismatch(text);
    }
  }

  fn consume_expect_with_error(&mut self, text: &str, message: &str) {
    if self.starts_with(text) {
      self.advance_by(text.len());
    } else {
      report_and_exit(message, &self.create_range(), &self.source);
    }
  }

  fn advance_by(&mut self, count: usize) {
    for _ in 0..count {
      self.advance();
    }
  }

  fn starts_with(&self, s: &str) -> bool {
    self.source.raw[self.cursor..].starts_with(s)
  }

  fn skip_whitespace(&mut self) {
    while !self.is_end() && self.peek().is_whitespace() {
      self.advance();
      self.pos_cursor = self.cursor;
    }
  }

  fn report_mismatch(&mut self, expected: &str) -> ! {
    let found = self.peek_many(expected.len());
    let text_error = format!("expected `{}`, found `{}`.", expected, found);
    report_and_exit(&text_error, &self.create_range(), &self.source);
  }

  pub fn take_source(&self) -> &Source {
    &self.source
  }
}
