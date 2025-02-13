use super::Builder;
use crate::{
	ast::{self},
	checker::types::TypeId,
	ir::{self, IrBasicValue},
};

impl Builder<'_> {
	pub fn build_call_expr(&mut self, expr: &mut ast::CallExpr) -> IrBasicValue {
		let ret_type = self.build_type(expr.get_ret_type_id(), expr.get_range());
		let dest = self.ctx.new_register(ret_type);
		let callee = self.resolve_callee(expr);
		let args = self.build_function_args(&mut expr.args, &expr.args_type);
		let call_instr = ir::CallInstr::new(dest.clone(), callee, ret_type, args);
		self.ctx.block.add_instr(call_instr.into());
		dest
	}

	#[inline(always)]
	fn resolve_callee(&mut self, expr: &ast::CallExpr) -> String {
		match expr.callee.as_ref() {
			ast::Expr::Ident(ident) => ident.lexeme().to_string(),
			ast::Expr::Associate(associate) => associate.method.lexeme().to_string(),
			ast::Expr::Member(member) => member.method.lexeme().to_string(),
			_ => todo!("unrecognized callee: {:?}", expr.callee),
		}
	}

	#[rustfmt::skip]
	fn build_function_args(&mut self, args_expr: &mut [ast::Expr], args_type: &[TypeId]) -> Vec<IrBasicValue>  {
		let mut basic_values = Vec::with_capacity(args_type.len());
		for (position, expr) in args_expr.iter_mut().enumerate() {
			let mut basic_value = self.build_expr(expr);
		  if 	let Some(expr_type) = args_type.get(position){
		    basic_value = basic_value.with_new_type(*expr_type);
		  }
		  basic_values.push(basic_value);
		}
		basic_values
	}

	// fn build_member_callee(&mut self, member: &ast::MemberExpr) -> String {
	// 	let left_type = self.build_type(member.left_type, member.get_range());
	// 	self.build_method_name(left_type, member.method.lexeme())
	// }

	// fn build_associate_callee(&mut self, associate: &ast::AssociateExpr) -> String {
	// 	let left_type = self.build_type(associate.left_type, associate.get_range());
	// 	self.build_method_name(left_type, associate.method.lexeme())
	// }

	// fn build_method_name(&mut self, type_id: TypeId, method: &str) -> String {
	// 	let type_name = self.type_store.get_struct_name(type_id);
	// 	self.create_bind_method_name(type_name, method)
	// }

	// #[inline(always)]
	// fn build_ident_callee(&mut self, ident: &ast::Ident, args_type: &[TypeId]) -> String {
	// let lexeme = ident.lexeme();
	// if let Some(monomorphic_calls) = self.type_store.monomorphic_store.get_fns(lexeme) {
	// 	if let Some(generics) = self.find_generic_function(&monomorphic_calls, args_type) {
	// 		return format!("{lexeme}_{}", self.create_hash_type(&generics));
	// 	}
	// }
	// throw_ir_build_error(format!("callee not found: '{lexeme}'"))
	// }

	// #[inline(always)]
	// pub fn create_bind_method_name(&self, self_name: &str, method_name: &str) -> String {
	// 	format!("{}__{}", self_name, method_name)
	// }

	// #[rustfmt::skip]
	// fn find_generic_function(&self, monos: &[FnType], args: &[TypeId]) -> Option<Vec<TypeId>> {
	// 	monos.iter().find(|mono| mono.generics.iter().zip(args).all(|(g, a)| g == a))
	// 		.map(|mono| mono.generics.clone())
	// }

	// #[inline(always)]
	// pub fn create_hash_type(&mut self, generics: &[TypeId]) -> String {
	// 	let mut hasher = DefaultHasher::new();
	// 	for generic in generics {
	// 		hasher.write_u64(generic.0);
	// 	}
	// 	let hash_value = hasher.finish();
	// 	// todo:  improve this...  maybe move 16 bits to 32 or more? humm...
	// 	format!("{:04x}", (hash_value & 0xFFFF))
	// }
}
