use std::hash::{DefaultHasher, Hasher};

use crate::{
	ast::{self},
	checker::types::{FnType, TypeId},
	ir::ir,
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_fn_stmt(&mut self, fn_stmt: &ast::FnStmt) {
		if fn_stmt.is_generic() {
			return self.build_generics_fns_stmt(fn_stmt);
		}
		let ret_id = self.get_type_id(fn_stmt.get_ret_id());
		let fn_id = ir::FnId::new(fn_stmt.name.lexeme());
		self.ir_ctx.enter_scope();
		let binds = self.build_fn_binds(&fn_stmt.params);
		let fn_native = ir::Fn::new_ln(fn_id, binds, ret_id);
		self.root.add_fn(fn_native);
		self.ir_ctx.add_fn(fn_stmt.name.lexeme());
		self.build_fn_body(&fn_stmt.body, ret_id);
		self.end_fn_scope();
	}

	pub fn build_generics_fns_stmt(&mut self, fn_stmt: &ast::FnStmt) {
		let fn_name = fn_stmt.lexeme();
		let mono_funs = match self.type_store.monomorphic_store.get_fns(fn_name) {
			Some(fns) => fns,
			None => throw_ir_build_error(format!("monomorphic funs not found '{}'", fn_name)),
		};

		for mono_fn_type in mono_funs {
			self.build_generic_fn_stmt(fn_stmt, mono_fn_type);
		}
	}

	pub fn build_generic_fn_stmt(&mut self, fn_stmt: &ast::FnStmt, fn_type: FnType) {
		let fn_name = fn_stmt.lexeme();
		let fn_name_hashed = format!("{fn_name}_{}", self.create_hash_type(&fn_type.generics));
		let fn_id = ir::FnId::new(fn_name_hashed.as_str());
		let ret_id = fn_type.ret;
		self.ir_ctx.set_ret_type(Some(ret_id));
		self.ir_ctx.enter_scope();
		let binds = self.build_mono_fn_binds(&fn_stmt.params, &fn_type.args);
		let fn_native = ir::Fn::new_ln(fn_id, binds, ret_id);
		self.root.add_fn(fn_native);
		self.ir_ctx.add_fn(fn_name_hashed.as_str());
		self.build_fn_body(&fn_stmt.body, ret_id);
		self.end_fn_scope();
	}

	#[inline(always)]
	pub fn build_mono_fn_binds(&mut self, params: &[ast::Binding], args: &[TypeId]) -> Vec<ir::Bind> {
		let mut binds = Vec::with_capacity(params.len());
		for (param, arg) in params.iter().zip(args) {
			let register = self.ir_ctx.new_register();
			self.ir_ctx.add_value(param.lexeme(), register);
			self.ir_ctx.add_type(register, *arg);
			binds.push(ir::Bind { register, type_id: *arg });
		}
		binds
	}

	#[inline(always)]
	pub fn create_hash_type(&mut self, generics: &[TypeId]) -> String {
		let mut hasher = DefaultHasher::new();
		for generic in generics {
			hasher.write_u64(generic.0);
		}
		let hash_value = hasher.finish();
		// todo:  improve this...  maybe move 16 bits to 32 or more? humm...
		format!("{:04x}", (hash_value & 0xFFFF))
	}

	#[inline(always)]
	pub fn build_fn_binds(&mut self, params: &[ast::Binding]) -> Vec<ir::Bind> {
		let mut binds = Vec::with_capacity(params.len());
		for param in params {
			let register = self.ir_ctx.new_register();
			let type_id = self.get_type_id(param.type_id);
			self.ir_ctx.add_value(param.lexeme(), register);
			self.ir_ctx.add_type(register, type_id);
			binds.push(ir::Bind { register, type_id });
		}
		binds
	}

	#[inline(always)]
	fn build_fn_body(&mut self, stmt: &ast::FnBody, ret_id: TypeId) {
		match stmt {
			ast::FnBody::Block(block_stmt) => self.build_fn_block_stmt(block_stmt),
			ast::FnBody::Expr(expr) => self.build_ret_expr(expr, ret_id),
		}
	}

	#[inline(always)]
	pub fn build_ret_expr(&mut self, expr: &ast::Expr, ret_id: TypeId) {
		let value = Some(self.build_expr(expr));
		let instr = ir::RetInstr { value, type_id: ret_id };
		self.ir_ctx.add_instr(instr.into());
	}

	#[inline(always)]
	fn build_fn_block_stmt(&mut self, block_stmt: &ast::BlockStmt) {
		for stmt in block_stmt.stmts.iter() {
			self.build_stmt(stmt);
		}
	}
}
