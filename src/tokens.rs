use serde::{Deserialize, Serialize};

use crate::range::Range;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
  // Keywords
  Fn,    // fn
  Let,   // let
  If,    // if
  Else,  // else
  Ret,   // return
  Null,  // null
  Match, // match
  // Operators
  Plus,        // +
  Minus,       // -
  Star,        // *
  Slash,       // /
  Assign,      // =
  PlusEq,      // +=
  MinusEq,     // -=
  StarEq,      // *=
  SlashEq,     // /=
  Eq,          // ==
  NotEq,       // !=
  Less,        // <
  Greater,     // >
  LessEq,      // <=
  GreaterEq,   // >=
  Extract,     // ?= (Error extraction operator)
  Arrow,       // =>
  And,         // &&
  Or,          // ||
  Dot,         // .
  DoubleDot,   // ..
  Bang,        // !
  Quest,       // ?
  Colon,       // :
  DoubleColon, // ::
  Pipe,        // |
  At,          // @

  // Delimiters
  LParen,   // (
  RParen,   // )
  LBrace,   // {
  RBrace,   // }
  LBracket, // [
  RBracket, // ]
  Semi,     // ;
  Comma,    // ,
  Identifier,
  String,
  Num,
  Bool,
  // Comments
  SkipLine,  // Line comments
  SkipBlock, // Block comments
  // Miscellaneous
  EOF, // End of file
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Self::new(TokenType::Num, Some(text), range)
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
      "match" => Self::new(TokenType::Match, None, range),
      _ => Self::new(TokenType::Identifier, Some(text.to_string()), range),
    }
  }
}
