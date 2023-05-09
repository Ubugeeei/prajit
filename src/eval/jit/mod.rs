#![allow(dead_code)]
#![allow(unreachable_code)]

use crate::parse::{ast::Node, parse};

use self::arch::{arm64::Arm64InlineAssembler, InlineAssembler};

pub mod arch;

pub(crate) fn jit_compile<T: 'static>(source: &str, isa: Isa) -> extern "C" fn() -> T {
    let ast = parse(source.to_string()).unwrap();
    let mut compiler = JitCompiler::new(ast, isa);
    compiler.compile()
}

pub(crate) enum Isa {
    Arm64,
}

struct JitCompiler<T> {
    ast: Node,
    arch: Box<dyn InlineAssembler<T>>,
}

impl<T: 'static> JitCompiler<T> {
    fn new(ast: Node, isa: Isa) -> Self {
        match isa {
            Isa::Arm64 => JitCompiler {
                arch: Box::new(Arm64InlineAssembler::new()),
                ast,
            },
        }
    }

    fn compile(&mut self) -> extern "C" fn() -> T {
        self.generate();
        self.arch.unsafe_transmute()
    }

    fn generate(&mut self) {
        todo!()
    }
}

#[allow(dead_code)]
fn sample() -> extern "C" fn() -> u64 {
    let machine_codes: &[u8] = &[
        0x40, 0x05, 0x80, 0xd2, 0x30, 0x00, 0x80, 0xd2, 0x01, 0x10, 0x00, 0xd4,
    ];
    let f: extern "C" fn() -> u64 = unsafe { std::mem::transmute(machine_codes.as_ptr()) };
    f
}
