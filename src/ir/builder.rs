#![allow(dead_code)]
use std::collections::HashMap;

use crate::{
  ast::{self, ast_type},
  report,
};

use super::ir;

pub struct Builder {
  ctx: Context,
}

pub struct Context {
  values: HashMap<String, ir::Bind>,
  reg: usize,
  block_id: usize,
}

impl Context {
  pub fn new() -> Self {
    Self { values: HashMap::new(), reg: 0, block_id: 0 }
  }

  pub fn add_variable(&mut self, name: String, ty: String) -> ir::Bind {
    let reg = self.fresh_reg();
    let bind = ir::Bind { reg, ty };
    self.values.insert(name, bind.clone());
    bind
  }

  pub fn get_variable(&self, name: &str) -> Option<&ir::Bind> {
    self.values.get(name)
  }

  pub fn fresh_reg(&mut self) -> ir::REG {
    let reg = format!("r{}", self.reg);
    self.reg += 1;
    reg
  }

  pub fn fresh_block_id(&mut self) -> usize {
    let block_id = self.block_id;
    self.block_id += 1;
    block_id
  }
}

impl Builder {
  pub fn new() -> Self {
    Self { ctx: Context::new() }
  }
  pub fn build(&mut self, program: ast::Program) -> ir::IR {
    let mut ir = ir::IR::new();
    for stmt in program.stmts {
      if let ast::Stmt::Fn(fn_stmt) = stmt {
        let fn_ir = self.build_fn_stmt(fn_stmt);
        ir.add_fn(fn_ir);
      }
    }
    ir
  }

  pub fn build_fn_stmt(&mut self, fn_stmt: ast::FnStmt) -> ir::FnIr {
    let mut binds = Vec::with_capacity(fn_stmt.params.len());

    for param in fn_stmt.params.iter() {
      binds.push(self.build_binding(param));
    }

    let ret_type = match &fn_stmt.ret_type {
      Some(ty) => Some(self.build_type(ty)),
      None => None,
    };

    let mut fn_ir = ir::FnIr::new(fn_stmt.text().to_owned(), binds, ret_type);

    fn_ir.add_block(self.build_stmt(fn_stmt.body.as_ref()));

    return fn_ir;
  }

  fn build_binding(&mut self, binding: &ast::Binding) -> ir::Bind {
    if let Some(ty) = binding.ty.as_ref() {
      let ir_type = self.build_type(ty);
      return self.ctx.add_variable(binding.text().to_string(), ir_type);
    }
    todo!()
  }

  fn build_stmt(&mut self, block: &ast::Stmt) -> ir::BlockIr {
    let mut ir_block = ir::BlockIr::new(self.ctx.fresh_block_id());
    match block {
      ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt, &mut ir_block),
      ast::Stmt::Expr(expr) => {
        let reg = self.build_expr(expr, &mut ir_block);
        ir_block.add_instr(ir::Instr::ret(reg));
      }
      ast::Stmt::Block(block) => {
        for stmt in block.stmts.iter() {
          match stmt {
            ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt, &mut ir_block),
            ast::Stmt::Expr(expr) => {
              let reg = self.build_expr(expr, &mut ir_block);
              ir_block.add_instr(ir::Instr::ret(reg));
            }
            _ => unimplemented!(),
          }
        }
      }
      _ => unimplemented!(),
    }
    ir_block
  }

  fn build_let_stmt(&mut self, let_stmt: &ast::LetStmt, ir_block: &mut ir::BlockIr) {
    let bind = self.build_binding(&let_stmt.name);
    let value_reg = self.build_expr(&let_stmt.expr, ir_block);

    let reg = bind.reg.clone();

    self.ctx.values.insert(let_stmt.get_name().to_owned(), bind);

    let instr = ir::Instr::OWN { value: value_reg, dest: reg };

    ir_block.add_instr(instr);
  }

  fn build_expr(&mut self, expr: &ast::Expr, ir_block: &mut ir::BlockIr) -> ir::REG {
    match expr {
      ast::Expr::Binary(binary) => self.build_binary_expr(binary, ir_block),
      ast::Expr::Literal(literal) => self.build_literal_expr(literal, ir_block),
      ast::Expr::Ident(ident) => self.build_ident_expr(ident),
      _ => unimplemented!(),
    }
  }

  fn build_binary_expr(&mut self, binary: &ast::BinaryExpr, ir_block: &mut ir::BlockIr) -> ir::REG {
    let lhs = self.build_expr(&binary.left, ir_block);
    let rhs = self.build_expr(&binary.right, ir_block);
    let dest = self.ctx.fresh_reg();

    let instr = match binary.operator {
      ast::Operator::ADD => ir::Instr::ADD { lhs, rhs, dest: dest.clone() },
      ast::Operator::SUB => ir::Instr::SUB { lhs, rhs, dest: dest.clone() },
      ast::Operator::MUL => ir::Instr::MUL { lhs, rhs, dest: dest.clone() },
      ast::Operator::DIV => ir::Instr::DIV { lhs, rhs, dest: dest.clone() },
      ast::Operator::MOD => ir::Instr::MOD { lhs, rhs, dest: dest.clone() },
      ast::Operator::RANGE => ir::Instr::CMPGT { lhs, rhs, dest: dest.clone() },
      ast::Operator::EQ => ir::Instr::CMPEQ { lhs, rhs, dest: dest.clone() },
      ast::Operator::NOTEQ => ir::Instr::CMPLT { lhs, rhs, dest: dest.clone() },
      ast::Operator::LE => ir::Instr::CMPLE { lhs, rhs, dest: dest.clone() },
      ast::Operator::GE => ir::Instr::CMPGT { lhs, rhs, dest: dest.clone() },
      ast::Operator::LT => ir::Instr::CMPLT { lhs, rhs, dest: dest.clone() },
      ast::Operator::GT => ir::Instr::CMPGT { lhs, rhs, dest: dest.clone() },
      ast::Operator::BOR => ir::Instr::JMPIF { cond: lhs, l1: dest.clone(), l0: rhs },
      ast::Operator::SHL => ir::Instr::JMPIF { cond: lhs, l1: dest.clone(), l0: rhs },
      ast::Operator::SHR => ir::Instr::JMPIF { cond: lhs, l1: dest.clone(), l0: rhs },
      ast::Operator::POW => ir::Instr::JMPIF { cond: lhs, l1: dest.clone(), l0: rhs },
      ast::Operator::AND => ir::Instr::JMPIF { cond: lhs, l1: dest.clone(), l0: rhs },
      ast::Operator::OR => ir::Instr::JMPIF { cond: lhs, l1: dest.clone(), l0: rhs },
      ast::Operator::XOR => ir::Instr::JMPIF { cond: lhs, l1: dest.clone(), l0: rhs },
      _ => unimplemented!(),
    };
    ir_block.add_instr(instr);
    dest
  }

  fn build_literal_expr(&mut self, literal: &ast::Literal, ir_block: &mut ir::BlockIr) -> ir::REG {
    let reg = self.ctx.fresh_reg();
    let value = match literal {
      ast::Literal::Num(num) => num.text.clone(),
      ast::Literal::String(string) => string.text.clone(),
      ast::Literal::Char(char) => char.value.to_string(),
      ast::Literal::Bool(bool) => bool.value.to_string(),
      ast::Literal::Null(_) => "null".to_owned(),
    };
    ir_block.add_instr(ir::Instr::OWN { value, dest: reg.clone() });
    reg
  }

  fn build_ident_expr(&mut self, ident: &ast::Ident) -> ir::REG {
    if let Some(bind) = self.ctx.get_variable(&ident.text) {
      bind.reg.clone()
    } else {
      let text = format!("not found value: {}", ident.text);
      report::throw_error(text);
    }
  }

  fn build_type(&mut self, ast_type: &ast_type::AstType) -> String {
    let ty = match ast_type {
      ast_type::AstType::Numb(numb) => numb.display(),
      ast_type::AstType::Float(float) => float.display(),
      ast_type::AstType::Bool(_) => "bool".to_owned(),
      ast_type::AstType::String(_) => "string".to_owned(),
      ast_type::AstType::Char(_) => "char".to_owned(),
      ast_type::AstType::Fn(fn_type) => self.build_fn_type(fn_type),
      _ => unimplemented!(),
    };
    ty
  }

  fn build_fn_type(&mut self, fn_type: &ast_type::FnType) -> String {
    let mut params = Vec::with_capacity(fn_type.params.len());

    for param in fn_type.params.iter() {
      params.push(self.build_type(param));
    }

    let ret_type = match &fn_type.ret_type {
      Some(ret_type) => format!(" -> {}", self.build_type(ret_type)),
      None => "".to_owned(),
    };
    format!("({}){}", params.join(", "), ret_type)
  }
}
