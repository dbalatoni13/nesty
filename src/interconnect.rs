use crate::apu::APU;
use crate::nes::Powerable;
use crate::ppu::PPU;
use crate::ram::RAM;
use crate::rom::ROM;

#[derive(Default)]
pub struct Interconnect {
    rom: ROM,
    ram: RAM,
    ppu: PPU,
    apu: APU,
}

impl Interconnect {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom.load(rom)
    }

    pub fn read_mem(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read_mem(address & 0x7FF), // remove mirroring
            0x2000..=0x3FFF => self.ppu.read_reg((address & 7) as u8),
            0x4014 => panic!("Read of OAMDMA not implemented"), // TODO
            _ => panic!("Address {} not implemented for read", address),
        }
    }

    pub fn write_mem(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write_mem(address & 0x7FF, value), // remove mirroring
            0x2000..=0x3FFF => self.ppu.write_reg((address & 7) as u8, value),
            0x4014 => panic!("Read of OAMDMA not implemented"), // TODO
            _ => panic!("Address {} not implemented for read", address),
        }
    }
}

impl Powerable for Interconnect {
    fn power_on(&mut self) {
        self.rom.power_on();
        self.ram.power_on();
        self.ppu.power_on();
        self.apu.power_on();
    }
    fn reset(&mut self) {
        self.rom.reset();
        self.ram.reset();
        self.ppu.reset();
        self.apu.reset();
    }
}
