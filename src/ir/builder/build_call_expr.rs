use std::process::id;

use crate::{
	ast,
	checker::types::{FnType, TypeId},
	ir::{
		ir::{self, IrValue},
		Bind, Register,
	},
	report::throw_ir_build_error,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_call_expr(&mut self, expr: &ast::CallExpr) -> Register {
		let dest = self.ir_ctx.new_register();
		let fn_id = self.build_callee(expr);
		let args = self.build_args(&expr.args, &expr.args_type);
		let type_id = self.get_type_id(expr.get_type_id());

		// todo: make it easier
		if let Some(owner) = self.ir_ctx.get_ret_owner() {
			self.ir_ctx.register_struct(*owner, dest);
		}

		let instr = ir::CallInstr { type_id, fn_id, args, dest };
		self.ir_ctx.add_instr(ir::Instr::Call(instr));
		dest
	}

	#[inline(always)]
	fn build_callee(&mut self, expr: &ast::CallExpr) -> String {
		match expr.callee.as_ref() {
			ast::Expr::Ident(ident) => self.build_ident_callee(ident, &expr.args_type),
			ast::Expr::Associate(associate) => self.build_associate_callee(associate),
			ast::Expr::Member(member) => self.build_member_callee(member),
			_ => todo!("callee not found {:?}", expr.callee),
		}
	}

	fn build_member_callee(&mut self, member: &ast::MemberExpr) -> String {
		let self_type = self.get_type_id(member.left_type);
		let self_name = self.type_store.get_struct_name(self_type);
		self.create_bind_method_name(self_name, member.method.lexeme())
	}

	fn build_associate_callee(&mut self, associate: &ast::AssociateExpr) -> String {
		let method = associate.method.lexeme();
		let self_type = self.get_type_id(associate.left_type);
		let self_name = self.type_store.get_struct_name(self_type);
		self.create_bind_method_name(self_name, method)
	}

	#[inline(always)]
	fn build_ident_callee(&mut self, ident: &ast::Ident, args_type: &[TypeId]) -> String {
		let lexeme = ident.lexeme();
		let monomo_callles = self.type_store.monomorphic_store.get_fns(lexeme);
		if monomo_callles.is_none() {
			return lexeme.to_string();
		}
		let generics = self.find_generic_fn(&monomo_callles.unwrap(), args_type);
		if let Some(generics) = generics {
			let lexeme_hashed = format!("{lexeme}_{}", self.create_hash_type(&generics));
			return lexeme_hashed;
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
