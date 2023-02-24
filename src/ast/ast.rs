
use token;


pub trait AToAny: 'static {
  fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AToAny for T {
  fn as_any(&self) -> &dyn std::any::Any {
      self
  }
}

pub trait Statement: AToAny {
  fn literal(&self) -> &str { "" }
  fn is_invalid(&self) -> bool {
    false
  }
}

pub trait Expression: AToAny {
  fn literal(&self) -> &str { "" }
  fn is_invalid(&self) -> bool {
    false
  }
}

#[derive(Debug)]
pub struct InvalidStmt {}

impl InvalidStmt {
  pub fn new() -> InvalidStmt {
    InvalidStmt {  }
  }
}

impl Statement for InvalidStmt {
  fn is_invalid(&self) -> bool {
      true
  }
}

#[derive(Debug)]
pub struct InvalidExpr {}

impl InvalidExpr {
  pub fn new() -> InvalidExpr {
    InvalidExpr {  }
  }
}

impl Expression for InvalidExpr {
  fn is_invalid(&self) -> bool {
      true
  }
}

pub struct Program {
  pub stmts: Vec<Box<dyn Statement>>,
}

impl Program {
  pub fn new(stmts: Vec<Box<dyn Statement>>) -> Program {
    Program { stmts: stmts }
  }

  fn literal(&self) -> &str {
    if !self.stmts.is_empty() {
      return self.stmts[0].literal();
    }
    ""
  }
}

#[derive(Debug)]
pub struct IdentExpr {
  token: token::Token,
  pub value: String
}

impl IdentExpr {
  pub fn new(tok: token::Token) -> IdentExpr {
    let value = tok.lit.clone();
    IdentExpr { token: tok, value: value }
  }
}

impl Expression for IdentExpr {
  fn literal(&self) -> &str {
    self.token.lit.as_str()
  }
}

pub struct LetStmt {
  token: token::Token,
  pub name: IdentExpr,
  value: Box<dyn Expression>,
}

impl LetStmt {
  pub fn new(tok: token::Token, id: IdentExpr) -> LetStmt {
    LetStmt { token: tok, name: id, value: Box::new(InvalidExpr::new()) }
  }
}

impl Statement for LetStmt {
  fn literal(&self) -> &str {
    self.token.lit.as_str()
  }
}
