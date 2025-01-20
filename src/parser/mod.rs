use logos::Lexer;

use crate::ast::{self, OperatorKind, BASE_BIN, BASE_DECIMAL, BASE_HEX, MIN_PDE};
use crate::diag::Diag;
use crate::lexer::Token;
use crate::loader::FileId;
use crate::range::Range;
mod parse_type;

// --- pde utils -----

// state

pub struct Parser<'lex> {
	pub lex: &'lex mut Lexer<'lex, Token>,
	token: Option<Token>,
	range: Range,
	file_id: FileId,
}
// --- parser  -----

type PResult<'lex, T> = Result<T, Diag>;

impl<'lex> Parser<'lex> {
	pub fn new(lex: &'lex mut Lexer<'lex, Token>, file_id: FileId) -> Self {
		// todo: remove unwrap
		let token = lex.next().map(|t| t.unwrap());
		let range = Range::from_span(lex.span());
		Self { lex, token, range, file_id }
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
			Some(Token::Const) => self.parse_const_stmt(),
			Some(Token::Fn) => self.parse_fn_stmt().map(ast::Stmt::Fn),
			Some(Token::LBrace) => self.parse_block_stmt().map(ast::Stmt::Block),
			Some(Token::Ret) => self.parse_ret_stmt().map(ast::Stmt::Ret),
			Some(Token::Extern) => self.parse_extern_fn_stmt().map(ast::Stmt::ExternFn),
			Some(Token::While) => self.parse_while_stmt().map(ast::Stmt::While),
			Some(Token::For) => self.parse_for_stmt().map(ast::Stmt::For),
			Some(Token::Type) => self.parse_type_def_stmt().map(ast::Stmt::TypeDef),
			_ => self.parse_expr(MIN_PDE).map(ast::Stmt::Expr),
		};
		self.match_take(Token::Semi)?;
		stmt
	}
	fn parse_ret_stmt(&mut self) -> PResult<'lex, ast::RetStmt> {
		let range = self.expect(Token::Ret)?;
		let mut expr = None;
		if !self.match_token(Token::Semi) {
			expr = Some(Box::new(self.parse_expr(MIN_PDE)?));
		}
		Ok(ast::RetStmt { expr, range, type_id: None })
	}
	fn parse_const_stmt(&mut self) -> PResult<'lex, ast::Stmt> {
		let range = self.expect(Token::Const)?;
		if self.match_token(Token::Fn) {
			return self.parse_const_fn_stmt(range).map(ast::Stmt::ConstFn);
		}
		self.parse_const_del_stmt(range).map(ast::Stmt::ConstDel)
	}

	fn parse_const_fn_stmt(&mut self, range: Range) -> PResult<'lex, ast::ConstFnStmt> {
		let fn_range = self.expect(Token::Fn)?;
		let name = self.parse_ident()?;
		self.expect(Token::LParen)?;

		let mut params = vec![];
		while !self.match_token(Token::RParen) {
			params.push(self.parse_binding()?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}
		self.expect(Token::RParen)?;

		let mut ret_type = None;
		if self.match_token(Token::Colon) {
			self.expect(Token::Colon)?;
			ret_type = Some(self.parse_type()?);
		}
		self.expect(Token::Assign)?; // take '='
		let body = self.parse_fn_body()?;
		Ok(ast::ConstFnStmt { name, params, ret_type, body, range, fn_range, ret_id: None })
	}

	fn parse_const_del_stmt(&mut self, range: Range) -> PResult<'lex, ast::ConstDelStmt> {
		let name = self.parse_binding()?;
		self.expect(Token::Assign)?; // take '='
		let expr = self.parse_expr(MIN_PDE)?;
		Ok(ast::ConstDelStmt { name, expr, range, type_id: None })
	}

	// type <name> = {} or type <name> = <type>
	fn parse_type_def_stmt(&mut self) -> PResult<'lex, ast::TypeDefStmt> {
		let range = self.expect(Token::Type)?;
		let name = self.parse_ident()?;
		self.expect(Token::Assign)?; // take '='

		if self.match_token(Token::LBrace) {
			let kind = ast::TypeDefKind::Struct(self.parse_struct_def()?);
			return Ok(ast::TypeDefStmt { name, range, kind });
		}
		let kind = ast::TypeDefKind::Alias(self.parse_type()?);
		Ok(ast::TypeDefStmt { name, range, kind })
	}

	pub fn parse_struct_def(&mut self) -> PResult<'lex, ast::StructType> {
		let mut range = self.expect(Token::LBrace)?;
		let mut fields = vec![];
		while !self.match_token(Token::RBrace) {
			let ident = self.parse_ident()?;
			self.expect(Token::Colon)?;
			let ast_type = self.parse_type()?;
			fields.push(ast::FieldType { ident, ast_type, is_pub: false });
			if !self.match_token(Token::RBrace) {
				self.expect(Token::Comma)?;
			}
		}
		range.merge(&self.expect(Token::RBrace)?);
		Ok(ast::StructType { fields, range })
	}

	fn parse_let_stmt(&mut self) -> PResult<'lex, ast::LetStmt> {
		let range = self.expect(Token::Let)?;
		let mut mutable = None;

		if self.match_token(Token::Mut) {
			mutable = Some(self.expect(Token::Mut)?);
		}

		let bind = self.parse_binding()?;
		self.expect(Token::Assign)?; // =
		let expr = self.parse_expr(MIN_PDE)?;
		Ok(ast::LetStmt { bind, mutable, expr, range, type_id: None })
	}

	fn parse_fn_stmt(&mut self) -> PResult<'lex, ast::FnStmt> {
		let range = self.expect(Token::Fn)?;
		let name = self.parse_ident()?;
		let generics = self.parse_generics()?;
		self.expect(Token::LParen)?;
		let mut params = vec![];
		while !self.match_token(Token::RParen) {
			params.push(self.parse_binding()?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}

		self.expect(Token::RParen)?; // take ')'

		let mut ret_type = None;
		if self.match_token(Token::Colon) {
			self.expect(Token::Colon)?;
			ret_type = Some(self.parse_type()?);
		}
		self.expect(Token::Assign)?; // take '='
		let body = self.parse_fn_body()?;
		Ok(ast::FnStmt { name, generics, params, ret_type, body, range, ret_id: None })
	}

	// <T, U: Eq>
	fn parse_generics(&mut self) -> PResult<'lex, Vec<ast::Generic>> {
		let mut generics = vec![];
		if !self.match_token(Token::Less) {
			return Ok(generics);
		}
		self.expect(Token::Less)?;
		while !self.match_token(Token::Greater) {
			generics.push(self.parse_generic()?);
			if !self.match_token(Token::Comma) {
				break;
			}
			self.expect(Token::Comma)?;
		}

		self.expect(Token::Greater)?;
		Ok(generics)
	}

	fn parse_generic(&mut self) -> PResult<'lex, ast::Generic> {
		let ident = self.parse_ident()?;
		let mut bound = None;
		if self.match_token(Token::Colon) {
			self.expect(Token::Colon)?;
			bound = Some(self.parse_type()?);
		}
		Ok(ast::Generic { ident, bound })
	}

	fn parse_fn_body(&mut self) -> PResult<'lex, ast::FnBody> {
		if self.match_token(Token::LBrace) {
			let block = self.parse_block_stmt()?;
			return Ok(ast::FnBody::Block(block));
		};
		let expr = self.parse_expr(MIN_PDE)?;
		Ok(ast::FnBody::Expr(expr))
	}

	fn parse_block_stmt(&mut self) -> PResult<'lex, ast::BlockStmt> {
		let mut range = self.expect(Token::LBrace)?;
		let mut stmts = vec![];
		while !self.match_token(Token::RBrace) {
			let stmt = self.parse_stmt()?;
			stmts.push(stmt);
		}
		range.merge(&self.expect(Token::RBrace)?);

		Ok(ast::BlockStmt::new(stmts, range))
	}

	// extern fn <name>(<pats>): <type> = { }
	fn parse_extern_fn_stmt(&mut self) -> PResult<'lex, ast::ExternFnStmt> {
		let range = self.expect(Token::Extern)?;
		let fn_range = self.expect(Token::Fn)?;
		let name = self.parse_ident()?;
		self.expect(Token::LParen)?;
		let mut var_packed = None;
		let mut params = vec![];
		while !self.match_token(Token::RParen) {
			if self.match_token(Token::DotDotDot) {
				var_packed = Some(self.expect(Token::DotDotDot)?);
				break;
			}
			params.push(self.parse_binding()?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}
		self.expect(Token::RParen)?;

		let mut ret_type = None;
		if self.match_token(Token::Colon) {
			self.expect(Token::Colon)?;
			ret_type = Some(self.parse_type()?);
		}
		self.expect(Token::Assign)?; // take '='

		// we need to parse block stmt here?
		self.expect(Token::LBrace)?;
		self.expect(Token::RBrace)?;
		let ret_id = None;
		Ok(ast::ExternFnStmt { name, params, ret_type, range, fn_range, var_packed, ret_id })
	}

	fn parse_while_stmt(&mut self) -> PResult<'lex, ast::WhileStmt> {
		let range = self.expect(Token::While)?;
		self.expect(Token::LParen)?;
		let test = Box::new(self.parse_expr(MIN_PDE)?);
		self.expect(Token::RParen)?;

		self.expect(Token::Assign)?;

		let body = Box::new(self.parse_stmt()?);

		Ok(ast::WhileStmt { test, body, range })
	}

	fn parse_for_stmt(&mut self) -> PResult<'lex, ast::ForStmt> {
		todo!("parse for stmt");
		// let range = self.expect(Token::For)?;
		// let value = self.parse_ident()?;
		// self.expect(Token::In)?;
		// let iterable = Box::new(self.parse_expr(MIN_PDE)?);
		// self.expect(Token::LBrace)?;
		// let body = self.parse_stmt()?;
		// self.expect(Token::RBrace)?;
		// Ok(ast::Stmt::For(ast::ForStmt { value, index: None, iterable, body, range }))
	}

	fn parse_expr(&mut self, min_pde: u8) -> PResult<'lex, ast::Expr> {
		let mut left = self.parse_primary(true)?;
		while let Some(operator) = self.match_operator(min_pde)? {
			let right = self.parse_expr(min_pde + 1)?;
			left = ast::Expr::Binary(ast::BinaryExpr::new(Box::new(left), operator, Box::new(right)));
		}
		Ok(left)
	}

	fn match_operator(&mut self, min_pde: u8) -> PResult<'lex, Option<ast::Operator>> {
		if let Some(operator) = self.parse_operator()? {
			if operator.pde() >= min_pde {
				return Ok(Some(operator));
			}
		}
		Ok(None)
	}

	fn parse_operator(&mut self) -> PResult<'lex, Option<ast::Operator>> {
		let kind = match self.token {
			Some(Token::Plus) => OperatorKind::ADD,
			Some(Token::Minus) => OperatorKind::SUB,
			Some(Token::Star) => OperatorKind::MUL,
			Some(Token::Slash) => OperatorKind::DIV,
			Some(Token::Eq) => OperatorKind::EQ,
			Some(Token::NotEq) => OperatorKind::NOTEQ,
			Some(Token::LessEq) => OperatorKind::LE,
			Some(Token::GreaterEq) => OperatorKind::GE,
			Some(Token::Less) => OperatorKind::LT,
			Some(Token::Greater) => OperatorKind::GT,
			Some(Token::And) => OperatorKind::AND,
			Some(Token::BarBar) => OperatorKind::OR,
			Some(Token::DotDot) => OperatorKind::RANGE,
			Some(Token::Rem) => OperatorKind::MOD,
			Some(Token::RemEq) => OperatorKind::MODEQ,
			Some(Token::Bar) => OperatorKind::BOR,
			Some(Token::Pow) => OperatorKind::POW,
			Some(Token::Pipe) => OperatorKind::PIPE,
			Some(Token::PlusEq) => OperatorKind::ADDEQ,
			Some(Token::MinusEq) => OperatorKind::SUBEQ,
			Some(Token::StarEq) => OperatorKind::MODEQ,
			Some(Token::SlashEq) => OperatorKind::DIVEQ,
			Some(Token::Bang) => OperatorKind::NOT,
			_ => return Ok(None),
		};
		let range = self.range.clone();
		self.next()?;
		Ok(Some(ast::Operator { kind, range }))
	}

	fn parse_primary(&mut self, with_right_hand: bool) -> PResult<'lex, ast::Expr> {
		let expr = match self.token {
			Some(Token::Ident) => self.parse_ident().map(ast::Expr::Ident)?,
			Some(Token::And) => self.parse_borrow_expr().map(ast::Expr::Borrow)?,
			Some(Token::Star) => self.parse_deref_expr().map(ast::Expr::Deref)?,
			Some(Token::Char) => self.parse_char().map(ast::Expr::Literal)?,
			Some(Token::String) => self.parse_string().map(ast::Expr::Literal)?,
			Some(Token::Fn) => self.parse_fn_expr().map(ast::Expr::Fn)?,
			Some(Token::If) => self.parse_if_expr().map(ast::Expr::If)?,
			Some(Token::Import) => self.parse_import_expr().map(ast::Expr::Import)?,
			Some(Token::Decimal) | Some(Token::Hex) | Some(Token::Bin) => {
				self.parse_numb().map(ast::Expr::Literal)?
			}
			_ => return Err(self.unexpected_token()),
		};
		if with_right_hand {
			return self.parse_right_hand_expr(expr);
		}
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
				Some(Token::ColonColon) => self.parse_associate_expr(expr)?,
				Some(Token::Dot) => self.parse_member_expr(expr)?,
				// Some(Token::LBracket) => self.parse_index_expr(expr)?,
				Some(Token::LBrace) => self.parse_struct_init_expr(expr).map(ast::Expr::StructInit)?,
				_ => break,
			};
		}
		Ok(expr)
	}
	// {  ident, ident: <expr> ... } todo: suport only { ident } maybe convert to { ident: ident }?
	fn parse_struct_init_expr(&mut self, expr: ast::Expr) -> PResult<'lex, ast::StructInitExpr> {
		if let ast::Expr::Ident(name) = expr {
			let mut range = self.expect(Token::LBrace)?;
			let mut fields = vec![];
			while !self.match_token(Token::RBrace) {
				let name = self.parse_ident()?;
				self.expect(Token::Colon)?;
				let value = self.parse_expr(MIN_PDE)?;
				let range = name.range.merged_with(&value.get_range());
				fields.push(ast::FiledExpr { name, value, range });
				if !self.match_token(Token::RBrace) {
					self.expect(Token::Comma)?;
				}
			}
			range.merge(&self.expect(Token::RBrace)?);
			return Ok(ast::StructInitExpr { name, fields, range });
		}
		//
		let diag = self.custom_diag("expected struct name", &expr.get_range());
		Err(diag.with_file_id(self.file_id))
	}

	// ::<expr>
	fn parse_associate_expr(&mut self, expr: ast::Expr) -> PResult<'lex, ast::Expr> {
		let range = self.expect(Token::ColonColon)?;
		let right = self.parse_expr(MIN_PDE)?;
		Ok(ast::Expr::Associate(ast::AssociateExpr {
			left: Box::new(expr),
			right: Box::new(right),
			range,
		}))
	}

	// exper.<expr>
	fn parse_member_expr(&mut self, expr: ast::Expr) -> PResult<'lex, ast::Expr> {
		let range = self.expect(Token::Dot)?;
		let right = self.parse_expr(MIN_PDE)?;
		let member = ast::MemberExpr { left: Box::new(expr), right: Box::new(right), range };
		Ok(ast::Expr::Member(member))
	}

	// import("std/mem.ln", os = "windows")
	fn parse_import_expr(&mut self) -> PResult<'lex, ast::ImportExpr> {
		let range = self.expect(Token::Import)?;
		self.expect(Token::LParen)?;
		let path = match self.parse_string()? {
			ast::Literal::String(string) => string,
			_ => return Err(self.unexpected_token()),
		};
		self.expect(Token::RParen)?;
		Ok(ast::ImportExpr { path, range })
	}
	// &mut <expr>
	fn parse_borrow_expr(&mut self) -> PResult<'lex, ast::BorrowExpr> {
		let range = self.expect(Token::And)?;
		let mut mutable = None;
		if self.match_token(Token::Mut) {
			mutable = Some(self.expect(Token::Mut)?);
		}
		let expr = self.parse_primary(false)?;
		Ok(ast::BorrowExpr { expr: Box::new(expr), range, mutable, type_id: None })
	}

	// *<expr>
	fn parse_deref_expr(&mut self) -> PResult<'lex, ast::DerefExpr> {
		let range = self.expect(Token::Star)?;
		let expr = self.parse_primary(false)?;
		Ok(ast::DerefExpr { expr: Box::new(expr), range, type_id: None })
	}

	fn parse_char(&mut self) -> PResult<'lex, ast::Literal> {
		self.ensure_char()?;
		let range = self.take_range();
		let inner = self.take_text_and_next()?;
		let value = self.parse_string_like(&inner, &range)?;
		let mut chars = value.chars();
		let value = match chars.next() {
			Some(char) => char,
			None => return Err(self.custom_diag("expected char literal", &range)),
		};

		if chars.next().is_some() {
			return Err(self.custom_diag("expected char literal", &range));
		}

		let char = ast::CharLiteral { value, range };
		Ok(ast::Literal::Char(char))
	}

	fn parse_string(&mut self) -> PResult<'lex, ast::Literal> {
		if !self.match_token(Token::String) {
			self.expect(Token::String)?;
		}
		let range = self.take_range();
		let inner = self.take_text_and_next()?;
		let text = self.parse_string_like(&inner, &range)?;
		let string = ast::StringLiteral { text, range };
		Ok(ast::Literal::String(string))
	}

	fn parse_string_like(&mut self, inner: &str, range: &Range) -> PResult<'lex, String> {
		let inner = &inner[1..inner.len() - 1];
		let mut str = String::with_capacity(inner.len());
		let mut chars = inner.chars();
		while let Some(char) = chars.next() {
			if char != '\\' {
				str.push(char);
				continue;
			}
			match chars.next() {
				Some(c @ ('\'' | '"' | '\\')) => str.push(c),
				Some('n') => str.push('\n'),
				Some('r') => str.push('\r'),
				Some('t') => str.push('\t'),
				Some('0') => str.push('\0'),
				_ => {
					let message = format!("unknown escape sequence '\\{}'", char);
					let diag = self.custom_diag(message, range);
					return Err(diag.with_file_id(self.file_id));
				}
			}
		}
		Ok(str)
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
		let range = self.expect(Token::Fn)?;
		let mut params = vec![];
		self.expect(Token::LParen)?; // take '('
		while !self.match_token(Token::RParen) {
			params.push(self.parse_binding()?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}
		self.expect(Token::RParen)?; // take ')'

		let ret_type = match self.token {
			Some(Token::Colon) => {
				self.expect(Token::Colon)?;
				Some(self.parse_type()?)
			}
			_ => None,
		};

		self.expect(Token::Assign)?; // take '='

		let body = Box::new(self.parse_stmt()?);

		Ok(ast::FnExpr { params, body, range, ret_type, type_id: None })
	}

	fn parse_if_expr(&mut self) -> PResult<'lex, ast::IfExpr> {
		let range = self.expect(Token::If)?;
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
		let generics = vec![];
		while !self.match_token(Token::RParen) {
			args.push(self.parse_expr(MIN_PDE)?);
			if !self.match_token(Token::RParen) {
				self.expect(Token::Comma)?;
			}
		}
		range.merge(&self.expect(Token::RParen)?); // consume ')'
		let call_expr = ast::CallExpr::new(callee, args, range, generics);
		Ok(ast::Expr::Call(call_expr))
	}

	fn parse_assign_expr(&mut self, left: ast::Expr) -> PResult<'lex, ast::Expr> {
		let range = self.expect(Token::Assign)?;
		if !left.valid_assign_expr() {
			let diag = self.custom_diag("left-hand side can't be assigned", &self.range);
			return Err(diag);
		}
		let right = self.parse_expr(MIN_PDE)?;
		let assign_expr =
			ast::AssignExpr { left: Box::new(left), right: Box::new(right), range, type_id: None };
		Ok(ast::Expr::Assign(assign_expr))
	}

	fn parse_pipe_expr(&mut self, left: ast::Expr) -> PResult<'lex, ast::Expr> {
		let range = self.expect(Token::Pipe)?;
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
			return Err(diag.with_file_id(self.file_id));
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
			return Err(diag.with_file_id(self.file_id));
		}
		let range = self.range.clone();
		self.next()?;
		Ok(range)
	}

	fn match_token(&mut self, token: Token) -> bool {
		self.token.as_ref().map(|t| *t == token).unwrap_or(false)
	}

	fn unexpected_token(&mut self) -> Diag {
		if let Some(token) = self.token {
			let message = format!("unexpected token '{}'", token);
			let diag = Diag::error(message, self.range.clone());
			return diag.with_file_id(self.file_id);
		}
		self.custom_diag("unsupported token", &self.range)
	}

	pub fn custom_diag(&self, message: impl Into<String>, range: &Range) -> Diag {
		Diag::error(message, range.clone()).with_file_id(self.file_id)
	}
}
