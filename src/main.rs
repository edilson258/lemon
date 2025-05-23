mod ast;
mod builder;
mod checker;
mod cli;
mod compiler;
// mod comptime;
mod cross;
mod disassembler;
mod file_system;
mod ir;
mod lexer;
mod linker;
mod llvm;
mod loader;
mod message;
mod time;
// mod optimize;
mod parser;
mod range;
mod report;
mod shio;
mod source;

use std::{path::Path, time::Instant};

use checker::{context::Context, Checker};
use compiler::compile;
use file_system::FileSystem;
use lexer::Token;
use loader::{Loader, ModId};
use logos::Logos;
use parser::Parser;
use report::throw_error;
use shio::ShioConfig;
use time::format_time;

pub fn parse_mod(mod_id: ModId, loader: &mut Loader) {
	let source = loader.lookup_source_unchecked(mod_id).clone();
	let mut lexer = Token::lexer(source.raw.as_str());
	let mut parser = Parser::new(&mut lexer, mod_id, loader);
	let ast = parser.parse_program().unwrap_or_else(|message| message.report(loader));
	loader.add_mod(mod_id, ast);
}
fn check(matches: &clap::ArgMatches) {
	let timer = Instant::now();
	let shio = match matches.get_one::<String>("file") {
		Some(path_name) => {
			let path = Path::new(path_name);
			ShioConfig::with_defaults(path.to_path_buf())
		}
		None => ShioConfig::load_from_toml(None).unwrap_or_else(|err| throw_error(err)),
	};
	let cwd = shio.loader.cwd.clone();
	let file_system = FileSystem::from_current_dir(cwd);
	let mut loader = Loader::new(shio, file_system);
	let mut ctx = Context::new();
	let mod_id = loader.load_entry().unwrap_or_else(|message| message.report(&loader));
	parse_mod(mod_id, &mut loader);
	let mut checker = Checker::new(&mut ctx, &mut loader);
	checker.check(mod_id);
	println!("ok in {}.", format_time(timer.elapsed(), true));
}

fn lex(path_name: &str) {
	let path = Path::new(path_name);
	let shio = ShioConfig::with_defaults(path.to_path_buf());
	let cwd = shio.loader.cwd.clone();
	let file_system = FileSystem::from_current_dir(cwd);
	let mut loader = Loader::new(shio, file_system);
	let mod_id = loader.load_entry().unwrap_or_else(|message| message.report(&loader));
	let source = loader.lookup_source_unchecked(mod_id).clone();
	let mut lexer = Token::lexer(&source.raw);
	while let Some(token) = lexer.next() {
		println!("{:?}: {:?}", token, lexer.slice());
	}
}

fn token(path_name: &str) {
	let path = Path::new(path_name);
	let shio = ShioConfig::with_defaults(path.to_path_buf());
	let cwd = shio.loader.cwd.clone();
	let file_system = FileSystem::from_current_dir(cwd);
	let mut loader = Loader::new(shio, file_system);
	let mod_id = loader.load_entry().unwrap_or_else(|message| message.report(&loader));
	let source = loader.lookup_source_unchecked(mod_id).clone();
	let mut lexer = Token::lexer(&source.raw);
	while let Some(token) = lexer.next() {
		println!("{:?}: {:?}", token, lexer.slice());
	}
}
fn ast(path_name: &str) {
	let path = Path::new(path_name);
	let shio = ShioConfig::with_defaults(path.to_path_buf());
	let cwd = shio.loader.cwd.clone();
	let file_system = FileSystem::from_current_dir(cwd);
	let mut loader = Loader::new(shio, file_system);
	let mod_id = loader.load_entry().unwrap_or_else(|message| message.report(&loader));
	let source = loader.lookup_source_unchecked(mod_id).clone();
	let mut lexer = Token::lexer(&source.raw);
	let mut parser = Parser::new(&mut lexer, mod_id, &mut loader);
	let ast = match parser.parse_program() {
		Ok(ast) => ast,
		Err(message) => message.report(&loader),
	};
	println!("{:#?}", ast);
}

fn main() {
	let matches = cli::command_line();
	match matches.subcommand() {
		Some(("check", matches)) => {
			// let path_name = matches.get_one::<String>("file").expect("file is required");
			check(matches);
		}

		Some(("compile", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			compile(path_name, matches);
		}
		Some(("lex", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			lex(path_name);
		}
		Some(("run", _matches)) => {
			// let path_name = matches.get_one::<String>("file").unwrap();
			todo!("run command is not implemented yet");
			// run(path_name);
		}

		Some(("token", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			token(path_name);
		}
		Some(("ast", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			ast(path_name);
		}
		_ => {
			panic!("unknown command");
		}
	}
}
