use crate::{
	diag::{self, Severity},
	range::Range,
};

#[derive(Debug, Clone)]
pub enum TypeCheckWarn {
	Unused(String, Range),
	UnusedBorrow { range: Range }, // Warn
}

impl From<TypeCheckWarn> for diag::Diag {
	fn from(warn: TypeCheckWarn) -> Self {
		match warn {
			TypeCheckWarn::UnusedBorrow { range } => {
				let text = "value borrowed but never used.".to_string();
				diag::Diag::new(Severity::Warn, text, range)
			}
			TypeCheckWarn::Unused(name, range) => {
				let text = format!("unused value '{}'", name);
				diag::Diag::new(Severity::Warn, text, range)
			}
		}
	}
}
