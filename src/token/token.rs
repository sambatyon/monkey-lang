use std::fmt;

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF,

  // Identifier + Literals
  IDENT,
  INT,

  // Operators
  ASSIGN,
  PLUS,

  // Delimiters
  COMMA,
  SEMICOLON,

  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,

  // Keywords
  FUNCTION,
  LET
}

// impl PartialEq for TokenType {
//   fn eq(&self, other: &Self) -> bool {
//     self == other
//   }
// }

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TokenType::ILLEGAL => write!(f, "ILLEGAL"),
      TokenType::EOF => write!(f, "EOF"),
      TokenType::IDENT => write!(f, "IDENT"),
      TokenType::INT => write!(f, "INT"),
      TokenType::ASSIGN => write!(f, "="),
      TokenType::PLUS => write!(f, "+"),
      TokenType::COMMA => write!(f, ","),
      TokenType::SEMICOLON => write!(f, ";"),
      TokenType::LPAREN => write!(f, "("),
      TokenType::RPAREN => write!(f, ")"),
      TokenType::LBRACE => write!(f, "{{"),
      TokenType::RBRACE => write!(f, "}}"),
      TokenType::FUNCTION => write!(f, "FUNCTION"),
      TokenType::LET => write!(f, "LET"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Token {
  pub typ: TokenType,
  pub lit: String,
}

impl Token {
  pub fn new(typ: TokenType, lit: &str) -> Token {
    Token { typ: typ, lit: lit.to_string() }
  }
}

