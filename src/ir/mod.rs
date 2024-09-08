use crate::parser::ast::{self, BinaryExpr, Expr, IdentExpr, LiteralExpr, Stmt};
use inkwell::builder::Builder;
use inkwell::context::{self, Context};
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use std::collections::HashMap;

pub struct Ir<'ctx> {
  context: &'ctx Context,
  builder: Builder<'ctx>,
  module: Module<'ctx>,
  variables: HashMap<String, PointerValue<'ctx>>,
  function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> Ir<'ctx> {
  pub fn new(context: &'ctx Context, module_name: &str) -> Self {
    let module = context.create_module(module_name);
    let builder = context.create_builder();

    Ir { context, builder, module, variables: HashMap::new(), function: None }
  }

  pub fn gen_from_ast(&mut self, ast: &ast::Ast) {
    for stmt in &ast.stmts {
      self.gen_stmt(stmt);
    }
  }

  fn gen_stmt(&mut self, stmt: &Stmt) {
    match stmt {
      Stmt::Let(let_stmt) => self.gen_let_stmt(let_stmt),
      Stmt::Fn(fn_stmt) => self.gen_fn_stmt(fn_stmt),
      Stmt::Expr(expr) => {
        self.gen_expr(expr);
      }
      Stmt::Empty => {}
      _ => panic!("Unsupported statement type"),
    }
  }

  fn gen_let_stmt(&mut self, let_stmt: &ast::LetStmt) {
    let value = self.gen_expr(&let_stmt.value);
    let alloca = self.create_entry_block_alloca(&let_stmt.name.name.name);
    self.builder.build_store(alloca, value);
    self.variables.insert(let_stmt.name.name.name.clone(), alloca);
  }

  fn gen_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) {
    let fn_name = &fn_stmt.name.name;
    let return_type = self.context.i32_type();
    let fn_type = return_type.fn_type(&[], false);
    let function = self.module.add_function(fn_name, fn_type, None);

    let basic_block = self.context.append_basic_block(function, "entry");
    self.builder.position_at_end(basic_block);

    self.function = Some(function);
    self.variables.clear();

    for stmt in &fn_stmt.body.body {
      self.gen_stmt(stmt);
    }

    if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
      self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
    }

    self.function = None;
  }

  fn gen_expr(&mut self, expr: &Expr) -> BasicValueEnum<'ctx> {
    match expr {
      Expr::Literal(literal) => self.gen_literal(literal),
      Expr::Binary(binary) => self.gen_binary(binary),
      Expr::Ident(ident) => self.gen_ident(ident),
      _ => panic!("Unsupported expression type: {:?}", expr),
    }
  }

  fn gen_literal(&self, literal: &LiteralExpr) -> BasicValueEnum<'ctx> {
    match literal {
      LiteralExpr::Number(num) => {
        let value = num.raw.parse::<i32>().expect("Failed to parse number");
        self.context.i32_type().const_int(value as u64, false).into()
      }
      _ => panic!("Unsupported literal type"),
    }
  }

  fn gen_binary(&mut self, binary: &BinaryExpr) -> BasicValueEnum<'ctx> {
    let left = self.gen_expr(&binary.left).into_int_value();
    let right = self.gen_expr(&binary.right).into_int_value();

    match binary.op.kind {
      ast::OperatorType::Plus => self.builder.build_int_add(left, right, "addtmp").unwrap().into(),
      ast::OperatorType::Minus => self.builder.build_int_sub(left, right, "subtmp").unwrap().into(),
      ast::OperatorType::Star => self.builder.build_int_mul(left, right, "multmp").unwrap().into(),
      ast::OperatorType::Slash => self.builder.build_int_signed_div(left, right, "divtmp").unwrap().into(),
      _ => panic!("Unsupported binary operator: {:?}", binary.op),
    }
  }

  fn gen_ident(&self, ident: &IdentExpr) -> BasicValueEnum<'ctx> {
    let i32_type = self.context.i32_type();
    match self.variables.get(&ident.name) {
      Some(var) => self.builder.build_load(i32_type, *var, &ident.name).unwrap(),
      None => panic!("Undefined variable: {}", ident.name),
    }
  }

  fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
    let builder = self.context.create_builder();
    let entry = self.function.unwrap().get_first_basic_block().unwrap();

    match entry.get_first_instruction() {
      Some(first_instr) => builder.position_before(&first_instr),
      None => builder.position_at_end(entry),
    }

    builder.build_alloca(self.context.i32_type(), name).unwrap()
  }

  pub fn print_to_string(&self) {
    println!("{}", self.module.print_to_string().to_string());
  }
}
