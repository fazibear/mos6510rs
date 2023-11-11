pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct FakeMemory();

impl Memory for FakeMemory {
    fn read(&self, _address: u16) -> u8 {
        0u8
    }

    fn write(&mut self, _address: u16, _value: u8) {}
}

impl FakeMemory {
    pub fn new() -> FakeMemory {
        FakeMemory()
    }
}
