use crate::{
	ast::Operator,
	diag::{self, Diag, Severity},
	range::Range,
};

#[derive(Debug, Clone)]
pub enum SyntaxErr<'tce> {
	// type errors
	TypeMismatch { expected: String, found: String, range: Range },
	NotFn { found: String, range: Range },
	UnsupportedOperator { left: String, right: String, operator: &'tce Operator },
	ValueExpected { value: String, range: Range },
	UnexpectedValue { value: String, range: Range },
	BoundsError { value: String, found: String, range: Range },
	ArgsMismatch { expected: usize, found: usize, range: Range },

	// fn errors
	RedefineFnInSameScope { name: String, range: Range },
	ReturnLocalBorrow { range: Range },

	// context errors
	ReturnOutsideFn { range: Range },
	ReturnNotInFnScope { range: Range },
	RequiredTypeNotation { range: Range },
	ConnotReturnLocalRerefence { range: Range },

	// borrow errors
	Immutable { name: &'tce str, range: Range },
	BorrowConflict { range: Range },
	DoubleMutBorrow { range: Range },
	InvalidBorrow { range: Range },
	BorrowedValueDropped { range: Range },
	CannotDereference { type_name: String, range: Range },
	CannotBorrowAsMutable { name: String, range: Range },
	CannotBorrowAsMutableMoreThanOnce { name: String, range: Range },
	BorrowExpected { range: Range },
	CannotAssignImmutable { name: String, range: Range },

	// other errors
	NotFoundValue { name: &'tce str, range: Range },
	NotFoundModule { name: &'tce str, range: Range },
	InvalidFloat { range: Range },
	NumberTooLarge { range: Range },
	ExpectedNumber { range: Range },
	NotAllPathsReturn { range: Range },
	// const errors
	ConstOutsideGlobalScope { range: Range },
	ConstRedefinition { range: Range },
	ConstRequiredTypeNotation { range: Range },
}

impl<'tce> SyntaxErr<'tce> {
	pub fn const_outside_global_scope(range: Range) -> Diag {
		Self::ConstOutsideGlobalScope { range }.into()
	}

	pub fn const_redefinition(range: Range) -> Diag {
		Self::ConstRedefinition { range }.into()
	}

	pub fn const_required_type_notation(range: Range) -> Diag {
		Self::ConstRequiredTypeNotation { range }.into()
	}

	pub fn immutable(name: &'tce str, range: Range) -> Diag {
		Self::Immutable { name, range }.into()
	}

	pub fn type_mismatch(expected: String, found: String, range: Range) -> Diag {
		Self::TypeMismatch { expected, found, range }.into()
	}

	pub fn args_mismatch(expected: usize, found: usize, range: Range) -> Diag {
		Self::ArgsMismatch { expected, found, range }.into()
	}

	pub fn not_found_value(name: &'tce str, range: Range) -> Diag {
		Self::NotFoundValue { name, range }.into()
	}

	pub fn bounds_error(value: String, found: String, range: Range) -> Diag {
		Self::BoundsError { value, found, range }.into()
	}

	pub fn required_type_notation(range: Range) -> Diag {
		Self::RequiredTypeNotation { range }.into()
	}

	pub fn value_expected(value: String, range: Range) -> Diag {
		Self::ValueExpected { value, range }.into()
	}

	pub fn unexpected_value(value: String, range: Range) -> Diag {
		Self::UnexpectedValue { value, range }.into()
	}

	pub fn return_outside_fn(range: Range) -> Diag {
		Self::ReturnOutsideFn { range }.into()
	}

	pub fn not_all_paths_return(range: Range) -> Diag {
		Self::NotAllPathsReturn { range }.into()
	}

	pub fn invalid_float(range: Range) -> Diag {
		Self::InvalidFloat { range }.into()
	}

	pub fn number_too_large(range: Range) -> Diag {
		Self::NumberTooLarge { range }.into()
	}

	pub fn expected_number(range: Range) -> Diag {
		Self::ExpectedNumber { range }.into()
	}

	pub fn return_not_in_fn_scope(range: Range) -> Diag {
		Self::ReturnNotInFnScope { range }.into()
	}
	pub fn not_found_module(name: &'tce str, range: Range) -> Diag {
		Self::NotFoundModule { name, range }.into()
	}

	pub fn cannot_dereference(type_name: String, range: Range) -> Diag {
		Self::CannotDereference { type_name, range }.into()
	}
	pub fn not_a_fn(found: String, range: Range) -> Diag {
		Self::NotFn { found, range }.into()
	}

	pub fn borrow_conflict(range: Range) -> Diag {
		Self::BorrowConflict { range }.into()
	}

	pub fn double_mut_borrow(range: Range) -> Diag {
		Self::DoubleMutBorrow { range }.into()
	}

	pub fn invalid_borrow(range: Range) -> Diag {
		Self::InvalidBorrow { range }.into()
	}

	pub fn borrowed_value_dropped(range: Range) -> Diag {
		Self::BorrowedValueDropped { range }.into()
	}

	pub fn cannot_borrow_as_mutable(name: &str, range: Range) -> Diag {
		Self::CannotBorrowAsMutable { name: name.to_string(), range }.into()
	}

	pub fn cannot_borrow_as_mutable_more_than_once(name: &str, range: Range) -> Diag {
		Self::CannotBorrowAsMutableMoreThanOnce { name: name.to_string(), range }.into()
	}
	pub fn connot_return_local_rerefence(range: Range) -> Diag {
		Self::ConnotReturnLocalRerefence { range }.into()
	}
	pub fn unsupported_operator(left: String, right: String, operator: &'tce Operator) -> Diag {
		Self::UnsupportedOperator { left, right, operator }.into()
	}

	pub fn redefine_fn_in_same_scope(name: &str, range: Range) -> Diag {
		Self::RedefineFnInSameScope { name: name.to_string(), range }.into()
	}

	pub fn return_local_borrow(range: Range) -> Diag {
		Self::ReturnLocalBorrow { range }.into()
	}

	pub fn borrow_expected(range: Range) -> Diag {
		Self::BorrowExpected { range }.into()
	}

	pub fn cannot_assign_immutable(name: &str, range: Range) -> Diag {
		Self::CannotAssignImmutable { name: name.to_string(), range }.into()
	}
}
impl<'tce> From<SyntaxErr<'tce>> for diag::Diag {
	fn from(err: SyntaxErr<'tce>) -> Self {
		match err {
			SyntaxErr::TypeMismatch { expected, found, range } => {
				let text = format!("expected '{}', found '{}'", expected, found);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::RequiredTypeNotation { range } => {
				let text = "required type notation, cannot infer type".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::Immutable { name, range } => {
				let text = format!("variable '{}' is not mutable", name);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::ArgsMismatch { expected, found, range } => {
				let text = format!("expected {} args, found {}", expected, found);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::NotFoundValue { name, range } => {
				let text = format!("value '{}' not found", name);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::BoundsError { value, found, range } => {
				let text = format!("'{}' out of bounds, expected '{}'", value, found);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::UnsupportedOperator { left, right, operator } => {
				let text = format!("cannot {} '{}' to '{}'", operator.display(), left, right);
				diag::Diag::new(Severity::Err, text, operator.get_range())
			}
			SyntaxErr::ValueExpected { value, range } => {
				let text = format!("expected '{}', found UNIT", value);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::UnexpectedValue { value, range } => {
				let text = format!("no expected value, found '{}'", value);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::BorrowConflict { range } => {
				let text = "mutable and immutable borrows conflict.".to_string();
				diag::Diag::new(Severity::Err, text, range)
					.with_note("cannot borrow as mutable and immutable at the same time.")
			}
			SyntaxErr::DoubleMutBorrow { range } => {
				let text = "already mutably borrowed.".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::InvalidBorrow { range } => {
				let text = "invalid borrow, value already freed.".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::BorrowedValueDropped { range } => {
				let text = "borrowed value was dropped before borrow ended.".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::InvalidFloat { range } => {
				let text = "floating-point number out of range".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::NumberTooLarge { range } => {
				let text = "unsupport number".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::ExpectedNumber { range } => {
				let text = "expected number".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::ReturnNotInFnScope { range } => {
				let text = "return cannot be used in this context".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::CannotDereference { type_name, range } => {
				let text = format!("'{}' cannot be dereferenced, expected a reference", type_name);
				diag::Diag::new(Severity::Err, text, range)
					.with_note("try to use a reference instead".to_string())
			}
			SyntaxErr::ReturnOutsideFn { range } => {
				let text = "cannot return outside of a fn".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::NotFn { found, range } => {
				let text = format!("expected a fn, found '{}'", found);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::ConnotReturnLocalRerefence { range } => {
				let text = "cannot return a reference to a scoped value".to_string();
				diag::Diag::new(Severity::Err, text, range).with_note("try to return a value instead")
			}

			// const errors
			SyntaxErr::ConstOutsideGlobalScope { range } => {
				let text = "const can only be defined at global scope".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::ConstRedefinition { range } => {
				let text = "const already defined".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::ConstRequiredTypeNotation { range } => {
				let text = "required type notation, cannot infer type".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::NotAllPathsReturn { range } => {
				let text = "expected return value in all cases".to_string();
				diag::Diag::new(Severity::Err, text, range)
					.with_note("ensure every path returns a value".to_string())
			}
			SyntaxErr::NotFoundModule { name, range } => {
				let text = format!("module '{}' not found", name);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::RedefineFnInSameScope { name, range } => {
				let text = format!("function '{}' is already defined in this scope", name);
				diag::Diag::new(Severity::Err, text, range)
					.with_note("consider renaming the function".to_string())
			}
			SyntaxErr::ReturnLocalBorrow { range } => {
				let text = "cannot return a local borrow".to_string();
				diag::Diag::new(Severity::Err, text, range).with_note("try to return a value instead")
			}

			SyntaxErr::BorrowExpected { range } => {
				let text = "consider adding a borrow".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}

			SyntaxErr::CannotBorrowAsMutableMoreThanOnce { name, range } => {
				let text = format!("cannot borrow '{}' as mutable more than once at a time", name);
				diag::Diag::new(Severity::Err, text, range)
			}
			SyntaxErr::CannotBorrowAsMutable { name, range } => {
				let text = format!("cannot borrow '{}' as mutable", name);
				diag::Diag::new(Severity::Err, text, range)
					.with_note(format!("consider change '{}' to mutable", name))
			}
			SyntaxErr::CannotAssignImmutable { name, range } => {
				let text = format!("cannot assign immutable '{}'", name);
				diag::Diag::new(Severity::Err, text, range)
					.with_note("consider making it mutable".to_string())
			}
		}
	}
}
