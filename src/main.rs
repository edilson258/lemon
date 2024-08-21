#![allow(dead_code, unused_variables)]

use lexer::lex::Lexer;

mod cli;
pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod utils;

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

fn read_file(path_name: &str) -> String {
  let raw = std::fs::read_to_string(path_name).unwrap();
  return raw;
}

fn run_check(path_name: &str) {
  println!("checking...");
}

fn run_compile(path_name: &str) {
  println!("compiling...");
  let raw = read_file(path_name);
  let mut lexer = Lexer::new(&raw, path_name);
  let tokens = lexer.lex_all();
  tokens.iter().for_each(|token| println!("{:#?}", token));
}

fn run(path_name: &str) {
  println!("running...");
}
