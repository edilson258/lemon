use super::Builder;
use crate::{
	ast, error_build,
	ir::{self, IrBasicValue},
};

type CalleeResolvedType = (String, Option<IrBasicValue>);

impl Builder<'_> {
	pub fn build_call_expr(&mut self, expr: &mut ast::CallExpr) -> IrBasicValue {
		let range = expr.get_range();
		let ret_type = self.lookup_event_type(range);
		let dest = self.create_basic_value(ret_type);
		let (callee, self_value) = self.resolve_callee(expr);
		let mut args = self.build_function_args(&mut expr.args);
		if let Some(self_value) = self_value {
			args.insert(0, self_value);
		}
		if !ret_type.is_empty_type() {
			if let Some(size) = self.is_need_heap_allocation(ret_type) {
				self.ctx.register_unbound_value(dest.clone());
				let unary_instr = ir::UnInstr::new(dest.clone(), size.into());
				self.append_instr(ir::Instr::Halloc(unary_instr), Some(range));
			} else {
				let salloc = ir::SallocInstr::new(dest.clone(), ret_type);
				self.append_instr(salloc.into(), Some(range));
			}
		}

		let call_instr = ir::CallInstr::new(dest.clone(), callee, ret_type, args);
		self.append_instr(call_instr.into(), Some(range));
		dest
	}

	#[inline(always)]
	fn resolve_callee(&mut self, expr: &mut ast::CallExpr) -> CalleeResolvedType {
		match &mut *expr.callee {
			ast::Expr::Ident(ident) => (ident.lexeme().to_string(), None),
			ast::Expr::Associate(associate_expr) => self.resolve_associate_expr(associate_expr),
			ast::Expr::Member(member) => self.resolve_member_expr(member),
			_ => todo!("unrecognized callee: {:?}", expr.callee),
		}
	}

	#[inline(always)]
	fn resolve_associate_expr(&mut self, expr: &mut ast::AssociateExpr) -> CalleeResolvedType {
		let self_name = expr.self_name.lexeme();
		let method_name = expr.method.lexeme();
		(self.create_bind_method_with_selfname(self_name, method_name), None)
	}

	#[inline(always)]
	fn resolve_member_expr(&mut self, member: &mut ast::MemberExpr) -> CalleeResolvedType {
		self.ctx.push_struct_member_scope();
		let self_value = self.build_expr(&mut member.left);
		// let self_register = self_value.value.as_str();
		self.ctx.pop_scope();
		let self_name = match self.type_store.lookup_struct_name(self_value.get_type()) {
			Some(name) => name,
			None => error_build!("callee '{}' not found", member.method.lexeme()).report(self.loader),
		};
		let method_name = member.method.lexeme();
		let method = self.create_bind_method_with_selfname(self_name, method_name);
		(method, Some(self_value))
	}

	fn build_function_args(&mut self, args_expr: &mut [ast::Expr]) -> Vec<IrBasicValue> {
		let mut basic_values = Vec::with_capacity(args_expr.len());
		for expr in args_expr.iter_mut() {
			let basic_value = self.build_expr(expr);
			let value = self.ensure_loaded(basic_value, expr.get_range());
			basic_values.push(value);
		}
		basic_values
	}
}
