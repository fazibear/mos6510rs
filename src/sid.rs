pub trait Sid {
    fn samples(&mut self, delta: u32, buffer: &mut [i16]) -> (usize, u32);
    fn write(&mut self, address: u8, value: u8);
}

pub struct FakeSid();

impl Sid for FakeSid {
    fn samples(&mut self, _delta: u32, _buffer: &mut [i16]) -> (usize, u32) {
        (0, 0)
    }

    fn write(&mut self, _address: u8, _value: u8) {}
}

impl FakeSid {
    pub fn new() -> FakeSid {
        FakeSid()
    }
}
