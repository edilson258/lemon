use lexer::Lexer;
use parser::Parser;
use utils::source::Source;
pub mod anf;
mod cli;
pub mod core;
pub mod diagnostics;
pub mod formatter;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod utils;
use inkwell::context::Context;

fn main() {
  let matches = cli::command_line();
  match matches.subcommand() {
    Some(("check", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_check(path_name);
    }
    Some(("compile", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_compile(path_name);
    }

    Some(("run", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run(path_name);
    }
    _ => panic!("No subcommand provided."),
  }
}

fn read_file<'s>(path_name: &'s str) -> Source<'s> {
  let raw = std::fs::read_to_string(path_name).expect("Failed to read file");
  let source = Source::new(raw, path_name);
  return source;
}

fn run_check(path_name: &str) {
  println!("checking...");
}

fn run_compile(path_name: &str) {
  let source = read_file(path_name);
  let mut lexer = Lexer::new(source);
  let mut parser = Parser::new(&mut lexer);
  let ast = parser.parse();

  let context = Context::create();

  let mut anf = anf::Anf::new(None);
  let ant_ast = anf.gen_ast(&ast);
  println!("{:#?}", ant_ast);

  let mut wat = core::wat::Wat::new();
  let wasm = wat.gen_ast(&ast);
  println!("{:#?}", wasm);
  // let mut ir = ir::Ir::new(&context,
  //  "lemon_main");
  // ir.gen_from_ast(&ant_ast);
  // ir.print_to_string();
}

fn run(path_name: &str) {
  println!("running...");
}
