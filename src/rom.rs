use crate::nes::Powerable;

#[derive(Default)]
pub struct ROM {
    memory: Vec<u8>,
}

impl ROM {
    pub fn load(&mut self, rom: Vec<u8>) {
        self.memory = rom;
    }

    pub fn mem_read(&self, address: u16) -> u8 {
        8
    }

    pub fn mem_write(&mut self, address: u16) {}
}

impl Powerable for ROM {
    fn power_on(&mut self) {}
    fn reset(&mut self) {}
}
