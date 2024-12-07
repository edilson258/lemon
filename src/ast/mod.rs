#![allow(dead_code)]
use crate::{
  lexer::Token,
  range::{Range, TraitRange},
};
use serde::{Deserialize, Serialize};
pub mod ast_type;

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
  Block(BlockStmt),
}

impl Stmt {
  pub fn last_stmt_range(&self) -> Option<Range> {
    match self {
      Stmt::Block(block_stmt) => block_stmt.last_stmt_range(),
      _ => None,
    }
  }

  pub fn is_block(&self) -> bool {
    match self {
      Stmt::Block(_) => true,
      _ => false,
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
  pub range: Range, // let range
}

impl LetStmt {
  pub fn get_name(&self) -> &str {
    &self.name.ident.text
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
    return &self.name.text;
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

  pub fn last_stmt_range(&self) -> Option<Range> {
    self.stmts.last().map(|stmt| stmt.range())
  }
}

impl TraitRange for BlockStmt {
  fn range(&self) -> Range {
    return self.range.clone();
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident {
  pub range: Range,
  pub text: String,
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
    return &self.ident.text;
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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnExpr {
  pub params: Vec<Binding>,
  // pub name: Option<Ident>,
  pub ret_type: Option<ast_type::AstType>,
  pub body: Box<Stmt>,
  pub range: Range, // fn range
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

impl TraitRange for UnaryExpr {
  fn range(&self) -> Range {
    self.range.merged_with(&self.operand.range())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
  pub callee: Box<Expr>,
  pub args: Vec<Expr>,
  pub range: Range, // fn(args...)
}

impl TraitRange for CallExpr {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfExpr {
  pub cond: Box<Expr>,
  pub then: Box<Stmt>,
  pub otherwise: Option<Box<Stmt>>,
  pub range: Range, // if range
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

impl TraitRange for BaseExpr {
  fn range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
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
      Token::Or => Some(Self::OR),
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

  pub fn pde(&self) -> u8 {
    match self {
      Operator::LT
      | Operator::LE
      | Operator::GT
      | Operator::GE
      | Operator::EQ
      | Operator::NOTEQ => CMP_PDE,
      Operator::ADD | Operator::SUB => ADD_PDE,
      Operator::MUL | Operator::DIV | Operator::MOD => MUL_PDE,
      Operator::POW => MAX_PDE,
      Operator::NOT => UNA_PDE,
      Operator::PIPE | Operator::RANGE => MIN_PDE,
      _ => MIN_PDE, // default as minimum
    }
  }
}
