use serde::{Deserialize, Serialize};

use crate::utils::range::{create_range_from, Range};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Ast {
  pub statements: Vec<Statement>,
}

impl Ast {
  pub fn new(statements: Vec<Statement>) -> Self {
    Self { statements }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum Statement {
  Let(LetStmt),
  Expr(Expr),
  Empty,
}

impl Statement {
  pub fn get_range(&self) -> Range {
    match self {
      Statement::Let(let_stmt) => let_stmt.get_range(),
      Statement::Expr(expr) => expr.get_range(),
      Statement::Empty => Range::new(0, 0),
    }
  }

  pub fn create_let(name: String, ty: Option<AstType>, value: Expr, range: Range) -> Self {
    Self::Let(LetStmt { name, ty, value, range })
  }

  pub fn create_expr(expr: Expr, range: Range) -> Self {
    Self::Expr(expr)
  }

  pub fn create_empty() -> Self {
    Self::Empty
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct LetStmt {
  pub name: String,
  pub ty: Option<AstType>,
  pub value: Expr,
  pub range: Range,
}

impl LetStmt {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(name: String, ty: Option<AstType>, value: Expr, range: Range) -> Self {
    Self { name, value, ty, range }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum Expr {
  Literal(LiteralExpr),
  List(ListExpr),
  Binary(BinaryExpr),
  Unary(UnaryExpr),
  Call(CallExpr),
  Index(IndexExpr),
  Member(MemberExpr),
  Function(FunctionExpr),
  If(IfExpr),
  Return(ReturnExpr),
  Ident(IdentExpr),
}

impl Expr {
  pub fn get_range(&self) -> Range {
    match self {
      Expr::Literal(literal) => literal.get_range(),
      Expr::Binary(binary) => binary.get_range(),
      Expr::Unary(unary) => unary.get_range(),
      Expr::Call(call) => call.get_range(),
      Expr::Index(index) => index.get_range(),
      Expr::Member(member) => member.get_range(),
      Expr::Function(function) => function.get_range(),
      Expr::If(if_expr) => if_expr.get_range(),
      Expr::Return(return_expr) => return_expr.get_range(),
      Expr::List(list) => list.get_range(),
      Expr::Ident(ident) => ident.get_range(),
    }
  }

  pub fn create_literal(literal: LiteralExpr) -> Self {
    Self::Literal(literal)
  }

  pub fn create_ident(ident: IdentExpr) -> Self {
    Self::Ident(ident)
  }

  pub fn create_binary(binary: BinaryExpr, range: Range) -> Self {
    Self::Binary(binary)
  }

  pub fn create_unary(unary: UnaryExpr, range: Range) -> Self {
    Self::Unary(unary)
  }

  pub fn create_call(call: CallExpr, range: Range) -> Self {
    Self::Call(call)
  }

  pub fn create_index(index: IndexExpr, range: Range) -> Self {
    Self::Index(index)
  }

  pub fn create_member(member: MemberExpr, range: Range) -> Self {
    Self::Member(member)
  }

  pub fn create_function(function: FunctionExpr, range: Range) -> Self {
    Self::Function(function)
  }

  pub fn create_if(if_expr: IfExpr, range: Range) -> Self {
    Self::If(if_expr)
  }

  pub fn create_return(return_expr: ReturnExpr, range: Range) -> Self {
    Self::Return(return_expr)
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct IdentExpr {
  pub name: String,
  pub range: Range,
}

impl IdentExpr {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(name: String, range: Range) -> Self {
    Self { name, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum LiteralExpr {
  Number(NumberLiteral),
  String(StringLiteral),
  Boolean(BooleanLiteral),
  Null(NullLiteral),
}

impl LiteralExpr {
  pub fn get_range(&self) -> Range {
    match self {
      LiteralExpr::Number(number) => number.range.clone(),
      LiteralExpr::String(string) => string.range.clone(),
      LiteralExpr::Boolean(boolean) => boolean.range.clone(),
      LiteralExpr::Null(null) => null.range.clone(),
    }
  }

  pub fn create_number(literal: NumberLiteral) -> Self {
    Self::Number(literal)
  }

  pub fn create_string(literal: StringLiteral) -> Self {
    Self::String(literal)
  }

  pub fn create_boolean(literal: BooleanLiteral) -> Self {
    Self::Boolean(literal)
  }

  pub fn create_null(literal: NullLiteral) -> Self {
    Self::Null(literal)
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct NumberLiteral {
  pub raw: String,
  pub range: Range,
}

impl NumberLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }

  pub fn create(text: String, range: Range) -> Self {
    Self { raw: text, range }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct StringLiteral {
  pub raw: String,
  pub range: Range,
}

impl StringLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }

  pub fn create(text: String, range: Range) -> Self {
    Self { raw: text, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct BooleanLiteral {
  pub value: bool,
  pub range: Range,
}

impl BooleanLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }

  pub fn create(value: bool, range: Range) -> Self {
    Self { value, range }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct NullLiteral {
  pub range: Range,
}

impl NullLiteral {
  pub fn get_range(&self) -> &Range {
    &self.range
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct ListExpr {
  pub elements: Vec<Expr>,
}

impl ListExpr {
  pub fn get_range(&self) -> Range {
    panic!("ListExpr has no elements");
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct BinaryExpr {
  pub left: Box<Expr>,
  pub right: Box<Expr>,
}

impl BinaryExpr {
  pub fn get_range(&self) -> Range {
    let left_range = &self.left.get_range();
    let right_range = &self.right.get_range();
    create_range_from(left_range, right_range)
  }

  pub fn create(left: Box<Expr>, right: Box<Expr>) -> Self {
    Self { left, right }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct UnaryExpr {
  pub operand: Box<Expr>,
}

impl UnaryExpr {
  pub fn get_range(&self) -> Range {
    self.operand.get_range()
  }

  pub fn create(operand: Box<Expr>) -> Self {
    Self { operand }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct CallExpr {
  pub callee: Box<Expr>,
  pub arguments: Vec<Expr>,
  pub range: Range,
}

impl CallExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct IndexExpr {
  pub object: Box<Expr>,
  pub index: Box<Expr>,
  pub range: Range,
}

impl IndexExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct MemberExpr {
  pub object: Box<Expr>,
  pub property: String,
  pub range: Range,
}

impl MemberExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct FunctionExpr {
  pub name: String,
  pub parameters: Vec<String>,
  pub body: Vec<Statement>,
  pub range: Range,
}

impl FunctionExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct IfExpr {
  pub condition: Box<Expr>,
  pub consequent: Vec<Statement>,
  pub alternate: Option<Vec<Statement>>,
  pub range: Range,
}

impl IfExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct ReturnExpr {
  pub value: Option<Box<Expr>>,
  pub range: Range,
}

impl ReturnExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

// ------
// Ast Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum AstType {
  Ident(IdentType),
  Int(IntType),
  Float(FloatType),
  Bool(BoolType),
  String(StringType),
  Char(CharType),
  List(ListType),
  Null(NullType),
  Function(FunctionType),
}

impl AstType {
  pub fn get_range(&self) -> Range {
    match self {
      AstType::Ident(ident) => ident.get_range(),
      AstType::Int(int) => int.get_range(),
      AstType::Float(float) => float.get_range(),
      AstType::Bool(bool) => bool.get_range(),
      AstType::String(string) => string.get_range(),
      AstType::Char(char) => char.get_range(),
      AstType::List(list) => list.get_range(),
      AstType::Null(null) => null.get_range(),
      AstType::Function(function) => function.get_range(),
    }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct IdentType {
  pub name: String,
  pub range: Range,
}

impl IdentType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(name: String, range: Range) -> Self {
    Self { name, range }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct IntType {
  pub range: Range,
}

impl IntType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct FloatType {
  pub range: Range,
}

impl FloatType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct BoolType {
  pub range: Range,
}

impl BoolType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}
// StringType
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct StringType {
  pub range: Range,
}

impl StringType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}
// CharType
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct CharType {
  pub range: Range,
}

impl CharType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}

// ------
// ListType
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct ListType {
  pub range: Range,
}

impl ListType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}
// ------
// NullType
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct NullType {
  pub range: Range,
}

impl NullType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}
// ------
// FunctionType
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct FunctionType {
  pub range: Range,
}

impl FunctionType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}
