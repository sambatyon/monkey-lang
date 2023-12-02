use std::io;
use std::mem::swap;

use ast::{self, InvalidExpr};
use lexer;
use token::TokenType;

pub struct Parser<T: io::Read> {
  lexer: lexer::Lexer<T>,
  cur: token::Token,
  peek: token::Token,
  errors: Vec<String>,
}

impl<T: std::io::Read> Parser<T> {
  pub fn new(lex: lexer::Lexer<T>) -> Result<Parser<T>, String> {
    let mut p = Parser{
      lexer: lex,
      cur: token::Token::illegal(),
      peek: token::Token::illegal(),
      errors: Vec::new(),
    };
    p.next_token()?;
    p.next_token()?;
    Ok(p)
  }

  pub fn program(&mut self) -> Result<Box<ast::Program>, String> {
    let mut stmts = Vec::new();
    let mut error = false;
    while self.cur.typ != token::TokenType::EOF {
      let stmt = self.stmt();
      if stmt.is_invalid() {
        error = true
      }
      stmts.push(stmt);
      match self.next_token() {
        Ok(_) => (),
        Err(msg) => {
          self.errors.push(msg);
          error = true;
          break;
        }
      }
    }
    if error {
      return Err(self.errors.join("\n"));
    }
    Ok(Box::new(ast::Program::new(stmts)))
  }

  fn stmt(&mut self) -> Box<dyn ast::Statement> {
    match self.cur.typ {
      token::TokenType::LET => {
        match self.let_stmt() {
          Some(l) => return Box::new(l),
          _ => Box::new(ast::InvalidStmt::new())
        }
      },
      token::TokenType::RETURN => {
        match self.return_stmt() {
          Some(r) => return Box::new(r),
          _ => Box::new(ast::InvalidStmt::new())
        }
      },
      _ => Box::new(ast::InvalidStmt::new())
    }
  }

  fn let_stmt(&mut self) -> Option<ast::LetStmt> {
    let tok = self.cur.clone();
    if !self.expect_peek(token::TokenType::IDENT) {
      return None;
    }
    let id = ast::IdentExpr::new(self.cur.clone());

    if !self.expect_peek(token::TokenType::ASSIGN) {
      return None;
    }

    while !self.match_cur(token::TokenType::SEMICOLON) {
      let r = self.next_token();
      if r.is_err() {
        self.errors.push(r.unwrap_err());
        return None;
      }
    }
    Some(ast::LetStmt::new(tok, id, Box::new(InvalidExpr::new())))
  }

  fn return_stmt(&mut self) -> Option<ast::ReturnStmt> {
    let tok = self.cur.clone();

    let r = self.next_token();
    if r.is_err() {
      self.errors.push(r.unwrap_err());
      return None;
    }

    while !self.match_cur(token::TokenType::SEMICOLON) {
      let r = self.next_token();
      if r.is_err() {
        self.errors.push(r.unwrap_err());
        return None;
      }
    }

    Some(ast::ReturnStmt::new(tok, Box::new(InvalidExpr::new())))
  }

  fn next_token(&mut self) -> Result<(), String> {
    swap(&mut self.cur, &mut self.peek);
    self.peek = match self.lexer.next_token() {
      Ok(tok) => tok,
      Err(e) => return Err(format!("Lexer error: {}", e))
    };
    Ok(())
  }

  fn expect_peek(&mut self, tok: TokenType) -> bool {
    if self.match_peek(tok) {
      match self.next_token() {
        Ok(_) => return true,
        Err(s) => {
          self.errors.push(s);
          return false
        }
      }
    }
    self.peek_error(tok);
    false
  }

  fn match_cur(&self, tok: TokenType) -> bool {
    self.cur.typ == tok
  }

  fn match_peek(&self, tok: TokenType) -> bool {
    self.peek.typ == tok
  }

  fn peek_error(&mut self, tok: TokenType) {
    self.errors.push(format!("syntax error: expected {} token, got {}", tok, self.peek.typ))
  }

  pub fn errors(&self) -> &Vec<String> {
    return &self.errors
  }
}

#[cfg(test)]
mod test {
use super::*;
use std::io;
use ast::{Statement, Node};
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

#[test]
fn test_return_stmts() {
  let src = r#"
  return 5;
  return 10;
  return 993322;
  "#;

  let lexer = lexer::Lexer::new(io::BufReader::new(StringReader::new(src))).expect("Lexer Build");
  let mut parser = Parser::new(lexer).expect("creating parser");

  let program = parser.program().expect("Parsing program");

  assert_eq!(program.stmts.len(), 3);
  for stmt in program.stmts {
    let r = stmt.as_ref().as_any().downcast_ref::<ast::ReturnStmt>().expect("Wrong Type for Return statement");
    assert_eq!(r.literal(), "return", "returnStmt.literal() not 'return': {}", r.literal());
  }
}
}
