#![allow(dead_code)]
use core::fmt;
use std::fmt::Display;

use crate::{
  lexer::Token,
  range::{Range, TraitRange},
};
use serde::{Deserialize, Serialize};
use visitor::Visitor;
pub mod ast_type;
pub mod visitor;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
  pub stmts: Vec<Stmt>,
}
impl Program {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_program(self)
  }
}

// ------- statements -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
  Let(LetStmt),
  Expr(Expr),
  Fn(FnStmt),
  Block(BlockStmt),
}

impl Stmt {
  pub fn is_block(&self) -> bool {
    matches!(self, Stmt::Block(_))
  }

  pub fn final_stmt_range(&self) -> Range {
    // match self {
    //   Stmt::Block(block_stmt) => block_stmt.final_stmt_range(),
    // _ => self.range(),
    // }
    self.range()
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    match self {
      Stmt::Let(let_stmt) => visitor.visit_let_stmt(let_stmt),
      Stmt::Fn(function_stmt) => visitor.visit_fn_stmt(function_stmt),
      Stmt::Block(block_stmt) => visitor.visit_block_stmt(block_stmt),
      Stmt::Expr(expr) => visitor.visit_expr(expr),
    }
  }
}

impl TraitRange for Stmt {
  fn range(&self) -> Range {
    match self {
      Stmt::Let(let_stmt) => let_stmt.range(),
      Stmt::Fn(function_stmt) => function_stmt.range(),
      Stmt::Block(block_stmt) => block_stmt.range(),
      Stmt::Expr(expr) => expr.range(),
    }
  }
}

// let <pat> = <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStmt {
  pub name: Binding,
  pub expr: Expr,
  pub mutable: Option<Range>,
  pub range: Range, // let range
}

impl LetStmt {
  pub fn get_name(&self) -> &str {
    &self.name.ident.text
  }

  pub fn is_mut(&self) -> bool {
    self.mutable.is_some()
  }
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_let_stmt(self)
  }
}

impl TraitRange for LetStmt {
  fn range(&self) -> Range {
    self.range.merged_with(&self.name.range().merged_with(&self.expr.range()))
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
}

impl FnStmt {
  pub fn text(&self) -> &str {
    &self.name.text
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_fn_stmt(self)
  }
}

impl TraitRange for FnStmt {
  fn range(&self) -> Range {
    // fn ... body
    self.range.merged_with(&self.body.range())
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
  pub fn final_stmt_range(&self) -> Range {
    let range = self.stmts.last().map(|stmt| stmt.final_stmt_range());
    match range {
      Some(range) => range,
      None => self.range(),
    }
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_block_stmt(self)
  }
}

impl TraitRange for BlockStmt {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident {
  pub range: Range,
  pub text: String,
}

impl Ident {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_ident(self)
  }
}

impl TraitRange for Ident {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binding {
  pub ident: Ident,
  pub ty: Option<ast_type::AstType>,
}

impl Binding {
  pub fn text(&self) -> &str {
    &self.ident.text
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_binding(self)
  }
}

impl TraitRange for Binding {
  fn range(&self) -> Range {
    if let Some(ty) = &self.ty {
      self.ident.range().merged_with(&ty.range())
    } else {
      self.ident.range()
    }
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
  Ret(RetExpr),
  Import(ImportExpr),
  Ident(Ident),
  Literal(Literal),
}

impl Expr {
  pub fn range(&self) -> Range {
    match self {
      Expr::Fn(fn_expr) => fn_expr.range(),
      Expr::Group(group) => group.range(),
      Expr::Binary(binary) => binary.range(),
      Expr::Pipe(pipe) => pipe.range(),
      Expr::Unary(unary) => unary.range(),
      Expr::Call(call) => call.range(),
      Expr::If(if_expr) => if_expr.range(),
      Expr::Ret(ret_expr) => ret_expr.range(),
      Expr::Ident(ident) => ident.range(),
      Expr::Assign(assign) => assign.range(),
      Expr::Literal(literal) => literal.range(),
      Expr::Import(import) => import.range(),
      Expr::For(for_expr) => for_expr.range(),
      Expr::While(while_expr) => while_expr.range(),
      Expr::Break(break_) => break_.range(),
      Expr::Skip(skip) => skip.range(),
    }
  }

  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    match self {
      Expr::Fn(fn_expr) => visitor.visit_fn_expr(fn_expr),
      Expr::Group(group) => visitor.visit_group_expr(group),
      Expr::Binary(binary) => visitor.visit_binary_expr(binary),
      Expr::Pipe(pipe) => visitor.visit_pipe_expr(pipe),
      Expr::Unary(unary) => visitor.visit_unary_expr(unary),
      Expr::Call(call) => visitor.visit_call_expr(call),
      Expr::If(if_expr) => visitor.visit_if_expr(if_expr),
      Expr::Ret(ret_expr) => visitor.visit_ret_expr(ret_expr),
      Expr::Ident(ident) => visitor.visit_ident(ident),
      Expr::Assign(assign) => visitor.visit_assign_expr(assign),
      Expr::Literal(literal) => visitor.visit_literal(literal),
      Expr::Import(import) => visitor.visit_import_expr(import),
      Expr::For(for_expr) => visitor.visit_for_expr(for_expr),
      Expr::While(while_expr) => visitor.visit_while_expr(while_expr),
      Expr::Break(break_) => visitor.visit_base_expr(break_),
      Expr::Skip(skip) => visitor.visit_base_expr(skip),
    }
  }
  pub fn valid_assign_expr(&self) -> bool {
    matches!(self, Expr::Ident(_))
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnExpr {
  pub params: Vec<Binding>,
  // pub name: Option<Ident>,
  pub ret_type: Option<ast_type::AstType>,
  pub body: Box<Stmt>,
  pub range: Range, // fn range
}

impl FnExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_fn_expr(self)
  }
}

impl TraitRange for FnExpr {
  fn range(&self) -> Range {
    self.range.merged_with(&self.body.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignExpr {
  pub left: Box<Expr>,
  pub right: Box<Expr>,
  pub range: Range, // assign range
}

impl TraitRange for AssignExpr {
  fn range(&self) -> Range {
    self.range.merged_with(&self.left.range()).merged_with(&self.right.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupExpr {
  pub expr: Box<Expr>,
  pub range: Range, // group range (  )
}

impl GroupExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_group_expr(self)
  }
}

impl TraitRange for GroupExpr {
  fn range(&self) -> Range {
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
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_pipe_expr(self)
  }
}

impl TraitRange for PipeExpr {
  fn range(&self) -> Range {
    self.left.range().merged_with(&self.range).merged_with(&self.right.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
  pub left: Box<Expr>,
  pub right: Box<Expr>,
  pub operator: Operator,
  pub range: Range, //  operator range
}

impl BinaryExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_binary_expr(self)
  }
}

impl TraitRange for BinaryExpr {
  fn range(&self) -> Range {
    self.left.range().merged_with(&self.range).merged_with(&self.right.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
  pub operand: Box<Expr>,
  pub operator: Operator,
  pub range: Range,
}

impl UnaryExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_unary_expr(self)
  }
}

impl TraitRange for UnaryExpr {
  fn range(&self) -> Range {
    self.range.merged_with(&self.operand.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
  pub callee: Box<Expr>,
  pub args: Vec<Expr>,
  pub range: Range, // (args...)
}

impl CallExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_call_expr(self)
  }
}

impl TraitRange for CallExpr {
  fn range(&self) -> Range {
    self.callee.range().merged_with(&self.range)
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
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_if_expr(self)
  }
}

impl TraitRange for IfExpr {
  fn range(&self) -> Range {
    match &self.otherwise {
      Some(otherwise) => self.range.merged_with(&otherwise.range()),
      None => self.range.merged_with(&self.then.range()),
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
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_for_expr(self)
  }
}

impl TraitRange for ForExpr {
  fn range(&self) -> Range {
    self.range.merged_with(&self.body.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileExpr {
  pub test: Box<Expr>,
  pub body: Box<Stmt>,
  pub range: Range, // while range
}

impl WhileExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_while_expr(self)
  }
}

impl TraitRange for WhileExpr {
  fn range(&self) -> Range {
    self.range.merged_with(&self.body.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetExpr {
  pub value: Option<Box<Expr>>,
  pub range: Range, // return range
}

impl TraitRange for RetExpr {
  fn range(&self) -> Range {
    match &self.value {
      Some(value) => self.range.merged_with(&value.range()),
      None => self.range.clone(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportExpr {
  pub path: StringLiteral,
  pub range: Range,
}

impl ImportExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_import_expr(self)
  }
}

impl TraitRange for ImportExpr {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
  Num(NumLiteral),
  String(StringLiteral),
  Char(CharLiteral),
  Bool(BoolLiteral),
  Null(BaseExpr),
}

impl Literal {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    match self {
      Literal::Num(num) => visitor.visit_num_literal(num),
      Literal::String(string) => visitor.visit_string_literal(string),
      Literal::Bool(bool) => visitor.visit_bool_literal(bool),
      Literal::Char(char) => visitor.visit_char_literal(char),
      Literal::Null(null) => visitor.visit_base_expr(null),
    }
  }
}

impl TraitRange for Literal {
  fn range(&self) -> Range {
    match self {
      Literal::Num(num) => num.range(),
      Literal::String(string) => string.range(),
      Literal::Bool(bool) => bool.range(),
      Literal::Null(null) => null.range(),
      Literal::Char(char) => char.range(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumLiteral {
  pub range: Range,
  pub text: String,
  pub base: u8,     // hex 0x = 16, bin 0b  = 2, decimal = 10
  pub as_dot: bool, // float
}

impl NumLiteral {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_num_literal(self)
  }
}

pub const BASE_DECIMAL: u8 = 10;
pub const BASE_HEX: u8 = 16;
pub const BASE_BIN: u8 = 2;

impl TraitRange for NumLiteral {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

impl NumLiteral {
  pub fn range(&self) -> Range {
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

impl TraitRange for StringLiteral {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

impl StringLiteral {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_string_literal(self)
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharLiteral {
  pub range: Range,
  pub value: char,
}

impl TraitRange for CharLiteral {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

impl CharLiteral {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_char_literal(self)
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolLiteral {
  pub range: Range,
  pub value: bool,
}

impl TraitRange for BoolLiteral {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseExpr {
  pub range: Range,
}

impl BaseExpr {
  pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_base_expr(self)
  }
}

impl TraitRange for BaseExpr {
  fn range(&self) -> Range {
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
