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
  MINUS,
  BANG,
  ASTERISK,
  SLASH,
  LT,
  GT,
  EQ,
  NOT_EQ,

  // Delimiters
  COMMA,
  SEMICOLON,

  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,

  // Keywords
  FUNCTION,
  LET,
  TRUE,
  FALSE,
  IF,
  ELSE,
  RETURN,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TokenType::ILLEGAL => write!(f, "ILLEGAL"),
      TokenType::EOF => write!(f, "EOF"),
      TokenType::IDENT => write!(f, "IDENT"),
      TokenType::INT => write!(f, "INT"),
      TokenType::ASSIGN => write!(f, "="),
      TokenType::PLUS => write!(f, "+"),
      TokenType::MINUS => write!(f, "-"),
      TokenType::BANG => write!(f, "!"),
      TokenType::ASTERISK => write!(f, "*"),
      TokenType::SLASH => write!(f, "/"),
      TokenType::LT => write!(f, "<"),
      TokenType::GT => write!(f, ">"),
      TokenType::EQ => write!(f, "=="),
      TokenType::NOT_EQ => write!(f, "!="),
      TokenType::COMMA => write!(f, ","),
      TokenType::SEMICOLON => write!(f, ";"),
      TokenType::LPAREN => write!(f, "("),
      TokenType::RPAREN => write!(f, ")"),
      TokenType::LBRACE => write!(f, "{{"),
      TokenType::RBRACE => write!(f, "}}"),
      TokenType::FUNCTION => write!(f, "FUNCTION"),
      TokenType::LET => write!(f, "LET"),
      TokenType::TRUE => write!(f, "TRUE"),
      TokenType::FALSE => write!(f, "FALSE"),
      TokenType::IF => write!(f, "IF"),
      TokenType::ELSE => write!(f, "ELSE"),
      TokenType::RETURN => write!(f, "RETURN"),
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

  pub fn new_identifier(s: &str) -> Token {
    let typ = match s {
      "let" => TokenType::LET,
      "fn" => TokenType::FUNCTION,
      "true" => TokenType::TRUE,
      "false" => TokenType::FALSE,
      "if" => TokenType::IF,
      "else" => TokenType::ELSE,
      "return" => TokenType::RETURN,
      _ => TokenType::IDENT
    };
    Token { typ: typ, lit: s.to_string() }
  }

  pub fn new_number(s: &str) -> Token {
    Token { typ: TokenType::INT, lit: s.to_string() }
  }
}

