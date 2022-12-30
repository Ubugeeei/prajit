use crate::parse::{ast::Node, token::Token};

pub fn compile(ast: Node) -> String {
    let mut compiler = Compiler::new(ast);
    compiler.compile()
}

enum Register {
    RAX,
    RDI,
}

struct Compiler {
    ast: Node,
    assembly: String,
}

impl Compiler {
    fn new(ast: Node) -> Self {
        Compiler {
            ast,
            assembly: String::new(),
        }
    }

    fn compile(&mut self) -> String {
        self.assembly = String::new();
        self.entry();
        self.compile_node(&self.ast.clone());
        self.pop(Register::RAX);
        self.ret();
        self.assembly.clone()
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

    fn entry(&mut self) {
        self.assembly.push_str(&format!(".global _main\n\n"));
        self.assembly.push_str(&format!("_main:\n"));
    }

    fn push(&mut self, value: i32) {
        self.assembly.push_str(&format!("    push ${}\n", value));
    }

    fn push_rax(&mut self) {
        self.assembly.push_str(&format!("    push %rax\n"));
    }

    fn pop(&mut self, register: Register) {
        match register {
            Register::RAX => self.assembly.push_str(&format!("    pop %rax\n")),
            Register::RDI => self.assembly.push_str(&format!("    pop %rdi\n")),
        }
    }

    fn add_rax_rdi(&mut self) {
        self.assembly.push_str(&format!("    add %rdi, %rax\n"));
    }

    fn sub_rax_rdi(&mut self) {
        self.assembly.push_str(&format!("    sub %rdi, %rax\n"));
    }

    fn mul_rax_rdi(&mut self) {
        self.assembly.push_str(&format!("    mul %rdi\n"));
    }

    fn div_rax_rdi(&mut self) {
        self.assembly.push_str(&format!("    div %rdi\n"));
    }

    fn ret(&mut self) {
        self.assembly.push_str(&format!("    ret\n"));
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;

    use super::*;

    #[test]
    fn test_compile() {
        let ast = parse(String::from("2 * 3 + 4 * 5")).unwrap();
        let byte_codes = compile(ast);
        assert_eq!(
            byte_codes,
            String::from(
                r#".global _main
_main:
    push 0x2
    push 0x3
    pop rdi
    pop rax
    mul rdi
    push rax
    push 0x4
    push 0x5
    pop rdi
    pop rax
    mul rdi
    push rax
    pop rdi
    pop rax
    add rax, rdi
    push rax
    pop rax
    ret"#
            )
        );

        let ast = parse(String::from("512 * 1024 + 2047 * 5")).unwrap();
        let byte_codes = compile(ast);
        assert_eq!(
            byte_codes,
            String::from(
                r#".global _main
_main:
    push 0x200
    push 0x400
    pop rdi
    pop rdi
    mul rdi
    push rax
    push 0x7ff
    push 0x005
    pop rdi
    pop rax
    mul rdi
    push rax
    pop rdi
    pop rax
    add rax, rdi
    push rax
    pop rax
    ret"#
            )
        );
    }
}
