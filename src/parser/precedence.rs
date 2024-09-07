#![allow(dead_code)]
use crate::lexer::token::TokenType;

use super::ast::OperatorType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
  Low, // lowest
  Ass, // =
  Or,  // ||
  And, // &&
  Eq,  // == !=
  Cop, // < > <= >=
  Add, // + -
  Mul, // * /
  Una, // ! -
  Cal, // . ()
  Pri, // ()
}

impl Precedence {
  pub fn op_to_precedence(token_type: &OperatorType) -> Self {
    match token_type {
      OperatorType::Assign => Precedence::Ass,
      OperatorType::Or => Precedence::Or,
      OperatorType::And => Precedence::And,
      OperatorType::Eq | OperatorType::NotEq => Precedence::Eq,
      OperatorType::Less | OperatorType::Greater | OperatorType::LessEq | OperatorType::GreaterEq => Precedence::Cop,
      OperatorType::Plus | OperatorType::Minus => Precedence::Add,
      OperatorType::Star | OperatorType::Slash => Precedence::Mul,
      OperatorType::Bang | OperatorType::Quest => Precedence::Una,
      _ => Precedence::Low,
    }
  }

  pub fn to_precedence(token_type: &TokenType) -> Self {
    match token_type {
      TokenType::Assign => Precedence::Ass,
      TokenType::Or => Precedence::Or,
      TokenType::And => Precedence::And,
      TokenType::Eq | TokenType::NotEq => Precedence::Eq,
      TokenType::Less | TokenType::Greater | TokenType::LessEq | TokenType::GreaterEq => Precedence::Cop,
      TokenType::Plus | TokenType::Minus => Precedence::Add,
      TokenType::Star | TokenType::Slash => Precedence::Mul,
      TokenType::Bang | TokenType::Quest => Precedence::Una,
      _ => Precedence::Low,
    }
  }

  pub fn to_next(self) -> Self {
    match self {
      Precedence::Low => Precedence::Ass,
      Precedence::Ass => Precedence::Or,
      Precedence::Or => Precedence::And,
      Precedence::And => Precedence::Eq,
      Precedence::Eq => Precedence::Cop,
      Precedence::Cop => Precedence::Add,
      Precedence::Add => Precedence::Mul,
      Precedence::Mul => Precedence::Una,
      Precedence::Una => Precedence::Cal,
      Precedence::Cal => Precedence::Pri,
      Precedence::Pri => Precedence::Pri, // or maybe `Low` if it's a loop
    }
  }
}
