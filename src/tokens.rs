use serde::{Deserialize, Serialize};

use crate::utils::range::Range;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
  // Keywords
  Fn,        // fn
  Let,       // let
  If,        // if
  Else,      // else
  Ret,       // return
  Null,      // null
  Owner,     // owner
  BorrowMut, // borrow_mut
  Borrow,    // borrow
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
  Pipe,      // |
  Arroba,    // @

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
  Identifier, // Identifiers like variables and functions (foo)
  String,     // String literals ("foo")
  Int,        // Numeric literals (42)
  Bool,       // true, false

  // Comments
  LineCmt,  // Line comments (// ...)
  BlockCmt, // Block comments (/* ... */)
  // Miscellaneous
  EOF, // End of file
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
  pub kind: TokenType,
  pub text: Option<String>,
  pub range: Range,
}

impl Token {
  pub fn new(kind: TokenType, text: Option<String>, range: Range) -> Self {
    Self { kind, text, range }
  }

  pub fn new_number(text: String, range: Range) -> Self {
    Self::new(TokenType::Int, Some(text), range)
  }

  pub fn new_eof(range: Range) -> Self {
    Self::new(TokenType::EOF, None, range)
  }

  pub fn new_string(text: String, range: Range) -> Self {
    Self::new(TokenType::String, Some(text), range)
  }

  pub fn new_identifier(text: &str, range: Range) -> Self {
    match text {
      "fn" => Self::new(TokenType::Fn, None, range),
      "let" => Self::new(TokenType::Let, None, range),
      "if" => Self::new(TokenType::If, None, range),
      "else" => Self::new(TokenType::Else, None, range),
      "return" => Self::new(TokenType::Ret, None, range),
      "null" => Self::new(TokenType::Null, None, range),
      "owner" => Self::new(TokenType::Owner, None, range),
      "borrow_mut" => Self::new(TokenType::BorrowMut, None, range),
      "borrow" => Self::new(TokenType::Borrow, None, range),
      _ => Self::new(TokenType::Identifier, Some(text.to_string()), range),
    }
  }
}
