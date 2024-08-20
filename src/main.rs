use nesty::nes::{Powerable, NES};

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut nes = NES::default();
    let ines = read_ines_file();
    nes.power_on();
    nes.load_rom(ines);
    nes.run();
}

fn read_ines_file() -> Vec<u8> {
    let rom_file_name = env::args().nth(1).unwrap(); // TODO error handling
    let mut ines_file = File::open(rom_file_name).unwrap();

    let mut ines = Vec::new();
    ines_file.read_to_end(&mut ines).unwrap();
    ines
}
