use console::Style;

use crate::{
	message::{Message, Severity},
	source::Source,
};

pub fn throw_error(text: impl Into<String>) -> ! {
	println!("{} {}", text_red("error:"), text_white(text.into().as_str()));
	std::process::exit(1);
}

pub fn text_red(text: &str) -> String {
	let red = Style::new().color256(9);
	red.apply_to(text).bold().to_string()
}

pub fn text_cyan(text: &str) -> String {
	let cyan = Style::new().color256(43);
	cyan.apply_to(text).bold().to_string()
}

pub fn text_white(text: &str) -> String {
	let white = Style::new().white();
	white.apply_to(text).bold().to_string()
}

pub fn report_message(message: &Message, source: &Source) -> ! {
	let Some(range) = message.range else {
		report_message_without_module(message);
	};
	let pathname = source.pathname.as_str();
	println!("{}", message);
	println!("---> {}", text_white(pathname));

	let start = range.start;
	let end = range.end;
	let code = match message.severity {
		Severity::Error => codelighter::highlight_error(start, end, &source.raw),
		Severity::Warning => codelighter::highlight_warn(start, end, &source.raw),
		Severity::Allow => codelighter::highlight_note(start, end, &source.raw),
	};
	println!("{}", code);

	for note in &message.notes {
		println!("{}: {}", text_cyan("help"), text_white(note.text.as_str()));
	}
	std::process::exit(1);
}

pub fn report_message_without_module(message: &Message) -> ! {
	// let pathname = source.pathname.as_str();
	println!("  {}", message);
	// println!("---> {}", text_white(pathname));
	for note in &message.notes {
		println!("  {}: {}", text_cyan("help"), text_white(note.text.as_str()));
	}
	std::process::exit(1);
}
