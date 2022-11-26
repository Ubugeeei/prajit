use super::{
    ast::{Node, OperatorNode},
    lexer::Lexer,
    token::Token,
};

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

    pub fn parse(&mut self) -> Node {
      self.parse_by_current_precedence(0)
    }

    fn parse_by_current_precedence(&mut self, precedence: u8) -> Node {
        let mut node = self.parse_unary();

        while precedence < self.peeked.get_precedence() && self.peeked != Token::EOF {
            if let Token::Plus | Token::Minus | Token::Asterisk | Token::Slash = self.peeked {
                self.next();
                node = self.parse_binary(node);
            }
        }

        node
    }

    fn parse_unary(&mut self) -> Node {
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

    fn parse_binary(&mut self, l: Node) -> Node {
        let o = self.current;
        let r = self.parse_by_current_precedence(o.get_precedence());
        Node::BinaryOperator(OperatorNode {
            op: o,
            left: Box::new(l),
            right: Box::new(r),
        })
    }

    fn next(&mut self) {
        self.current = self.peeked;
        self.peeked = self.lx.next();
    }
}
