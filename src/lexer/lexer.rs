use std::io;
use std::io::{ErrorKind, Read};

use token;

pub struct Lexer<T: io::Read> {
  input: io::BufReader<T>,
  pos: i32,
  read_pos: i32,
  ch: u8,
}

impl<T: std::io::Read>  Lexer<T> {
  pub fn new(source: io::BufReader<T>) -> io::Result<Lexer<T>> {
    let mut l = Lexer {
      input: source,
      pos: 0,
      read_pos: 0,
      ch: b' ',
    };
    l.read_char()?;
    Ok(l)
  }

  pub fn next_token(&mut self) -> io::Result<token::Token> {
    while self.ch.is_ascii_whitespace() {
      self.read_char()?;
    }

    let tok = match self.ch {
      b'=' => token::Token::new(token::TokenType::ASSIGN, "="),
      b';' => token::Token::new(token::TokenType::SEMICOLON, ";"),
      b'(' => token::Token::new(token::TokenType::LPAREN, "("),
      b')' => token::Token::new(token::TokenType::RPAREN, ")"),
      b',' => token::Token::new(token::TokenType::COMMA, ","),
      b'+' => token::Token::new(token::TokenType::PLUS, "+"),
      b'{' => token::Token::new(token::TokenType::LBRACE, "{"),
      b'}' => token::Token::new(token::TokenType::RBRACE, "}"),
      b'\0' => token::Token::new(token::TokenType::EOF, ""),
      ch => if ch.is_ascii_alphabetic() || ch == b'_' {
        return self.read_ident()
      } else if ch.is_ascii_digit() {
        return self.read_number()
      } else {
        token::Token::new(token::TokenType::ILLEGAL, format!("{}", ch).as_str())
      }
    };
    self.read_char()?;
    Ok(tok)
  }

  fn read_char(&mut self) -> io::Result<()> {
    let mut buf = [0; 1];
    match self.input.read_exact(&mut buf) {
      Ok(_) => (),
      Err(err) => match err.kind() {
        ErrorKind::UnexpectedEof => {
          self.ch = b'\0';
          return Ok(())
        }
        _ => return Err(err)
      }
    }
    self.ch = buf[0];
    Ok(())
  }

  fn read_number(&mut self) -> io::Result<token::Token> {
    let mut bytes: Vec<u8> = vec![];
    loop {
      bytes.push(self.ch);
      self.read_char()?;
      if !self.ch.is_ascii_digit() {
        break
      }
    }
    let s = match std::str::from_utf8(&bytes) {
      Ok(val) => val,
      Err(e) => return Err(io::Error::new(ErrorKind::InvalidData, format!("{}", e)))
    };
    Ok(token::Token::new_number(s))
  }

  fn read_ident(&mut self) -> io::Result<token::Token> {
    let mut bytes: Vec<u8> = vec![];
    loop {
      bytes.push(self.ch);
      self.read_char()?;
      if !self.ch.is_ascii_alphabetic() && self.ch != b'_' {
        break
      }
    }
    let s = match std::str::from_utf8(&bytes) {
      Ok(val) => val,
      Err(e) => return Err(io::Error::new(ErrorKind::InvalidData, format!("{}", e)))
    };
    Ok(token::Token::new_identifier(s))
  }
}

#[cfg(test)]
mod test {
use super::*;
use stringreader::StringReader;
use token::{Token, TokenType};

#[test]
fn test_next_token() {
  let src = r#"
  let five = 5;
  let ten = 10;
  let add = fn(x, y) {
    x + y;
  };

  let result = add(five, ten);
  "#;

  let tests: Vec<Token> = vec![
    Token::new(TokenType::LET, "let"),
    Token::new(TokenType::IDENT, "five"),
    Token::new(TokenType::ASSIGN, "="),
    Token::new(TokenType::INT, "5"),
    Token::new(TokenType::SEMICOLON, ";"),

    Token::new(TokenType::LET, "let"),
    Token::new(TokenType::IDENT, "ten"),
    Token::new(TokenType::ASSIGN, "="),
    Token::new(TokenType::INT, "10"),
    Token::new(TokenType::SEMICOLON, ";"),

    Token::new(TokenType::LET, "let"),
    Token::new(TokenType::IDENT, "add"),
    Token::new(TokenType::ASSIGN, "="),
    Token::new(TokenType::FUNCTION, "fn"),
    Token::new(TokenType::LPAREN, "("),
    Token::new(TokenType::IDENT, "x"),
    Token::new(TokenType::COMMA, ","),
    Token::new(TokenType::IDENT, "y"),
    Token::new(TokenType::RPAREN, ")"),
    Token::new(TokenType::LBRACE, "{"),
    Token::new(TokenType::IDENT, "x"),
    Token::new(TokenType::PLUS, "+"),
    Token::new(TokenType::IDENT, "y"),
    Token::new(TokenType::SEMICOLON, ";"),
    Token::new(TokenType::RBRACE, "}"),
    Token::new(TokenType::SEMICOLON, ";"),

    Token::new(TokenType::LET, "let"),
    Token::new(TokenType::IDENT, "result"),
    Token::new(TokenType::ASSIGN, "="),
    Token::new(TokenType::IDENT, "add"),
    Token::new(TokenType::LPAREN, "("),
    Token::new(TokenType::IDENT, "five"),
    Token::new(TokenType::COMMA, ","),
    Token::new(TokenType::IDENT, "ten"),
    Token::new(TokenType::RPAREN, ")"),
    Token::new(TokenType::SEMICOLON, ";"),

    Token::new(TokenType::EOF, ""),
  ];
  let mut lexer = Lexer::new(io::BufReader::new(StringReader::new(src))).expect("Lexer Build");

  for tc in tests {
    let tok = lexer.next_token().expect("Token");
    assert_eq!(tok.typ, tc.typ);
    assert_eq!(tok.lit, tc.lit);
  }
}
}
