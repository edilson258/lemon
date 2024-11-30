use serde::{Deserialize, Serialize};

use crate::range::Range;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
  // Keywords
  Fn,     // fn
  Let,    // let
  If,     // if
  For,    // for
  In,     // in
  While,  // while
  Loop,   // loop
  Break,  // break
  Skip,   // skip
  Else,   // else
  ElseIf, // else if
  Ret,    // return
  Null,   // null
  Match,  // match
  Import, // import
  // Operators
  Plus,       // +
  Minus,      // -
  Star,       // *
  Slash,      // /
  Assign,     // =
  Pow,        // ^
  PowEq,      // ^=
  Rem,        // %
  RemEq,      // %=
  PlusEq,     // +=
  MinusEq,    // -=
  StarEq,     // *=
  SlashEq,    // /=
  Eq,         // ==
  NotEq,      // !=
  Less,       // <
  Greater,    // >
  LessEq,     // <=
  GreaterEq,  // >=
  Extract,    // ?= (Error extraction operator)
  Arrow,      // =>
  And,        // &&
  Or,         // ||
  Dot,        // .
  DotDot,     // ..
  Bang,       // !
  Quest,      // ?
  Colon,      // :
  ColonColon, // ::
  Pipe,       // |>
  Bar,        // |
  At,         // @

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
      "false" => Self::new(TokenType::Bool, Some("false".to_owned()), range),
      "true" => Self::new(TokenType::Bool, Some("true".to_owned()), range),
      "import" => Self::new(TokenType::Import, None, range),
      "for" => Self::new(TokenType::For, None, range),
      "in" => Self::new(TokenType::In, None, range),
      "while" => Self::new(TokenType::While, None, range),
      "loop" => Self::new(TokenType::Loop, None, range),
      "break" => Self::new(TokenType::Break, None, range),
      "skip" => Self::new(TokenType::Skip, None, range),
      _ => Self::new(TokenType::Identifier, Some(text.to_string()), range),
    }
  }
}
