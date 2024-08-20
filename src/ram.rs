use crate::nes::Powerable;

const RAM_SIZE: usize = 2 * 1024;

#[derive(Default)]
pub struct RAM {
    memory: Vec<u8>,
}

impl RAM {
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

impl Powerable for RAM {
    fn power_on(&mut self) {
        self.memory = vec![0; RAM_SIZE];
    }
    fn reset(&mut self) {
        self.memory = vec![0; RAM_SIZE]; // TODO remove?
    }
}
