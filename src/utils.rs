pub fn build_u16(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) + (lsb as u16)
}

pub fn get_msb(value: u16) -> u8 {
    value.wrapping_shr(8) as u8
}

pub fn get_lsb(value: u16) -> u8 {
    (value & 0xFF) as u8
}
