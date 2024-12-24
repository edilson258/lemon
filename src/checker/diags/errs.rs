use crate::{
	ast::Operator,
	diag::{self, Diag, Severity},
	range::Range,
};

#[derive(Debug, Clone)]
pub enum TypeCheckError<'tce> {
	// type errors
	TypeMismatch { expected: String, found: String, range: Range },
	NotFn { found: String, range: Range },
	UnsupportedOperator { left: String, right: String, op: &'tce Operator, range: Range },
	ValueExpected { value: String, range: Range },
	UnexpectedValue { value: String, range: Range },
	BoundsError { value: String, found: String, range: Range },
	ArgsMismatch { expected: usize, found: usize, range: Range },

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
	ConnotBorrowAsMutable { range: Range },

	// other errors
	NotFoundValue { name: &'tce str, range: Range },
	InvalidFloat { range: Range },
	NumberTooLarge { range: Range },
	ExpectedNumber { range: Range },
	DerefOfNonRef { range: Range },
}

impl<'tce> TypeCheckError<'tce> {
	pub fn immutable(name: &'tce str, range: Range) -> Diag {
		Self::Immutable { name, range }.into()
	}

	pub fn type_mismatch(expected: String, found: String, range: Range) -> Diag {
		Self::TypeMismatch { expected, found, range }.into()
	}

	pub fn arg_mismatch(expected: usize, found: usize, range: Range) -> Diag {
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

	pub fn deref_of_non_ref(range: Range) -> Diag {
		Self::DerefOfNonRef { range }.into()
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

	pub fn connot_borrow_as_mutable(range: Range) -> Diag {
		Self::ConnotBorrowAsMutable { range }.into()
	}
	pub fn connot_return_local_rerefence(range: Range) -> Diag {
		Self::ConnotReturnLocalRerefence { range }.into()
	}
	pub fn unsupported_operator(
		left: String,
		right: String,
		op: &'tce Operator,
		range: Range,
	) -> Diag {
		Self::UnsupportedOperator { left, right, op, range }.into()
	}
}
impl<'tce> From<TypeCheckError<'tce>> for diag::Diag {
	fn from(err: TypeCheckError<'tce>) -> Self {
		match err {
			TypeCheckError::TypeMismatch { expected, found, range } => {
				let text = format!("expected '{}', found '{}'", expected, found);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::RequiredTypeNotation { range } => {
				let text = "required type notation, cannot infer type".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::Immutable { name, range } => {
				let text = format!("variable '{}' is not mutable", name);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::ArgsMismatch { expected, found, range } => {
				let text = format!("expected {} args, found {}", expected, found);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::NotFoundValue { name, range } => {
				let text = format!("value '{}' not found", name);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::BoundsError { value, found, range } => {
				let text = format!("'{}' out of bounds, expected '{}'", value, found);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::UnsupportedOperator { left, right, op, range } => {
				let text = format!("cannot apply '{}' with '{}' and '{}'", op, left, right);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::ValueExpected { value, range } => {
				let text = format!("expected '{}', found nothing", value);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::UnexpectedValue { value, range } => {
				let text = format!("no expected value, found '{}'", value);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::BorrowConflict { range } => {
				let text = "mutable and immutable borrows conflict.".to_string();
				diag::Diag::new(Severity::Err, text, range)
					.with_note("cannot borrow as mutable and immutable at the same time.")
			}
			TypeCheckError::DoubleMutBorrow { range } => {
				let text = "already mutably borrowed.".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::InvalidBorrow { range } => {
				let text = "invalid borrow, value already freed.".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::BorrowedValueDropped { range } => {
				let text = "borrowed value was dropped before borrow ended.".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::InvalidFloat { range } => {
				let text = "floating-point number out of range".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::NumberTooLarge { range } => {
				let text = "unsupport number".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::ExpectedNumber { range } => {
				let text = "expected number".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::ConnotBorrowAsMutable { range } => {
				let text = "cannot borrow as mutable".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::ReturnNotInFnScope { range } => {
				let text = "return cannot be used in this context".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::DerefOfNonRef { range } => {
				let text = "cannot deref a non-reference value".to_string();
				diag::Diag::new(Severity::Err, text, range)
					.with_note("try to use a reference instead".to_string())
			}
			TypeCheckError::ReturnOutsideFn { range } => {
				let text = "cannot return outside of a fn".to_string();
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::NotFn { found, range } => {
				let text = format!("expected a fn, found '{}'", found);
				diag::Diag::new(Severity::Err, text, range)
			}
			TypeCheckError::ConnotReturnLocalRerefence { range } => {
				let text = "cannot return a reference to a scoped value".to_string();
				diag::Diag::new(Severity::Err, text, range).with_note("try to return a value instead")
			}
		}
	}
}
