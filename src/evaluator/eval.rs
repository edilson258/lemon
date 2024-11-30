#![allow(dead_code, unused_variables)]
use std::collections::HashMap;
use std::path::PathBuf;

use super::ctx::Ctx;
use super::errors;
use super::native::NativeRegistry;
use super::stack::CallStack;
use super::value::{value_factory, FnValue, ObjectValue, StringValue, Value};
use crate::lexer::Lexer;
use crate::loader::Loader;
use crate::parser::Parser;
use crate::source::Source;
use crate::{ast, diag::Diag, range::Range};

pub struct Evaluator {
  native_registry: NativeRegistry,
  call_stack: CallStack,
  path: PathBuf,
  pipe_back: Option<Value>,
  loader: Loader,
  breaked: bool,
  skiped: bool,
}

type EvalResult = Result<Value, Diag>;

impl Evaluator {
  pub fn new(path: PathBuf) -> Self {
    let cwd = std::env::current_dir().unwrap();
    let call_stack = CallStack::new(path.clone());
    let loader = Loader::new(cwd);
    let pipe_back = None;
    let native_registry = NativeRegistry::new();
    let breaked = false;
    let skiped = false;
    Self { call_stack, pipe_back, loader, path, native_registry, breaked, skiped }
  }

  pub fn eval(&mut self, program: &ast::Program, ctx: &mut Ctx) -> Result<(), Diag> {
    for stmt in &program.stmts {
      match self.eval_stmt(stmt, ctx) {
        Ok(_) => {}
        Err(diag) => return Err(diag),
      }
    }
    return Ok(());
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
    Ok(value_factory::create_null())
  }

  fn eval_fn_stmt(&mut self, fn_stmt: &ast::FunctionStmt, ctx: &mut Ctx) -> EvalResult {
    let name = fn_stmt.name.text.clone();
    let pats: Vec<String> = fn_stmt.pats.iter().map(|pat| pat.ident.text.clone()).collect();
    let fn_ctx = Ctx::new(Some(Box::new(ctx.clone())));
    let fn_value = FnValue::new(fn_ctx, pats, fn_stmt.body.clone());
    ctx.set(name, Value::Fn(fn_value));
    Ok(value_factory::create_null())
  }

  fn eval_block_stmt(&mut self, block_stmt: &ast::BlockStmt, ctx: &mut Ctx) -> EvalResult {
    let mut ctx = Ctx::new(Some(Box::new(ctx.clone())));
    let mut value = value_factory::create_null();
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
      ast::Expr::Pipe(pipe) => self.eval_pipe_expr(pipe, ctx),
      ast::Expr::Unary(unary) => self.eval_unary_expr(unary, ctx),
      ast::Expr::Call(call) => self.eval_call_expr(call, ctx),
      ast::Expr::Match(match_expr) => self.eval_match_expr(match_expr, ctx),
      ast::Expr::Idx(index) => self.eval_idx_expr(index, ctx),
      ast::Expr::Member(member) => self.eval_member_expr(member, ctx),
      ast::Expr::Import(import) => self.eval_import(import, ctx),
      ast::Expr::For(for_expr) => self.eval_for_expr(for_expr, ctx),
      // ast::Expr::While(while_expr) => self.eval_while_expr(while_expr, ctx),
      // ast::Expr::Loop(loop_expr) => self.eval_loop_expr(loop_expr, ctx),
      ast::Expr::Break(break_expr) => self.eval_break_expr(break_expr, ctx),
      ast::Expr::Skip(skip_expr) => self.eval_skip_expr(skip_expr, ctx),
      ast::Expr::If(if_expr) => self.eval_if_expr(if_expr, ctx),
      ast::Expr::Return(return_expr) => self.eval_return_expr(return_expr, ctx),
      ast::Expr::Ident(ident) => self.eval_ident(ident, ctx),
      ast::Expr::Object(object) => self.eval_object_expr(object, ctx),
      ast::Expr::Array(array) => self.eval_array_expr(array, ctx),
      ast::Expr::Literal(literal) => self.eval_literal(literal, ctx),
      _ => self.create_diag("unknown expression".to_owned(), &expr.get_range()),
    }
  }

  fn eval_fn_expr(&mut self, fn_expr: &ast::FnExpr, ctx: &mut Ctx) -> EvalResult {
    let fn_ctx = Ctx::new(Some(Box::new(ctx.clone())));
    let pats: Vec<String> = fn_expr.pats.iter().map(|pat| pat.ident.text.clone()).collect();
    let fn_value = FnValue::new(fn_ctx, pats, fn_expr.body.clone());
    Ok(Value::Fn(fn_value))
  }

  fn eval_group_expr(&mut self, group: &ast::GroupExpr, ctx: &mut Ctx) -> EvalResult {
    self.eval_expr(&group.expr, ctx)
  }

  fn eval_pipe_expr(&mut self, pipe: &ast::PipeExpr, ctx: &mut Ctx) -> EvalResult {
    let left = self.eval_expr(&pipe.left, ctx)?;
    self.pipe_back = Some(left);
    let right = self.eval_expr(&pipe.right, ctx)?;

    if let Value::NativeFn(native) = right {
      let arg = self.pipe_back.take().unwrap();
      return native.apply(vec![arg], &self.path, &pipe.get_range());
    }

    if let Value::Fn(fn_value) = right {
      let arg = self.pipe_back.take().unwrap();
      return self.apply_fn(&fn_value, &vec![arg], pipe.get_range());
    }

    if self.pipe_back.is_some() {
      return self.create_diag(errors::format_expected_function(&right), pipe.right.get_range());
    }
    return Ok(right);
  }

  fn eval_binary_expr(&mut self, binary: &ast::BinaryExpr, ctx: &mut Ctx) -> EvalResult {
    let left = self.eval_expr(&binary.left, ctx)?;
    let right = self.eval_expr(&binary.right, ctx)?;
    match (&left, &right) {
      (Value::Num(lt), Value::Num(rt)) => {
        let value = match binary.operator {
          ast::Operator::ADD => value_factory::create_num(lt.get() + rt.get()),
          ast::Operator::SUB => value_factory::create_num(lt.get() - rt.get()),
          ast::Operator::MUL => value_factory::create_num(lt.get() * rt.get()),
          ast::Operator::DIV => value_factory::create_num(lt.get() / rt.get()),
          ast::Operator::BOR => value_factory::create_num((lt.get() as i64 | rt.get() as i64) as f64),
          ast::Operator::REM => value_factory::create_num(lt.get() % rt.get()),
          ast::Operator::LT => value_factory::create_bool(lt.get() < rt.get()),
          ast::Operator::GT => value_factory::create_bool(lt.get() > rt.get()),
          _ => {
            let msg = errors::format_unsupported_operator(&left, &binary.operator, &right);
            return self.create_diag(msg, &binary.range_op);
          }
        };
        Ok(value)
      }
      (Value::String(lt), Value::String(rt)) => match binary.operator {
        ast::Operator::RANGE => {
          let mut string_value = StringValue::with_capacity(lt.len() + rt.len());
          string_value.push_str(lt.get());
          string_value.push_str(rt.get());
          Ok(Value::String(string_value))
        }
        _ => self.create_diag("unknown operator".to_owned(), &binary.range_op),
      },
      (Value::Bool(lt), Value::Bool(rt)) => match binary.operator {
        ast::Operator::AND => Ok(value_factory::create_bool(lt.get() && rt.get())),
        ast::Operator::OR => Ok(value_factory::create_bool(lt.get() || rt.get())),
        ast::Operator::XOR => Ok(value_factory::create_bool(lt.get() ^ rt.get())),
        _ => {
          let msg = errors::format_unsupported_operator(&left, &binary.operator, &right);
          return self.create_diag(msg, &binary.range_op);
        }
      },
      _ => self.create_diag("unknown operator".to_owned(), &binary.range_op),
    }
  }

  fn eval_unary_expr(&mut self, unary: &ast::UnaryExpr, ctx: &mut Ctx) -> EvalResult {
    let value = self.eval_expr(&unary.operand, ctx)?;
    match unary.operator {
      ast::Operator::SUB => {
        if let Value::Num(num) = value {
          return Ok(value_factory::create_num(-num.get()));
        }
        let msg = errors::format_unsupported_operator(&value, &unary.operator, &value_factory::create_num(0.0));
        return self.create_diag(msg, &unary.range_op);
      }
      ast::Operator::NOT => {
        if let Value::Bool(bool) = value {
          return Ok(value_factory::create_bool(!bool.get()));
        }
        let msg = errors::format_unsupported_operator(&value, &unary.operator, &value_factory::create_bool(false));
        return self.create_diag(msg, &unary.range_op);
      }
      _ => self.create_diag("unknown operator".to_owned(), &unary.range_op),
    }
  }

  fn eval_call_expr(&mut self, call: &ast::CallExpr, ctx: &mut Ctx) -> EvalResult {
    let callee = self.eval_expr(&call.callee, ctx)?;

    let mut args = Vec::new();

    if let Some(pipe_back) = self.pipe_back.take() {
      args.push(pipe_back);
    }

    match callee {
      Value::Fn(fn_value) => {
        args.extend(call.args.iter().map(|arg| self.eval_expr(arg, ctx)).collect::<Result<Vec<Value>, Diag>>()?);
        self.apply_fn(&fn_value, &args, call.get_range())
      }
      Value::NativeFn(native) => {
        args.extend(call.args.iter().map(|arg| self.eval_expr(arg, ctx)).collect::<Result<Vec<Value>, Diag>>()?);
        return native.apply(args, &self.path, &call.get_range());
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
    Ok(value_factory::create_null())
  }

  fn eval_idx_expr(&mut self, index: &ast::IdxExpr, ctx: &mut Ctx) -> EvalResult {
    let obj = self.eval_expr(&index.object, ctx)?;
    let idx = self.eval_expr(&index.index, ctx)?;
    match (&obj, &idx) {
      (Value::Array(array), Value::Num(idx)) => {
        if let Some(value) = array.get(idx.get() as usize) {
          return Ok(value.to_owned());
        }
        return Ok(value_factory::create_null());
      }
      (Value::String(string), Value::Num(idx)) => {
        let value = string.get().chars().nth(idx.get() as usize).unwrap_or('\0');
        return Ok(value_factory::create_string(value.to_string()));
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
        ctx.set_self(Value::Object(object));
        return self.eval_expr(&member.property, ctx);
      }
      _ => {
        return self.create_diag(errors::format_missing_field(), &member.property.get_range());
      }
    }
  }
  fn eval_import(&mut self, import: &ast::ImportExpr, ctx: &mut Ctx) -> EvalResult {
    if let Some(mut patterns) = self.loader.get_native(&import.path.text) {
      let (root, module) = self.native_registry.get_nested(&mut patterns);
      if let Some(module) = module {
        return Ok(Value::NativeFn(module.clone()));
      }

      if let Some(root) = root {
        return Ok(Value::Object(ObjectValue::with_native(root)));
      }

      let msg = errors::format_missing_module(&patterns[0]);
      return self.create_diag(msg, &import.get_range());
    }
    match self.loader.load_module_str(&import.path.text, &self.path) {
      Ok(source) => {
        let exports = self.run_eval(source)?;
        return Ok(Value::Object(exports));
      }
      Err(e) => self.create_diag(e, &import.get_range()),
    }
  }

  // for <pat> in <expr> = { <stmts> } or for <idx>, <value> in <expr> = { <stmts> }
  fn eval_for_expr(&mut self, for_expr: &ast::ForExpr, ctx: &mut Ctx) -> EvalResult {
    let mut expr_value = self.eval_expr(&for_expr.expr, ctx)?;
    let value = expr_value.as_array(&for_expr.expr.get_range(), &self.path)?;
    let value_text = for_expr.value.text.clone();
    let pos_text = for_expr.pos.as_ref().map(|p| p.text.as_str());
    let mut for_ctx = Ctx::new(Some(Box::new(ctx.clone())));
    for (index, value) in value.value.iter().enumerate() {
      self.set_value_in_ctx(pos_text, value_factory::create_num(index as f64), &mut for_ctx);
      self.set_value_in_ctx(Some(&value_text), value.to_owned(), &mut for_ctx);
      self.skiped = false;
      match &*for_expr.body {
        ast::Stmts::Block(block_stmt) => {
          for stmt in &block_stmt.stmts {
            let result = self.eval_stmt(stmt, &mut for_ctx)?;
            if self.breaked {
              self.breaked = false;
              return Ok(result);
            }
            if self.skiped {
              self.skiped = false;
              break;
            }
          }
          value_factory::create_null()
        }
        _ => self.eval_stmt(&for_expr.body, &mut for_ctx)?,
      };
    }
    Ok(value_factory::create_null())
  }

  // fn eval_while_expr(&mut self, while_expr: &ast::WhileExpr, ctx: &mut Ctx) -> EvalResult {
  //   let expr = self.eval_expr(&while_expr.expr, ctx)?;
  //   let mut ctx = ctx.create_ctx_object(expr.clone());
  //   while self.eval_expr(&while_expr.expr, &mut ctx)?.is_true() {
  //     self.eval_stmt(&while_expr.body, &mut ctx)?;
  //   }
  //   Ok(value_factory::create_null())
  // }

  // fn eval_loop_expr(&mut self, loop_expr: &ast::LoopExpr, ctx: &mut Ctx) -> EvalResult {
  //   self.eval_stmt(&loop_expr.body, ctx)?;
  //   Ok(value_factory::create_null())
  // }
  //
  fn eval_if_expr(&mut self, if_expr: &ast::IfExpr, ctx: &mut Ctx) -> EvalResult {
    let condition = self.eval_expr(&if_expr.condition, ctx)?;
    let bool_condition = condition.as_bool(&if_expr.condition.get_range(), &self.path)?;
    if bool_condition.get() {
      return self.eval_stmt(&if_expr.consequent, ctx);
    }
    return if let Some(alternate) = &if_expr.alternate {
      self.eval_stmt(alternate, ctx)
    } else {
      Ok(value_factory::create_null())
    };
  }

  fn eval_break_expr(&mut self, break_expr: &ast::BreakExpr, ctx: &mut Ctx) -> EvalResult {
    self.breaked = true;
    if let Some(value) = break_expr.value.as_ref() {
      return self.eval_expr(value, ctx);
    }
    Ok(value_factory::create_null())
  }

  fn eval_skip_expr(&mut self, skip_expr: &ast::SkipExpr, ctx: &mut Ctx) -> EvalResult {
    self.skiped = true;
    Ok(value_factory::create_null())
  }

  fn eval_ident(&mut self, ident: &ast::Identifier, ctx: &mut Ctx) -> EvalResult {
    let text = ident.text.as_str();

    if let Some(this) = ctx.get_self_ctx() {
      if this.is_object() {
        let this = this.as_object(&ident.get_range(), &self.path)?;
        if let Some(value) = this.get(text) {
          return Ok(value.clone());
        }
      }

      if this.is_array() {
        let this = this.as_array(&ident.get_range(), &self.path)?;
        if let Some(value) = this.method(text) {
          return Ok(Value::MethodFn(value.clone()));
        }
      }
    }

    if let Some(value) = ctx.get(&ident.text) {
      return Ok(value.clone());
    }

    if let Some(value) = self.call_stack.get_last_value() {
      return Ok(value.clone());
    }
    let msg = errors::format_undeclared_variable(&ident.text);
    return self.create_diag(msg, &ident.get_range());
  }

  fn eval_return_expr(&mut self, return_expr: &ast::ReturnExpr, ctx: &mut Ctx) -> EvalResult {
    self.breaked = true;
    if let Some(expr) = &return_expr.value {
      self.eval_expr(expr, ctx)?;
    }
    Ok(value_factory::create_null())
  }

  fn eval_object_expr(&mut self, object: &ast::ObjectExpr, ctx: &mut Ctx) -> EvalResult {
    let mut value: HashMap<String, Value> = HashMap::new();
    for field in &object.fields {
      let right = self.eval_expr(&field.right, ctx)?;
      value.insert(field.left.text.clone(), right);
    }
    let self_value = value_factory::create_object(value.clone());
    value.iter_mut().all(|(key, v)| {
      if let Value::Fn(fn_value) = v {
        fn_value.create_self(self_value.clone());
      }
      true
    });

    Ok(value_factory::create_object(value))
  }

  fn eval_array_expr(&mut self, array: &ast::ArrayExpr, ctx: &mut Ctx) -> EvalResult {
    let mut value = Vec::with_capacity(array.fields.len());
    for expr in &array.fields {
      value.push(self.eval_expr(expr, ctx)?);
    }
    Ok(value_factory::create_array(value))
  }
  fn eval_literal(&mut self, literal: &ast::Literal, ctx: &mut Ctx) -> EvalResult {
    match literal {
      ast::Literal::Number(number) => Ok(value_factory::create_num(number.text.parse::<f64>().unwrap())),
      ast::Literal::String(string) => Ok(value_factory::create_string(string.text.clone())),
      ast::Literal::Boolean(boolean) => Ok(value_factory::create_bool(boolean.value)),
      ast::Literal::Null(_) => Ok(value_factory::create_null()),
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
    Err(Diag::create_err(message, range.clone(), self.path.clone()))
  }

  // helpers
  fn check_ident(&self, expr: &ast::Expr, expect: &str) -> bool {
    match expr {
      ast::Expr::Ident(ident) => ident.text == expect,
      _ => false,
    }
  }

  fn is_ident(&self, expr: &ast::Expr) -> bool {
    match expr {
      ast::Expr::Ident(_) => true,
      _ => false,
    }
  }

  fn get_ident_text(&self, expr: &ast::Expr) -> Option<String> {
    match expr {
      ast::Expr::Ident(ident) => Some(ident.text.to_owned()),
      _ => None,
    }
  }

  pub fn get_path(&self) -> &PathBuf {
    &self.path
  }

  pub fn set_value_in_ctx(&mut self, key: Option<&str>, value: Value, ctx: &mut Ctx) {
    if let Some(key) = key {
      ctx.set(key.to_owned(), value);
    }
  }

  pub fn run_eval(&mut self, source: Source) -> Result<ObjectValue, Diag> {
    let path = source.path.clone();
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();
    let mut eval = Evaluator::new(path);
    let mut module_ctx = Ctx::new(None);
    eval.eval(&ast, &mut module_ctx)?;
    let exports = module_ctx.to_object();
    return Ok(exports);
  }
}
