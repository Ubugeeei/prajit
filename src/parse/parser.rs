use super::{ast::Node, lexer::Lexer, token::Token};

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

    fn parse_prefix(&mut self) -> Node {
        match self.current {
            Token::Minus => {
                let n = match self.peeked {
                    Token::Number(i) => i,
                    _ => panic!(),
                };
                self.next();
                Node::Number(-n)
            }
            Token::Number(n) => Node::Number(n),
            _ => panic!(),
        }
    }

    fn next(&mut self) {
        self.current = self.peeked;
        self.peeked = self.lx.next();
    }
}
