use crate::{
	diag::{self, Diag, Severity},
	range::Range,
};

#[derive(Debug, Clone)]
pub enum EngineError {
	StackOverflow { fn_name: String },
	StackUnderflow,
	RegisterNotFound { reg: usize },
	UninitializedRegister { reg: usize },
	DivisionByZero { location: String },
	InvalidInstruction { instr: String },
	ComptimeError { message: String },
}

impl EngineError {
	pub fn stack_overflow(fn_name: String) -> Diag {
		Self::StackOverflow { fn_name }.into()
	}

	pub fn stack_underflow() -> Diag {
		Self::StackUnderflow.into()
	}

	pub fn register_not_found(reg: usize) -> Diag {
		Self::RegisterNotFound { reg }.into()
	}

	pub fn uninitialized_register(reg: usize) -> Diag {
		Self::UninitializedRegister { reg }.into()
	}

	pub fn division_by_zero(location: String) -> Diag {
		Self::DivisionByZero { location }.into()
	}

	pub fn invalid_instruction(instr: String) -> Diag {
		Self::InvalidInstruction { instr }.into()
	}

	pub fn comptime_error(message: String) -> Diag {
		Self::ComptimeError { message }.into()
	}
}
impl From<EngineError> for diag::Diag {
	fn from(err: EngineError) -> Self {
		match err {
			EngineError::StackOverflow { fn_name } => {
				let text = format!("stack overflow in function '{}'", fn_name);
				diag::Diag::new(Severity::Err, text, Range::default())
			}
			EngineError::StackUnderflow => {
				let text = "stack underflow".to_string();
				diag::Diag::new(Severity::Err, text, Range::default())
			}
			EngineError::RegisterNotFound { reg } => {
				let text = format!("register not found '{}'", reg);
				diag::Diag::new(Severity::Err, text, Range::default())
			}
			EngineError::UninitializedRegister { reg } => {
				let text = format!("register '{}' is not initialized", reg);
				diag::Diag::new(Severity::Err, text, Range::default())
			}
			EngineError::DivisionByZero { location } => {
				let text = format!("division by zero at '{}'", location);
				diag::Diag::new(Severity::Err, text, Range::default())
			}
			EngineError::InvalidInstruction { instr } => {
				let text = format!("invalid instruction '{}'", instr);
				diag::Diag::new(Severity::Err, text, Range::default())
			}
			EngineError::ComptimeError { message } => {
				let text = format!("comptime error: {}", message);
				diag::Diag::new(Severity::Err, text, Range::default())
			}
		}
	}
}
