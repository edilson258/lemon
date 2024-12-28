use logos::Lexer;

use crate::ast::{self, BASE_BIN, BASE_DECIMAL, BASE_HEX, MIN_PDE};
use crate::diag::Diag;
use crate::lexer::Token;
use crate::range::Range;
mod parse_type;

// --- pde utils -----

// state

pub struct Parser<'lex> {
	pub lex: &'lex mut Lexer<'lex, Token>,
	token: Option<Token>,
	range: Range,
}
// --- parser  -----

type PResult<'lex, T> = Result<T, Diag>;

impl<'lex> Parser<'lex> {
	pub fn new(lex: &'lex mut Lexer<'lex, Token>) -> Self {
		// todo: remove unwrap
		let token = lex.next().map(|t| t.unwrap());
		let range = Range::from_span(lex.span());
		Self { lex, token, range }
	}
	pub fn parse_program(&mut self) -> PResult<'lex, ast::Program> {
		let mut stmts = vec![];
		while !self.is_end() {
			stmts.push(self.parse_stmt()?);
		}
		Ok(ast::Program { stmts })
	}

	fn parse_stmt(&mut self) -> PResult<'lex, ast::Stmt> {
		let stmt = match self.token {
			Some(Token::Let) => self.parse_let_stmt().map(ast::Stmt::Let),
			Some(Token::Const) => self.parse_const_stmt().map(ast::Stmt::Const),
			Some(Token::Fn) => self.parse_fn_stmt().map(ast::Stmt::Fn),
			Some(Token::LBrace) => self.parse_block_stmt().map(ast::Stmt::Block),
			Some(Token::Ret) => self.parse_ret_stmt().map(ast::Stmt::Ret),
			_ => self.parse_expr(MIN_PDE).map(ast::Stmt::Expr),
		};
		self.match_take(Token::Semi)?;
		stmt
	}
	fn parse_ret_stmt(&mut self) -> PResult<'lex, ast::RetStmt> {
		let range = self.expect(Token::Ret)?.clone();
		let mut expr = None;
		if !self.match_token(Token::Semi) {
			expr = Some(Box::new(self.parse_expr(MIN_PDE)?));
		}
		Ok(ast::RetStmt { expr, range, type_id: None })
	}
	fn parse_const_stmt(&mut self) -> PResult<'lex, ast::ConstStmt> {
		let range = self.expect(Token::Const)?.clone();
		let name = self.parse_binding()?;
		self.expect(Token::Assign)?; // take '='
		let expr = self.parse_expr(MIN_PDE)?;
		Ok(ast::ConstStmt { name, expr, range, type_id: None })
	}

	fn parse_let_stmt(&mut self) -> PResult<'lex, ast::LetStmt> {
		let range = self.expect(Token::Let)?.clone();
		let mut mutable = None;

		if self.match_token(Token::Mut) {
			mutable = Some(self.expect(Token::Mut)?);
		}

		let name = self.parse_binding()?;
		self.expect(Token::Assign)?; // =
		let expr = self.parse_expr(MIN_PDE)?;
		Ok(ast::LetStmt { name, mutable, expr, range, type_id: None })
	}

	fn parse_fn_stmt(&mut self) -> PResult<'lex, ast::FnStmt> {
		let range = self.expect(Token::Fn)?.clone();
		let name = self.parse_ident()?;
		self.expect(Token::LParen)?;

		let mut params = vec![];
		while !self.match_token(Token::RParen) {
			params.push(self.parse_binding()?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}

		self.expect(Token::RParen)?; // take ')'

		let mut return_type = None;
		if self.match_token(Token::Colon) {
			self.expect(Token::Colon)?;
			return_type = Some(self.parse_type()?);
		}
		self.expect(Token::Assign)?; // take '='
		let body = Box::new(self.parse_stmt()?);
		Ok(ast::FnStmt { name, params, return_type, body, range, type_id: None })
	}

	fn parse_block_stmt(&mut self) -> PResult<'lex, ast::BlockStmt> {
		let mut range = self.expect(Token::LBrace)?.clone();
		let mut stmts = vec![];
		while !self.match_token(Token::RBrace) {
			let stmt = self.parse_stmt()?;
			stmts.push(stmt);
		}
		range.merge(&self.expect(Token::RBrace)?);

		Ok(ast::BlockStmt::new(stmts, range))
	}

	fn parse_expr(&mut self, pde: u8) -> PResult<'lex, ast::Expr> {
		let mut left = self.parse_primary()?;
		while let Some((operator, range)) = self.match_operator(pde) {
			let right = self.parse_expr(pde + 1)?;
			left = ast::Expr::Binary(ast::BinaryExpr {
				left: Box::new(left),
				operator,
				right: Box::new(right),
				range,
				type_id: None,
			});
		}
		Ok(left)
	}

	fn match_operator(&mut self, pde: u8) -> Option<(ast::Operator, Range)> {
		if let Some(operator) = self.token.as_ref().and_then(ast::Operator::from_token) {
			if operator.pde() >= pde {
				let range = self.range.to_owned();
				self.next().ok();
				return Some((operator, range));
			}
		}
		None
	}

	fn parse_primary(&mut self) -> PResult<'lex, ast::Expr> {
		let expr = match self.token {
			Some(Token::Ident) => self.parse_ident().map(ast::Expr::Ident)?,
			Some(Token::And) => self.parse_ref_expr().map(ast::Expr::Ref)?,
			Some(Token::Star) => self.parse_deref_expr().map(ast::Expr::Deref)?,
			Some(Token::Char) => self.parse_char().map(ast::Expr::Literal)?,
			Some(Token::String) => self.parse_string().map(ast::Expr::Literal)?,
			Some(Token::Fn) => self.parse_fn_expr().map(ast::Expr::Fn)?,
			Some(Token::If) => self.parse_if_expr().map(ast::Expr::If)?,
			Some(Token::Decimal) | Some(Token::Hex) | Some(Token::Bin) => {
				self.parse_numb().map(ast::Expr::Literal)?
			}
			_ => return Err(self.unexpected_token()),
		};
		let expr = self.parse_right_hand_expr(expr)?;
		Ok(expr)
	}

	fn parse_right_hand_expr(&mut self, left: ast::Expr) -> PResult<'lex, ast::Expr> {
		let mut expr = left;
		loop {
			expr = match self.token {
				Some(Token::LParen) => self.parse_call_expr(expr)?,
				// assign
				Some(Token::Assign) => self.parse_assign_expr(expr)?,
				Some(Token::Pipe) => self.parse_pipe_expr(expr)?,
				// Some(Token::Dot) => self.parse_member_expr(expr)?,
				// Some(Token::LBracket) => self.parse_index_expr(expr)?,
				_ => break,
			};
		}
		Ok(expr)
	}

	// &mut <expr>
	fn parse_ref_expr(&mut self) -> PResult<'lex, ast::RefExpr> {
		let range = self.expect(Token::And)?.clone();
		let mut mutable = None;
		if self.match_token(Token::Mut) {
			mutable = Some(self.expect(Token::Mut)?);
		}
		let expr = self.parse_ref_and_deref_value()?;
		Ok(ast::RefExpr { expr: Box::new(expr), range, mutable, type_id: None })
	}

	// *<expr>
	fn parse_deref_expr(&mut self) -> PResult<'lex, ast::DerefExpr> {
		let range = self.expect(Token::Star)?.clone();
		let expr = self.parse_ref_and_deref_value()?;
		Ok(ast::DerefExpr { expr: Box::new(expr), range, type_id: None })
	}

	fn parse_ref_and_deref_value(&mut self) -> PResult<'lex, ast::Expr> {
		let exper = match self.token {
			Some(Token::Ident) => self.parse_ident().map(ast::Expr::Ident)?,
			Some(Token::And) => self.parse_ref_expr().map(ast::Expr::Ref)?,
			Some(Token::Star) => self.parse_deref_expr().map(ast::Expr::Deref)?,
			Some(token) => {
				let message = format!("expected identifier, ref or deref, but got '{}'", token);
				let diag = self.create_custom_diag(message);
				return Err(diag);
			}
			_ => return Err(self.unexpected_token()),
		};
		let expr = self.parse_right_hand_expr(exper)?;
		Ok(expr)
	}

	fn parse_char(&mut self) -> PResult<'lex, ast::Literal> {
		self.ensure_char()?;
		let range = self.take_range();
		let text = self.take_text_and_next()?;
		let value = text.chars().nth(1).unwrap(); // skipe '
		let char = ast::CharLiteral { value, range };
		Ok(ast::Literal::Char(char))
	}

	fn parse_string(&mut self) -> PResult<'lex, ast::Literal> {
		if !self.match_token(Token::String) {
			self.expect(Token::String)?;
		}
		let range = self.take_range();
		let text = self.take_text_and_next()?;
		let string = ast::StringLiteral { text, range };
		Ok(ast::Literal::String(string))
	}

	fn parse_numb(&mut self) -> PResult<'lex, ast::Literal> {
		self.ensure_numb()?;
		let range = self.take_range();
		let text = self.take_text_and_next()?;
		let (base, cleaned_text) = self.detect_numb_base(&text);
		let as_dot = text.contains('.');
		let text = self.normalize_number(&cleaned_text);
		let number = ast::NumberLiteral { base, as_dot, text, range };
		Ok(ast::Literal::Number(number))
	}

	fn parse_fn_expr(&mut self) -> PResult<'lex, ast::FnExpr> {
		let range = self.expect(Token::Fn)?.clone();
		let mut params = vec![];
		self.expect(Token::LParen)?; // take '('
		while !self.match_token(Token::RParen) {
			params.push(self.parse_binding()?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}
		self.expect(Token::RParen)?; // take ')'

		let return_type = match self.token {
			Some(Token::Colon) => {
				self.expect(Token::Colon)?;
				Some(self.parse_type()?)
			}
			_ => None,
		};

		self.expect(Token::Assign)?; // take '='

		let body = Box::new(self.parse_stmt()?);

		Ok(ast::FnExpr { params, body, range, return_type, type_id: None })
	}

	fn parse_if_expr(&mut self) -> PResult<'lex, ast::IfExpr> {
		let range = self.expect(Token::If)?.clone();
		self.expect(Token::LParen)?; // take '('
		let cond = Box::new(self.parse_expr(MIN_PDE)?);
		self.expect(Token::RParen)?; // take ')'
		let then = Box::new(self.parse_stmt()?);
		let mut otherwise = None;
		if self.match_token(Token::Else) {
			self.expect(Token::Else)?;
			otherwise = Some(Box::new(self.parse_stmt()?));
		}
		Ok(ast::IfExpr { cond, then, otherwise, range })
	}

	fn parse_call_expr(&mut self, callee: ast::Expr) -> PResult<'lex, ast::Expr> {
		let mut range = self.expect(Token::LParen)?; // consume '('
		let mut args = Vec::new();
		while !self.match_token(Token::RParen) {
			args.push(self.parse_expr(MIN_PDE)?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}
		range.merge(&self.expect(Token::RParen)?); // consume ')'
		let call_expr = ast::CallExpr { callee: Box::new(callee), args, range, type_id: None };
		Ok(ast::Expr::Call(call_expr))
	}

	fn parse_assign_expr(&mut self, left: ast::Expr) -> PResult<'lex, ast::Expr> {
		let range = self.expect(Token::Assign)?.clone();
		if !left.valid_assign_expr() {
			let diag = self.create_custom_diag("left-hand side can't be assigned");
			return Err(diag);
		}
		let right = self.parse_expr(MIN_PDE)?;
		let assign_expr =
			ast::AssignExpr { left: Box::new(left), right: Box::new(right), range, type_id: None };
		Ok(ast::Expr::Assign(assign_expr))
	}

	fn parse_pipe_expr(&mut self, left: ast::Expr) -> PResult<'lex, ast::Expr> {
		let range = self.expect(Token::Pipe)?.clone();
		let right = self.parse_expr(MIN_PDE)?;
		let pipe_expr = ast::PipeExpr { left: Box::new(left), right: Box::new(right), range };
		Ok(ast::Expr::Pipe(pipe_expr))
	}

	// fn parse_member_exper(&mut self, obj: ast::Expr) -> PResult<'lex, ast::Expr> {
	//   self.expect(Token::Dot)?; // consume '.'
	//   let member = self.parse_ident()?;
	//   Ok(ast::Expr::MemberAccess { object: Box::new(obj), member })
	// }

	// fn parse_index_exper(&mut self, array: ast::Expr) -> PResult<'lex, ast::Expr> {
	//   self.expect(Token::LBracket)?; // consume '['
	//   let index = self.parse_expr(MIN_PDE)?;
	//   self.expect(Token::RBracket)?; // consume ']'
	//   Ok(ast::Expr::IndexAccess { array: Box::new(array), index: Box::new(index) })
	// }

	// helpers

	fn ensure_numb(&mut self) -> PResult<'lex, ()> {
		if !matches!(self.token, Some(Token::Decimal | Token::Hex | Token::Bin)) {
			self.expect(Token::Decimal)?;
		}
		Ok(())
	}

	fn ensure_char(&mut self) -> PResult<'lex, ()> {
		if !matches!(self.token, Some(Token::Char)) {
			self.expect(Token::Char)?;
		}

		// include "'"
		if self.take_text().len() > 3 {
			let diag = Diag::error("expected char literal", self.range.clone());
			return Err(diag);
		}
		Ok(())
	}

	fn detect_numb_base(&self, text: &str) -> (u8, String) {
		if text.starts_with("0x") {
			return (BASE_HEX, text.trim_start_matches("0x").to_string());
		}

		if text.starts_with("0b") {
			return (BASE_BIN, text.trim_start_matches("0b").to_string());
		}

		(BASE_DECIMAL, text.to_string())
	}

	fn parse_binding(&mut self) -> PResult<'lex, ast::Binding> {
		let ident = self.parse_ident()?;
		let mut ty = None;
		if self.match_token(Token::Colon) {
			self.expect(Token::Colon)?;
			ty = Some(self.parse_type()?);
		}
		Ok(ast::Binding { ident, ty, type_id: None })
	}

	fn parse_ident(&mut self) -> PResult<'lex, ast::Ident> {
		if !self.match_token(Token::Ident) {
			self.expect(Token::Ident)?;
		}
		let range = self.range.clone();
		let text = self.take_text_and_next()?;
		Ok(ast::Ident { text, range, type_id: None })
	}

	// helpers
	//

	fn normalize_number(&self, text: &str) -> String {
		text.replace('_', "")
	}

	fn is_end(&self) -> bool {
		self.token.is_none()
	}

	fn take_text(&mut self) -> &'lex str {
		self.lex.slice()
	}

	fn take_text_and_next(&mut self) -> PResult<'lex, String> {
		let text = self.take_text();
		self.next()?;
		Ok(text.to_string())
	}

	fn next(&mut self) -> PResult<'lex, Option<Token>> {
		let temp = self.token.take();
		self.token = self.lex.next().transpose().map_err(|_| self.unexpected_token())?;
		self.range = Range::from_span(self.lex.span());
		Ok(temp)
	}

	fn take_range(&mut self) -> Range {
		self.range.clone()
	}

	fn match_take(&mut self, token: Token) -> PResult<'lex, Option<&Range>> {
		if self.match_token(token) {
			self.next()?;
			Ok(Some(&self.range))
		} else {
			Ok(None)
		}
	}

	fn expect(&mut self, token: Token) -> PResult<'lex, Range> {
		if !self.match_token(token) {
			// todo: add error message
			let peeked = self.token.map(|t| t.to_string()).unwrap_or_else(|| "unkown".to_string());
			let diag = Diag::error(format!("expected {} but got {}", token, peeked), self.range.clone());
			return Err(diag);
		}
		let range = self.range.clone();
		self.next()?;
		Ok(range)
	}

	fn match_token(&mut self, token: Token) -> bool {
		self.token.as_ref().map(|t| *t == token).unwrap_or(false)
	}

	fn unexpected_token(&mut self) -> Diag {
		Diag::error(format!("unexpected token '{}'", self.token.as_ref().unwrap()), self.range.clone())
	}

	pub fn create_custom_diag(&self, message: impl Into<String>) -> Diag {
		Diag::error(message, self.range.clone())
	}
}
