mod ast;
mod checker;
mod cli;
mod utils;
// mod compiler;
mod comptime;
mod cross_compiler;
mod diag;
mod ir;
mod lexer;
mod linker;
mod llvm;
mod parser;
mod range;
mod report;
mod source;
use std::{path::Path, str::FromStr};
use target_lexicon::{Triple, HOST};

use checker::{context::Context, Checker};
use clap::ArgMatches;
// use compiler::Compiler;
use comptime::engine;
use cross_compiler::CrossCompiler;
use diag::DiagGroup;
use inkwell::targets::FileType;
use lexer::Token;
use linker::Linker;
use llvm::Llvm;
use logos::Logos;
use parser::Parser;
use report::{throw_cross_compile_error, throw_error, throw_linker_error};

use source::Source;
use utils::{get_current_user_machine, Machine};
fn loader(path_name: &str) -> Source {
	if !path_name.ends_with(".ln") && !path_name.ends_with(".lemon") {
		throw_error(format!("unsupported file extension {}. expected '.lm' or '.ln'", path_name));
	}
	let raw = std::fs::read_to_string(path_name).unwrap_or_else(|err| match err.kind() {
		std::io::ErrorKind::NotFound => throw_error(format!("not found '{}'.", path_name)),
		_ => throw_error(format!("reading file `{}`, reason '{}'.", path_name, err)),
	});
	Source::new(raw, Path::new(path_name).to_owned())
}

fn check(source: Source) {
	let mut lexer = Token::lexer(source.raw());
	let mut parser = Parser::new(&mut lexer);
	let mut ast = match parser.parse_program() {
		Ok(ast) => ast,
		Err(diag) => diag.report_syntax_err_wrap(&source),
	};
	// println!("{:?}", ast);
	let mut diag_group = DiagGroup::new(&source);
	let mut ctx = Context::new();
	let mut checker = Checker::new(&mut diag_group, &mut ctx);
	let _ = match checker.check_program(&mut ast) {
		Ok(tyy) => tyy,
		Err(diag) => diag.report_type_err_wrap(&source),
	};
	println!("ok.");
}

fn generate_output_filename(as_asm: bool, filename: String) -> String {
	let obj_ext = match get_current_user_machine() {
		Machine::Win => ".obj",
		Machine::Linux => ".o",
		Machine::Mac => ".o",
		Machine::Unknown => ".o",
	};
	let to = if as_asm { ".s" } else { obj_ext };
	if filename.ends_with(".lemon") {
		return filename.replace(".lemon", to);
	}
	filename.replace(".ln", to)
}

fn get_output_file_type(as_asm: bool) -> FileType {
	if as_asm {
		FileType::Assembly
	} else {
		FileType::Object
	}
}

fn compile(source: Source, matches: &ArgMatches) {
	let linkder_choice = match matches.get_one::<String>("linker") {
		Some(choice) => match choice.as_str() {
			"mold" => linker::Choice::Mold,
			"lld" => linker::Choice::Llvm,
			"clang" => linker::Choice::Clang,
			choice => throw_linker_error(format!("unknown '{}'", choice)),
		},
		None => linker::Choice::Clang,
	};

	let mut lexer = Token::lexer(source.raw());
	let mut parser = Parser::new(&mut lexer);
	let mut ast = match parser.parse_program() {
		Ok(ast) => ast,
		Err(diag) => diag.report_syntax_err_wrap(&source),
	};
	let mut diag_group = DiagGroup::new(&source);
	let mut ctx = Context::new();
	println!("checking...");
	let mut checker = Checker::new(&mut diag_group, &mut ctx);
	let _ = match checker.check_program(&mut ast) {
		Ok(tyy) => tyy,
		Err(diag) => diag.report_type_err_wrap(&source),
	};
	// println!("ok.");
	println!("emitting lnr...");
	let mut ir_builder = ir::Builder::new(&ctx.type_store);
	let mut ir = ir_builder.build(&ast);
	// let disassembler = ir::Disassembler::new(&ctx.type_store);
	// println!("--- unoptimized ir ---");
	// println!("{}", disassembler.disassemble(&ir));
	println!("optimizing lnr...");
	let mut engine = engine::Engine::new(&mut ir);
	match engine.execute() {
		Ok(_) => {}
		Err(diag) => diag.report_engine_err_wrap(),
	}
	if matches.get_flag("lnr") {
		let disassembler = ir::Disassembler::new(&ctx.type_store);
		println!("{}", disassembler.disassemble(&ir));
		return;
	}
	// println!("--- optimized ir ---");
	// println!("{}", disassembler.disassemble(&ir));
	println!("emitting llvm...");
	let llvm_context = inkwell::context::Context::create();
	let llvm_module = llvm::create_module_from_source(&llvm_context, &source);
	let mut llvm = Llvm::new(&llvm_context, llvm_module, &ctx.type_store);
	llvm.compile_root_ir(&ir);

	if matches.get_flag("llr") {
		println!("{}", llvm.module.print_to_string().to_string());
		return;
	}
	let llvm_module = llvm.module;
	let as_asm = matches.get_flag("assembly");
	let filename = source.file_name();
	let file_type = get_output_file_type(as_asm);
	// cross compiler
	let triple = match matches.get_one::<String>("target") {
		Some(target) => {
			let triple =
				Triple::from_str(target).unwrap_or_else(|err| throw_cross_compile_error(err.to_string()));
			triple.to_string()
		}
		None => HOST.to_string(),
	};
	println!("cross compiling...");
	let cross_compiler = CrossCompiler::new(triple.as_str());
	let output = match matches.get_one::<String>("output") {
		Some(path) => path.to_owned(),
		None => generate_output_filename(as_asm, filename.to_string()),
	};
	let mut linker = Linker::new(linkder_choice);
	let output_path = Path::new(&output);
	linker.add_output(output_path);
	cross_compiler.create_object(&llvm_module, file_type, output_path);
	println!("linking...");
	linker.link();
}

fn lex(source: Source) {
	let mut lexer = Token::lexer(source.raw());
	while let Some(token) = lexer.next() {
		println!("{:?}: {:?}", token, lexer.slice());
	}
}

fn main() {
	let matches = cli::command_line();
	match matches.subcommand() {
		Some(("check", matches)) => {
			let file = matches.get_one::<String>("file").expect("file is required");
			let source = loader(file);
			check(source);
		}

		Some(("compile", matches)) => {
			let file = matches.get_one::<String>("file").unwrap();
			let source = loader(file);
			compile(source, matches);
		}
		Some(("lex", matches)) => {
			let file = matches.get_one::<String>("file").unwrap();
			let source = loader(file);
			lex(source);
		}

		// Some(("run", matches)) => {
		//   let file = matches.get_one::<String>("file").unwrap();
		//   let source = loader(file);
		// }
		// Some(("run-vm", matches)) => {
		//   let file = matches.get_one::<String>("file").unwrap();
		//   let source = loader(file);
		// }
		_ => {
			panic!("unknown command");
		}
	}
}

// fn main() {
//   let input = r#"
//     let x = 10; // Isto é um comentário
//     let y = x + 20; // Outro comentário
//     "#;

//   let mut lexer = Token::lexer(input);
//   while let Some(token) = lexer.next() {
//     println!("{:?}: {:?}", token, lexer.slice());
//   }
// }
