#![allow(dead_code)]
use crate::{range::Range, tokens::TokenType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
  pub stmts: Vec<Stmts>,
}

// ------- statements -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmts {
  Let(LetStmt),
  Expr(Expr),
  Fn(FunctionStmt),
  Block(BlockStmt),
}

impl Stmts {
  pub fn get_range(&self) -> &Range {
    match self {
      Stmts::Let(let_stmt) => let_stmt.get_range(),
      Stmts::Expr(expr) => expr.get_range(),
      Stmts::Fn(function_stmt) => function_stmt.get_range(),
      Stmts::Block(block_stmt) => block_stmt.get_range(),
    }
  }
}

// let <pat> = <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStmt {
  pub pat: Pat,
  pub expr: Expr,
  pub range: Range,
}
impl LetStmt {
  pub fn create(pat: Pat, expr: Expr, range: Range) -> Self {
    Self { pat, expr, range }
  }
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

// fn <name>(<pats>): <type> = { <stmts> }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionStmt {
  pub name: Identifier,
  pub pats: Vec<Pat>,
  pub ty: Option<String>,
  pub body: Box<Stmts>,
  pub range: Range,
}

impl FunctionStmt {
  pub fn create(name: Identifier, pats: Vec<Pat>, body: Stmts, ty: Option<String>, range: Range) -> Self {
    Self { name, pats, body: Box::new(body), ty, range }
  }
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStmt {
  pub stmts: Vec<Stmts>,
  pub range: Range,
}

impl BlockStmt {
  pub fn create(stmts: Vec<Stmts>, range: Range) -> Self {
    Self { stmts, range }
  }
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
  pub range: Range,
  pub text: String,
}

impl Identifier {
  pub fn create(text: String, range: Range) -> Self {
    Self { range, text }
  }
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pat {
  pub ident: Identifier,
  pub ty: Option<String>,
  pub range: Range,
}

impl Pat {
  pub fn create(ident: Identifier, ty: Option<String>, range: Range) -> Self {
    Self { ident, ty, range }
  }
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

// ------- expressions -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
  Group(GroupExpr),
  Fn(FnExpr),
  Binary(BinaryExpr),
  Object(ObjectExpr),
  Pipe(PipeExpr),
  Unary(UnaryExpr),
  Call(CallExpr),
  Match(MatchExpr),
  Idx(IdxExpr),
  Member(MemberExpr),
  If(IfExpr),
  Return(ReturnExpr),
  Import(ImportExpr),
  Ident(Identifier),
  Literal(Literal),
}

impl Expr {
  pub fn get_range(&self) -> &Range {
    match self {
      Expr::Fn(fn_expr) => fn_expr.get_range(),
      Expr::Group(group) => group.get_range(),
      Expr::Binary(binary) => binary.get_range(),
      Expr::Pipe(pipe) => pipe.get_range(),
      Expr::Unary(unary) => unary.get_range(),
      Expr::Call(call) => call.get_range(),
      Expr::Match(match_expr) => match_expr.get_range(),
      Expr::Idx(index) => index.get_range(),
      Expr::Member(member) => member.get_range(),
      Expr::If(if_expr) => if_expr.get_range(),
      Expr::Return(return_expr) => return_expr.get_range(),
      Expr::Ident(ident) => ident.get_range(),
      Expr::Literal(literal) => literal.get_range(),
      Expr::Object(object) => object.get_range(),
      Expr::Import(import) => import.get_range(),
    }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnExpr {
  pub pats: Vec<Pat>,
  pub name: Option<Identifier>,
  pub body: Box<Stmts>,
  pub range: Range,
}

impl FnExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupExpr {
  pub range: Range,
  pub expr: Box<Expr>,
}

impl GroupExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipeExpr {
  pub left: Box<Expr>,
  pub right: Box<Expr>,
  pub range_op: Range,
  pub range: Range,
}

impl PipeExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
  pub left: Box<Expr>,
  pub right: Box<Expr>,
  pub operator: Operator,
  pub range: Range,
  pub range_op: Range,
}

impl BinaryExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
  pub operand: Box<Expr>,
  pub operator: Operator,
  pub range: Range,
  pub range_op: Range,
}

impl UnaryExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
  pub callee: Box<Expr>,
  pub args: Vec<Expr>,
  pub range: Range,
}

impl CallExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

// match <expr> { <arms> }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchExpr {
  pub expr: Box<Expr>,
  pub arms: Vec<Arm>,
  pub range: Range,
}

impl MatchExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

// { <guard> } => { <stmts> }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arm {
  pub guard: Box<Expr>,
  pub body: Stmts,
  pub range: Range,
}

impl Arm {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdxExpr {
  pub object: Box<Expr>,
  pub index: Box<Expr>,
  pub range: Range,
}

impl IdxExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberExpr {
  pub object: Box<Expr>,
  pub property: Box<Expr>,
  pub range: Range,
}

impl MemberExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfExpr {
  pub condition: Box<Expr>,
  pub consequent: Vec<Stmts>,
  pub alternate: Option<Vec<Stmts>>,
  pub range: Range,
}

impl IfExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectExpr {
  pub range: Range,
  pub fields: Vec<Field>,
}

impl ObjectExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
  pub left: Identifier,
  pub right: Box<Expr>,
  pub range: Range,
}

impl Field {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnExpr {
  pub value: Option<Box<Expr>>,
  pub range: Range,
}

impl ReturnExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportExpr {
  pub path: StringLiteral,
  pub range: Range,
}

impl ImportExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
  Number(NumberLiteral),
  String(StringLiteral),
  Boolean(BooleanLiteral),
  Null(NullLiteral),
}

impl Literal {
  pub fn get_range(&self) -> &Range {
    match self {
      Literal::Number(number) => number.get_range(),
      Literal::String(string) => string.get_range(),
      Literal::Boolean(boolean) => boolean.get_range(),
      Literal::Null(null) => null.get_range(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberLiteral {
  pub range: Range,
  pub text: String,
}

impl NumberLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringLiteral {
  pub range: Range,
  pub text: String,
}

impl StringLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanLiteral {
  pub range: Range,
  pub value: bool,
}

impl BooleanLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullLiteral {
  pub range: Range,
}

impl NullLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator {
  ADD,   // +
  SUB,   // -
  MUL,   // *
  DIV,   // /
  REM,   // %
  REQ,   // %=
  RANGE, // ..
  EQ,    // ==
  NEQ,   // !=
  LT,    // <
  GT,    // >
  AND,   // &&
  OR,    // ||
  XOR,   // ^
  BOR,   // |
  // XORQ,  // ^=
  SHL,  // <<
  SHR,  // >>
  POW,  // **
  LE,   // <=
  GE,   // >=
  NOT,  // !
  PIPE, // |>
}

impl Operator {
  pub fn from_token(token: &TokenType) -> Option<Self> {
    match token {
      TokenType::Plus => Some(Self::ADD),
      TokenType::Minus => Some(Self::SUB),
      TokenType::Star => Some(Self::MUL),
      TokenType::Slash => Some(Self::DIV),
      TokenType::Eq => Some(Self::EQ),
      TokenType::NotEq => Some(Self::NEQ),
      TokenType::LessEq => Some(Self::LE),
      TokenType::GreaterEq => Some(Self::GE),
      TokenType::Less => Some(Self::LT),
      TokenType::Greater => Some(Self::GT),
      TokenType::And => Some(Self::AND),
      TokenType::Or => Some(Self::OR),
      TokenType::DotDot => Some(Self::RANGE),
      TokenType::Rem => Some(Self::REM),
      TokenType::RemEq => Some(Self::REQ),
      TokenType::Bar => Some(Self::BOR),
      TokenType::Pow => Some(Self::POW),
      // TokenType::PowEq => Some(Self::POWQ),
      // TokenType::Dot => Some(Self::DOT),
      // TokenType::Extract => Some(Self::EXTRACT),
      TokenType::Pipe => Some(Self::PIPE),
      // TokenType::Assign => Some(Self::ASSIGN),
      TokenType::PlusEq => Some(Self::ADD),
      TokenType::MinusEq => Some(Self::SUB),
      TokenType::StarEq => Some(Self::MUL),
      TokenType::SlashEq => Some(Self::DIV),
      TokenType::Bang => Some(Self::NOT),
      _ => None,
    }
  }
}
