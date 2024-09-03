use crate::lexer::Lexer;

pub struct Parser<'p> {
  lexer: &'p mut Lexer<'p>,
}

impl<'p> Parser<'p> {
  pub fn new(lexer: &'p mut Lexer<'p>) -> Self {
    Self { lexer }
  }
  pub fn parse(&mut self) {
    todo!()
  }
}
