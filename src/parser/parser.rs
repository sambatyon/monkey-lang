use std::io;
use std::mem::swap;

use ast;
use lexer;
use token::TokenType;

pub struct Parser<T: io::Read> {
  lexer: lexer::Lexer<T>,
  cur: token::Token,
  peek: token::Token,
}

impl<T: std::io::Read> Parser<T> {
  pub fn new(lex: lexer::Lexer<T>) -> Result<Parser<T>, String> {
    let mut p = Parser{
      lexer: lex,
      cur: token::Token::illegal(),
      peek: token::Token::illegal(),
    };
    p.next_token()?;
    p.next_token()?;
    Ok(p)
  }

  pub fn program(&mut self) -> Result<Box<ast::Program>, String> {
    let mut stmts = Vec::new();
    while self.cur.typ != token::TokenType::EOF {
      let stmt = self.stmt()?;
      stmts.push(stmt);
      self.next_token()?;
    }
    Ok(Box::new(ast::Program::new(stmts)))
  }

  fn stmt(&mut self) -> Result<Box<dyn ast::Statement>, String> {
    match self.cur.typ {
      token::TokenType::LET => {
        let l = self.let_stmt()?;
        Ok(Box::new(l))
      },
      _ => Ok(Box::new(ast::InvalidStmt::new()))
    }
  }

  fn let_stmt(&mut self) -> Result<ast::LetStmt, String> {
    let tok = self.cur.clone();
    if !self.expect_peek(token::TokenType::IDENT)? {
      return Err(format!("syntax error, expected identifier, got {}", self.peek));
    }
    let id = ast::IdentExpr::new(self.cur.clone());

    if !self.expect_peek(token::TokenType::ASSIGN)? {
      return Err(format!("syntax error, expected '=', got {}", self.peek))
    }

    while !self.match_cur(token::TokenType::SEMICOLON) {
      self.next_token()?;
    }
    Ok(ast::LetStmt::new(tok, id))
  }

  fn next_token(&mut self) -> Result<(), String> {
    swap(&mut self.cur, &mut self.peek);
    self.peek = match self.lexer.next_token() {
      Ok(tok) => tok,
      Err(e) => return Err(format!("Lexer error: {}", e))
    };
    Ok(())
  }

  fn expect_peek(&mut self, tok: TokenType) -> Result<bool, String> {
    if self.match_peek(tok) {
      self.next_token()?;
      return Ok(true);
    }
    Ok(false)
  }

  fn match_cur(&self, tok: TokenType) -> bool {
    self.cur.typ == tok
  }

  fn match_peek(&self, tok: TokenType) -> bool {
    self.peek.typ == tok
  }
}

#[cfg(test)]
mod test {
use super::*;
use std::io;
use ast::{Statement, Expression};
use stringreader::StringReader;

#[test]
fn test_let_stmts() {
  let src = r#"
  let x = 5;
  let y = 10;
  let foobar = 838383;
  "#;

  let lexer = lexer::Lexer::new(io::BufReader::new(StringReader::new(src))).expect("Lexer Build");
  let mut parser = Parser::new(lexer).expect("creating parser");

  let program = parser.program().expect("Parsing program");
  assert_eq!(program.stmts.len(), 3);
  let mut idx = 0;
  for id in vec!["x", "y", "foobar"] {
    test_let_stmt(&program.stmts[idx], id);
    idx += 1;
  }
}

fn test_let_stmt(stmt: &Box<dyn Statement>, id: &str) {
  assert_eq!("let", stmt.as_ref().literal());

  // Not pretty but follows the same downcast style as the book.
  let l = stmt.as_ref().as_any().downcast_ref::<ast::LetStmt>().expect("Wrong Type for Let statement");

  assert_eq!(l.name.value.as_str(), id);
  assert_eq!(l.name.literal(), id);
}
}
