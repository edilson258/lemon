use crate::{
  ast,
  lexer::Lexer,
  tokens::TokenType,
  utils::{range::Range, report::report_and_exit},
};

pub struct Parser {
  pub lex: Lexer,
}

impl Parser {
  pub fn new(lex: Lexer) -> Self {
    Self { lex }
  }
  pub fn parse_program(&mut self) -> ast::Program {
    let mut stmts = Vec::new();
    while !self.lex.is_end() {
      stmts.push(self.parse_stmt());
      if self.lex.is_end() {
        break;
      }
    }
    ast::Program { stmts }
  }

  fn parse_stmt(&mut self) -> ast::Stements {
    let token = self.lex.peek_token();
    let stmt = match token.kind {
      TokenType::Let => {
        let let_stmt = self.parse_let_stmt();
        ast::Stements::Let(let_stmt)
      }
      TokenType::Fn => {
        let function_stmt = self.parse_fn_stmt();
        ast::Stements::Fn(function_stmt)
      }
      _ => self.parse_expr_stmt(),
    };
    // todo: semicolon is required or not?
    self.match_and_consume(TokenType::Semi);
    return stmt;
  }

  fn parse_let_stmt(&mut self) -> ast::LetStmt {
    let mut range = self.consume_expect(TokenType::Let);
    let name = self.parse_pat();
    self.consume_expect(TokenType::Assign);
    let value = self.parse_expr();
    range.merge(value.get_range());
    return ast::LetStmt::create(name, value, range);
  }

  // fn <name>(<inputs>): <type> { <body> }
  fn parse_fn_stmt(&mut self) -> ast::FunctionStmt {
    let mut range = self.consume_expect(TokenType::Fn);
    let name = self.parse_ident();
    let inputs = self.parse_fn_pats();
    // let mut output = None;
    // if self.match_and_consume(TokenType::Colon).is_some() {
    //   output = Some(self.parse_type());
    // }
    let body = self.parse_block();

    range.merge(body.get_range());
    return ast::FunctionStmt::create(name, inputs, body, None, range);
  }

  // ( <inputs> )
  fn parse_fn_pats(&mut self) -> Vec<ast::Pat> {
    let mut inputs = Vec::new();
    self.consume_expect(TokenType::LParen);
    while !self.match_token(TokenType::RParen) {
      let pat = self.parse_pat();
      inputs.push(pat);
      if self.match_token(TokenType::RParen) {
        break;
      }
      self.consume_expect(TokenType::Comma);
    }
    self.consume_expect(TokenType::RParen);
    return inputs;
  }

  // <name>: <type>
  fn parse_pat(&mut self) -> ast::Pat {
    let ident = self.parse_ident();
    let ty = None;
    // if self.match_and_consume(TokenType::Colon).is_some() {
    //   ty = Some(self.parse_type());
    // }
    let range = ident.get_range().clone();
    return ast::Pat::create(ident, ty, range);
  }

  fn parse_ident(&mut self) -> ast::Identifier {
    let token = self.lex.peek_token();
    if token.kind == TokenType::Identifier {
      return ast::Identifier::create(token.text.unwrap(), token.range);
    }
    let message = format!("expected identifier, found `{:#?}`.", token.kind);
    self.report_custom_error(&message, &token.range);
  }

  // { <stmts> }
  fn parse_block(&mut self) -> ast::BlockStmt {
    let mut range = self.consume_expect(TokenType::LBrace);
    let mut stmts = Vec::new();
    while !self.match_token(TokenType::RBrace) {
      stmts.push(self.parse_stmt());
    }
    let end_range = self.consume_expect(TokenType::RBrace);
    range.merge(&end_range);
    ast::BlockStmt::create(stmts, range)
  }

  // <expr stmt>
  fn parse_expr_stmt(&mut self) -> ast::Stements {
    let expr = self.parse_expr();
    return ast::Stements::Expr(expr);
  }

  // -------- utils --------
  fn consume_expect(&mut self, token_type: TokenType) -> Range {
    let token = self.lex.next_token();
    if token.kind != token_type {
      self.report_expect(token.kind, token_type, &token.range);
    }
    token.range.clone()
  }

  fn match_token(&mut self, token_type: TokenType) -> bool {
    let token = self.lex.peek_token();
    token.kind == token_type
  }

  fn match_and_consume(&mut self, token_type: TokenType) -> Option<Range> {
    let token = self.lex.peek_token();
    if self.match_token(token_type) {
      self.lex.next_token();
      return Some(token.range.clone());
    }
    return None;
  }

  fn report_expect(&mut self, current: TokenType, expected: TokenType, range: &Range) -> ! {
    let message = format!("expected `{:#?}`, found `{:#?}`.", expected, current);
    report_and_exit(&message, range, &self.lex.take_source());
  }

  fn report_custom_error(&self, message: &str, range: &Range) -> ! {
    report_and_exit(message, range, &self.lex.take_source());
  }
}
