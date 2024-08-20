use crate::apu::APU;
use crate::cartridge::Cartridge;
use crate::ines::{Flags6, NametableArrangement};
use crate::nes::Powerable;
use crate::ppu::PPU;
use crate::ram::RAM;

#[derive(Default)]
pub struct Interconnect {
    cartridge: Cartridge,
    ram: RAM,
    ppu: PPU,
    apu: APU,
}

impl Interconnect {
    pub fn load_rom(&mut self, ines: Vec<u8>) {
        let header = &ines[..16];
        let prg_rom_size = header[4] as usize * 16 * 1024;
        let chr_rom_size = header[5] as usize * 8 * 1024;
        let flags = Flags6::from_bits(header[6]);

        let mut prg_rom = vec![];
        let prg_slice = match flags.trainer() {
            true => &ines[16..16 + 512 + prg_rom_size],
            false => &ines[16..16 + prg_rom_size],
        };
        prg_rom.extend_from_slice(prg_slice);
        // TODO implement mapper logic, only direct mapping now
        self.cartridge.load_at(0xC000 - 0x4020, prg_rom);
        // TODO do the rest of the flags
    }

    pub fn read_mem(&self, address: u16) -> u8 {
        let val = match address {
            0x0000..=0x1FFF => self.ram.read_mem(address & 0x7FF), // remove mirroring
            0x2000..=0x3FFF => self.ppu.read_reg((address & 7) as u8),
            0x4014 => panic!("Read of OAMDMA not implemented"), // TODO
            0x4020..=0xFFFF => self.cartridge.read_mem(address - 0x4020),
            _ => panic!("Address {} not implemented for read", address),
        };
        println!("Read {:#02x} from address {:#02x}", val, address);
        val
    }

    pub fn write_mem(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.write_mem(address & 0x7FF, value), // remove mirroring
            0x2000..=0x3FFF => self.ppu.write_reg((address & 7) as u8, value),
            0x4014 => panic!("Read of OAMDMA not implemented"), // TODO
            0x4020..=0xFFFF => self.cartridge.write_mem(address - 0x4020, value),
            _ => panic!("Address {} not implemented for read", address),
        }
        println!("Wrote {:#02x} to address {:#02x}", value, address);
    }
}

impl Powerable for Interconnect {
    fn power_on(&mut self) {
        self.cartridge.power_on();
        self.ram.power_on();
        self.ppu.power_on();
        self.apu.power_on();
    }
    fn reset(&mut self) {
        self.cartridge.reset();
        self.ram.reset();
        self.ppu.reset();
        self.apu.reset();
    }
}
