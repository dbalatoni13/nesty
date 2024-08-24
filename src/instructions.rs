pub mod addressing;
pub mod logic;

#[derive(Debug)]
pub enum InstructionType {
    Illegal,
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

#[derive(Debug)]
pub enum AddressingMode {
    Illegal,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    IndexedIndirect,
    IndirectIndexed,
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    Absolute,
    Relative, // to PC
    Indirect,
}

#[derive(Debug)]
pub struct Instruction {
    pub inst_type: InstructionType,
    pub addr_mode: AddressingMode,
}

pub fn get_inst_type(byte: u8) -> InstructionType {
    // TODO handle illegal instructions
    match byte & 0b11 {
        0 => {
            // red
            match byte & 0b11111100 {
                0x00 => InstructionType::BRK,
                0x08 => InstructionType::PHP,
                0x10 => InstructionType::BPL,
                0x18 => InstructionType::CLC,
                0x20 => InstructionType::JSR,
                0x24 => InstructionType::BIT,
                0x28 => InstructionType::PLP,
                0x2C => InstructionType::BIT,
                0x30 => InstructionType::BMI,
                0x38 => InstructionType::SEC,
                0x40 => InstructionType::RTI,
                0x48 => InstructionType::PHA,
                0x4C => InstructionType::JMP,
                0x50 => InstructionType::BVC,
                0x58 => InstructionType::CLI,
                0x60 => InstructionType::RTS,
                0x68 => InstructionType::PLA,
                0x6C => InstructionType::JMP,
                0x70 => InstructionType::BVS,
                0x78 => InstructionType::SEI,
                0x84 => InstructionType::STY,
                0x88 => InstructionType::DEY,
                0x8C => InstructionType::STY,
                0x90 => InstructionType::BCC,
                0x94 => InstructionType::STY,
                0x98 => InstructionType::TYA,
                0xA0 => InstructionType::LDY,
                0xA4 => InstructionType::LDY,
                0xA8 => InstructionType::TAY,
                0xAC => InstructionType::LDY,
                0xB0 => InstructionType::BCS,
                0xB4 => InstructionType::LDY,
                0xB8 => InstructionType::CLV,
                0xBC => InstructionType::LDY,
                0xC0 => InstructionType::CPY,
                0xC4 => InstructionType::CPY,
                0xC8 => InstructionType::INY,
                0xCC => InstructionType::CPY,
                0xD0 => InstructionType::BNE,
                0xD8 => InstructionType::CLD,
                0xE0 => InstructionType::CPX,
                0xE4 => InstructionType::CPX,
                0xE8 => InstructionType::INX,
                0xEC => InstructionType::CPX,
                0xF0 => InstructionType::BEQ,
                0xF8 => InstructionType::SED,
                _ => InstructionType::Illegal,
            }
        }
        1 => {
            // green
            match byte & 0b11100000 {
                0x00 => InstructionType::ORA,
                0x20 => InstructionType::AND,
                0x40 => InstructionType::EOR,
                0x60 => InstructionType::ADC,
                0x80 => InstructionType::STA,
                0xA0 => InstructionType::LDA,
                0xC0 => InstructionType::CMP,
                0xE0 => InstructionType::SBC,
                _ => InstructionType::Illegal,
            }
        }
        2 => {
            // blue
            match byte & 0b11100000 {
                0x00 => InstructionType::ASL,
                0x20 => InstructionType::ROL,
                0x40 => InstructionType::LSR,
                0x60 => InstructionType::ROR,
                0x80 => match (byte & 0b00011100) >> 2 {
                    0 => InstructionType::NOP,
                    1 | 3 | 5 => InstructionType::STX,
                    2 => InstructionType::TXA,
                    6 => InstructionType::TXS,
                    _ => InstructionType::Illegal,
                },
                0xA0 => match (byte & 0b00011100) >> 2 {
                    2 => InstructionType::TAX,
                    6 => InstructionType::TSX,
                    _ => InstructionType::LDX,
                },
                0xC0 => match (byte & 0b00011100) >> 2 {
                    2 => InstructionType::DEX,
                    _ => InstructionType::DEC,
                },
                0xE0 => match (byte & 0b00011100) >> 2 {
                    2 => InstructionType::NOP,
                    _ => InstructionType::LDX,
                },
                _ => InstructionType::Illegal,
            }
        }
        3 => {
            // gray
            match byte & 0b11100000 {
                _ => InstructionType::Illegal,
            }
        }
        _ => InstructionType::Illegal,
    }
}

pub fn get_addr_mode(byte: u8) -> AddressingMode {
    match byte & 0b11 {
        0 => {
            // red
            match (byte & 0b00011100) >> 2 {
                0 => match byte & 0b11100000 {
                    0x10 | 0x40 | 0x60 => AddressingMode::Implicit,
                    0x20 => AddressingMode::Absolute,
                    _ => AddressingMode::Immediate,
                },
                1 => AddressingMode::ZeroPage,
                2 => AddressingMode::Implicit,
                3 => match byte & 0b11100000 {
                    0x60 => AddressingMode::Indirect,
                    _ => AddressingMode::Absolute,
                },
                4 => AddressingMode::Relative,
                5 => AddressingMode::ZeroPageIndexedX,
                6 => AddressingMode::Implicit,
                7 => AddressingMode::AbsoluteIndexedX,
                _ => AddressingMode::Illegal,
            }
        }
        1 => {
            // green
            match (byte & 0b00011100) >> 2 {
                0 => AddressingMode::ZeroPageIndexedX,
                1 => AddressingMode::ZeroPage,
                2 => AddressingMode::Immediate,
                3 => AddressingMode::Absolute,
                4 => AddressingMode::IndirectIndexed,
                5 => AddressingMode::IndexedIndirect,
                6 => AddressingMode::AbsoluteIndexedY,
                7 => AddressingMode::AbsoluteIndexedX,
                _ => AddressingMode::Illegal,
            }
        }
        2 => {
            // blue
            match (byte & 0b00011100) >> 2 {
                0 => AddressingMode::Immediate,
                1 => AddressingMode::ZeroPage,
                2 => match byte & 0b11100000 {
                    0x00..=0x60 => AddressingMode::Accumulator,
                    _ => AddressingMode::Implicit,
                },
                3 => AddressingMode::Absolute,
                4 => AddressingMode::Implicit,
                5 | 7 => match byte & 0b11100000 {
                    0x00..=0x60 | 0xC0..=0xE0 => AddressingMode::ZeroPageIndexedX,
                    0x80 | 0xA0 => AddressingMode::ZeroPageIndexedY,
                    _ => AddressingMode::Illegal,
                },
                6 => AddressingMode::Implicit,
                _ => AddressingMode::Illegal,
            }
        }
        3 => {
            // gray
            match byte & 0b11100000 {
                _ => AddressingMode::Illegal,
            }
        }
        _ => AddressingMode::Illegal,
    }
}

pub fn get_num_of_operands(addr_mode: &AddressingMode) -> usize {
    match addr_mode {
        AddressingMode::Illegal => 0,
        AddressingMode::ZeroPageIndexedX => 1,
        AddressingMode::ZeroPageIndexedY => 1,
        AddressingMode::AbsoluteIndexedX => 2,
        AddressingMode::AbsoluteIndexedY => 2,
        AddressingMode::IndexedIndirect => 1,
        AddressingMode::IndirectIndexed => 1,
        AddressingMode::Implicit => 0,
        AddressingMode::Accumulator => 0,
        AddressingMode::Immediate => 1,
        AddressingMode::ZeroPage => 1,
        AddressingMode::Absolute => 2,
        AddressingMode::Relative => 1,
        AddressingMode::Indirect => 2,
    }
}
