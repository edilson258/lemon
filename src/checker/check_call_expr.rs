use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{CheckResult, Checker, ExpectSome, TypedValue};
use crate::ast::{self};
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_call_expr(&mut self, c: &mut ast::CallExpr) -> CheckResult {
		let callee = self.check_expr(&mut c.callee).some(c.callee.get_range())?;
		let (params, return_type, var_packed) = self.fn_signature(callee.type_id, c.get_range())?;

		if let Err(message) = self.check_call_arguments(&mut c.args, &params, var_packed) {
			return Err(message.range_if_some(Some(c.get_range())));
		}
		// self.call_args_match(&params, &args, c.get_range(), var_packed)?;
		// self.register_multi_type(params, c.get_range());
		self.register_type(return_type, c.get_range());

		let owner = self.ctx.borrow.create_owner();
		let value = TypedValue::new(return_type, owner);
		Ok(Some(value))
		// self.make_ret_value(&args, ret_ty, c.get_range())
	}

	pub fn check_call_arguments(
		&mut self,
		arguments: &mut [ast::Expr],
		expecteds: &[TypeId],
		var_packed: bool,
	) -> MessageResult<()> {
		if !var_packed && arguments.len() != expecteds.len() {
			return Err(SyntaxErr::unexpected_arity(expecteds.len(), arguments.len(), None));
		}

		for (argument, expected) in arguments.iter_mut().zip(expecteds) {
			let argument_range = argument.get_range();
			self.register_type(*expected, argument_range);
			let typed_value = self.check_expr(argument).some(argument_range)?;
			let found_tyi = typed_value.type_id;
			let found_infered = self.infer_type_from_expected(*expected, found_tyi);
			if found_infered != *expected {
				let exp = self.display_type(*expected);
				let got = self.display_type(found_tyi);
				return Err(SyntaxErr::type_mismatch(exp, got, argument_range));
			}
			self.register_type(found_infered, argument_range);
		}
		Ok(())
	}

	// pub fn check_call_arguments(&mut self, args: &mut [ast::Expr]) -> MessageResult<Vec<TypedValue>> {
	// 	let mut values = Vec::with_capacity(args.len());
	// 	for argument in args {
	// 		let arg_range = argument.get_range();
	// 		let tv = self.check_expr(argument).some(arg_range)?;
	// 		self.register_type(tv.type_id, arg_range);
	// 		values.push(tv);
	// 	}
	// 	Ok(values)
	// }
}
