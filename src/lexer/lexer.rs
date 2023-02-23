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
      ch: b'\0',
    };
    l.read_char()?;
    Ok(l)
  }

  pub fn next_token(&mut self) -> io::Result<token::Token> {
    // Ok(token::Token::new(token::TokenType::EOF, ""))
    let tok = match self.ch {
      b'=' => token::Token::new(token::TokenType::ASSIGN, "="),
      b';' => token::Token::new(token::TokenType::SEMICOLON, ";"),
      b'(' => token::Token::new(token::TokenType::LPAREN, "("),
      b')' => token::Token::new(token::TokenType::RPAREN, ")"),
      b',' => token::Token::new(token::TokenType::COMMA, ","),
      b'+' => token::Token::new(token::TokenType::PLUS, "+"),
      b'{' => token::Token::new(token::TokenType::LBRACE, "{"),
      b'}' => token::Token::new(token::TokenType::RBRACE, "}"),
      _ => token::Token::new(token::TokenType::EOF, ""),
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
}

#[cfg(test)]
mod test {
use super::*;
use stringreader::StringReader;
use token::{Token, TokenType};

#[test]
fn test_next_token() {
  let src = "=+(){},;";

  let tests: Vec<Token> = vec![
    Token::new(TokenType::ASSIGN, "="),
    Token::new(TokenType::PLUS, "+"),
    Token::new(TokenType::LPAREN, "("),
    Token::new(TokenType::RPAREN, ")"),
    Token::new(TokenType::LBRACE, "{"),
    Token::new(TokenType::RBRACE, "}"),
    Token::new(TokenType::COMMA, ","),
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
