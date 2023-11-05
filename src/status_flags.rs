#[derive(Debug)]
pub struct StatusFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub brk: bool,
    pub ignored: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl Default for StatusFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusFlags {
    pub fn new() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            brk: false,
            ignored: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte = 0x00;
        if self.carry {
            byte |= 0x01;
        }
        if self.zero {
            byte |= 0x02;
        }
        if self.interrupt {
            byte |= 0x04;
        }
        if self.decimal {
            byte |= 0x08;
        }
        if self.brk {
            byte |= 0x10;
        }
        if self.ignored {
            byte |= 0x20;
        }
        if self.overflow {
            byte |= 0x40;
        }
        if self.negative {
            byte |= 0x80;
        }
        byte
    }

    pub fn from_byte(&self, byte: u8) -> Self {
        Self {
            carry: byte & 0x01 != 0,
            zero: byte & 0x02 != 0,
            interrupt: byte & 0x04 != 0,
            decimal: byte & 0x08 != 0,
            brk: byte & 0x10 != 0,
            ignored: byte & 0x20 != 0,
            overflow: byte & 0x40 != 0,
            negative: byte & 0x80 != 0,
        }
    }
}
