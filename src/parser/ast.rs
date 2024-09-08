use serde::{Deserialize, Serialize};

use crate::{
  lexer::token::TokenType,
  utils::range::{create_range_from, Range},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Ast {
  pub stmts: Vec<Stmt>,
}

impl Ast {
  pub fn new(stmts: Vec<Stmt>) -> Self {
    Self { stmts }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum Stmt {
  Let(LetStmt),
  Expr(Expr),
  Fn(FnStmt),
  Block(BlockStmt),
  Empty,
}

impl Stmt {
  pub fn get_range(&self) -> Range {
    match self {
      Stmt::Let(let_stmt) => let_stmt.get_range(),
      Stmt::Expr(expr) => expr.get_range(),
      Stmt::Fn(function_stmt) => function_stmt.get_range(),
      Stmt::Block(block_stmt) => block_stmt.get_range(),
      Stmt::Empty => Range::new(0, 0),
    }
  }

  pub fn create_let(name: PatType, value: Expr, range: Range) -> Stmt {
    Stmt::Let(LetStmt { name, value, range })
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
  pub name: PatType,
  pub value: Expr,
  pub range: Range,
}

impl LetStmt {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(name: PatType, value: Expr, range: Range) -> Self {
    Self { name, value, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct PatType {
  pub name: IdentExpr,
  pub ty: Option<AstType>,
}

impl PatType {
  pub fn get_range(&self) -> Range {
    let left_range = self.name.get_range();
    if let Some(ty) = &self.ty {
      let right_range = &ty.get_range();
      return create_range_from(&left_range, right_range);
    } else {
      return left_range;
    }
  }

  pub fn create(name: IdentExpr, ty: Option<AstType>) -> Self {
    Self { name, ty }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct FnStmt {
  pub name: IdentExpr,
  pub inputs: Vec<PatType>,
  pub output: Option<AstType>,
  pub body: BlockStmt,
  pub range: Range,
}

impl FnStmt {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(name: IdentExpr, inputs: Vec<PatType>, body: BlockStmt, output: Option<AstType>, range: Range) -> Self {
    Self { name, inputs, body, output, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct BlockStmt {
  pub body: Vec<Stmt>,
  pub range: Range,
}

impl BlockStmt {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }

  pub fn create(body: Vec<Stmt>, range: Range) -> Self {
    Self { body, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum Expr {
  Literal(LiteralExpr),
  Group(GroupExpr),
  Binary(BinaryExpr),
  Unary(UnaryExpr),
  Call(CallExpr),
  Index(IndexExpr),
  Member(MemberExpr),
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
      Expr::If(if_expr) => if_expr.get_range(),
      Expr::Return(return_expr) => return_expr.get_range(),
      Expr::Group(group) => group.get_range(),
      Expr::Ident(ident) => ident.get_range(),
    }
  }

  pub fn create_literal(literal: LiteralExpr) -> Self {
    Self::Literal(literal)
  }

  pub fn create_ident(ident: IdentExpr) -> Self {
    Self::Ident(ident)
  }

  pub fn create_binary(binary: BinaryExpr) -> Self {
    Self::Binary(binary)
  }

  pub fn create_unary(unary: UnaryExpr) -> Self {
    Self::Unary(unary)
  }

  pub fn create_call(call: CallExpr) -> Self {
    Self::Call(call)
  }

  pub fn create_index(index: IndexExpr) -> Self {
    Self::Index(index)
  }

  pub fn create_member(member: MemberExpr) -> Self {
    Self::Member(member)
  }

  pub fn create_if(if_expr: IfExpr) -> Self {
    Self::If(if_expr)
  }

  pub fn create_return(return_expr: ReturnExpr) -> Self {
    Self::Return(return_expr)
  }

  pub fn create_group(group: GroupExpr) -> Self {
    Self::Group(group)
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
pub struct GroupExpr {
  pub range: Range,
  pub list: Vec<Expr>,
}

impl GroupExpr {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(list: Vec<Expr>, range: Range) -> Self {
    Self { list, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct BinaryExpr {
  pub left: Box<Expr>,
  // nb: make sure op is a valid binary operator
  pub op: Operator,
  pub right: Box<Expr>,
  pub range: Range,
}

impl BinaryExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
  pub fn create(left: Expr, op: Operator, right: Expr) -> Self {
    let range = create_range_from(&left.get_range(), &right.get_range());
    Self { left: Box::new(left), right: Box::new(right), op, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct UnaryExpr {
  pub operand: Box<Expr>,
  pub op: Operator,
  pub range: Range,
}

impl UnaryExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }

  pub fn create(operand: Expr, op: Operator) -> Self {
    let range = create_range_from(&operand.get_range(), &op.get_range());
    Self { operand: Box::new(operand), op, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct CallExpr {
  pub callee: Box<Expr>,
  pub args: Vec<Expr>,
  pub range: Range,
}

impl CallExpr {
  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }

  pub fn create(callee: Expr, args: Vec<Expr>, range: Range) -> Self {
    Self { callee: Box::new(callee), args, range }
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
pub struct IfExpr {
  pub condition: Box<Expr>,
  pub consequent: Vec<Stmt>,
  pub alternate: Option<Vec<Stmt>>,
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

  pub fn create(value: Option<Box<Expr>>, range: Range) -> Self {
    Self { value, range }
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
  Fn(FnType),
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
      AstType::Fn(function) => function.get_range(),
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
// FnType
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct FnType {
  pub range: Range,
}

impl FnType {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(range: Range) -> Self {
    Self { range }
  }
}

// ----------------
// operator kinds
//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Operator {
  pub kind: OperatorType,
  pub range: Range,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum OperatorType {
  Plus,      // + binary
  Minus,     // - unary or binary
  Star,      // * binary
  Slash,     // / binary
  Assign,    // = binary
  PlusEq,    // += binary
  MinusEq,   // -= binary
  StarEq,    // *= binary
  SlashEq,   // /= binary
  Eq,        // == binary
  NotEq,     // != binary
  Less,      // < binary
  Greater,   // > binary
  LessEq,    // <= binary
  GreaterEq, // >= binary
  Extract,   // ?= binary
  Arrow,     // => binary
  And,       // && binary
  Or,        // || binary
  Bang,      // ! unary
  Quest,     // ? unary
}

impl Operator {
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }

  pub fn create(kind: OperatorType, range: Range) -> Self {
    Self { kind, range }
  }

  pub fn as_binary(&self) -> bool {
    match self.kind {
      OperatorType::Plus
      | OperatorType::Minus
      | OperatorType::Star
      | OperatorType::Slash
      | OperatorType::PlusEq
      | OperatorType::MinusEq
      | OperatorType::StarEq
      | OperatorType::SlashEq
      | OperatorType::Eq
      | OperatorType::NotEq
      | OperatorType::Less
      | OperatorType::Greater
      | OperatorType::LessEq
      | OperatorType::GreaterEq
      | OperatorType::Extract
      | OperatorType::Arrow
      | OperatorType::And
      | OperatorType::Or => true,
      _ => false,
    }
  }

  pub fn to_operator(kind: &TokenType) -> Option<OperatorType> {
    let kind = match kind {
      TokenType::Plus => OperatorType::Plus,
      TokenType::Minus => OperatorType::Minus,
      TokenType::Star => OperatorType::Star,
      TokenType::Slash => OperatorType::Slash,
      TokenType::Assign => OperatorType::Assign,
      TokenType::PlusEq => OperatorType::PlusEq,
      TokenType::MinusEq => OperatorType::MinusEq,
      TokenType::StarEq => OperatorType::StarEq,
      TokenType::SlashEq => OperatorType::SlashEq,
      TokenType::Eq => OperatorType::Eq,
      TokenType::NotEq => OperatorType::NotEq,
      TokenType::Less => OperatorType::Less,
      TokenType::Greater => OperatorType::Greater,
      TokenType::LessEq => OperatorType::LessEq,
      TokenType::GreaterEq => OperatorType::GreaterEq,
      TokenType::Extract => OperatorType::Extract,
      TokenType::Arrow => OperatorType::Arrow,
      TokenType::And => OperatorType::And,
      TokenType::Or => OperatorType::Or,
      TokenType::Bang => OperatorType::Bang,
      TokenType::Quest => OperatorType::Quest,
      _ => return None,
    };
    return Some(kind);
  }

  pub fn as_unary(&self) -> bool {
    match self.kind {
      OperatorType::Minus | OperatorType::Bang | OperatorType::Quest => true,
      _ => false,
    }
  }
}
