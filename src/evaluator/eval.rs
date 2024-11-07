#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use super::ctx::Ctx;
use super::errors;
use super::stack::CallStack;
use super::std::{create_std_fn, StdFn};
use super::value::{create_bool, create_null, create_num, create_object, create_string, FnValue, Value};
use crate::{ast, diag::Diag, range::Range};

pub struct Evaluator {
  std: HashMap<&'static str, StdFn>,
  call_stack: CallStack,
}

type EvalResult = Result<Value, Diag>;

impl Evaluator {
  pub fn new() -> Self {
    Self { std: create_std_fn(), call_stack: CallStack::new() }
  }

  pub fn eval(&mut self, program: &ast::Program) -> EvalResult {
    let mut ctx = Ctx::new(None);
    let mut value: Option<Value> = None;
    for stmt in &program.stmts {
      value = Some(self.eval_stmt(stmt, &mut ctx)?);
    }
    Ok(value.unwrap_or_else(|| create_null()))
  }

  fn eval_stmt(&mut self, stmt: &ast::Stmts, ctx: &mut Ctx) -> EvalResult {
    match stmt {
      ast::Stmts::Let(let_stmt) => self.eval_let_stmt(let_stmt, ctx),
      ast::Stmts::Expr(expr) => self.eval_expr(expr, ctx),
      ast::Stmts::Fn(fn_stmt) => self.eval_fn_stmt(fn_stmt, ctx),
      ast::Stmts::Block(block_stmt) => self.eval_block_stmt(block_stmt, ctx),
    }
  }
  // -------- statements -------
  fn eval_let_stmt(&mut self, let_stmt: &ast::LetStmt, ctx: &mut Ctx) -> EvalResult {
    let value = self.eval_expr(&let_stmt.expr, ctx)?;
    ctx.set(let_stmt.pat.ident.text.clone(), value);
    Ok(create_null())
  }

  fn eval_fn_stmt(&mut self, fn_stmt: &ast::FunctionStmt, ctx: &mut Ctx) -> EvalResult {
    let name = fn_stmt.name.text.clone();
    let pats: Vec<String> = fn_stmt.pats.iter().map(|pat| pat.ident.text.clone()).collect();
    let fn_ctx = Ctx::new(Some(Box::new(ctx.clone())));
    let mut fn_value = FnValue::new(fn_ctx, Some(name.clone()), pats, fn_stmt.body.clone());
    fn_value.set_value(&fn_stmt.name.text, Value::Fn(fn_value.clone()));
    ctx.set(name, Value::Fn(fn_value));
    Ok(create_null())
  }

  fn eval_block_stmt(&mut self, block_stmt: &ast::BlockStmt, ctx: &mut Ctx) -> EvalResult {
    let mut ctx = Ctx::new(Some(Box::new(ctx.clone())));
    let mut value = create_null();
    for stmt in &block_stmt.stmts {
      value = self.eval_stmt(stmt, &mut ctx)?;
    }
    Ok(value)
  }

  // -------- expressions -------
  fn eval_expr(&mut self, expr: &ast::Expr, ctx: &mut Ctx) -> EvalResult {
    match expr {
      ast::Expr::Fn(fn_expr) => self.eval_fn_expr(fn_expr, ctx),
      ast::Expr::Group(group) => self.eval_group_expr(group, ctx),
      ast::Expr::Binary(binary) => self.eval_binary_expr(binary, ctx),
      // ast::Expr::Pipe(pipe) => self.eval_pipe_expr(pipe, ctx),
      ast::Expr::Unary(unary) => self.eval_unary_expr(unary, ctx),
      ast::Expr::Call(call) => self.eval_call_expr(call, ctx),
      ast::Expr::Match(match_expr) => self.eval_match_expr(match_expr, ctx),
      ast::Expr::Idx(index) => self.eval_idx_expr(index, ctx),
      ast::Expr::Member(member) => self.eval_member_expr(member, ctx),
      // ast::Expr::If(if_expr) => self.eval_if_expr(if_expr, ctx),
      // ast::Expr::Return(return_expr) => self.eval_return_expr(return_expr, ctx),
      ast::Expr::Ident(ident) => self.eval_ident(ident, ctx),
      ast::Expr::Object(object) => self.eval_object_expr(object, ctx),
      ast::Expr::Literal(literal) => self.eval_literal(literal, ctx),
      _ => self.create_diag("unknown expression".to_owned(), &expr.get_range()),
    }
  }

  fn eval_fn_expr(&mut self, fn_expr: &ast::FnExpr, ctx: &mut Ctx) -> EvalResult {
    let fn_ctx = Ctx::new(Some(Box::new(ctx.clone())));
    let pats: Vec<String> = fn_expr.pats.iter().map(|pat| pat.ident.text.clone()).collect();
    let fn_value = FnValue::new(fn_ctx, None, pats, fn_expr.body.clone());
    Ok(Value::Fn(fn_value))
  }

  fn eval_group_expr(&mut self, group: &ast::GroupExpr, ctx: &mut Ctx) -> EvalResult {
    self.eval_expr(&group.expr, ctx)
  }

  fn eval_pipe_expr(&mut self, pipe: &ast::BinaryExpr, ctx: &mut Ctx) -> EvalResult {
    let left = self.eval_expr(&pipe.left, ctx)?;
    let right = self.eval_expr(&pipe.right, ctx)?;
    match right {
      Value::Fn(fn_value) => {
        let args = vec![left];
        // put right value as first argument
        return self.apply_fn(&fn_value, &args, &pipe.range);
      }
      Value::StdFn(_, fn_value) => {
        let args = vec![left];
        return Ok(fn_value(args));
      }
      _ => return self.create_diag(errors::format_expected_function(&right), pipe.right.get_range()),
    }
  }

  fn eval_binary_expr(&mut self, binary: &ast::BinaryExpr, ctx: &mut Ctx) -> EvalResult {
    if binary.operator == ast::Operator::PIPE {
      return self.eval_pipe_expr(&binary, ctx);
    }
    let left = self.eval_expr(&binary.left, ctx)?;
    let right = self.eval_expr(&binary.right, ctx)?;
    match (&left, &right) {
      (Value::Num(lt), Value::Num(rt)) => {
        let value = match binary.operator {
          ast::Operator::ADD => lt.get() + rt.get(),
          ast::Operator::SUB => lt.get() - rt.get(),
          ast::Operator::MUL => lt.get() * rt.get(),
          ast::Operator::DIV => lt.get() / rt.get(),
          ast::Operator::REM => lt.get() % rt.get(),
          _ => {
            let msg = errors::format_unsupported_operator(&left, &binary.operator, &right);
            return self.create_diag(msg, &binary.range_op);
          }
        };
        Ok(create_num(value))
      }
      (Value::String(left), Value::String(right)) => match binary.operator {
        ast::Operator::RANGE => Ok(create_string(left.get().to_owned() + right.get())),
        _ => self.create_diag("unknown operator".to_owned(), &binary.range_op),
      },
      (Value::Bool(lt), Value::Bool(rt)) => match binary.operator {
        ast::Operator::AND => Ok(create_bool(lt.get() && rt.get())),
        ast::Operator::OR => Ok(create_bool(lt.get() || rt.get())),
        ast::Operator::XOR => Ok(create_bool(lt.get() ^ rt.get())),
        _ => {
          let msg = errors::format_unsupported_operator(&left, &binary.operator, &right);
          return self.create_diag(msg, &binary.range_op);
        }
      },
      _ => self.create_diag("unknown operator".to_owned(), &binary.range_op),
    }
  }

  fn eval_unary_expr(&mut self, unary: &ast::UnaryExpr, ctx: &mut Ctx) -> EvalResult {
    todo!("implement eval_unary_expr")
  }

  fn eval_call_expr(&mut self, call: &ast::CallExpr, ctx: &mut Ctx) -> EvalResult {
    let callee = self.eval_expr(&call.callee, ctx)?;
    match callee {
      Value::Fn(fn_value) => {
        let args = call.args.iter().map(|arg| self.eval_expr(arg, ctx)).collect::<Result<Vec<Value>, Diag>>()?;
        self.apply_fn(&fn_value, &args, call.get_range())
      }
      Value::StdFn(_, fn_value) => {
        let args = call.args.iter().map(|arg| self.eval_expr(arg, ctx)).collect::<Result<Vec<Value>, Diag>>()?;
        return Ok(fn_value(args));
      }
      _ => return self.create_diag(errors::format_expected_function(&callee), call.callee.get_range()),
    }
  }

  fn eval_match_expr(&mut self, match_expr: &ast::MatchExpr, ctx: &mut Ctx) -> EvalResult {
    let value = self.eval_expr(&match_expr.expr, ctx)?;
    for arm in &match_expr.arms {
      if self.check_ident(&arm.guard, "_") {
        return self.eval_stmt(&arm.body, ctx);
      }
      let guard_value = self.eval_expr(&arm.guard, ctx)?;
      if value.is_eq(&guard_value) {
        return self.eval_stmt(&arm.body, ctx);
      }
    }
    Ok(create_null())
  }

  fn eval_idx_expr(&mut self, index: &ast::IdxExpr, ctx: &mut Ctx) -> EvalResult {
    let obj = self.eval_expr(&index.object, ctx)?;
    let idx = self.eval_expr(&index.index, ctx)?;
    match (&obj, &idx) {
      (Value::Array(array), Value::Num(idx)) => {
        if let Some(value) = array.get(idx.get() as usize) {
          return Ok(value.to_owned());
        }
        return Ok(create_null());
      }
      (Value::String(string), Value::Num(idx)) => {
        let value = string.get().chars().nth(idx.get() as usize).unwrap_or('\0');
        return Ok(create_string(value.to_string()));
      }
      (_, Value::Num(_)) => {
        let msg = errors::format_mismatched_types("string", &obj);
        return self.create_diag(msg, &index.object.get_range());
      }
      (Value::Object(_), _) => {
        let msg = errors::format_mismatched_types("string", &idx);
        return self.create_diag(msg, &index.index.get_range());
      }
      _ => {
        let msg = errors::format_unsupported_operator(&obj, &ast::Operator::RANGE, &idx);
        return self.create_diag(msg, &index.index.get_range());
      }
    }
  }

  fn eval_member_expr(&mut self, member: &ast::MemberExpr, ctx: &mut Ctx) -> EvalResult {
    let object = self.eval_expr(&member.object, ctx)?;
    match object {
      Value::Object(object) => {
        let _ctx = ctx.create_ctx_object(object.value.clone());
        let mut new_ctx = Ctx::new(Some(Box::new(_ctx)));
        return self.eval_expr(&member.property, &mut new_ctx);
      }
      _ => {
        return self.create_diag("unknown operator".to_owned(), &member.property.get_range());
      }
    }
  }

  fn eval_ident(&mut self, ident: &ast::Identifier, ctx: &mut Ctx) -> EvalResult {
    let text = ident.text.as_str();
    if let Some(value) = ctx.get(&ident.text) {
      return Ok(value.to_owned());
    }
    if let Some(value) = self.call_stack.get(&text) {
      return Ok(value.to_owned());
    }

    if let Some(value) = self.std.get(&text) {
      return Ok(Value::StdFn(text.to_owned(), value.to_owned()));
    }

    let msg = errors::format_undeclared_variable(&ident.text);
    return self.create_diag(msg, &ident.get_range());
  }

  fn eval_return_expr(&mut self, return_expr: &ast::ReturnExpr, ctx: &mut Ctx) -> EvalResult {
    if let Some(expr) = &return_expr.value {
      self.eval_expr(expr, ctx)?;
    }
    Ok(create_null())
  }

  fn eval_object_expr(&mut self, object: &ast::ObjectExpr, ctx: &mut Ctx) -> EvalResult {
    let mut value: HashMap<String, Value> = HashMap::new();
    for field in &object.fields {
      let right = self.eval_expr(&field.right, ctx)?;
      value.insert(field.left.text.clone(), right);
    }
    let self_value = create_object(value.clone());
    value.iter_mut().all(|(key, v)| {
      if let Value::Fn(fn_value) = v {
        fn_value.create_self(self_value.clone());
      }
      true
    });

    Ok(create_object(value))
  }

  fn eval_literal(&mut self, literal: &ast::Literal, ctx: &mut Ctx) -> EvalResult {
    match literal {
      ast::Literal::Number(number) => Ok(create_num(number.text.parse::<f64>().unwrap())),
      ast::Literal::String(string) => Ok(create_string(string.text.clone())),
      ast::Literal::Boolean(boolean) => Ok(create_bool(boolean.value)),
      ast::Literal::Null(_) => Ok(create_null()),
    }
  }

  // --- private utils ---

  fn apply_fn(&mut self, fn_value: &FnValue, args: &[Value], range: &Range) -> EvalResult {
    let mut ctx = fn_value.ctx.to_owned();

    if args.len() != fn_value.pats.len() {
      let msg = errors::format_function_arity_mismatch(fn_value.pats.len(), args.len());
      return self.create_diag(msg, range);
    }
    self.call_stack.push_frame(range.clone())?;
    self.call_stack.push(Value::Fn(fn_value.clone()));
    for (pat, arg) in fn_value.pats.iter().zip(args.iter()) {
      ctx.set(pat.clone(), arg.to_owned());
    }
    let result = self.eval_stmt(&fn_value.stmt, &mut ctx);

    self.call_stack.pop_frame();

    return result;
  }

  fn create_diag(&self, message: String, range: &Range) -> EvalResult {
    Err(Diag::create_err(message, range.clone()))
  }

  fn check_ident(&self, expr: &ast::Expr, expect: &str) -> bool {
    match expr {
      ast::Expr::Ident(ident) => ident.text == expect,
      _ => false,
    }
  }

  pub fn std(&self) -> &HashMap<&'static str, StdFn> {
    &self.std
  }
}
