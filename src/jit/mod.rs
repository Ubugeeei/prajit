use crate::parse::{ast::Node, token::Token};

pub fn compile(ast: Node) -> Vec<u8> {
    let mut compiler = Compiler::new(ast);
    compiler.compile()
}

enum Register {
    RAX,
    RDI,
}

struct Compiler {
    ast: Node,
    code: Vec<u8>,
}
impl Compiler {
    fn new(ast: Node) -> Self {
        Compiler { ast, code: vec![] }
    }

    fn compile(&mut self) -> Vec<u8> {
        self.code = vec![];
        self.compile_node(&self.ast.clone());
        self.pop(Register::RAX);
        self.ret();
        self.code.clone()
    }

    fn compile_node(&mut self, node: &Node) {
        match node {
            Node::Number(num) => self.push(*num),
            Node::BinaryOperator(binary_expr) => {
                self.compile_node(&*binary_expr.left);
                self.compile_node(&*binary_expr.right);
                self.pop(Register::RDI);
                self.pop(Register::RAX);
                match binary_expr.op {
                    Token::Plus => self.add_rax_rdi(),
                    Token::Minus => self.sub_rax_rdi(),
                    Token::Asterisk => self.mul_rax_rdi(),
                    Token::Slash => self.div_rax_rdi(),
                    _ => unreachable!(),
                };
                self.push_rax();
            }
        }
    }

    fn push(&mut self, value: i32) {
        self.code.push(0x68);
        for u in self.little_endian(value) {
            self.code.push(u);
        }
    }

    fn push_rax(&mut self) {
        self.code.push(0x50);
    }

    fn pop(&mut self, register: Register) {
        match register {
            Register::RAX => self.code.push(0x58),
            Register::RDI => self.code.push(0x5f),
        }
    }

    fn add_rax_rdi(&mut self) {
        self.code.push(0x48);
        self.code.push(0x01);
        self.code.push(0xf8);
    }

    fn sub_rax_rdi(&mut self) {
        self.code.push(0x48);
        self.code.push(0x29);
        self.code.push(0xc7);
    }

    fn mul_rax_rdi(&mut self) {
        self.code.push(0x48);
        self.code.push(0xf7);
        self.code.push(0xe7);
    }

    fn div_rax_rdi(&mut self) {
        self.code.push(0x48);
        self.code.push(0xf7);
        self.code.push(0xf7);
    }

    fn ret(&mut self) {
        self.code.push(0xc3);
    }

    fn little_endian(&mut self, value: i32) -> Vec<u8> {
        (0..4)
            .map(|it| ((value >> (it * 8)) & 0xff) as u8)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parser::parse;

    use super::*;

    #[test]
    fn test_compile() {
        let ast = parse(String::from("2 * 3 + 4 * 5")).unwrap();
        let byte_codes = compile(ast);
        assert_eq!(
            byte_codes,
            vec![
                0x68, 0x02, 0x00, 0x00, 0x00, // push   0x2
                0x68, 0x03, 0x00, 0x00, 0x00, // push   0x3
                0x5f, //                         pop    rdi
                0x58, //                         pop    rax
                0x48, 0xf7, 0xe7, //             mul    rdi
                0x50, //                         push   rax
                0x68, 0x04, 0x00, 0x00, 0x00, // push   0x4
                0x68, 0x05, 0x00, 0x00, 0x00, // push   0x5
                0x5f, //                         pop    rdi
                0x58, //                         pop    rax
                0x48, 0xf7, 0xe7, //             mul    rdi
                0x50, //                         push   rax
                0x5f, //                         pop    rdi
                0x58, //                         pop    rax
                0x48, 0x01, 0xf8, //             add    rax,  rdi
                0x50, //                         push   rax
                0x58, //                         pop    rax
                0xc3, //                         ret
            ]
        );

        let ast = parse(String::from("512 * 1024 + 2047 * 5")).unwrap();
        let byte_codes = compile(ast);
        assert_eq!(
            byte_codes,
            vec![
                0x68, 0x00, 0x02, 0x00, 0x00, // push   0x200
                0x68, 0x00, 0x04, 0x00, 0x00, // push   0x400
                0x5f, //                         pop    rdi
                0x58, //                         pop    rdi
                0x48, 0xf7, 0xe7, //             mul    rdi
                0x50, //                         push   rax
                0x68, 0xff, 0x07, 0x00, 0x00, // push   0x7ff
                0x68, 0x05, 0x00, 0x00, 0x00, // push   0x005
                0x5f, //                         pop    rdi
                0x58, //                         pop    rax
                0x48, 0xf7, 0xe7, //             mul    rdi
                0x50, //                         push   rax
                0x5f, //                         pop    rdi
                0x58, //                         pop    rax
                0x48, 0x01, 0xf8, //             add    rax,  rdi
                0x50, //                         push   rax
                0x58, //                         pop    rax
                0xc3, //                         ret
            ]
        );
    }
}
