use crate::ast;
use crate::ast::ast_type;

pub trait Visitor<T> {
  fn visit_program(&mut self, program: &ast::Program) -> T;
  fn visit_expr(&mut self, expr: &ast::Expr) -> T;
  fn visit_let_stmt(&mut self, stmt: &ast::LetStmt) -> T;
  fn visit_fn_stmt(&mut self, stmt: &ast::FnStmt) -> T;
  fn visit_block_stmt(&mut self, stmt: &ast::BlockStmt) -> T;
  fn visit_assign_expr(&mut self, expr: &ast::AssignExpr) -> T;
  fn visit_binary_expr(&mut self, expr: &ast::BinaryExpr) -> T;
  fn visit_call_expr(&mut self, expr: &ast::CallExpr) -> T;
  fn visit_group_expr(&mut self, expr: &ast::GroupExpr) -> T;
  fn visit_if_expr(&mut self, expr: &ast::IfExpr) -> T;
  fn visit_literal(&mut self, expr: &ast::Literal) -> T;
  fn visit_unary_expr(&mut self, expr: &ast::UnaryExpr) -> T;
  fn visit_import_expr(&mut self, expr: &ast::ImportExpr) -> T;
  fn visit_num_literal(&mut self, expr: &ast::NumLiteral) -> T;
  fn visit_string_literal(&mut self, expr: &ast::StringLiteral) -> T;
  fn visit_char_literal(&mut self, expr: &ast::CharLiteral) -> T;
  fn visit_bool_literal(&mut self, expr: &ast::BoolLiteral) -> T;
  fn visit_base_expr(&mut self, expr: &ast::BaseExpr) -> T;
  fn visit_ident(&mut self, expr: &ast::Ident) -> T;
  fn visit_binding(&mut self, expr: &ast::Binding) -> T;
  fn visit_fn_expr(&mut self, expr: &ast::FnExpr) -> T;
  fn visit_pipe_expr(&mut self, expr: &ast::PipeExpr) -> T;
  fn visit_for_expr(&mut self, expr: &ast::ForExpr) -> T;
  fn visit_while_expr(&mut self, expr: &ast::WhileExpr) -> T;
  fn visit_ret_expr(&mut self, expr: &ast::RetExpr) -> T;

  // types
  fn visit_ast_type(&mut self, expr: &ast_type::AstType) -> T;
  fn visit_numb_type(&mut self, expr: &ast_type::NumbType) -> T;
  fn visit_float_type(&mut self, expr: &ast_type::FloatType) -> T;
  fn visit_base_type(&mut self, expr: &ast_type::BaseType) -> T;
  fn visit_ident_type(&mut self, expr: &ast_type::IdentType) -> T;
  fn visit_fn_type(&mut self, expr: &ast_type::FnType) -> T;
}
