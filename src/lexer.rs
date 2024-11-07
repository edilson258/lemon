#![allow(dead_code)]

use crate::diag::Diag;
use crate::range::Range;
use crate::report::report_wrap;
use crate::source::Source;
use crate::tokens::{Token, TokenType};

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
      '/' => match self.peek_many(2).as_str() {
        "/-" => self.read_commets(),
        _ => self.read_check_ahead("/=", TokenType::SlashEq, TokenType::Slash),
      },
      '=' => match self.peek_many(2).as_str() {
        "=>" => self.read_check_ahead("=>", TokenType::Arrow, TokenType::Assign),
        _ => self.read_check_ahead("==", TokenType::Eq, TokenType::Assign),
      },
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
    let msg = format!("unknown token `{}`.", text);
    let diag = self.create_diag(&msg);
    report_wrap(&diag, &self.source);
  }

  fn read_number(&mut self) -> Token {
    let number = self.read_while(match_number);
    Token::new_number(number, self.create_range())
  }

  fn read_commets(&mut self) -> Token {
    match self.peek_many(3).as_str() {
      "/--" => self.read_skip_block(),
      _ => self.read_skip_line(),
    }
  }

  fn read_skip_block(&mut self) -> Token {
    self.consume_expect("/--");
    let start = self.cursor;
    while !self.is_end() && !self.starts_with("--/") {
      self.advance();
    }
    self.consume_expect("--/");
    let text = self.source.raw[start..self.cursor].to_string();
    let range = self.create_range();
    return Token::new(TokenType::SkipBlock, Some(text), range);
  }

  fn read_skip_line(&mut self) -> Token {
    self.consume_expect("/-");
    let text = self.read_while(|ch| ch != '\n');
    let range = self.create_range();
    return Token::new(TokenType::SkipLine, Some(text), range);
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
    self.source.raw[start..self.cursor].to_owned()
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
      let diag = self.create_diag(message);
      report_wrap(&diag, &self.source);
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
    let diag = self.create_diag(&text_error);
    report_wrap(&diag, &self.source);
  }

  pub fn take_source(&self) -> &Source {
    &self.source
  }

  pub fn create_diag(&mut self, message: &str) -> Diag {
    let range = self.create_range();
    Diag::create_err(message.to_string(), range)
  }
}

fn match_number(character: char) -> bool {
  "1234567890.".contains(character)
}
