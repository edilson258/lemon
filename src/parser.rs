use crate::ast::{self, Operator};
use crate::diag::Diag;
use crate::lexer::Lexer;
use crate::range::Range;
use crate::report::report_wrap;
use crate::tokens::{Token, TokenType};

// --- precedence utils -----
const MIN_PDE: u8 = 0; // e.g `|`, `..`
const CMP_PDE: u8 = 1; // e.g `<`, `<=`, `>`, `>=`
const ADD_PDE: u8 = 2; // e.g `+`, `-`
const MUL_PDE: u8 = 3; // e.g `*`, `/`, `%`
const MAX_PDE: u8 = 4; // e.g `^`, `**`

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
    let stmt = match self.peek_token().kind {
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
    let mut expr = self.parse_unary_expr();
    if self.match_token(TokenType::Pipe) {
      return self.parse_pipe_expr(expr);
    }
    while let Some(curr_pde) = self.get_pde() {
      if curr_pde < pde {
        break;
      }
      let range_op = self.peek_token().range;
      let operator = self.take_opeerator_exept();
      let right = self.parse_expr(curr_pde + 1);
      let mut range = expr.get_range().clone();
      range.merge(&right.get_range());

      let binary = ast::BinaryExpr { left: Box::new(expr), right: Box::new(right), range, range_op, operator };

      expr = ast::Expr::Binary(binary);

      if self.match_token(TokenType::Semi) {
        break;
      }
    }

    return expr;
  }

  fn parse_pipe_expr(&mut self, left: ast::Expr) -> ast::Expr {
    let mut expr = left;
    while self.match_token(TokenType::Pipe) {
      let range_op = self.take_or_expect(TokenType::Pipe); // take the `|>`
      let right = self.parse_expr(MIN_PDE);
      let mut range = expr.get_range().clone();
      range.merge(&right.get_range());
      expr = ast::Expr::Pipe(ast::PipeExpr { left: Box::new(expr), right: Box::new(right), range, range_op });
      if self.match_token(TokenType::Semi) {
        break;
      }
    }
    return expr;
  }

  fn parse_unary_expr(&mut self) -> ast::Expr {
    match self.peek_token().kind {
      TokenType::Minus => self.parse_unary_minus_expr(),
      TokenType::Bang => self.parse_unary_bang_expr(),
      _ => self.parse_primary(),
    }
  }

  // -------- unary expressions -------
  fn parse_unary_minus_expr(&mut self) -> ast::Expr {
    let token = self.lexer.next_token();
    let mut range = token.range.clone();
    let range_op = token.range.clone();
    let expr = self.parse_unary_expr();
    range.merge(&expr.get_range());
    ast::Expr::Unary(ast::UnaryExpr { operand: Box::new(expr), operator: ast::Operator::SUB, range, range_op })
  }

  fn parse_unary_bang_expr(&mut self) -> ast::Expr {
    let token = self.lexer.next_token();
    let mut range = token.range.clone();
    let range_op = token.range.clone();
    let expr = self.parse_unary_expr();
    range.merge(&expr.get_range());
    ast::Expr::Unary(ast::UnaryExpr { operand: Box::new(expr), operator: ast::Operator::NOT, range, range_op })
  }

  fn parse_primary(&mut self) -> ast::Expr {
    let mut expr = match self.peek_token().kind {
      TokenType::Identifier => ast::Expr::Ident(self.parse_ident()),
      TokenType::Import => ast::Expr::Import(self.parse_import_expr()),
      TokenType::Fn => ast::Expr::Fn(self.parse_fn_expr(None)),
      TokenType::Match => ast::Expr::Match(self.parse_match_expr()),
      TokenType::LBrace => ast::Expr::Object(self.parse_object_expr()),
      TokenType::LBracket => ast::Expr::Array(self.parse_array_expr()),
      TokenType::LParen => ast::Expr::Group(self.parse_group_expr()),
      _ => {
        let literal = self.parse_literal();
        ast::Expr::Literal(literal)
      }
    };

    while self.match_token(TokenType::ColonColon) || self.match_token(TokenType::LParen) {
      expr = match self.peek_token().kind {
        // call expr
        TokenType::LParen => self.parse_call_expr(expr),
        // member expr
        TokenType::ColonColon => self.parse_member_expr(expr),
        _ => continue,
      };
    }
    expr
  }

  // (<expr>)
  fn parse_group_expr(&mut self) -> ast::GroupExpr {
    let mut range = self.take_or_expect(TokenType::LParen); // take the `(`
    let expr = self.parse_expr(MIN_PDE);
    range.merge(&self.take_or_expect(TokenType::RParen)); // take the `)`
    ast::GroupExpr { range, expr: Box::new(expr) }
  }

  // import("path")
  fn parse_import_expr(&mut self) -> ast::ImportExpr {
    let mut range = self.take_or_expect(TokenType::Import); // take the `import`
    self.take_or_expect(TokenType::LParen); // take the `(`
    let path = self.parse_string_literal();
    let rigth_range = self.take_or_expect(TokenType::RParen); // take the `)`
    range.merge(&rigth_range);
    ast::ImportExpr { path, range }
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

  // { <ident>: <expr>, ... }
  fn parse_object_expr(&mut self) -> ast::ObjectExpr {
    let lt_range = self.take_or_expect(TokenType::LBrace); // take the `{`
    let fields = self.parse_object_fields();
    let rg_range = self.take_or_expect(TokenType::RBrace); // take the `}`
    let mut range = lt_range.clone();
    range.merge(&rg_range);
    return ast::ObjectExpr { fields, range };
  }

  fn parse_object_fields(&mut self) -> Vec<ast::Field> {
    let mut fields = Vec::new();
    while !self.match_token(TokenType::RBrace) {
      fields.push(self.parse_field());
      if self.match_token(TokenType::RBrace) {
        break;
      }
      self.take_or_expect(TokenType::Comma); // take the `,`
    }
    fields
  }
  fn parse_field(&mut self) -> ast::Field {
    let left = self.parse_ident();
    self.take_or_expect(TokenType::Colon); // take the `=`
    let right = self.parse_expr(MIN_PDE);
    let mut range = left.get_range().clone();
    range.merge(right.get_range());
    ast::Field { left, right: Box::new(right), range }
  }

  // [<expr>, ...]
  fn parse_array_expr(&mut self) -> ast::ArrayExpr {
    let mut range = self.take_or_expect(TokenType::LBracket); // take the `[`
    let mut fields = Vec::new();
    while !self.match_token(TokenType::RBracket) {
      fields.push(self.parse_expr(MIN_PDE));
      if self.match_token(TokenType::RBracket) {
        break;
      }
      self.take_or_expect(TokenType::Comma); // take the `,`
    }
    range.merge(&self.take_or_expect(TokenType::RBracket)); // take the `]`
    ast::ArrayExpr { range, fields }
  }

  // fn(<pats>) = { <stmts> }
  fn parse_fn_expr(&mut self, fn_range: Option<Range>) -> ast::FnExpr {
    let mut range = fn_range.unwrap_or_else(|| self.take_or_expect(TokenType::Fn)); // take the `fn`
    let mut name = None;
    if self.match_token(TokenType::Identifier) {
      name = Some(self.parse_ident())
    }
    let pats = self.parse_fn_pats();
    self.take_or_expect(TokenType::Assign); // take the `=`
    let body = self.parse_stmt();
    range.merge(body.get_range());
    ast::FnExpr { name, pats, body: Box::new(body), range }
  }

  fn parse_call_expr(&mut self, callee: ast::Expr) -> ast::Expr {
    let mut range = callee.get_range().clone();
    self.take_or_expect(TokenType::LParen); // take the `(`
    let mut args = vec![];
    while !self.match_token(TokenType::RParen) {
      args.push(self.parse_expr(MIN_PDE));
      if self.match_token(TokenType::RParen) {
        break;
      }
      self.take_or_expect(TokenType::Comma); // take the `,`
    }
    range.merge(&self.take_or_expect(TokenType::RParen)); // take the `)`
    ast::Expr::Call(ast::CallExpr { callee: Box::new(callee), args, range })
  }

  fn parse_member_expr(&mut self, object: ast::Expr) -> ast::Expr {
    self.take_or_expect(TokenType::ColonColon); // take the `::`
    let property = self.parse_expr(MAX_PDE);
    let mut range = object.get_range().clone();
    range.merge(property.get_range());
    ast::Expr::Member(ast::MemberExpr { object: Box::new(object), property: Box::new(property), range })
  }

  fn parse_literal(&mut self) -> ast::Literal {
    let token = self.next_token();
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

  fn parse_string_literal(&mut self) -> ast::StringLiteral {
    let token = self.next_token();
    match token.kind {
      TokenType::String => {
        let text = token.text.unwrap();
        ast::StringLiteral { range: token.range.clone(), text }
      }
      _ => self.unexpected_expect(token, "string literal"),
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
    let token = self.next_token();
    if token.kind != TokenType::Identifier {
      self.report_expect(token.kind, TokenType::Identifier, &token.range)
    }
    ast::Identifier::create(token.text.clone().unwrap(), token.range.clone())
  }
  // -------- utils --------
  fn take_or_expect(&mut self, token_type: TokenType) -> Range {
    let token = self.next_token();
    if token.kind != token_type {
      self.report_expect(token.kind, token_type, &token.range);
    }
    token.range.clone()
  }

  fn match_token(&mut self, token_type: TokenType) -> bool {
    self.peek_token().kind == token_type
  }

  fn match_and_take(&mut self, token_type: TokenType) -> Option<Range> {
    if self.match_token(token_type) {
      Some(self.next_token().range)
    } else {
      None
    }
  }

  // --- precedence utils -----
  #[rustfmt::skip]
  fn get_pde(&mut self) -> Option<u8> {
    match self.peek_token().kind {
      TokenType::Pipe | TokenType::Bar| TokenType::DotDot => Some(MIN_PDE),

      TokenType::Less | TokenType::LessEq |
      TokenType::Greater | TokenType::GreaterEq |
      TokenType::NotEq => Some(CMP_PDE),

      TokenType::Plus | TokenType::Minus => Some(ADD_PDE),
      TokenType::Star | TokenType::Slash => Some(MUL_PDE),
      TokenType::Rem | TokenType::RemEq => Some(MUL_PDE),
      _ => None,
    }
  }

  fn take_opeerator_exept(&mut self) -> Operator {
    let token = self.next_token();
    if let Some(operator) = Operator::from_token(&token.kind) {
      operator
    } else {
      self.unexpected_expect(token, "operator")
    }
  }

  // --- error utils -----
  fn report_expect(&mut self, found: TokenType, expected: TokenType, range: &Range) -> ! {
    let message = format!("expected {}, found {}.", expected, found);
    let diag = self.create_diag(&message, range);
    report_wrap(&diag);
  }

  fn unexpected_expect(&mut self, token: Token, expected: &str) -> ! {
    let message = format!("unexpected {}, expected {} token.", token.kind, expected);
    let diag = self.create_diag(&message, &token.range);
    report_wrap(&diag);
  }

  fn create_diag(&self, message: &str, range: &Range) -> Diag {
    Diag::create_err(message.to_string(), range.clone(), self.lexer.take_source().path.clone())
  }
  fn next_token(&mut self) -> Token {
    let token = self.lexer.next_token();
    match token.kind {
      TokenType::SkipLine | TokenType::SkipBlock => self.next_token(),
      _ => token,
    }
  }

  fn peek_token(&mut self) -> Token {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenType::SkipLine | TokenType::SkipBlock => {
        self.lexer.next_token();
        self.peek_token()
      }
      _ => token,
    }
  }
}
