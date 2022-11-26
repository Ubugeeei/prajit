use super::{lexer::Lexer, token::Token};

pub struct Parser {
    lx: Lexer,
    current: Token,
    peeked: Token,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut lx = Lexer::new(input);
        let current = lx.next();
        let peeked = lx.next();
        Parser {
            lx,
            current,
            peeked,
        }
    }
}
