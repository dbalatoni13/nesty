use bitfield_struct::bitfield;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NametableArrangement {
    VERTICAL,
    HORIZONTAL,
}

impl NametableArrangement {
    pub const fn into_bits(self) -> u8 {
        self as _
    }
    pub const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::VERTICAL,
            1 => Self::HORIZONTAL,
            _ => panic!("Failed to parse enum"),
        }
    }
}

#[bitfield(u8)]
pub struct Flags6 {
    #[bits(1)]
    pub nametable_arrangement: NametableArrangement,
    #[bits(1)]
    pub battery_backed_prg_ram: bool,
    #[bits(1)]
    pub trainer: bool,
    #[bits(1)]
    pub alt_nametable_layout: bool,
    #[bits(4)]
    pub mapper_number_lower_nibble: u8,
}
