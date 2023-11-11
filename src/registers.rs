#[derive(Debug)]
pub struct Registers {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            program_counter: 0x00,
            stack_pointer: 0xff,
            accumulator: 0x00,
            x: 0x00,
            y: 0x00,
        }
    }
}
