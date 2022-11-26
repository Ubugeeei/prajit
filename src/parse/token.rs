#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Illegal,
    EOF,
    Number(f64),
    Plus,
    Minus,
    Slash,
    Asterisk,
}

impl Token {
    pub fn get_precedence(&self) -> u8 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Asterisk | Token::Slash => 2,
            _ => 0,
        }
    }
}
