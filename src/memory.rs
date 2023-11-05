
pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

#[derive(Debug)]
pub struct SimpleMemory([u8; 65536]);

impl Default for SimpleMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for SimpleMemory {
    fn read(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.0[address as usize] = value;
    }
}

impl SimpleMemory {
    pub fn new() -> Self {
        Self([0; 65536])
    }
}
