#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;

use serde::ser;

use crate::{
	ast,
	checker::{
		context::store,
		types::{self, TypeId},
	},
};

use super::ir::{self, HeapValue};

mod build_block_stmt;
mod build_fn_stmt;

pub struct IrContext {
	values: Vec<HashMap<String, ir::Register>>,
	fn_value: HashMap<ir::Register, ir::FnId>,
}

impl IrContext {
	pub fn new() -> Self {
		Self { values: Vec::from_iter(vec![HashMap::new()]), fn_value: HashMap::new() }
	}

	pub fn add_value(&mut self, name: &str, register: ir::Register) {
		self.get_current_scope().insert(name.to_string(), register);
	}
	pub fn get_value(&self, name: &str) -> Option<&ir::Register> {
		self.values.iter().rev().find_map(|scope| scope.get(name))
	}
	pub fn get_fn_value(&mut self, register: ir::Register) -> Option<&ir::FnId> {
		self.fn_value.get(&register)
	}
	pub fn add_fn_value(&mut self, fn_id: ir::FnId, register: ir::Register) {
		self.fn_value.insert(register, fn_id);
	}

	pub fn enter_scope(&mut self) {
		self.values.push(HashMap::new());
	}
	pub fn exit_scope(&mut self) {
		self.values.pop();
	}

	pub fn get_current_scope(&mut self) -> &mut HashMap<String, ir::Register> {
		self.values.last_mut().unwrap()
	}
}

impl Default for IrContext {
	fn default() -> Self {
		Self::new()
	}
}

pub struct Builder<'br> {
	store: &'br store::Store,
	type_store: &'br types::TypeStore,
	ir_ctx: IrContext,
	// internal
	register_id: usize,
	label_id: usize,
	fn_ir: Option<ir::Fn>,
	block_ir: Vec<ir::Block>,

	pub ir: ir::Ir,
}

impl<'br> Builder<'br> {
	pub fn new(store: &'br store::Store, type_store: &'br types::TypeStore) -> Self {
		let register_id = 0;
		let label_id = 0;
		let ir = ir::Ir::new();
		let ir_ctx = IrContext::new();
		Self { store, type_store, ir_ctx, register_id, label_id, ir, fn_ir: None, block_ir: vec![] }
	}

	pub fn build(&mut self, program: &ast::Program) -> ir::Ir {
		for stmt in program.stmts.iter() {
			self.build_stmt(stmt);
		}
		self.ir.clone()
	}

	pub fn build_stmt(&mut self, stmt: &ast::Stmt) {
		match stmt {
			ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.build_fn_stmt(fn_stmt),
			ast::Stmt::Block(block_stmt) => self.build_block_stmt(block_stmt),
			ast::Stmt::Expr(expr) => {
				self.build_expr(expr);
			}
		}
	}

	fn build_let_stmt(&mut self, let_stmt: &ast::LetStmt) {
		let label = self.take_label();
		self.start_block(label);
		let bind = self.build_binding(&let_stmt.name);
		let value = self.build_expr(&let_stmt.expr);
		self.add_instr(ir::Instr::new(ir::Code::OWN { value, dest: bind.register }));
		self.end_block();
	}

	fn build_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) {
		let register = self.take_register();
		let name = fn_stmt.name.lexeme().to_owned();
		self.ir_ctx.add_value(fn_stmt.lexeme(), register);
		let params = self.build_fn_params(&fn_stmt.params);
		let ret_id = self.build_fn_return_type(fn_stmt.return_type.as_ref());
		self.start_fn(register, name, params, ret_id);
		self.ir_ctx.enter_scope();
		self.build_fn_body(fn_stmt.body.as_ref());
		self.end_fn();
		self.ir_ctx.exit_scope();
	}

	fn build_fn_params(&mut self, params: &[ast::Binding]) -> Vec<ir::Bind> {
		let mut binds = Vec::with_capacity(params.len());
		for param in params {
			let bind = self.build_binding(param);
			binds.push(bind);
		}
		binds
	}

	fn build_binding(&mut self, binding: &ast::Binding) -> ir::Bind {
		let register = self.take_register();
		self.ir_ctx.add_value(binding.lexeme(), register);
		ir::Bind { register, type_id: self.nothing_type_id() }
	}

	fn build_fn_return_type(&mut self, _ret_type: Option<&ast::AstType>) -> TypeId {
		self.nothing_type_id()
	}

	fn build_block_stmt(&mut self, block_stmt: &ast::BlockStmt) {
		self.ir_ctx.enter_scope();
		let label = self.take_label();
		self.start_block(label);
		for stmt in block_stmt.stmts.iter() {
			self.build_stmt(stmt);
		}
		self.end_block();
		self.ir_ctx.exit_scope();
	}

	fn build_fn_body(&mut self, stmt: &ast::Stmt) {
		let label = self.take_label();
		self.start_block(label);
		match stmt {
			ast::Stmt::Block(block_stmt) => {
				for stmt in block_stmt.stmts.iter() {
					self.build_stmt(stmt);
				}
			}
			_ => self.build_stmt(stmt),
		}
		self.end_block();
	}

	fn build_expr(&mut self, expr: &ast::Expr) -> ir::Register {
		match expr {
			ast::Expr::Binary(binary) => self.build_binary_expr(binary),
			ast::Expr::Literal(literal) => self.build_literal_expr(literal),
			ast::Expr::If(if_expr) => self.build_if_expr(if_expr),
			ast::Expr::Ident(ident) => self.build_ident_expr(ident),
			ast::Expr::Call(call) => self.build_call_expr(call),
			ast::Expr::Ret(ret) => self.build_ret_expr(ret),
			ast::Expr::Deref(deref) => self.build_deref_expr(deref),
			ast::Expr::Ref(ref_expr) => self.build_ref_expr(ref_expr),
			_ => todo!(),
		}
	}
	// &mut <expr>
	fn build_ref_expr(&mut self, ref_expr: &ast::RefExpr) -> ir::Register {
		let register = self.take_register();
		let value = self.build_expr(&ref_expr.expr);
		if ref_expr.mutable.is_some() {
			self.add_instr(ir::Instr::new(ir::Code::BORROW_MUT { value, dest: register }));
		} else {
			self.add_instr(ir::Instr::new(ir::Code::BORROW { value, dest: register }));
		}
		register
	}

	// *<expr>
	fn build_deref_expr(&mut self, deref_expr: &ast::DerefExpr) -> ir::Register {
		let register = self.take_register();
		let value = self.build_expr(&deref_expr.expr);
		self.add_instr(ir::Instr::new(ir::Code::LOAD { value, dest: register }));
		register
	}

	fn build_literal_expr(&mut self, literal: &ast::Literal) -> ir::Register {
		match literal {
			ast::Literal::Number(number) => self.build_number_expr(number),
			ast::Literal::String(string) => self.build_string_expr(string),
			ast::Literal::Char(char) => self.build_char_expr(char),
			ast::Literal::Bool(bool) => self.build_bool_expr(bool),
			ast::Literal::Null(_) => todo!(),
		}
	}

	fn build_number_expr(&mut self, number: &ast::NumberLiteral) -> ir::Register {
		let register = self.take_register();
		let value = if number.as_dot() {
			HeapValue::new_float(&number.text)
		} else {
			HeapValue::new_int(&number.text)
		};
		self.add_instr(ir::Instr::new_heap(value, register));
		register
	}

	fn build_string_expr(&mut self, string: &ast::StringLiteral) -> ir::Register {
		let register = self.take_register();
		let value = HeapValue::new_string(&string.text);
		self.add_instr(ir::Instr::new_heap(value, register));
		register
	}

	fn build_char_expr(&mut self, char: &ast::CharLiteral) -> ir::Register {
		let register = self.take_register();
		let value = HeapValue::new_char(char.value);
		self.add_instr(ir::Instr::new_heap(value, register));
		register
	}
	fn build_bool_expr(&mut self, bool: &ast::BoolLiteral) -> ir::Register {
		let register = self.take_register();
		let value = HeapValue::new_bool(bool.value);
		self.add_instr(ir::Instr::new_heap(value, register));
		register
	}

	fn build_ident_expr(&mut self, ident: &ast::Ident) -> ir::Register {
		if let Some(bind) = self.ir_ctx.get_value(ident.lexeme()) {
			*bind
		} else {
			panic!("unknown identifier '{}'", ident.lexeme());
		}
	}

	fn build_call_expr(&mut self, call_expr: &ast::CallExpr) -> ir::Register {
		let callee_id = self.build_expr(&call_expr.callee);
		let fn_id = self.get_fn_id(callee_id);
		let mut args = Vec::with_capacity(call_expr.args.len());
		for arg in call_expr.args.iter() {
			args.push(self.build_expr(arg));
		}
		let register = self.take_register();
		self.add_instr(ir::Instr::new(ir::Code::CALL { fn_id, args, dest: register }));
		register
	}

	fn build_ret_expr(&mut self, ret_expr: &ast::RetExpr) -> ir::Register {
		let value = match &ret_expr.value {
			Some(value) => self.build_expr(value),
			None => ir::Register(0),
		};
		self.add_instr(ir::Instr::new(ir::Code::RET { value }));
		value
	}

	fn get_fn_id(&mut self, fn_register: ir::Register) -> ir::FnId {
		let fn_id = self.ir_ctx.get_fn_value(fn_register);
		if fn_id.is_none() {
			panic!("unknown fn id '{}'", fn_register);
		}
		fn_id.unwrap().clone()
	}

	fn build_if_expr(&mut self, if_expr: &ast::IfExpr) -> ir::Register {
		let mut l0 = ir::Label(self.label_id);
		let label = self.take_label();
		self.start_block(label);
		let cond = self.build_expr(&if_expr.cond);
		self.end_block();
		let l1 = self.take_label();
		self.start_block(l1);
		self.build_stmt(&if_expr.then);
		self.end_block();
		if let Some(otherwise) = &if_expr.otherwise {
			l0 = self.take_label();
			self.start_block(label);
			self.build_stmt(otherwise);
			self.end_block();
		}
		let instr = ir::Instr::new(ir::Code::JMPIF { cond, l1, l0 });
		self.add_instr(instr);
		self.take_register() // todo: we need to return the value
	}

	pub fn build_binary_expr(&mut self, binary: &ast::BinaryExpr) -> ir::Register {
		let left = self.build_expr(&binary.left);
		let right = self.build_expr(&binary.right);
		let register = self.take_register();
		let instr = match binary.operator {
			ast::Operator::ADD => ir::Instr::new(ir::Code::ADD { lhs: left, rhs: right, dest: register }),
			ast::Operator::SUB => ir::Instr::new(ir::Code::SUB { lhs: left, rhs: right, dest: register }),
			ast::Operator::MUL => ir::Instr::new(ir::Code::MUL { lhs: left, rhs: right, dest: register }),
			ast::Operator::DIV => ir::Instr::new(ir::Code::DIV { lhs: left, rhs: right, dest: register }),
			ast::Operator::MOD => ir::Instr::new(ir::Code::MOD { lhs: left, rhs: right, dest: register }),
			ast::Operator::RANGE => {
				ir::Instr::new(ir::Code::CMPGT { lhs: left, rhs: right, dest: register })
			}
			ast::Operator::EQ => {
				ir::Instr::new(ir::Code::CMPEQ { lhs: left, rhs: right, dest: register })
			}
			ast::Operator::NOTEQ => {
				ir::Instr::new(ir::Code::CMPLT { lhs: left, rhs: right, dest: register })
			}
			ast::Operator::LE => {
				ir::Instr::new(ir::Code::CMPLE { lhs: left, rhs: right, dest: register })
			}
			ast::Operator::GE => {
				ir::Instr::new(ir::Code::CMPGT { lhs: left, rhs: right, dest: register })
			}
			ast::Operator::LT => {
				ir::Instr::new(ir::Code::CMPLT { lhs: left, rhs: right, dest: register })
			}
			ast::Operator::GT => {
				ir::Instr::new(ir::Code::CMPGT { lhs: left, rhs: right, dest: register })
			}
			_ => todo!(),
		};

		self.add_instr(instr);
		register
	}

	pub fn take_register(&mut self) -> ir::Register {
		let reg = ir::Register(self.register_id);
		self.register_id += 1;
		reg
	}

	pub fn take_label(&mut self) -> ir::Label {
		let label = ir::Label(self.label_id);
		self.label_id += 1;
		label
	}

	pub fn start_fn(
		&mut self,
		reg: ir::Register,
		name: String,
		params: Vec<ir::Bind>,
		ret_ty: TypeId,
	) {
		let fn_id = ir::FnId(name);
		self.ir_ctx.add_fn_value(fn_id.clone(), reg);
		let fn_ir = ir::Fn::new(fn_id, params, ret_ty);
		self.fn_ir = Some(fn_ir);
	}

	pub fn end_fn(&mut self) {
		if let Some(fn_ir) = self.fn_ir.take() {
			self.ir.add_fn(fn_ir);
		}
	}

	pub fn start_block(&mut self, label: ir::Label) {
		let block = ir::Block::new(label);
		self.block_ir.push(block);
	}

	pub fn end_block(&mut self) {
		if let Some(block) = self.block_ir.pop() {
			if let Some(fn_ir) = self.fn_ir.as_mut() {
				fn_ir.add_block(block);
			}
		}
	}

	pub fn add_instr(&mut self, instr: ir::Instr) {
		if let Some(block) = self.block_ir.last_mut() {
			block.add_instr(instr)
		} else {
			panic!("no block to add instr");
		}
	}

	pub fn add_fn(&mut self) {
		let fn_ir = self.fn_ir.take().unwrap();
		self.ir.add_fn(fn_ir);
	}

	pub fn nothing_type_id(&mut self) -> TypeId {
		TypeId::NOTHING
	}
}
