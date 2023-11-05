#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    XIndirect,
    IndirectY,
    Implied,
    Unknown,
}
