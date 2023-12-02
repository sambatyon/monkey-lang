
use token;
use std::fmt;


pub trait AToAny: 'static {
  fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AToAny for T {
  fn as_any(&self) -> &dyn std::any::Any {
      self
  }
}

pub trait Node: AToAny + fmt::Display {
  fn literal(&self) -> &str { "" }
}

pub trait Statement: Node {
  fn is_invalid(&self) -> bool {
    false
  }
}

pub trait Expression: Node {
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

impl Node for InvalidStmt {}

impl fmt::Display for InvalidStmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "«INVALID_STMT»")
  }
}

#[derive(Debug)]
pub struct InvalidExpr {}

impl InvalidExpr {
  pub fn new() -> InvalidExpr {
    InvalidExpr {  }
  }
}

impl Node for InvalidExpr {}

impl fmt::Display for InvalidExpr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "«INVALID_EXPR»")
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
}

impl Node for Program {
  fn literal(&self) -> &str {
    if !self.stmts.is_empty() {
      return self.stmts[0].literal();
    }
    ""
  }
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut prog = String::new();
    for stmt in &self.stmts {
      prog.push_str(format!("{}\n", stmt).as_str());
    }
    write!(f, "{}", prog)
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

impl Expression for IdentExpr {}

impl Node for IdentExpr {
  fn literal(&self) -> &str {
    self.token.lit.as_str()
  }
}

impl fmt::Display for IdentExpr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

pub struct LetStmt {
  token: token::Token,
  pub name: IdentExpr,
  value: Box<dyn Expression>,
}

impl LetStmt {
  pub fn new(tok: token::Token, id: IdentExpr, val: Box<dyn Expression>) -> LetStmt {
    LetStmt { token: tok, name: id, value: val }
  }
}

impl Statement for LetStmt {}

impl Node for LetStmt {
  fn literal(&self) -> &str {
    self.token.lit.as_str()
  }
}

impl fmt::Display for LetStmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "let {} = {};", self.name, self.value)
  }
}

pub struct ReturnStmt {
  token: token::Token,
  expr: Box<dyn Expression>,
}

impl ReturnStmt {
  pub fn new(tok: token::Token, val: Box<dyn Expression>) -> ReturnStmt {
    ReturnStmt { token: tok, expr: val }
  }
}

impl Statement for ReturnStmt {}

impl Node for ReturnStmt {
  fn literal(&self) -> &str {
    self.token.lit.as_str()
  }
}

impl fmt::Display for ReturnStmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "return {}", self.expr)
  }
}

pub struct ExprStmt {
  token: token::Token,
  expr: Box<dyn Expression>,
}

impl ExprStmt {
  pub fn new(tok: token::Token, expr: Box<dyn Expression>) -> ExprStmt {
    ExprStmt { token: tok , expr: expr }
  }
}

impl Statement for ExprStmt {}

impl Node for ExprStmt {
  fn literal(&self) -> &str {
    self.token.lit.as_str()
  }
}

impl fmt::Display for ExprStmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.expr)
  }
}
