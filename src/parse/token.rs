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
