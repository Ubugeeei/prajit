pub(crate) mod arm64;

pub(crate) trait InlineAssembler<T> {
    fn unsafe_transmute(&mut self) -> extern "C" fn() -> T;
    fn mov(&mut self, _reg: InlineAssembler64BitsRegister, _value: u64);
    fn push(&mut self, reg: InlineAssembler64BitsRegister);
    fn pop(&mut self, reg: InlineAssembler64BitsRegister);
    fn add(&mut self, a: u64, b: u64);
    fn sub(&mut self, a: u64, b: u64);
    fn mul(&mut self, a: u64, b: u64);
    fn div(&mut self, a: u64, b: u64);
    fn register_map(&self, reg: InlineAssembler64BitsRegister) -> u8;
}

pub(crate) enum InlineAssembler64BitsRegister {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}
