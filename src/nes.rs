use crate::cpu::CPU;

pub trait Powerable {
    fn power_on(&mut self);
    fn reset(&mut self);
}

#[derive(Default)]
pub struct NES {
    cpu: CPU,
}

impl NES {
    pub fn load_rom(&mut self, ines: Vec<u8>) {
        self.cpu.load_rom(ines)
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}

impl Powerable for NES {
    fn power_on(&mut self) {
        self.cpu.power_on();
    }
    fn reset(&mut self) {
        self.cpu.reset();
    }
}
