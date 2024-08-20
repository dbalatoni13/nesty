use nesty::nes::{Powerable, NES};

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut nes = NES::default();
    let rom = read_rom();
    nes.power_on();
    nes.load_rom(rom);
    nes.run();
}

fn read_rom() -> Vec<u8> {
    let rom_file_name = env::args().nth(1).unwrap(); // TODO error handling
    let mut rom_file = File::open(rom_file_name).unwrap();

    let mut rom = Vec::new();
    rom_file.read_to_end(&mut rom).unwrap();
    rom
}
