use crate::ast::{self, Operator};
use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::utils::{range::Range, report::report_and_exit};

// --- precedence utils -----
const MIN_PDE: u8 = 0;
const ADD_PDE: u8 = 1;
const MUL_PDE: u8 = 2;
const MAX_PDE: u8 = 3;

// --- parser  -----
pub struct Parser {
  lexer: Lexer,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Self {
    Self { lexer }
  }

  pub fn parse_program(&mut self) -> ast::Program {
    let mut stmts = Vec::new();
    while !self.lexer.is_end() {
      stmts.push(self.parse_stmt());
      if self.match_token(TokenType::EOF) {
        break;
      }
    }
    ast::Program { stmts }
  }

  // -------- statements -------
  fn parse_stmt(&mut self) -> ast::Stmts {
    let stmt = match self.lexer.peek_token().kind {
      TokenType::Let => ast::Stmts::Let(self.parse_let_stmt()),
      TokenType::Fn => self.parse_fn_stmt(),
      TokenType::LBrace => ast::Stmts::Block(self.parse_block_stmt()),
      _ => ast::Stmts::Expr(self.parse_expr(MIN_PDE)),
    };
    self.match_and_take(TokenType::Semi);
    stmt
  }

  fn parse_let_stmt(&mut self) -> ast::LetStmt {
    let mut range = self.take_or_expect(TokenType::Let); // take the `let`
    let pat = self.parse_pat();
    self.match_and_take(TokenType::Assign); // take the `=`
    let value = self.parse_expr(MIN_PDE);
    range.merge(value.get_range());
    ast::LetStmt::create(pat, value, range)
  }

  fn parse_fn_stmt(&mut self) -> ast::Stmts {
    let mut range = self.take_or_expect(TokenType::Fn); // take the `fn`
    if self.match_token(TokenType::LParen) {
      let expr = ast::Expr::Fn(self.parse_fn_expr(Some(range)));
      return ast::Stmts::Expr(expr);
    }
    let name = self.parse_ident();
    let inputs = self.parse_fn_pats();
    self.take_or_expect(TokenType::Assign); // take the `=`
    let body = self.parse_stmt();
    range.merge(body.get_range());
    let fn_stmt = ast::FunctionStmt::create(name, inputs, body, None, range);
    ast::Stmts::Fn(fn_stmt)
  }

  fn parse_fn_pats(&mut self) -> Vec<ast::Pat> {
    let mut inputs = Vec::new();
    self.take_or_expect(TokenType::LParen); // take the `(`
    while !self.match_token(TokenType::RParen) {
      inputs.push(self.parse_pat());
      if self.match_token(TokenType::RParen) {
        break;
      }
      self.take_or_expect(TokenType::Comma); // take the `,`
    }
    self.take_or_expect(TokenType::RParen); // take the `)`
    inputs
  }

  fn parse_block_stmt(&mut self) -> ast::BlockStmt {
    let mut range = self.take_or_expect(TokenType::LBrace); // take the `{`
    let mut stmts = Vec::new();
    while !self.match_token(TokenType::RBrace) {
      stmts.push(self.parse_stmt());
    }
    let end_range = self.take_or_expect(TokenType::RBrace); // take the `}`
    range.merge(&end_range);
    ast::BlockStmt::create(stmts, range)
  }

  // -------- expressions -------
  //

  fn parse_expr(&mut self, pde: u8) -> ast::Expr {
    let mut expr = self.parse_primary();
    while let Some(curr_pde) = self.get_pde() {
      if curr_pde < pde {
        break;
      }

      let range_op = self.lexer.peek_token().range;
      let operator = self.take_opeerator_exept();
      let right = self.parse_expr(curr_pde + 1);
      let mut range = expr.get_range().clone();
      range.merge(&right.get_range());

      let binary = ast::BinaryExpr { left: Box::new(expr), right: Box::new(right), range, range_op, operator };

      expr = ast::Expr::Binary(binary);
    }

    return expr;
  }

  fn parse_primary(&mut self) -> ast::Expr {
    let mut expr = match self.lexer.peek_token().kind {
      TokenType::Identifier => ast::Expr::Ident(self.parse_ident()),
      TokenType::Fn => ast::Expr::Fn(self.parse_fn_expr(None)),
      TokenType::Match => ast::Expr::Match(self.parse_match_expr()),
      _ => {
        let literal = self.parse_literal();
        ast::Expr::Literal(literal)
      }
    };

    while self.match_token(TokenType::DoubleColon) || self.match_token(TokenType::LParen) {
      expr = match self.lexer.peek_token().kind {
        // call expr
        TokenType::LParen => self.parse_call_expr(expr),
        // member expr
        TokenType::DoubleColon => self.parse_member_expr(expr),
        _ => continue,
      };
    }
    expr
  }

  // match <expr> { <arms> } or match _ { <arms> }
  fn parse_match_expr(&mut self) -> ast::MatchExpr {
    let mut range = self.take_or_expect(TokenType::Match); // take the `match`
    let expr = self.parse_expr(MIN_PDE);
    self.take_or_expect(TokenType::LBrace); // take the `{`
    let arms = self.parse_match_arms();
    range.merge(&self.take_or_expect(TokenType::RBrace)); // take the `}`
    ast::MatchExpr { expr: Box::new(expr), arms, range }
  }

  // <expr> => { <arms> }
  fn parse_match_arms(&mut self) -> Vec<ast::Arm> {
    let mut arms = Vec::new();
    while !self.match_token(TokenType::RBrace) {
      arms.push(self.parse_match_arm());
      if self.match_token(TokenType::RBrace) {
        break;
      }
      self.take_or_expect(TokenType::Comma); // take the `,`
    }
    arms
  }

  // <guard> => { <stmts> }
  fn parse_match_arm(&mut self) -> ast::Arm {
    let guard = self.parse_expr(MIN_PDE);
    let mut range = guard.get_range().clone();
    self.take_or_expect(TokenType::Arrow); // take the `=>`
    let body = self.parse_stmt();
    range.merge(body.get_range());
    ast::Arm { guard: Box::new(guard), body, range }
  }

  // fn(<pats>) = { <stmts> }
  fn parse_fn_expr(&mut self, fn_range: Option<Range>) -> ast::FnExpr {
    let mut range = fn_range.unwrap_or_else(|| self.take_or_expect(TokenType::Fn)); // take the `fn`
    let pats = self.parse_fn_pats();
    self.take_or_expect(TokenType::Assign); // take the `=`
    let body = self.parse_stmt();
    range.merge(body.get_range());
    ast::FnExpr { pats, body: Box::new(body), range }
  }

  fn parse_call_expr(&mut self, callee: ast::Expr) -> ast::Expr {
    let mut range = self.take_or_expect(TokenType::LParen); // take the `(`
    let mut args = vec![];
    while !self.match_token(TokenType::RParen) {
      args.push(self.parse_expr(MIN_PDE));
      if self.match_token(TokenType::RParen) {
        break;
      }
      self.take_or_expect(TokenType::Comma); // take the `,`
    }
    range.merge(&self.take_or_expect(TokenType::RParen)); // take the `)`
    range.merge(callee.get_range());
    ast::Expr::Call(ast::CallExpr { callee: Box::new(callee), args, range })
  }

  fn parse_member_expr(&mut self, object: ast::Expr) -> ast::Expr {
    self.take_or_expect(TokenType::DoubleColon); // take the `::`
    let property = self.parse_expr(MAX_PDE);
    let mut range = object.get_range().clone();
    range.merge(property.get_range());
    ast::Expr::Member(ast::MemberExpr { object: Box::new(object), property: Box::new(property), range })
  }

  fn parse_literal(&mut self) -> ast::Literal {
    let token = self.lexer.next_token();
    match token.kind {
      TokenType::Num => {
        let text = token.text.unwrap();
        ast::Literal::Number(ast::NumberLiteral { range: token.range.clone(), text })
      }
      TokenType::String => {
        let text = token.text.unwrap();
        ast::Literal::String(ast::StringLiteral { range: token.range.clone(), text })
      }
      TokenType::Bool => {
        let value = token.text.unwrap() == "true";
        ast::Literal::Boolean(ast::BooleanLiteral { range: token.range.clone(), value })
      }
      TokenType::Null => ast::Literal::Null(ast::NullLiteral { range: token.range.clone() }),
      _ => self.unexpected_expect(token, "literal"),
    }
  }

  // --- ast utils -----
  //

  fn parse_pat(&mut self) -> ast::Pat {
    let ident = self.parse_ident();
    let range = ident.get_range().clone();
    ast::Pat::create(ident, None, range)
  }

  fn parse_ident(&mut self) -> ast::Identifier {
    let token = self.lexer.next_token();
    if token.kind != TokenType::Identifier {
      self.report_expect(token.kind, TokenType::Identifier, &token.range)
    }
    ast::Identifier::create(token.text.clone().unwrap(), token.range.clone())
  }
  // -------- utils --------
  fn take_or_expect(&mut self, token_type: TokenType) -> Range {
    let token = self.lexer.next_token();
    if token.kind != token_type {
      self.report_expect(token.kind, token_type, &token.range);
    }
    token.range.clone()
  }

  fn match_token(&mut self, token_type: TokenType) -> bool {
    self.lexer.peek_token().kind == token_type
  }

  fn match_and_take(&mut self, token_type: TokenType) -> Option<Range> {
    if self.match_token(token_type) {
      Some(self.lexer.next_token().range)
    } else {
      None
    }
  }

  // --- precedence utils -----
  fn get_pde(&mut self) -> Option<u8> {
    match self.lexer.peek_token().kind {
      TokenType::Pipe => Some(MIN_PDE),
      TokenType::Plus | TokenType::Minus => Some(ADD_PDE),
      TokenType::Star | TokenType::Slash => Some(MUL_PDE),
      _ => None,
    }
  }

  fn take_opeerator_exept(&mut self) -> Operator {
    let token = self.lexer.next_token();
    if let Some(operator) = Operator::from_token(&token.kind) {
      operator
    } else {
      self.unexpected_expect(token, "operator")
    }
  }

  // --- error utils -----
  fn report_expect(&mut self, found: TokenType, expected: TokenType, range: &Range) -> ! {
    let message = format!("expected {:?}, found {:?}.", expected, found);
    report_and_exit(&message, range, &self.lexer.take_source())
  }

  fn unexpected_expect(&mut self, token: Token, expected: &str) -> ! {
    let message = format!("unexpected {:?}, expected {:?} token.", token.kind, expected);
    report_and_exit(&message, &token.range, &self.lexer.take_source())
  }
}
