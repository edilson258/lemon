use crate::utils::range::Range;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum TokenType {
  // Keywords
  Fn,       // fn
  Try,      // try
  Let,      // let
  If,       // if
  Else,     // else
  Return,   // return
  For,      // for
  In,       // in
  While,    // while
  Break,    // break
  Continue, // continue
  True,     // true
  False,    // false
  Null,     // null
  This,     // this
  Import,   // import
  From,     // from
  Keyword,  // fn, let, etc.

  // Operators
  Plus,               // +
  Minus,              // -
  Star,               // *
  Slash,              // /
  Assign,             // =
  PlusAssign,         // +=
  MinusAssign,        // -=
  StarAssign,         // *=
  SlashAssign,        // /=
  Equal,              // ==
  NotEqual,           // !=
  LessThan,           // <
  GreaterThan,        // >
  LessThanOrEqual,    // <=
  GreaterThanOrEqual, // >=
  And,                // &&
  Or,                 // ||
  Dot,                // .
  Bang,               // !
  Question,           // ?
  Colon,              // :

  // Delimiters
  OpenParen,    // (
  CloseParen,   // )
  OpenBrace,    // {
  CloseBrace,   // }
  OpenBracket,  // [
  CloseBracket, // ]
  Semicolon,    // ;
  Comma,        // ,

  // Identifiers
  Identifier, // foo
  String,     // "foo"
  Number,     // 42
  Boolean,    // true

  // Misc
  Comment,
  EOF,
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
    Self::new(TokenType::Number, Some(text), range)
  }

  pub fn create_identifier(text: String, range: Range) -> Self {
    match text.as_str() {
      "fn" => Self::new(TokenType::Fn, None, range),
      "import" => Self::new(TokenType::Import, None, range),
      "from" => Self::new(TokenType::From, None, range),
      "try" => Self::new(TokenType::Try, None, range),
      "let" => Self::new(TokenType::Let, None, range),
      "if" => Self::new(TokenType::If, None, range),
      "else" => Self::new(TokenType::Else, None, range),
      "return" => Self::new(TokenType::Return, None, range),
      "for" => Self::new(TokenType::For, None, range),
      "in" => Self::new(TokenType::In, None, range),
      "while" => Self::new(TokenType::While, None, range),
      "break" => Self::new(TokenType::Break, None, range),
      "continue" => Self::new(TokenType::Continue, None, range),
      "true" => Self::new(TokenType::True, None, range),
      "false" => Self::new(TokenType::False, None, range),
      "null" => Self::new(TokenType::Null, None, range),
      "this" => Self::new(TokenType::This, None, range),
      _ => Self::new(TokenType::Identifier, Some(text), range),
    }
  }

  pub fn create_string(text: String, range: Range) -> Self {
    Self::new(TokenType::String, Some(text), range)
  }

  pub fn create_boolean(text: String, range: Range) -> Self {
    Self::new(TokenType::Boolean, Some(text), range)
  }

  pub fn create_comment(text: String, range: Range) -> Self {
    Self::new(TokenType::Comment, Some(text), range)
  }

  pub fn create_eof(range: Range) -> Self {
    Self::new(TokenType::EOF, None, range)
  }

  // getters
  pub fn get_text(&self) -> &str {
    &self.text.as_ref().expect("Token text is None")
  }

  pub fn get_range(&self) -> &Range {
    &self.range
  }

  pub fn get_kind(&self) -> &TokenType {
    &self.kind
  }
}
