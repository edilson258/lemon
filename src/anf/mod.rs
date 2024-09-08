use crate::{parser::ast, utils::range::Range};
use std::collections::HashMap;

pub struct Anf {
  pub prefix: String,
  scopes: Vec<HashMap<String, String>>,
  pub name_count: usize,
}

impl Anf {
  pub fn new(prefix: Option<String>) -> Self {
    let prefix = prefix.unwrap_or_else(|| "%lt_".to_string());
    let scopes = vec![HashMap::new()];
    Self { prefix, name_count: 0, scopes }
  }

  pub fn gen_ast(&mut self, ast: &ast::Ast) -> ast::Ast {
    let mut anf_stmts = vec![];
    for stmt in &ast.stmts {
      let anf_stmt = self.gen_stmt(stmt);
      anf_stmts.extend(anf_stmt);
    }
    ast::Ast::new(anf_stmts)
  }

  pub fn gen_stmt(&mut self, stmt: &ast::Stmt) -> Vec<ast::Stmt> {
    match stmt {
      ast::Stmt::Let(let_stmt) => self.gen_let_stmt(let_stmt),
      ast::Stmt::Fn(function_stmt) => vec![ast::Stmt::Fn(self.gen_fn_stmt(function_stmt))],
      ast::Stmt::Empty => vec![],
      _ => panic!("Statement type not implemented"),
    }
  }

  fn gen_let_stmt(&mut self, let_stmt: &ast::LetStmt) -> Vec<ast::Stmt> {
    let mut anf_stmts = vec![];
    let (anf_expr, expr_stmts) = self.gen_expr(&let_stmt.value);
    anf_stmts.extend(expr_stmts);
    let name = self.gen_pat_type(&let_stmt.name);
    anf_stmts.push(ast::Stmt::Let(ast::LetStmt::create(name, anf_expr, let_stmt.get_range())));
    anf_stmts
  }

  fn gen_expr(&mut self, expr: &ast::Expr) -> (ast::Expr, Vec<ast::Stmt>) {
    match expr {
      ast::Expr::Literal(_) | ast::Expr::Ident(_) => self.gen_simple_expr(expr),
      ast::Expr::Binary(binary) => self.gen_binary_expr(binary),
      ast::Expr::Unary(unary) => self.gen_unary_expr(unary),
      ast::Expr::Call(call) => self.gen_call_expr(call),
      _ => panic!("Expression type not implemented"),
    }
  }

  fn gen_simple_expr(&self, expr: &ast::Expr) -> (ast::Expr, Vec<ast::Stmt>) {
    (expr.clone(), vec![])
  }

  fn gen_binary_expr(&mut self, binary: &ast::BinaryExpr) -> (ast::Expr, Vec<ast::Stmt>) {
    let mut anf_stmts = vec![];

    let (left_expr, left_stmts) = self.gen_expr(&binary.left);
    let (right_expr, right_stmts) = self.gen_expr(&binary.right);

    anf_stmts.extend(left_stmts);
    anf_stmts.extend(right_stmts);

    let temp_name = self.create_temp_name();
    let binary_expr = ast::BinaryExpr::create(left_expr, binary.op.clone(), right_expr);
    anf_stmts.push(ast::Stmt::Let(ast::LetStmt::create(
      self.create_pat(temp_name.clone()),
      ast::Expr::create_binary(binary_expr),
      binary.get_range(),
    )));
    let expr = ast::Expr::create_ident(ast::IdentExpr::create(temp_name, binary.get_range()));
    (expr, anf_stmts)
  }

  fn gen_unary_expr(&mut self, unary: &ast::UnaryExpr) -> (ast::Expr, Vec<ast::Stmt>) {
    let mut anf_stmts = vec![];

    let (operand_expr, operand_stmts) = self.gen_expr(&unary.operand);
    anf_stmts.extend(operand_stmts);

    let temp_name = self.create_temp_name();
    let unary_expr = ast::UnaryExpr::create(operand_expr, unary.op.clone());
    anf_stmts.push(ast::Stmt::Let(ast::LetStmt::create(
      self.create_pat(temp_name.clone()),
      ast::Expr::create_unary(unary_expr),
      unary.get_range(),
    )));

    let expr = ast::Expr::create_ident(ast::IdentExpr::create(temp_name, unary.get_range()));
    (expr, anf_stmts)
  }

  fn gen_call_expr(&mut self, call: &ast::CallExpr) -> (ast::Expr, Vec<ast::Stmt>) {
    let mut anf_stmts = vec![];

    let (callee_expr, callee_stmts) = self.gen_expr(&call.callee);
    let mut args = vec![];
    anf_stmts.extend(callee_stmts);

    for arg in &call.args {
      let (arg_expr, arg_stmts) = self.gen_expr(arg);
      anf_stmts.extend(arg_stmts);
      args.push(arg_expr);
    }

    let temp_name = self.create_temp_name();
    let call_expr = ast::CallExpr::create(callee_expr, args, call.get_range());
    anf_stmts.push(ast::Stmt::Let(ast::LetStmt::create(
      self.create_pat(temp_name.clone()),
      ast::Expr::create_call(call_expr),
      call.get_range(),
    )));

    let expr = ast::Expr::create_ident(ast::IdentExpr::create(temp_name, call.get_range()));
    (expr, anf_stmts)
  }

  fn gen_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) -> ast::FnStmt {
    let name_anf = self.gen_ident_expr(&fn_stmt.name);
    let block_anf = self.gen_block_stmt(&fn_stmt.body);
    let inputs_anf = self.gen_fn_inputs(&fn_stmt.inputs);
    ast::FnStmt::create(name_anf, inputs_anf, block_anf, fn_stmt.output.clone(), fn_stmt.get_range())
  }

  fn gen_fn_inputs(&mut self, inputs: &[ast::PatType]) -> Vec<ast::PatType> {
    inputs.iter().map(|input| self.gen_pat_type(input)).collect()
  }

  fn gen_block_stmt(&mut self, block_stmt: &ast::BlockStmt) -> ast::BlockStmt {
    let mut anf_stmts = vec![];
    for stmt in &block_stmt.body {
      anf_stmts.extend(self.gen_stmt(stmt));
    }
    ast::BlockStmt::create(anf_stmts, block_stmt.get_range())
  }

  fn gen_pat_type(&mut self, pat_type: &ast::PatType) -> ast::PatType {
    let anf_ident = self.gen_ident_expr(&pat_type.name);
    ast::PatType::create(anf_ident, pat_type.ty.clone())
  }

  fn gen_ident_expr(&mut self, ident_expr: &ast::IdentExpr) -> ast::IdentExpr {
    let name = self.create_name(&ident_expr.name);
    ast::IdentExpr::create(name, ident_expr.get_range())
  }

  fn create_pat(&mut self, name: String) -> ast::PatType {
    // todo: range is not correct
    let ident_expr = ast::IdentExpr::create(name.clone(), Range::default());
    ast::PatType::create(ident_expr, None)
  }

  fn create_temp_name(&mut self) -> String {
    let name = format!("{}{}", self.prefix, self.name_count);
    self.name_count += 1;
    name
  }

  fn create_name(&mut self, name: &str) -> String {
    self.name_count += 1;
    let prefix_name = format!("{}{}", self.prefix, self.name_count);
    if let Some(scope) = self.scopes.last_mut() {
      if let Some(existing_name) = scope.get(name) {
        return existing_name.clone();
      }
      scope.insert(name.to_string(), prefix_name.clone());
      return prefix_name;
    }
    let mut scope = HashMap::new();
    scope.insert(name.to_string(), prefix_name.clone());
    self.scopes.push(scope);
    prefix_name
  }
}
