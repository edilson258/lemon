mod ast;
mod cli;
mod lexer;
mod parser;
mod tokens;
mod utils;
use lexer::Lexer;
use tokens::TokenType;
use utils::source::Source;

fn loader(path_name: &str) -> Source {
  let raw = std::fs::read_to_string(path_name).unwrap();
  let filename = path_name.to_string();
  Source::new(raw.as_str(), filename.as_str())
}

fn check(source: Source) {
  let mut lexer = Lexer::new(source);
  let mut token = lexer.next_token();
  while !lexer.is_end() {
    println!("{:#?}", token);
    token = lexer.next_token();
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
    // Some(("compile", matches)) => {
    //   let file = matches.get_one::<String>("file").unwrap();
    //   let source = loader(file);
    // }
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
//   let mut lexer = Lexer::new(source);
//   while !lexer.is_end() {
//     let token = lexer.next_token();
//     println!("{:?}", token);
//     if token.kind == TokenType::EOF {
//       break;
//     }
//   }
// }
