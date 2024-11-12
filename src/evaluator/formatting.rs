#![allow(dead_code, unused_variables)]
use core::fmt;

use crate::tokens::TokenType;

use super::value::{
  ArrayValue, BoolValue, BufferValue, BytesValue, FnValue, NullValue, NumValue, ObjectValue, StringValue,
};
use super::value::{NativeFnValue, Value};

struct Formatting<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Formatting<F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0(f)
  }
}

pub fn display_value<'a>(v: &'a Value) -> impl fmt::Display + 'a {
  Formatting(move |f| match v {
    Value::Null(NullValue {}) => write!(f, "nil"),
    Value::Num(NumValue { value }) => write!(f, "{}", value),
    Value::String(StringValue { value }) => write!(f, "{}", value),
    Value::Bool(BoolValue { value }) => write!(f, "{}", value),
    Value::Array(ArrayValue { value }) => write!(f, "{}", display_array(v)),
    Value::Object(ObjectValue { value }) => write!(f, "{}", display_object(v)),
    Value::Buffer(BufferValue { value }) => {
      write!(f, "buffer::{:?}", value.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(""))
    }
    Value::Bytes(BytesValue { value, position }) => {
      write!(
        f,
        "bytes::{:?}... at {}",
        value.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(""),
        position
      )
    }
    Value::Fn(FnValue { pats, stmt, ctx }) => {
      write!(f, "fn({}) = ...", pats.join(", "))
    }
    Value::NativeFn(NativeFnValue { name, native }) => write!(f, "native::{}", name),
  })
}

pub fn display_object<'a>(v: &'a Value) -> impl fmt::Display + 'a {
  Formatting(move |f| match v {
    Value::Object(ObjectValue { value }) => {
      write!(f, "{{")?;
      for (i, (k, v)) in value.iter().enumerate() {
        write!(f, "{}: {}", k, display_value(v))?;
        if i != value.len() - 1 {
          write!(f, ", ")?;
        }
      }
      write!(f, "}}")
    }
    _ => write!(f, "{}", display_value(v)),
  })
}

pub fn display_array<'a>(v: &'a Value) -> impl fmt::Display + 'a {
  Formatting(move |f| match v {
    Value::Array(ArrayValue { value }) => {
      write!(f, "[")?;
      for (i, v) in value.iter().enumerate() {
        write!(f, "{}", display_value(v))?;
        if i != value.len() - 1 {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
    _ => write!(f, "{}", display_value(v)),
  })
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TokenType::Fn => write!(f, "fn"),
      TokenType::Let => write!(f, "let"),
      TokenType::If => write!(f, "if"),
      TokenType::Else => write!(f, "else"),
      TokenType::Ret => write!(f, "return"),
      TokenType::Null => write!(f, "null"),
      TokenType::Match => write!(f, "match"),
      TokenType::Plus => write!(f, "+"),
      TokenType::Minus => write!(f, "-"),
      TokenType::Star => write!(f, "*"),
      TokenType::Slash => write!(f, "/"),
      TokenType::Bar => write!(f, "|"),
      TokenType::Assign => write!(f, "="),
      TokenType::Pow => write!(f, "^"),
      TokenType::PowEq => write!(f, "^="),
      TokenType::Rem => write!(f, "%"),
      TokenType::RemEq => write!(f, "%="),
      TokenType::PlusEq => write!(f, "+="),
      TokenType::MinusEq => write!(f, "-="),
      TokenType::StarEq => write!(f, "*="),
      TokenType::SlashEq => write!(f, "/="),
      TokenType::Eq => write!(f, "=="),
      TokenType::NotEq => write!(f, "!="),
      TokenType::Less => write!(f, "<"),
      TokenType::Greater => write!(f, ">"),
      TokenType::LessEq => write!(f, "<="),
      TokenType::GreaterEq => write!(f, ">="),
      TokenType::Extract => write!(f, "?="),
      TokenType::Arrow => write!(f, "=>"),
      TokenType::And => write!(f, "&&"),
      TokenType::Or => write!(f, "||"),
      TokenType::DotDot => write!(f, ".."),
      TokenType::Bang => write!(f, "!"),
      TokenType::Quest => write!(f, "?"),
      TokenType::Colon => write!(f, ":"),
      TokenType::ColonColon => write!(f, "::"),
      TokenType::Pipe => write!(f, "|"),
      TokenType::At => write!(f, "@"),
      TokenType::LParen => write!(f, "("),
      TokenType::RParen => write!(f, ")"),
      TokenType::LBrace => write!(f, "{{"),
      TokenType::RBrace => write!(f, "}}"),
      TokenType::LBracket => write!(f, "["),
      TokenType::RBracket => write!(f, "]"),
      TokenType::Semi => write!(f, ";"),
      TokenType::Comma => write!(f, ","),
      TokenType::Identifier => write!(f, "identifier"),
      TokenType::String => write!(f, "string"),
      TokenType::Num => write!(f, "number"),
      TokenType::Import => write!(f, "import"),
      TokenType::Bool => write!(f, "boolean"),
      TokenType::SkipLine => write!(f, "skip line"),
      TokenType::SkipBlock => write!(f, "skip block"),
      TokenType::EOF => write!(f, "eof"),
      TokenType::Dot => write!(f, "."),
    }
  }
}
