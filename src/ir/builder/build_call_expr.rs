use std::process::id;

use crate::{
	ast,
	checker::types::{FnType, TypeId},
	ir::{
		ir::{self, IrValue},
		Bind, FnId, Register,
	},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	// pub fn call_built_in(&mut self, expr: &ast::CallExpr, dest: Register) -> Option<Register> {
	// 	let fn_id = self.build_callee(expr);
	// 	let args = self.build_args(&expr.args, &expr.args_type);

	// 	let mut type_id = None;
	// 	if let Some(first) = args.first() {
	// 		type_id = self.ir_ctx.get_type(first.register);
	// 	}
	// 	match fn_id.as_string() {
	// 		"is_float" => {
	// 			let value = type_id.map(|type_id| type_id.is_float()).unwrap_or(false).into();
	// 			let instr = ir::StoreInstr { type_id: TypeId::BOOL, value, dest };
	// 			self.ir_ctx.add_instr(ir::Instr::Store(instr));
	// 			Some(dest)
	// 		}
	// 		"is_int" => {
	// 			let value = type_id.map(|type_id| type_id.is_int()).unwrap_or(false).into();
	// 			let instr = ir::StoreInstr { type_id: TypeId::BOOL, value, dest };
	// 			self.ir_ctx.add_instr(ir::Instr::Store(instr));
	// 			Some(dest)
	// 		}
	// 		"is_str" => {
	// 			let value = type_id.map(|type_id| type_id.is_str()).unwrap_or(false).into();
	// 			let instr = ir::StoreInstr { type_id: TypeId::BOOL, value, dest };
	// 			self.ir_ctx.add_instr(ir::Instr::Store(instr));
	// 			Some(dest)
	// 		}
	// 		"is_char" => {
	// 			let value = type_id.map(|type_id| type_id.is_char()).unwrap_or(false).into();
	// 			let instr = ir::StoreInstr { type_id: TypeId::BOOL, value, dest };
	// 			self.ir_ctx.add_instr(ir::Instr::Store(instr));
	// 			Some(dest)
	// 		}
	// 		_ => None,
	// 	}
	// }

	pub fn build_call_expr(&mut self, expr: &ast::CallExpr) -> Register {
		let dest = self.ir_ctx.new_register();
		// if let Some(dest) = self.call_built_in(expr, dest) {
		// 	return dest;
		// }
		let fn_id = self.build_callee(expr);
		let args = self.build_args(&expr.args, &expr.args_type);
		let type_id = self.get_type_id(expr.get_type_id());
		let instr = ir::CallInstr { type_id, fn_id, args, dest };
		self.ir_ctx.add_instr(ir::Instr::Call(instr));
		dest
	}

	#[inline(always)]
	fn build_callee(&mut self, expr: &ast::CallExpr) -> FnId {
		match expr.callee.as_ref() {
			ast::Expr::Ident(ident) => self.build_ident_callee(ident, &expr.args_type),
			_ => todo!(),
		}
	}

	#[inline(always)]
	fn build_ident_callee(&mut self, ident: &ast::Ident, args_type: &[TypeId]) -> FnId {
		let lexeme = ident.lexeme();
		let monomo_callles = self.type_store.monomorphic_store.get_fns(lexeme);
		if monomo_callles.is_none() {
			return ir::FnId::new(lexeme);
		}
		let generics = self.find_generic_fn(&monomo_callles.unwrap(), args_type);
		if let Some(generics) = generics {
			let lexeme_hashed = format!("{lexeme}_{}", self.create_hash_type(&generics));
			return ir::FnId::new(&lexeme_hashed);
		}
		throw_ir_build_error(format!("callee not found '{lexeme}'"));
	}

	fn find_generic_fn(&self, monos: &[FnType], args: &[TypeId]) -> Option<Vec<TypeId>> {
		for mono in monos {
			if mono.generics.iter().zip(args).all(|(generic, arg)| generic == arg) {
				return Some(mono.generics.clone());
			}
		}
		None
	}

	#[inline(always)]
	fn build_args(&mut self, args: &[ast::Expr], args_type: &[TypeId]) -> Vec<Bind> {
		let mut binds = Vec::with_capacity(args.len());
		for (index, arg) in args.iter().enumerate() {
			let reg = self.build_expr(arg);
			let arg_type = match args_type.get(index) {
				Some(type_id) => type_id,
				None => self.ir_ctx.get_type(reg).expect("type not found"),
			};

			binds.push(Bind::new(reg, *arg_type));
		}
		binds
	}
}
