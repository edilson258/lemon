mod ast;
mod checker;
mod cli;
mod compiler;
// mod comptime;
mod cross;
mod diag;
mod ir;
mod lexer;
mod linker;
mod llvm;
mod loader;
mod parser;
mod range;
mod report;
mod source;

use checker::{context::Context, Checker};
use compiler::{compile, run};
use diag::DiagGroup;
use lexer::Token;
use loader::Loader;
use logos::Logos;
use parser::Parser;

fn check(path_name: &str) {
	let mut loader = Loader::new();
	let file_id = loader.load(path_name);
	let source = loader.get_source(file_id);
	let mut lexer = Token::lexer(source.raw());
	let mut parser = Parser::new(&mut lexer, file_id);

	let mut ast = match parser.parse_program() {
		Ok(ast) => ast,
		Err(diag) => diag.report_syntax_err_wrap(source),
	};
	let mut diag_group = DiagGroup::new(&loader);
	let mut ctx = Context::new();
	let mut checker = Checker::new(&mut diag_group, &mut ctx);
	let _ = match checker.check_program(&mut ast) {
		Ok(tyy) => tyy,
		Err(diag) => diag.report_type_err_wrap(source),
	};
	println!("ok.");
}

fn lex(path_name: &str) {
	let mut loader = Loader::new();
	let file_id = loader.load(path_name);
	let source = loader.get_source(file_id);
	let mut lexer = Token::lexer(source.raw());
	while let Some(token) = lexer.next() {
		println!("{:?}: {:?}", token, lexer.slice());
	}
}

fn main() {
	let matches = cli::command_line();
	match matches.subcommand() {
		Some(("check", matches)) => {
			let path_name = matches.get_one::<String>("file").expect("file is required");
			check(path_name);
		}

		Some(("compile", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			compile(path_name, matches);
		}
		Some(("lex", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			lex(path_name);
		}
		Some(("run", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			run(path_name);
		}
		_ => {
			panic!("unknown command");
		}
	}
}
