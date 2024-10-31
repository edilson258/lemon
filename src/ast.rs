use crate::utils::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
  pub stmts: Vec<Stements>,
}

// ------- statements -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stements {
  Let(LetStmt),
  Expr(Expression),
  Fn(FunctionStmt),
  Block(BlockStmt),
}

impl Stements {
  pub fn get_range(&self) -> &Range {
    match self {
      Stements::Let(let_stmt) => let_stmt.get_range(),
      Stements::Expr(expr) => expr.get_range(),
      Stements::Fn(function_stmt) => function_stmt.get_range(),
      Stements::Block(block_stmt) => block_stmt.get_range(),
    }
  }
}

// let <pat> = <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStmt {
  pub pat: Pat,
  pub expr: Expression,
  pub range: Range,
}
impl LetStmt {
  pub fn create(pat: Pat, expr: Expression, range: Range) -> Self {
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
  pub body: BlockStmt,
  pub range: Range,
}

impl FunctionStmt {
  pub fn create(name: Identifier, pats: Vec<Pat>, body: BlockStmt, ty: Option<String>, range: Range) -> Self {
    Self { name, pats, body, ty, range }
  }
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStmt {
  pub stmts: Vec<Stements>,
  pub range: Range,
}

impl BlockStmt {
  pub fn create(stmts: Vec<Stements>, range: Range) -> Self {
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
pub enum Expression {
  Group(GroupExpr),
  Binary(BinaryExpr),
  Unary(UnaryExpr),
  Call(CallExpr),
  Match(MatchExpr),
  Index(IndexExpr),
  Member(MemberExpr),
  If(IfExpr),
  Return(ReturnExpr),
  Ident(Identifier),
  Literal(Literal),
}

impl Expression {
  pub fn get_range(&self) -> &Range {
    match self {
      Expression::Group(group) => group.get_range(),
      Expression::Binary(binary) => binary.get_range(),
      Expression::Unary(unary) => unary.get_range(),
      Expression::Call(call) => call.get_range(),
      Expression::Match(match_expr) => match_expr.get_range(),
      Expression::Index(index) => index.get_range(),
      Expression::Member(member) => member.get_range(),
      Expression::If(if_expr) => if_expr.get_range(),
      Expression::Return(return_expr) => return_expr.get_range(),
      Expression::Ident(ident) => ident.get_range(),
      Expression::Literal(literal) => literal.get_range(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupExpr {
  pub range: Range,
  pub expr: Box<Expression>,
}

impl GroupExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
  pub left: Box<Expression>,
  pub right: Box<Expression>,
  pub range: Range,
}

impl BinaryExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
  pub operand: Box<Expression>,
  pub operator: Operator,
  pub range: Range,
}

impl UnaryExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
  pub callee: Box<Expression>,
  pub args: Vec<Expression>,
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
  pub expr: Box<Expression>,
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
  pub guard: Option<Box<Expression>>,
  pub body: Vec<Stements>,
  pub range: Range,
}

impl Arm {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexExpr {
  pub object: Box<Expression>,
  pub index: Box<Expression>,
  pub range: Range,
}

impl IndexExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberExpr {
  pub object: Box<Expression>,
  pub property: String,
  pub range: Range,
}

impl MemberExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfExpr {
  pub condition: Box<Expression>,
  pub consequent: Vec<Stements>,
  pub alternate: Option<Vec<Stements>>,
  pub range: Range,
}

impl IfExpr {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnExpr {
  pub value: Option<Box<Expression>>,
  pub range: Range,
}

impl ReturnExpr {
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
pub struct Operator {
  pub range: Range,
  pub kind: OperatorEnum,
}

impl Operator {
  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperatorEnum {
  ADD, // +
  SUB, // -
  MUL, // *
  DIV, // /
  REM, // %
  EQ,  // ==
  NEQ, // !=
  LT,  // <
  GT,  // >
  AND, // &&
  OR,  // ||
  XOR, // ^
  SHL, // <<
  SHR, // >>
  POW, // **
  LE,  // <=
  GE,  // >=
}
