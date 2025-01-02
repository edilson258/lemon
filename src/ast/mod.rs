#![allow(dead_code)]
use core::fmt;
use std::fmt::Display;

use crate::{checker::types::TypeId, lexer::Token, range::Range};
use serde::{Deserialize, Serialize};
mod ast_type;
pub use ast_type::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
	pub stmts: Vec<Stmt>,
}

// ------- statements -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
	Let(LetStmt),
	Expr(Expr),
	Fn(FnStmt),
	Ret(RetStmt),
	ConstDel(ConstDelStmt),
	ConstFn(ConstFnStmt),
	Block(BlockStmt),
}

impl Stmt {
	pub fn is_block(&self) -> bool {
		matches!(self, Stmt::Block(_))
	}
	pub fn get_range(&self) -> Range {
		match self {
			Stmt::Let(let_stmt) => let_stmt.get_range(),
			Stmt::Fn(function_stmt) => function_stmt.get_range(),
			Stmt::Block(block_stmt) => block_stmt.get_range(),
			Stmt::Expr(expr) => expr.get_range(),
			Stmt::ConstDel(const_del) => const_del.get_range(),
			Stmt::ConstFn(const_stmt) => const_stmt.get_range(),
			Stmt::Ret(ret_stmt) => ret_stmt.get_range(),
		}
	}
	pub fn ends_with_ret(&self) -> bool {
		match self {
			Stmt::Ret(_) => true,
			Stmt::Block(block_stmt) => block_stmt.ends_with_ret(),
			_ => false,
		}
	}
	pub fn last_stmt_range(&self) -> Range {
		match self {
			Stmt::Block(block_stmt) => block_stmt.last_stmt_range(),
			_ => self.get_range(),
		}
	}
}

// ret <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetStmt {
	pub expr: Option<Box<Expr>>,
	pub range: Range, // return range
	pub type_id: Option<TypeId>,
}

impl RetStmt {
	pub fn get_range(&self) -> Range {
		match &self.expr {
			Some(expr) => self.range.merged_with(&expr.get_range()),
			None => self.range.clone(),
		}
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

// const <fn>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstFnStmt {
	pub name: Ident,
	pub params: Vec<Binding>,
	pub ret_type: Option<ast_type::AstType>,
	pub body: Box<Stmt>,
	pub range: Range,    // const range
	pub fn_range: Range, // fn range
	pub ret_id: Option<TypeId>,
}

impl ConstFnStmt {
	pub fn lexeme(&self) -> &str {
		&self.name.text
	}
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.body.get_range())
	}

	pub fn set_ret_id(&mut self, type_id: TypeId) {
		self.ret_id = Some(type_id);
	}
	pub fn get_ret_id(&self) -> Option<TypeId> {
		self.ret_id
	}
}

// const <pat> = <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstDelStmt {
	pub name: Binding,
	pub expr: Expr,
	pub range: Range, // let range
	pub type_id: Option<TypeId>,
}

impl ConstDelStmt {
	pub fn lexeme(&self) -> &str {
		&self.name.ident.text
	}
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.name.get_range().merged_with(&self.expr.get_range()))
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

// let <pat> = <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStmt {
	pub name: Binding,
	pub expr: Expr,
	pub mutable: Option<Range>,
	pub range: Range, // let range
	pub type_id: Option<TypeId>,
}

impl LetStmt {
	pub fn lexeme(&self) -> &str {
		&self.name.ident.text
	}

	pub fn is_mut(&self) -> bool {
		self.mutable.is_some()
	}
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.name.get_range().merged_with(&self.expr.get_range()))
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
		self.name.set_type_id(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

// fn <name>(<pats>): <type> = { <stmts> }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnStmt {
	pub name: Ident,
	pub params: Vec<Binding>,
	pub ret_type: Option<ast_type::AstType>, // todo: implement this
	pub body: Box<Stmt>,
	pub range: Range, // fn range
	pub ret_id: Option<TypeId>,
}

impl FnStmt {
	pub fn lexeme(&self) -> &str {
		&self.name.text
	}
	pub fn get_range(&self) -> Range {
		// fn ... body
		self.range.merged_with(&self.body.get_range())
	}

	pub fn set_ret_id(&mut self, ret_id: TypeId) {
		self.ret_id = Some(ret_id);
	}
	pub fn get_ret_id(&self) -> Option<TypeId> {
		self.ret_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStmt {
	pub stmts: Vec<Stmt>,
	range: Range,
}

impl BlockStmt {
	pub fn new(stmts: Vec<Stmt>, range: Range) -> Self {
		Self { stmts, range }
	}

	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn ends_with_ret(&self) -> bool {
		self.stmts.last().map(|stmt| stmt.ends_with_ret()).unwrap_or(false)
	}

	pub fn last_stmt_range(&self) -> Range {
		self.stmts.last().map(|stmt| stmt.get_range()).unwrap_or(self.range.clone())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident {
	pub range: Range,
	pub text: String,
	pub type_id: Option<TypeId>,
}

impl Ident {
	pub fn lexeme(&self) -> &str {
		&self.text
	}
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binding {
	pub ident: Ident,
	pub ty: Option<ast_type::AstType>,
	pub type_id: Option<TypeId>,
}

impl Binding {
	pub fn lexeme(&self) -> &str {
		&self.ident.text
	}
	pub fn get_range(&self) -> Range {
		if let Some(ty) = &self.ty {
			self.ident.get_range().merged_with(&ty.get_range())
		} else {
			self.ident.get_range()
		}
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

// ------- expressions -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
	Group(GroupExpr),
	Fn(FnExpr),
	Assign(AssignExpr),
	Binary(BinaryExpr),
	For(ForExpr),
	While(WhileExpr),
	Break(BaseExpr),
	Skip(BaseExpr),
	Pipe(PipeExpr),
	Unary(UnaryExpr),
	Call(CallExpr),
	If(IfExpr),
	Import(ImportExpr),
	Ident(Ident),
	Literal(Literal),
	Ref(RefExpr),
	Deref(DerefExpr),
}

impl Expr {
	pub fn get_range(&self) -> Range {
		match self {
			Expr::Fn(fn_expr) => fn_expr.get_range(),
			Expr::Group(group) => group.get_range(),
			Expr::Binary(binary) => binary.get_range(),
			Expr::Pipe(pipe) => pipe.get_range(),
			Expr::Unary(unary) => unary.get_range(),
			Expr::Call(call) => call.get_range(),
			Expr::If(if_expr) => if_expr.get_range(),
			// Expr::Ret(ret_expr) => ret_expr.get_range(),
			Expr::Ident(ident) => ident.get_range(),
			Expr::Assign(assign) => assign.get_range(),
			Expr::Literal(literal) => literal.get_range(),
			Expr::Import(import) => import.get_range(),
			Expr::For(for_expr) => for_expr.get_range(),
			Expr::While(while_expr) => while_expr.get_range(),
			Expr::Break(break_) => break_.get_range(),
			Expr::Skip(skip) => skip.get_range(),
			Expr::Ref(ref_expr) => ref_expr.get_range(),
			Expr::Deref(deref_expr) => deref_expr.get_range(),
		}
	}

	pub fn valid_assign_expr(&self) -> bool {
		matches!(self, Expr::Ident(_)) | matches!(self, Expr::Ref(_)) | matches!(self, Expr::Deref(_))
	}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnExpr {
	pub params: Vec<Binding>,
	pub ret_type: Option<ast_type::AstType>,
	pub body: Box<Stmt>,
	pub range: Range, // fn range
	pub type_id: Option<TypeId>,
}

impl FnExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.body.get_range())
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub range: Range, // assign range
	pub type_id: Option<TypeId>,
}

impl AssignExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.left.get_range()).merged_with(&self.right.get_range())
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupExpr {
	pub expr: Box<Expr>,
	pub range: Range, // group range (  )
}

impl GroupExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipeExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub range: Range, // pipe range
}

impl PipeExpr {
	pub fn get_range(&self) -> Range {
		self.left.get_range().merged_with(&self.range).merged_with(&self.right.get_range())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub operator: Operator,
	pub range: Range, //  operator range
	pub type_id: Option<TypeId>,
}

impl BinaryExpr {
	pub fn get_range(&self) -> Range {
		self.left.get_range().merged_with(&self.range).merged_with(&self.right.get_range())
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
	pub operand: Box<Expr>,
	pub operator: Operator,
	pub range: Range,
}

impl UnaryExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.operand.get_range())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
	pub callee: Box<Expr>,
	pub args: Vec<Expr>,
	pub range: Range, // (args...)
	pub type_id: Option<TypeId>,
}

impl CallExpr {
	pub fn get_range(&self) -> Range {
		self.callee.get_range().merged_with(&self.range)
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfExpr {
	pub cond: Box<Expr>,
	pub then: Box<Stmt>,
	pub otherwise: Option<Box<Stmt>>,
	pub range: Range, // if range
}

impl IfExpr {
	pub fn get_range(&self) -> Range {
		match &self.otherwise {
			Some(otherwise) => self.range.merged_with(&otherwise.get_range()),
			None => self.range.merged_with(&self.then.get_range()),
		}
	}
}

// for <pat> in <expr> = { <stmts> } or for <idx>, <value> in <expr> = { <stmts> }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForExpr {
	pub value: Ident,
	pub index: Option<Ident>,
	pub iterable: Box<Expr>,
	pub body: Box<Stmt>,
	pub range: Range, // for range
}

impl ForExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.body.get_range())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileExpr {
	pub test: Box<Expr>,
	pub body: Box<Stmt>,
	pub range: Range, // while range
}
impl WhileExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.body.get_range())
	}
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct RetExpr {
// 	pub value: Option<Box<Expr>>,
// 	pub range: Range, // return range
// }

// impl RetExpr {
// 	pub fn get_range(&self) -> Range {
// 		match &self.value {
// 			Some(value) => self.range.merged_with(&value.get_range()),
// 			None => self.range.clone(),
// 		}
// 	}
// }

// &<expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefExpr {
	pub expr: Box<Expr>,
	pub range: Range,           // ref range
	pub mutable: Option<Range>, // mutable range
	pub type_id: Option<TypeId>,
}

impl RefExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.expr.get_range())
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerefExpr {
	pub expr: Box<Expr>,
	pub range: Range, // deref range
	pub type_id: Option<TypeId>,
}

impl DerefExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.expr.get_range())
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&mut self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportExpr {
	pub path: StringLiteral,
	pub range: Range,
}

impl ImportExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
	Number(NumberLiteral),
	String(StringLiteral),
	Char(CharLiteral),
	Bool(BoolLiteral),
	Null(BaseExpr),
}

impl Literal {
	pub fn get_range(&self) -> Range {
		match self {
			Literal::Number(num) => num.get_range(),
			Literal::String(string) => string.get_range(),
			Literal::Bool(bool) => bool.get_range(),
			Literal::Null(null) => null.get_range(),
			Literal::Char(char) => char.get_range(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberLiteral {
	pub range: Range,
	pub text: String,
	pub base: u8,     // hex 0x = 16, bin 0b  = 2, decimal = 10
	pub as_dot: bool, // float
}

pub const BASE_DECIMAL: u8 = 10;
pub const BASE_HEX: u8 = 16;
pub const BASE_BIN: u8 = 2;

impl NumberLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn as_dot(&self) -> bool {
		self.as_dot
	}

	pub fn as_hex(&self) -> bool {
		self.base == BASE_HEX
	}

	pub fn as_bin(&self) -> bool {
		self.base == BASE_BIN
	}

	pub fn as_decimal(&self) -> bool {
		self.base == BASE_DECIMAL
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringLiteral {
	pub range: Range,
	pub text: String,
}

impl StringLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharLiteral {
	pub range: Range,
	pub value: char,
}

impl CharLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolLiteral {
	pub range: Range,
	pub value: bool,
}

impl BoolLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseExpr {
	pub range: Range,
}

impl BaseExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Operator {
	ADD,   // +
	SUB,   // -
	MUL,   // *
	DIV,   // /
	MOD,   // %
	RANGE, // ..
	EQ,    // ==
	NOTEQ, // !=
	ADDEQ, // +=
	SUBEQ, // -=
	MULEQ, // *=
	DIVEQ, // /=
	MODEQ, // %=
	LT,    // <
	GT,    // >
	AND,   // &&
	OR,    // ||
	XOR,   // ^
	BOR,   // |
	SHL,   // <<
	SHR,   // >>
	POW,   // **
	LE,    // <=
	GE,    // >=
	NOT,   // !
	PIPE,  // |>
}
pub const MIN_PDE: u8 = 0; // e.g `|`, `..`
pub const CMP_PDE: u8 = 1; // e.g `<`, `<=`, `>`, `>=`, `==`, `!=`
pub const ADD_PDE: u8 = 2; // e.g `+`, `-`
pub const MUL_PDE: u8 = 3; // e.g `*`, `/`, `%`
pub const MAX_PDE: u8 = 4; // e.g `^`, `**`
pub const UNA_PDE: u8 = 5; // e.g `!`, `-`

impl Operator {
	pub fn from_token(token: &Token) -> Option<Self> {
		match token {
			Token::Plus => Some(Self::ADD),
			Token::Minus => Some(Self::SUB),
			Token::Star => Some(Self::MUL),
			Token::Slash => Some(Self::DIV),
			Token::Eq => Some(Self::EQ),
			Token::NotEq => Some(Self::NOTEQ),
			Token::LessEq => Some(Self::LE),
			Token::GreaterEq => Some(Self::GE),
			Token::Less => Some(Self::LT),
			Token::Greater => Some(Self::GT),
			Token::And => Some(Self::AND),
			Token::BarBar => Some(Self::OR),
			Token::DotDot => Some(Self::RANGE),
			Token::Rem => Some(Self::MOD),
			Token::RemEq => Some(Self::MODEQ),
			Token::Bar => Some(Self::BOR),
			Token::Pow => Some(Self::POW),
			Token::Pipe => Some(Self::PIPE),
			Token::PlusEq => Some(Self::ADDEQ),
			Token::MinusEq => Some(Self::SUBEQ),
			Token::StarEq => Some(Self::MODEQ),
			Token::SlashEq => Some(Self::DIVEQ),
			Token::Bang => Some(Self::NOT),
			_ => None,
		}
	}

  #[rustfmt::skip]
	pub fn pde(&self) -> u8 {
    match self {
      Operator::LT | Operator::LE | Operator::GT |
      Operator::GE | Operator::EQ | Operator::NOTEQ => CMP_PDE,
      Operator::ADD | Operator::SUB => ADD_PDE,
      Operator::MUL | Operator::DIV | Operator::MOD => MUL_PDE,
      Operator::POW => MAX_PDE,
      Operator::NOT => UNA_PDE,
      Operator::PIPE | Operator::RANGE => MIN_PDE,
      _ => MIN_PDE, // default as minimum
    }
  }
}

impl Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Operator::ADD => write!(f, "+"),
			Operator::SUB => write!(f, "-"),
			Operator::MUL => write!(f, "*"),
			Operator::DIV => write!(f, "/"),
			Operator::MOD => write!(f, "%"),
			Operator::RANGE => write!(f, ".."),
			Operator::EQ => write!(f, "=="),
			Operator::NOTEQ => write!(f, "!="),
			Operator::ADDEQ => write!(f, "+="),
			Operator::SUBEQ => write!(f, "-="),
			Operator::MULEQ => write!(f, "*="),
			Operator::DIVEQ => write!(f, "/="),
			Operator::MODEQ => write!(f, "%="),
			Operator::LT => write!(f, "<"),
			Operator::GT => write!(f, ">"),
			Operator::AND => write!(f, "&&"),
			Operator::OR => write!(f, "||"),
			Operator::XOR => write!(f, "^"),
			Operator::BOR => write!(f, "|"),
			Operator::SHL => write!(f, "<<"),
			Operator::SHR => write!(f, ">>"),
			Operator::POW => write!(f, "**"),
			Operator::LE => write!(f, "<="),
			Operator::GE => write!(f, ">="),
			Operator::NOT => write!(f, "!"),
			Operator::PIPE => write!(f, "|>"),
		}
	}
}
