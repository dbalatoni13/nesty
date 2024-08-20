use crate::nes::Powerable;

const MEMORY_SIZE: usize = 0xFFFF - 0x4020;

#[derive(Default)]
pub struct Cartridge {
    memory: Vec<u8>,
}

impl Cartridge {
    pub fn load_at(&mut self, address: u16, mem: Vec<u8>) {
        self.memory
            .splice(address as usize..MEMORY_SIZE, mem.iter().cloned());
    }

    pub fn read_mem(&self, address: u16) -> u8 {
        *self
            .memory
            .get(address as usize)
            .expect("Memory address out of range")
    }

    pub fn write_mem(&mut self, address: u16, value: u8) {
        let elem = self
            .memory
            .get_mut(address as usize)
            .expect("Memory address out of range");
        *elem = value;
    }
}

impl Powerable for Cartridge {
    fn power_on(&mut self) {
        self.memory = vec![0; MEMORY_SIZE];
    }
    fn reset(&mut self) {}
}
