use std::io::{self, Write};
use token;
use stringreader::StringReader;

const PROMPT: &str = ">> ";

pub fn start() -> io::Result<()> {
  loop {
    print!("{}", PROMPT);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let mut lex = lexer::Lexer::new(io::BufReader::new(StringReader::new(&buf)))?;
    loop {
      let tok = lex.next_token()?;
      println!("{}", tok);
      if tok.typ == token::TokenType::EOF {
        break;
      }
    }
  }
}
