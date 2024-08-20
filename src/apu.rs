use crate::nes::Powerable;

#[derive(Default)]
pub struct APU {}

impl APU {}

impl Powerable for APU {
    fn power_on(&mut self) {}
    fn reset(&mut self) {}
}
