use crate::utils::range::Range;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum TokenType {
  // Keywords
  Fn,     // fn
  Let,    // let
  If,     // if
  Else,   // else
  Ret,    // return
  Null,   // null
  Use,    // use (e.g., use { stdin, stdout } "io")
  Ensure, // ensure

  // Operators
  Plus,      // +
  Minus,     // -
  Star,      // *
  Slash,     // /
  Assign,    // =
  PlusEq,    // +=
  MinusEq,   // -=
  StarEq,    // *=
  SlashEq,   // /=
  Eq,        // ==
  NotEq,     // !=
  Less,      // <
  Greater,   // >
  LessEq,    // <=
  GreaterEq, // >=
  Extract,   // ?= (Error extraction operator)
  Arrow,     // =>
  And,       // &&
  Or,        // ||
  Dot,       // .
  Bang,      // !
  Quest,     // ?
  Colon,     // :

  // Delimiters
  LParen,   // (
  RParen,   // )
  LBrace,   // {
  RBrace,   // }
  LBracket, // [
  RBracket, // ]
  Semi,     // ;
  Comma,    // ,
  // Identifiers and Literals
  Ident,  // Identifiers like variables and functions (foo)
  String, // String literals ("foo")
  Num,    // Numeric literals (42)
  Bool,   // true, false

  // Comments
  LineCmt,  // Line comments (// ...)
  BlockCmt, // Block comments (/* ... */)
  // Miscellaneous
  EOF, // End of file
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Token {
  pub kind: TokenType,
  pub text: Option<String>,
  pub range: Range,
}

impl Token {
  pub fn new(kind: TokenType, text: Option<String>, range: Range) -> Self {
    Self { kind, text, range }
  }

  pub fn create_number(text: String, range: Range) -> Self {
    Self::new(TokenType::Num, Some(text), range)
  }

  pub fn create_identifier(text: String, range: Range) -> Self {
    match text.as_str() {
      "fn" => Self::new(TokenType::Fn, None, range),
      "let" => Self::new(TokenType::Let, None, range),
      "if" => Self::new(TokenType::If, None, range),
      "else" => Self::new(TokenType::Else, None, range),
      "return" => Self::new(TokenType::Ret, None, range),
      "null" => Self::new(TokenType::Null, None, range),
      "true" => Self::new(TokenType::Bool, Some(text), range),
      "false" => Self::new(TokenType::Bool, Some(text), range),
      _ => Self::new(TokenType::Ident, Some(text), range),
    }
  }

  pub fn create_string(text: String, range: Range) -> Self {
    Self::new(TokenType::String, Some(text), range)
  }

  pub fn create_boolean(text: String, range: Range) -> Self {
    Self::new(TokenType::Bool, Some(text), range)
  }

  pub fn create_eof(range: Range) -> Self {
    Self::new(TokenType::EOF, None, range)
  }

  // getters
  pub fn get_text(&self) -> Option<&str> {
    if let Some(text) = &self.text {
      return Some(text);
    }
    None
  }

  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn be_operator(&self) -> bool {
    match self.kind {
      TokenType::Plus
      | TokenType::Minus
      | TokenType::Star
      | TokenType::Slash
      | TokenType::Assign
      | TokenType::PlusEq
      | TokenType::MinusEq
      | TokenType::StarEq
      | TokenType::SlashEq
      | TokenType::Eq
      | TokenType::NotEq
      | TokenType::Less
      | TokenType::Greater
      | TokenType::LessEq
      | TokenType::GreaterEq
      | TokenType::Extract
      | TokenType::Arrow
      | TokenType::And
      | TokenType::Or
      | TokenType::Bang
      | TokenType::Quest => true,
      _ => false,
    }
  }
  pub fn get_kind(&self) -> &TokenType {
    &self.kind
  }
}
