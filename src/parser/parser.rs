use crate::diagnostics::report::report_and_exit;
use crate::lexer::token::TokenType;
use crate::lexer::Lexer;
use crate::utils::range::{create_range_from, Range};

use super::ast::{self, PatType};
use super::precedence::Precedence;

pub struct Parser<'p> {
  lexer: &'p mut Lexer<'p>,
}

impl<'p> Parser<'p> {
  pub fn new(lexer: &'p mut Lexer<'p>) -> Self {
    Self { lexer }
  }

  pub fn parse(&mut self) -> ast::Ast {
    let mut stmts = Vec::new();
    while !self.lexer.is_end() {
      stmts.push(self.parse_stmt());
    }
    // println!("stmts: {:#?}", stmts);
    ast::Ast::new(stmts)
  }

  fn parse_stmt(&mut self) -> ast::Stmt {
    let token = self.lexer.peek_token();
    if self.match_token(TokenType::EOF) {
      return ast::Stmt::create_empty();
    };

    let stmt = match token.get_kind() {
      TokenType::Let => {
        let let_stmt = self.parse_let_stmt();
        ast::Stmt::Let(let_stmt)
      }
      TokenType::Fn => {
        let function_stmt = self.parse_fn_stmt();
        ast::Stmt::Fn(function_stmt)
      }
      _ => self.parse_expr_stmt(),
    };

    // todo: semicolon is required or not?
    self.match_and_consume(TokenType::Semi);
    return stmt;
  }

  // let <name> : <type> = <expr>
  fn parse_let_stmt(&mut self) -> ast::LetStmt {
    let let_range = self.consume_expect(TokenType::Let);
    let name = self.parse_pat_type();
    self.consume_expect(TokenType::Assign);
    let value = self.parse_expr();
    let range = create_range_from(&let_range, &value.get_range());
    return ast::LetStmt::create(name, value, range);
  }

  // fn <name>(<inputs>): <type> { <body> }
  fn parse_fn_stmt(&mut self) -> ast::FnStmt {
    let fn_range = self.consume_expect(TokenType::Fn);
    let name = self.parse_ident_expr();
    let inputs = self.parse_fn_inputs();

    let mut output = None;
    if self.match_and_consume(TokenType::Colon).is_some() {
      output = Some(self.parse_type());
    }

    let body = self.parse_block();
    let range = create_range_from(&fn_range, &body.get_range());

    return ast::FnStmt::create(name, inputs, body, output, range);
  }

  // ( <inputs> )
  fn parse_fn_inputs(&mut self) -> Vec<PatType> {
    let mut inputs = Vec::new();

    if self.match_and_consume(TokenType::LParen).is_none() {
      return inputs;
    }
    while !self.match_token(TokenType::RParen) {
      let pat_type = self.parse_pat_type();
      inputs.push(pat_type);
      if self.match_token(TokenType::RParen) {
        break;
      }
      self.consume_expect(TokenType::Comma);
    }
    self.consume_expect(TokenType::RParen);
    return inputs;
  }

  // <name>: <type>
  fn parse_pat_type(&mut self) -> PatType {
    let ident = self.parse_ident_expr();
    let mut ty = None;
    if self.match_and_consume(TokenType::Colon).is_some() {
      ty = Some(self.parse_type());
    }
    return PatType::create(ident, ty);
  }

  // { <stmts> }
  fn parse_block(&mut self) -> ast::BlockStmt {
    let start_range = self.consume_expect(TokenType::LBrace);
    let mut stmts = Vec::new();
    while !self.match_token(TokenType::RBrace) {
      stmts.push(self.parse_stmt());
    }
    let end_range = self.consume_expect(TokenType::RBrace);
    let range = create_range_from(&start_range, &end_range);
    ast::BlockStmt::create(stmts, range)
  }

  // <expr stmt>
  fn parse_expr_stmt(&mut self) -> ast::Stmt {
    let expr = self.parse_expr();
    return ast::Stmt::Expr(expr);
  }

  // <expr>
  fn parse_expr(&mut self) -> ast::Expr {
    self.parse_precedence_expr(Precedence::Ass)
  }

  // precedence
  fn parse_precedence_expr(&mut self, precedence: Precedence) -> ast::Expr {
    let mut left = self.parse_unary_or_primary_expr();
    while let Some(next_precedence) = self.peek_precedence() {
      if precedence >= next_precedence {
        break;
      }
      let operator = self.parse_operator();
      let right = self.parse_precedence_expr(next_precedence);
      let binary = ast::BinaryExpr::create(left, operator, right);
      left = ast::Expr::create_binary(binary);
    }
    left
  }

  fn parse_unary_or_primary_expr(&mut self) -> ast::Expr {
    if self.match_any_token(vec![TokenType::Minus, TokenType::Bang, TokenType::Quest]) {
      let unary = self.parse_unary_expr();
      return ast::Expr::create_unary(unary);
    }
    self.parse_primary_expr()
  }

  fn parse_primary_expr(&mut self) -> ast::Expr {
    let token = self.lexer.peek_token();
    let mut expr = match token.get_kind() {
      TokenType::Num | TokenType::String | TokenType::Bool | TokenType::Null => {
        let literal = self.parse_literal_expr();
        ast::Expr::create_literal(literal)
      }
      TokenType::LParen => {
        let group = self.parse_group_expr();
        ast::Expr::create_group(group)
      }
      TokenType::Ident => {
        let ident = self.parse_ident_expr();
        ast::Expr::create_ident(ident)
      }
      _ => {
        let message = format!("expected primary expr, found `{}`.", token.get_kind());
        self.report_custom_error(&message, &token.range);
      }
    };

    while self.match_token(TokenType::LParen) {
      let call = self.parse_call_expr(expr);
      expr = ast::Expr::create_call(call);
    }
    return expr;
  }
  // ... ( <args> )
  fn parse_call_expr(&mut self, callee: ast::Expr) -> ast::CallExpr {
    let start_range = self.consume_expect(TokenType::LParen); // consume (
    let mut args = Vec::new();
    while !self.match_token(TokenType::RParen) {
      args.push(self.parse_expr());
      if self.match_token(TokenType::RParen) {
        break;
      }
      self.consume_expect(TokenType::Comma);
    }
    let end_range = self.consume_expect(TokenType::RParen); //consume )
    let arg_range = create_range_from(&start_range, &end_range);
    let range = create_range_from(&callee.get_range(), &arg_range);

    return ast::CallExpr::create(callee, args, range);
  }
  // ( <group> )
  fn parse_group_expr(&mut self) -> ast::GroupExpr {
    let start_range = self.consume_expect(TokenType::LParen);
    let mut list = Vec::new();
    while !self.match_token(TokenType::RParen) {
      list.push(self.parse_expr());
    }
    let end_range = self.consume_expect(TokenType::RParen);
    let range = create_range_from(&start_range, &end_range);
    return ast::GroupExpr::create(list, range);
  }

  // <unary>
  fn parse_unary_expr(&mut self) -> ast::UnaryExpr {
    if self.match_any_token(vec![TokenType::Minus, TokenType::Bang, TokenType::Quest]) {
      let operator = self.parse_operator();
      let operand = self.parse_precedence_expr(Precedence::Una);
      return ast::UnaryExpr::create(operand, operator);
    }
    let token = self.lexer.peek_token();
    let message = format!("expected unary, found `{}`.", token.get_kind());
    self.report_custom_error(&message, &token.range);
  }

  // <literal>
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
      TokenType::Bool => {
        let literal = self.parse_boolean_literal();
        ast::LiteralExpr::create_boolean(literal)
      }
      TokenType::Null => {
        let literal = self.parse_null_literal();
        ast::LiteralExpr::create_null(literal)
      }
      _ => {
        let message = format!("unsupported literal, found `{}`.", token.get_kind());
        let range = &token.range;
        self.report_custom_error(&message, range)
      }
    }
  }

  // <identifier>
  fn parse_ident_expr(&mut self) -> ast::IdentExpr {
    if !self.match_token(TokenType::Ident) {
      let current = self.lexer.peek_token();
      self.report_expect(current.kind, TokenType::Ident, &current.range);
    }
    let token = self.lexer.next_token();
    let text = token.get_text().unwrap().to_string(); // unwrap is safe because we save text in lexer
    return ast::IdentExpr::create(text, token.get_range());
  }

  // <string literal>
  fn parse_string_literal(&mut self) -> ast::StringLiteral {
    if !self.match_token(TokenType::String) {
      let current = self.lexer.peek_token();
      self.report_expect(current.kind, TokenType::String, &current.range);
    }
    let token = self.lexer.next_token();
    let text = token.get_text().unwrap().to_string(); // unwrap is safe because we save text in lexer
    return ast::StringLiteral::create(text, token.get_range());
  }

  // <boolean literal>
  fn parse_boolean_literal(&mut self) -> ast::BooleanLiteral {
    if !self.match_token(TokenType::Bool) {
      let current = self.lexer.peek_token();
      self.report_expect(current.kind, TokenType::Bool, &current.range);
    }
    let token = self.lexer.next_token();
    let text = token.get_text().unwrap().to_string(); // unwrap is safe because we save text in lexer
    return ast::BooleanLiteral::create(text == "true", token.get_range());
  }

  // <null literal>
  fn parse_null_literal(&mut self) -> ast::NullLiteral {
    let range = self.consume_expect(TokenType::Null);
    return ast::NullLiteral::create(range);
  }

  // <number literal>
  fn parse_number_literal(&mut self) -> ast::NumberLiteral {
    if !self.match_token(TokenType::Num) {
      let current = self.lexer.peek_token();
      self.report_expect(current.kind, TokenType::Num, &current.range);
    }

    let token = self.lexer.next_token();
    let text = token.get_text().unwrap().to_string(); // unwrap is safe because we save text in lexer
    return ast::NumberLiteral::create(text, token.get_range());
  }

  // <operator>
  fn parse_operator(&mut self) -> ast::Operator {
    let token = self.lexer.next_token();
    if let Some(kind) = ast::Operator::to_operator(token.get_kind()) {
      if token.be_operator() {
        let operator = ast::Operator::create(kind, token.get_range());
        return operator;
      }
    }
    let message = format!("unsupported operator, found `{}`.", token.get_kind());
    self.report_custom_error(&message, &token.range);
  }

  // types
  fn parse_type(&mut self) -> ast::AstType {
    let token = self.lexer.peek_token();
    match token.get_kind() {
      TokenType::Ident => self.parse_text_to_type(),
      _ => {
        let message = format!("unsupported type, found `{}`.", token.get_kind());
        self.report_custom_error(&message, &token.range);
      }
    }
  }

  fn parse_text_to_type(&mut self) -> ast::AstType {
    let token = self.lexer.next_token();
    let text = token.get_text();
    if text.is_none() {
      let message = format!("expected type, found `{}`.", token.get_kind());
      self.report_custom_error(&message, &token.range);
    }

    let range = token.get_range();
    let text = text.unwrap();
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

  fn match_any_token(&mut self, token_types: Vec<TokenType>) -> bool {
    let token = self.lexer.peek_token();
    token_types.contains(&token.kind)
  }

  fn peek_precedence(&mut self) -> Option<Precedence> {
    let token = self.lexer.peek_token();
    if token.be_operator() {
      let kind = token.get_kind();
      return Some(Precedence::to_precedence(&kind));
    }
    None
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
    let message = format!("expected `{}`, found `{}`.", expected, current);
    report_and_exit(&message, range, &self.lexer.take_souce());
  }

  fn report_custom_error(&mut self, message: &str, range: &Range) -> ! {
    report_and_exit(message, range, &self.lexer.take_souce());
  }
}
