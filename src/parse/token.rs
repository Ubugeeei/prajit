#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Number(f64),
    Plus,
    Minus,
    Slash,
    Asterisk,
}
