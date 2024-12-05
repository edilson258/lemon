mod ast;
mod cli;
mod diag;
// mod evaluator;
mod lexer;
// mod loader;
mod checker;
mod ir;
mod parser;
mod range;
mod report;
mod runtime;
mod source;
use std::path::Path;

use checker::{context::Context, Checker};
use diag::DiagGroup;
use lexer::Token;
use logos::Logos;
use parser::Parser;
use report::throw_error;
use source::Source;
fn loader(path_name: &str) -> Source {
  let raw = std::fs::read_to_string(path_name).unwrap_or_else(|err| match err.kind() {
    std::io::ErrorKind::NotFound => throw_error(format!("not found '{}'.", path_name)),
    _ => throw_error(format!("reading file `{}`, reason '{}'.", path_name, err.to_string())),
  });
  Source::new(raw, Path::new(path_name).to_owned())
}

fn check(source: Source) {
  let mut lexer = Token::lexer(&source.raw());
  let mut parser = Parser::new(&mut lexer);
  let ast = match parser.parse_program() {
    Ok(ast) => ast,
    Err(diag) => diag.report_wrap(source.path()),
  };
  // println!("{:?}", ast);
  let mut diag_group = DiagGroup::new(&source);
  let ctx = Context::new();
  let mut checker = Checker::new(&mut diag_group, ctx);
  let tyy = match checker.check_program(&ast) {
    Ok(tyy) => tyy,
    Err(diag) => diag.report_wrap(source.path()),
  };
  if let Some(tyy) = tyy {
    println!("ok: {}", tyy);
  } else {
    println!("ok.");
  }
}

fn compile(_source: Source) {
  let ir = ir::IR {
    fns: vec![
      ir::FnIr {
        name: "compute".to_string(),
        params: vec![
          ir::Bind { reg: "a".to_string(), ty: "u32".to_string() },
          ir::Bind { reg: "b".to_string(), ty: "u32".to_string() },
        ],
        ret_ty: Some("u32".to_string()),
        body: vec![
          ir::Instr::ADD { lhs: "a".to_string(), rhs: "b".to_string(), dest: "sum".to_string() },
          ir::Instr::CMPGT {
            lhs: "sum".to_string(),
            rhs: "100".to_string(),
            dest: "cond".to_string(),
          },
          ir::Instr::JMPIF { cond: "cond".to_string(), l1: "l1".to_string(), l0: "l0".to_string() },
          ir::Instr::GOTO { to: "l0".to_string() },
          ir::Instr::SUB {
            lhs: "sum".to_string(),
            rhs: "50".to_string(),
            dest: "diff".to_string(),
          },
          ir::Instr::RET { tag: "diff".to_string() },
          ir::Instr::RET { tag: "sum".to_string() },
        ],
      },
      ir::FnIr {
        name: "main".to_string(),
        params: vec![],
        ret_ty: Some("u32".to_string()),
        body: vec![
          ir::Instr::OWN { tag: "42".to_string(), dest: "x".to_string() },
          ir::Instr::OWN { tag: "58".to_string(), dest: "y".to_string() },
          ir::Instr::CALL {
            name: "compute".to_string(),
            args: vec!["x".to_string(), "y".to_string()],
            dest: "result".to_string(),
          },
          ir::Instr::RET { tag: "result".to_string() },
        ],
      },
    ],
  };
  println!("{}", ir);
}

fn main() {
  let matches = cli::command_line();
  match matches.subcommand() {
    // let a;
    Some(("check", matches)) => {
      let file = matches.get_one::<String>("file").expect("file is required");
      let source = loader(file);
      check(source);
    }

    Some(("compile", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      let source = loader(file);
      compile(source);
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
