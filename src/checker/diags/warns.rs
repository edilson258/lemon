use crate::range::Range;

#[derive(Debug, Clone)]
pub enum TypeCheckWarn {
	Unused(String, Range),
	UnusedBorrow { range: Range }, // Warn
	Ureachable { range: Range },   // Warn
}

// impl TypeCheckWarn {
// 	pub fn unreachable(range: Range) -> Diag {
// 		Self::Ureachable { range }.into()
// 	}
// }

// impl From<TypeCheckWarn> for diag::Diag {
// 	fn from(warn: TypeCheckWarn) -> Self {
// 		match warn {
// 			TypeCheckWarn::UnusedBorrow { range } => {
// 				let text = "value borrowed but never used.".to_string();
// 				diag::Diag::new(Severity::Warn, text, range)
// 			}
// 			TypeCheckWarn::Unused(name, range) => {
// 				let text = format!("unused value '{}'", name);
// 				diag::Diag::new(Severity::Warn, text, range)
// 			}
// 			TypeCheckWarn::Ureachable { range } => {
// 				let text = "unreachable code".to_string();
// 				diag::Diag::new(Severity::Warn, text, range).with_note("consider removing it".to_string())
// 			}
// 		}
// 	}
// }
