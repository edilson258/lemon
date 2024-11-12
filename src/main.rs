mod ast;
mod cli;
mod diag;
mod evaluator;
mod lexer;
mod loader;
mod parser;
mod range;
mod report;
mod source;
mod tokens;
use evaluator::{ctx::Ctx, eval::Evaluator};
use lexer::Lexer;
use parser::Parser;
use source::Source;

fn loader(path_name: &str) -> Source {
  let raw = std::fs::read_to_string(path_name).unwrap();
  let filename = path_name.to_string();
  Source::new(raw.as_str(), filename.as_str())
}

fn check(source: Source) {
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer);
  let ast = parser.parse_program();
  println!("{:#?}", ast);
}

pub(crate) fn eval(source: Source) {
  let path = source.path.clone();
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer);
  let ast = parser.parse_program();
  let mut eval = Evaluator::new(path);
  let mut ctx = Ctx::new(None);
  match eval.eval(&ast, &mut ctx) {
    Err(diag) => diag.report(),
    _ => {}
  };
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
    Some(("eval", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      let source = loader(file);
      eval(source);
    }
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
