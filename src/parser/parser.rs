use crate::diagnostics::report::report_and_exit;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::utils::range::{create_range_from, Range};

use super::ast;

pub struct Parser<'p> {
  lexer: &'p mut Lexer<'p>,
}

impl<'p> Parser<'p> {
  pub fn new(lexer: &'p mut Lexer<'p>) -> Self {
    Self { lexer }
  }

  pub fn parse(&mut self) -> ast::Ast {
    let mut statements = Vec::new();
    while !self.lexer.is_end() {
      statements.push(self.parse_stmt());
      // todo: semicolon is required or not?
      self.match_and_consume(TokenType::Semi);
    }
    ast::Ast::new(statements)
  }

  fn parse_stmt(&mut self) -> ast::Statement {
    let token = self.lexer.peek_token();
    if self.match_token(TokenType::EOF) {
      return ast::Statement::create_empty();
    }
    match token.get_kind() {
      TokenType::Let => {
        let let_stmt = self.parse_let_statement();
        return ast::Statement::Let(let_stmt);
      }
      TokenType::Ident => self.parse_expr_statement(),
      _ => {
        let message = format!("expected statement, found `{}`", token.get_kind());
        self.report_custom_error(&message, &token.range);
      }
    }
  }

  // let <name> : <type> = <expr>
  fn parse_let_statement(&mut self) -> ast::LetStmt {
    let let_range = self.consume_expect(TokenType::Let);
    let token = self.lexer.next_token();
    let name = token.get_text().to_string();
    let mut ty = None;
    if self.match_and_consume(TokenType::Colon).is_some() {
      ty = Some(self.parse_type());
    }
    self.consume_expect(TokenType::Assign);
    let value = self.parse_expr();
    let range = create_range_from(&let_range, &value.get_range());
    return ast::LetStmt::create(name, ty, value, range);
  }

  fn parse_expr_statement(&mut self) -> ast::Statement {
    let expr = self.parse_expr();
    return ast::Statement::Expr(expr);
  }

  fn parse_expr(&mut self) -> ast::Expr {
    let token = self.lexer.peek_token();
    match token.get_kind() {
      TokenType::Num | TokenType::String | TokenType::True | TokenType::False | TokenType::Null => {
        let literal = self.parse_literal_expr();
        ast::Expr::create_literal(literal)
      }
      TokenType::Ident => {
        let ident = self.parse_ident_expr();
        ast::Expr::create_ident(ident)
      }
      _ => {
        let message = format!("expected expression, found `{}`", token.get_kind());
        self.report_custom_error(&message, &token.range);
      }
    }
  }

  fn parse_literal_expr(&mut self) -> ast::LiteralExpr {
    let token = self.lexer.peek_token();
    match token.get_kind() {
      TokenType::Num => {
        let literal = self.parse_number_literal();
        ast::LiteralExpr::create_number(literal)
      }
      TokenType::String => {
        let literal = self.parse_string_literal();
        ast::LiteralExpr::create_string(literal)
      }
      TokenType::True | TokenType::False => {
        let value = self.match_token(TokenType::True);
        let literal = self.parse_boolean_literal(value);
        ast::LiteralExpr::create_boolean(literal)
      }
      TokenType::Null => {
        let literal = self.parse_null_literal();
        return ast::LiteralExpr::create_null(literal);
      }
      _ => {
        let message = format!("expected literal, found `{}`", token.get_kind());
        let range = &token.range;
        self.report_custom_error(&message, range)
      }
    }
  }

  fn parse_ident_expr(&mut self) -> ast::IdentExpr {
    let token = self.lexer.next_token();
    let text = token.get_text().to_string();
    return ast::IdentExpr::create(text, token.get_range().clone());
  }

  fn parse_string_literal(&mut self) -> ast::StringLiteral {
    let token = self.lexer.next_token();
    let text = token.get_text().to_string();
    return ast::StringLiteral::create(text, token.get_range());
  }

  fn parse_boolean_literal(&mut self, value: bool) -> ast::BooleanLiteral {
    let token = self.lexer.next_token();
    match (token.get_kind(), value) {
      (TokenType::True, true) => return ast::BooleanLiteral::create(true, token.get_range()),
      (TokenType::False, false) => return ast::BooleanLiteral::create(false, token.get_range()),
      _ => {
        let message = format!("expected `{}` found `{}`", value, token.get_kind());
        self.report_custom_error(&message, &token.get_range())
      }
    }
  }

  fn parse_null_literal(&mut self) -> ast::NullLiteral {
    let range = self.consume_expect(TokenType::Null);
    return ast::NullLiteral::create(range);
  }

  fn parse_number_literal(&mut self) -> ast::NumberLiteral {
    let token = self.lexer.next_token();
    let text = token.get_text().to_string();
    return ast::NumberLiteral::create(text, token.get_range());
  }

  // -----------------
  // types
  fn parse_type(&mut self) -> ast::AstType {
    let token = self.lexer.peek_token();
    match token.get_kind() {
      TokenType::Ident => self.parse_text_to_type(),
      _ => {
        let message = format!("unknown type `{}`", token.get_kind());
        self.report_custom_error(&message, &token.range);
      }
    }
  }

  fn parse_text_to_type(&mut self) -> ast::AstType {
    let token = self.lexer.next_token();
    let text = token.get_text();
    let range = token.get_range();
    match text {
      "int" => ast::AstType::Int(ast::IntType::create(range)),
      "float" => ast::AstType::Float(ast::FloatType::create(range)),
      "bool" => ast::AstType::Bool(ast::BoolType::create(range)),
      "string" => ast::AstType::String(ast::StringType::create(range)),
      "char" => ast::AstType::Char(ast::CharType::create(range)),
      "null" => ast::AstType::Null(ast::NullType::create(range)),
      _ => {
        let ident = ast::IdentType::create(text.to_string(), range);
        ast::AstType::Ident(ident)
      }
    }
  }

  // -----------------
  // utilities
  fn consume_expect(&mut self, token_type: TokenType) -> Range {
    let token = self.lexer.next_token();
    if token.kind != token_type {
      self.report_expect(token.kind, token_type, &token.range);
    }
    token.range.clone()
  }

  fn match_token(&mut self, token_type: TokenType) -> bool {
    let token = self.lexer.peek_token();
    token.kind == token_type
  }

  fn match_and_consume(&mut self, token_type: TokenType) -> Option<Range> {
    let token = self.lexer.peek_token();
    if self.match_token(token_type) {
      self.lexer.next_token();
      return Some(token.range.clone());
    }
    return None;
  }

  fn report_expect(&mut self, current: TokenType, expected: TokenType, range: &Range) -> ! {
    let message = format!("expected '{}', found '{}'", expected, current);
    report_and_exit(&message, range, &self.lexer.take_souce());
  }

  fn report_unexpected(&mut self, token: Token) -> ! {
    let message = format!("unexpected token '{}'", token.get_kind());
    report_and_exit(&message, &token.range, &self.lexer.take_souce());
  }

  fn report_custom_error(&mut self, message: &str, range: &Range) -> ! {
    report_and_exit(message, range, &self.lexer.take_souce());
  }
}
