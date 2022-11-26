use super::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    BinaryOperator(OperatorNode),
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub struct OperatorNode {
    pub op: Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
}
