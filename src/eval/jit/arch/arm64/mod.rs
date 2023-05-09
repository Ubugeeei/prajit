#![allow(dead_code)]
#![allow(unreachable_code)]

use super::{InlineAssembler, InlineAssembler64BitsRegister};

pub(crate) struct Arm64InlineAssembler<T> {
    pub codes: Vec<u8>,
    stack_len: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Arm64InlineAssembler<T> {
    pub(crate) fn new() -> Self {
        Arm64InlineAssembler {
            codes: vec![],
            stack_len: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> InlineAssembler<T> for Arm64InlineAssembler<T> {
    fn unsafe_transmute(&mut self) -> extern "C" fn() -> T {
        unsafe {
            let ptr = self.codes.as_ptr() as *const T;
            std::mem::transmute(ptr)
        }
    }

    fn mov(&mut self, _reg: InlineAssembler64BitsRegister, _value: u64) {
        todo!();
        // self.codes.push(0xd2);
        // self.codes.push(0xd2);
        // self.codes.push(0xd2);
        self.codes.push(0xd2);
    }

    // <stur reg [sp, #-offset](H)>: 1110 -- reg number -- complement of offset -- 0011
    // <stur reg [sp, #-offset](L)>: 0x1f -- 0xf8
    fn push(&mut self, _reg: InlineAssembler64BitsRegister) {
        todo!();
        let reg = self.register_map(_reg);
        let offset = (self.stack_len + 1) as u8 * 8;

        self.codes.push((0b1110 << 4) | reg);
        self.codes.push((offset << 4) | 0b0011);
        self.codes.push(0x1f);
        self.codes.push(0xf8);

        self.stack_len += 1;
    }

    // <stur reg [sp, #-offset](H)>: 1110 -- reg number -- complement of offset -- 0011
    // <stur reg [sp, #-offset](L)>: 0x5f -- 0xf8
    fn pop(&mut self, _reg: InlineAssembler64BitsRegister) {
        todo!();
        let reg = self.register_map(_reg);
        let offset = (self.stack_len + 1) as u8 * 8;

        self.codes.push((0b1110 << 4) | reg);
        self.codes.push((offset << 4) | 0b0011);
        self.codes.push(0x5f);
        self.codes.push(0xf8);

        self.stack_len += 1;
    }

    fn add(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn sub(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn mul(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn div(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn register_map(&self, reg: InlineAssembler64BitsRegister) -> u8 {
        match reg {
            InlineAssembler64BitsRegister::R0 => Registers::X0,
            InlineAssembler64BitsRegister::R1 => Registers::X1,
            InlineAssembler64BitsRegister::R2 => Registers::X2,
            InlineAssembler64BitsRegister::R3 => Registers::X3,
            InlineAssembler64BitsRegister::R4 => Registers::X4,
            InlineAssembler64BitsRegister::R5 => Registers::X5,
            InlineAssembler64BitsRegister::R6 => Registers::X6,
            InlineAssembler64BitsRegister::R7 => Registers::X7,
        }
    }
}

#[allow(non_snake_case)]
mod Registers {
    pub const X0: u8 = 0;
    pub const X1: u8 = 1;
    pub const X2: u8 = 2;
    pub const X3: u8 = 3;
    pub const X4: u8 = 4;
    pub const X5: u8 = 5;
    pub const X6: u8 = 6;
    pub const X7: u8 = 7;
}
