use crate::parser::ast;

pub struct Wat {
  wat_code: String,
  ind: usize,
}

impl Wat {
  pub fn new() -> Self {
    Self { wat_code: String::new(), ind: 0 }
  }

  pub fn gen_ast(&mut self, ast: &ast::Ast) -> String {
    self.wat_code.clear();
    self.ind = 0;

    self.append_line("(module");
    self.indent();

    self.append_line("(memory 1)");
    self.append_line("(export \"memory\" (memory 0))");

    for stmt in &ast.stmts {
      self.gen_stmt(stmt);
    }

    self.append_line("(func $start)");
    self.append_line("(start $start)");

    self.unindent();
    self.append_line(")");

    self.wat_code.clone()
  }

  fn gen_stmt(&mut self, stmt: &ast::Stmt) {
    match stmt {
      ast::Stmt::Let(let_stmt) => self.gen_let_stmt(let_stmt),
      _ => {}
    }
  }

  fn gen_let_stmt(&mut self, let_stmt: &ast::LetStmt) {
    let var_name = &let_stmt.name.name.name;

    self.append_line(&format!("(global ${} (mut i32) (i32.const 0))", var_name));

    match &let_stmt.value {
      ast::Expr::Binary(binary_expr) => self.gen_binary_expr(binary_expr, var_name),
      ast::Expr::Ident(ident_expr) => self.gen_ident_expr(ident_expr, var_name),
      _ => {}
    }
  }

  fn gen_binary_expr(&mut self, binary_expr: &ast::BinaryExpr, result_var: &str) {
    self.append_line("(func $binary_op (result i32)");
    self.indent();

    self.gen_expr(&binary_expr.left);
    self.gen_expr(&binary_expr.right);

    match binary_expr.op.kind {
      ast::OperatorType::Plus => self.append_line("i32.add"),
      ast::OperatorType::Minus => self.append_line("i32.sub"),
      ast::OperatorType::Star => self.append_line("i32.mul"),
      ast::OperatorType::Slash => self.append_line("i32.div_s"),
      _ => self.append_line(";; Unsupported operator"),
    }

    self.append_line(&format!("global.set ${}", result_var));

    self.unindent();
    self.append_line(")");
  }

  fn gen_ident_expr(&mut self, ident_expr: &ast::IdentExpr, result_var: &str) {
    self.append_line("(func $move_var");
    self.indent();

    self.append_line(&format!("global.get ${}", &ident_expr.name));

    self.append_line(&format!("global.set ${}", result_var));

    self.unindent();
    self.append_line(")");
  }

  fn gen_expr(&mut self, expr: &ast::Expr) {
    match expr {
      ast::Expr::Literal(literal) => self.gen_literal(literal),
      ast::Expr::Ident(ident) => self.append_line(&format!("global.get ${}", &ident.name)),
      _ => {}
    }
  }

  fn gen_literal(&mut self, literal: &ast::LiteralExpr) {
    match literal {
      ast::LiteralExpr::Number(num) => {
        self.append_line(&format!("i32.const {}", num.raw));
      }
      _ => self.append_line(";; Unsupported literal type"),
    }
  }

  fn append_line(&mut self, line: &str) {
    self.wat_code.push_str(&"  ".repeat(self.ind));
    self.wat_code.push_str(line);
    self.wat_code.push('\n');
  }

  fn indent(&mut self) {
    self.ind += 1;
  }

  fn unindent(&mut self) {
    if self.ind > 0 {
      self.ind -= 1;
    }
  }
}
