#![allow(unused_variables, dead_code)]

use std::fmt::Display;

use crate::{
	loader::{Loader, ModId},
	range::Range,
	report::{report_message, report_message_without_module, text_red},
};

#[macro_export]
macro_rules! error_syntax {
	($($arg:tt)*) => {
		$crate::message::Message::error_syntax(format!( $($arg)* ))
	}
}
#[macro_export]
macro_rules! error_resolve {
	($($arg:tt)*) => {
		$crate::message::Message::error_resolve(format!( $($arg)* ))
	}
}
#[macro_export]
macro_rules! error_type {
	($($arg:tt)*) => {
		$crate::message::Message::error_type(format!( $($arg)* ))
	}
}

#[macro_export]
macro_rules! error_ownership {
	($($arg:tt)*) => {
		$crate::message::Message::error_ownership(format!( $($arg)* ))
	}
}

#[macro_export]
macro_rules! error_build {
	($($arg:tt)*) => {
		$crate::message::Message::error_build(format!( $($arg)* ))
	}
}
// #[macro_export]
// macro_rules! error_build {
// 	($($arg:tt)*) => {
// 		$crate::message::Message::error_build(format!( $($arg)* ))
// 	}
// }

#[macro_export]
macro_rules! error_codegen {
	($($arg:tt)*) => {
		$crate::message::Message::error_codegen(format!( $($arg)* ))
	}
}

#[macro_export]
macro_rules! warning {
	($($arg:tt)*) => {
		$crate::message::Message::warning(format!( $($arg)* ))
	}
}

#[macro_export]
macro_rules! throw_error {
	($($arg:tt)*) => {
		$crate::report::throw_error(format!( $($arg)* ))
	}
}

#[macro_export]
macro_rules! note {
	($($arg:tt)*) => {
	$crate::message::Note::new(format!( $($arg)* ))
	}
}

pub type MessageResult<T> = std::result::Result<T, Message>;

#[derive(Debug)]
pub struct Messages {
	messages: Vec<Message>,
	any_errors: bool,
}

impl Messages {
	pub fn new() -> Self {
		Messages { messages: Vec::new(), any_errors: false }
	}

	pub fn any_messages(&self) -> bool {
		!self.messages.is_empty()
	}

	pub fn message(&mut self, message: Message) {
		self.any_errors |= message.severity == Severity::Error;
		self.messages.push(message);
	}
}

impl Default for Messages {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug)]
pub struct RootMessages {
	messages: Vec<Messages>,
}

impl RootMessages {
	pub fn new() -> Self {
		RootMessages { messages: Vec::new() }
	}
}

impl Default for RootMessages {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Stage {
	Syntax,    // parser + lexer
	Resolve,   // loader
	Type,      // type checker
	Ownership, // type checker (ownership tracking)
	Build,     // build ir
	Codegen,   // codegen (llvm, wasm, etc)
	Comptime,  // comptime
}

impl Stage {
	pub fn new() -> Self {
		Self::Syntax
	}
	fn name(self) -> &'static str {
		match self {
			Stage::Syntax => "syntax",
			Stage::Resolve => "resolve",
			Stage::Type => "type",
			Stage::Ownership => "ownership",
			Stage::Build => "build",
			Stage::Codegen => "codegen",
			Stage::Comptime => "comptime",
		}
	}
}

impl Default for Stage {
	fn default() -> Self {
		Self::Type
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Severity {
	Allow,
	Warning,
	Error,
}

impl Severity {
	pub fn new() -> Self {
		Self::Error
	}
	fn name(self) -> &'static str {
		match self {
			Severity::Error => "error",
			Severity::Warning => "warning",
			Severity::Allow => "allow",
		}
	}
}

impl Default for Severity {
	fn default() -> Self {
		Self::Error
	}
}

#[derive(Debug)]
pub struct Message {
	pub severity: Severity,
	pub text: String,
	pub stage: Option<Stage>,
	pub range: Option<Range>,
	pub mod_id: Option<ModId>,
	pub notes: Vec<Note>,
}

impl Message {
	pub fn error(text: impl Into<String>) -> Message {
		Message {
			severity: Severity::Error,
			text: text.into(),
			stage: None,
			range: None,
			notes: Vec::new(),
			mod_id: None,
		}
	}

	pub fn error_internal(text: impl Into<String>, stage: Stage, range: Range) -> Message {
		let message = Message::error(text);
		message.stage(stage).range(range).note_internal()
	}

	pub fn warning(text: impl Into<String>) -> Message {
		Message {
			severity: Severity::Warning,
			text: text.into(),
			stage: None,
			range: None,
			notes: Vec::new(),
			mod_id: None,
		}
	}

	pub fn error_syntax(text: impl Into<String>) -> Message {
		let message = Message::error(text);
		message.stage(Stage::Syntax)
	}

	pub fn error_resolve(text: impl Into<String>) -> Message {
		let message = Message::error(text);
		message.stage(Stage::Resolve)
	}

	pub fn error_type(text: impl Into<String>) -> Message {
		let message = Message::error(text);
		message.stage(Stage::Type)
	}

	pub fn error_ownership(text: impl Into<String>) -> Message {
		let message = Message::error(text);
		// message.stage(Stage::Ownership)
		message.stage(Stage::Type)
	}

	pub fn error_build(text: impl Into<String>) -> Message {
		let message = Message::error(text);
		message.stage(Stage::Build)
	}

	pub fn error_codegen(text: String) -> Message {
		let message = Message::error(text);
		message.stage(Stage::Codegen)
	}

	pub fn error_comptime(text: impl Into<String>) -> Message {
		let message = Message::error(text);
		message.stage(Stage::Comptime)
	}

	// warings
	pub fn warning_syntax(text: impl Into<String>) -> Message {
		let message = Message::warning(text);
		message.stage(Stage::Syntax)
	}
	pub fn warning_resolve(text: impl Into<String>) -> Message {
		let message = Message::warning(text);
		message.stage(Stage::Resolve)
	}
	pub fn warning_type(text: impl Into<String>) -> Message {
		let message = Message::warning(text);
		message.stage(Stage::Type)
	}
	pub fn warning_ownership(text: impl Into<String>) -> Message {
		let message = Message::warning(text);
		message.stage(Stage::Ownership)
	}
	pub fn warning_build(text: impl Into<String>) -> Message {
		let message = Message::warning(text);
		message.stage(Stage::Build)
	}
	pub fn warning_codegen(text: impl Into<String>) -> Message {
		let message = Message::warning(text);
		message.stage(Stage::Codegen)
	}
	pub fn warning_comptime(text: impl Into<String>) -> Message {
		let message = Message::warning(text);
		message.stage(Stage::Comptime)
	}

	// ============ Helpers ============

	pub fn range(mut self, range: Range) -> Message {
		self.range = Some(range);
		self
	}
	pub fn mod_id(mut self, mod_id: ModId) -> Message {
		self.mod_id = Some(mod_id);
		self
	}

	pub fn range_if_some(mut self, range: Option<Range>) -> Message {
		self.range = self.range.or(range);
		self
	}

	pub fn stage(mut self, stage: Stage) -> Message {
		self.stage = Some(stage);
		self
	}

	pub fn note(mut self, note: Note) -> Message {
		self.notes.push(note);
		self
	}

	pub fn note_if_some(mut self, range: Option<Range>, text: impl Into<String>) -> Message {
		if let Some(note) = Note::maybe_new(text, range) {
			self.notes.push(note);
		}
		self
	}

	pub fn note_internal(mut self) -> Message {
		self.notes.push(Note::new("internal compiler error — please report"));
		self
	}
	fn _lookup_mod_id(&self) -> ModId {
		match self.mod_id {
			Some(mod_id) => mod_id,
			None => report_message_without_module(self),
		}
	}
	// ============ Reports ============
	pub fn report(&self, loader: &Loader) -> ! {
		let mod_id = self._lookup_mod_id();
		let source = loader.lookup_source_unchecked(mod_id);
		report_message(self, source)
	}
}

#[derive(Debug)]
pub struct Note {
	pub text: String,
	pub range: Option<Range>,
}

impl Note {
	pub fn new(text: impl Into<String>) -> Note {
		Note { text: text.into(), range: None }
	}

	pub fn range(mut self, range: Range) -> Note {
		self.range = Some(range);
		self
	}

	pub fn maybe_new(text: impl Into<String>, range: Option<Range>) -> Option<Note> {
		Some(Note { text: text.into(), range })
	}
}

impl Display for Message {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let severity = self.severity;
		let text = self.text.as_str();
		let stage = self.stage.unwrap_or_default();
		let slug = text_red(format!("{} {}", stage, severity).as_str());
		write!(f, "{}: {}", slug, text)
	}
}

impl Display for Note {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let text = self.text.as_str();
		let range = self.range.map(|range| range.to_string()).unwrap_or_default();
		write!(f, "== {} {}", text, range)
	}
}

impl Display for Stage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let stage = match self {
			Stage::Syntax => "syntax",
			Stage::Resolve => "resolve",
			Stage::Type => "type",
			Stage::Ownership => "ownership",
			Stage::Build => "build",
			Stage::Codegen => "codegen",
			Stage::Comptime => "comptime",
		};
		write!(f, "{}", stage)
	}
}

impl Display for Severity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let severity = match self {
			Severity::Error => "error",
			Severity::Warning => "warning",
			Severity::Allow => "allow",
		};
		write!(f, "{}", severity)
	}
}
