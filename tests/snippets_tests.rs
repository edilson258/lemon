use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};

use insta::assert_snapshot;
use lemon::{diag::Diag, lexer::Token, loader::Loader, parser::Parser, range::Range};
use logos::Logos;
use stdext::function_name;
use walkdir::WalkDir;

type RunFn = dyn Fn(&str) -> Result<String, Diag>;

const ROOT_TEST_FOLDER: &str = "/tests/snippets/";

fn run_single_snippet(path: &Path, run: &[&RunFn]) -> Result<(), String> {
	println!("{}", path.display());
	let file_name = path
		.to_str()
		.and_then(|p| p.rsplit_once(ROOT_TEST_FOLDER))
		.map(|(_, name)| name)
		.ok_or_else(|| "not found".to_string())?;

	let file_path = format!("{}{}", &ROOT_TEST_FOLDER[1..], file_name);
	let mut results: HashMap<&Path, Vec<String>> = HashMap::new();

	for fun in run {
		let result = fun(file_path.as_str()).unwrap_or_else(|err| err.to_string());
		let file_path = Path::new(file_path.as_str());
		results.entry(file_path).or_default().push(result);
	}

	let results = results
		.into_values()
		.map(|v| v.join("\n"))
		.collect::<Vec<_>>();

	let mut settings = insta::Settings::clone_current();
	settings.set_prepend_module_to_snapshot(false);
	settings.set_omit_expression(true);
	settings.set_input_file(path);

	settings.bind(|| {
		for result in results {
			assert_snapshot!(file_name, result);
		}
	});

	Ok(())
}

fn run_snippets_dir_multiple(test_name: &str, run: &[&RunFn]) {
	let root = PathBuf::from(format!(
		"{}{ROOT_TEST_FOLDER}{}",
		env!("CARGO_MANIFEST_DIR"),
		test_name.rsplit_once(':').unwrap().1
	));
	println!("Root: {}", root.display());

	let walker = WalkDir::new(&root)
		.sort_by_file_name()
		.max_depth(1)
		.into_iter()
		.filter_entry(|e| {
			let path = e.path();
			path == root
				|| path.is_dir()
				|| (path.is_file()
					&& path
						.extension()
						.map_or(false, |x| x == "lemon" || x == "ln"))
		});

	for entry in walker {
		let entry = entry.unwrap();
		let path = entry.path();
		if path.is_file() {
			eprintln!("Testing {}", path.display());
			run_single_snippet(path, run).unwrap();
		}
	}
}

fn run_snippets_dir(test_name: &str, run: &RunFn) {
	println!("Running snippets dir: {}", test_name);
	run_snippets_dir_multiple(test_name, &[run])
}

fn show_pretty_lexer(result: &[(Range, Token, String)]) -> String {
	let mut pretty = String::new();
	for (i, (range, token, token_text)) in result.iter().enumerate() {
		pretty.push_str(&format!(
			"- entry {}:\n    range: {}\n    token: {:?}\n    lexeme: {}\n",
			i + 1,
			range,
			token,
			token_text
		));
	}
	pretty
}
#[test]
fn lexer() {
	run_snippets_dir(function_name!(), &|path| {
		let mut loader = Loader::new();
		let file_id = loader.load(path);
		let source = loader.get_source(file_id);
		let mut lexer = Token::lexer(source.raw());
		let mut result = Vec::new();

		while let Some(token) = lexer.next() {
			match token {
				Ok(token) => {
					let range = Range::from_span(lexer.span());
					let token_text = lexer.slice().to_string();
					result.push((range, token, token_text));
				}
				Err(_) => {
					let range = Range::from_span(lexer.span());
					let token_text = lexer.slice().to_string();
					let pretty = format!(
						"- error: {}\n    range: {:?}\n    lexeme: {}",
						token_text,
						range,
						lexer.slice()
					);
					return Ok(pretty);
				}
			}
		}
		Ok(show_pretty_lexer(&result))
	});
}

#[test]
fn parser() {
	run_snippets_dir(function_name!(), &|path| {
		let mut loader = Loader::new();
		let file_id = loader.load(path);
		let source = loader.get_source(file_id);
		let mut lex = Token::lexer(source.raw());
		let mut parser = Parser::new(&mut lex, file_id);
		let result = parser.parse_program();
		if let Err(err) = result {
			return Ok(format!("{:?}", err));
		}
		if let Ok(program) = result {
			return Ok(format!("{:#?}", program));
		}
		Ok(String::new())
	});
}
