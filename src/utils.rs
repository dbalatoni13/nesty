pub fn build_u16(msb: u8, lsb: u8) -> u16 {
    (msb as u16) << 8 + lsb as u16
}
