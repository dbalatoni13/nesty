use crate::interconnect::Interconnect;
use crate::nes::Powerable;
//use enum_map::{enum_map, Enum, EnumMap};
//use once_cell::sync::Lazy;

#[derive(Debug)]
enum InstructionType {
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

// impl Default for InstructionType {
//     fn default() -> Self {
//         Self::None
//     }
// }

// const NUM_OF_OPERANDS: Lazy<EnumMap<AddressingMode, usize>> = Lazy::new(|| {
//     enum_map! {
//         Illegal => 0,
//         None => 0,
//         ZeroPageIndexedX => ,
//         ZeroPageIndexedY,
//         AbsoluteIndexedX,
//         AbsoluteIndexedY,
//         IndexedIndirect,
//         IndirectIndexed,
//         Implicit,
//         Accumulator,
//         Immediate,
//         ZeroPage,
//         Absolute,
//         Relative,
//         Indirect,
//     }
// });

#[derive(Debug)]
enum AddressingMode {
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

// impl Default for AddressingMode {
//     fn default() -> Self {
//         Self::None
//     }
// }

#[derive(Debug)]
struct Instruction {
    inst_type: InstructionType,
    addr_mode: AddressingMode,
}

#[derive(Default)]
pub struct CPU {
    cycle: u64,
    curr_inst_byte: Option<u8>,
    curr_inst: Option<Instruction>,
    operands: Vec<u8>,

    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_pc: u16,
    reg_s: u8,
    reg_c: u8,
    reg_z: u8,
    reg_i: u8,
    reg_d: u8,
    reg_v: u8,
    reg_n: u8,

    interconnect: Interconnect,
}

impl CPU {
    pub fn load_rom(&mut self, ines: Vec<u8>) {
        self.interconnect.load_rom(ines)
    }

    pub fn run(&mut self) {
        for _ in 0..10 {
            self.do_cycle();
        }
    }

    fn do_cycle(&mut self) {
        if self.cycle > 0 {
            self.fetch();
            self.decode();
            self.execute();
        } else {
            let lsb = self.interconnect.read_mem(self.reg_pc) as u16;
            let msb = self.interconnect.read_mem(self.reg_pc + 1) as u16;
            self.reg_pc = msb << 8 | lsb; // jump to start of code
            println!("Started execution at {:#02x}", self.reg_pc);
        }

        self.cycle += 1;
    }

    fn get_inst_type(&self, byte: u8) -> InstructionType {
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
                    0x80 => InstructionType::STX,
                    0xA0 => InstructionType::LDX,
                    0xC0 => InstructionType::DEC,
                    0xE0 => InstructionType::INC,
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

    fn get_addr_mode(&self, byte: u8) -> AddressingMode {
        match byte & 0b11 {
            0 => {
                // red
                match (byte & 0b00011100) >> 2 {
                    0 => AddressingMode::Immediate,
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
                        0x00..=0x60 | 0xC0..=0xE0 => AddressingMode::Accumulator,
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

    fn fetch(&mut self) {
        let byte = self.interconnect.read_mem(self.reg_pc);

        if self.curr_inst_byte.is_some() {
            self.operands.push(byte);
        } else {
            self.curr_inst_byte = Some(byte);
        }
    }

    fn decode(&mut self) {
        if let Some(byte) = self.curr_inst_byte {
            let inst_type = self.get_inst_type(byte);
            let addr_mode = self.get_addr_mode(byte);
            self.curr_inst = Some(Instruction {
                inst_type: inst_type,
                addr_mode: addr_mode,
            });
        }
    }

    fn execute(&mut self) {
        if let Some(inst) = self.curr_inst.as_mut() {
            let num_of_operands = match inst.addr_mode {
                AddressingMode::Illegal => 0,
                AddressingMode::ZeroPageIndexedX => 1,
                AddressingMode::ZeroPageIndexedY => 1,
                AddressingMode::AbsoluteIndexedX => 1,
                AddressingMode::AbsoluteIndexedY => 1,
                AddressingMode::IndexedIndirect => 1,
                AddressingMode::IndirectIndexed => 1,
                AddressingMode::Implicit => 0,
                AddressingMode::Accumulator => 0,
                AddressingMode::Immediate => 1,
                AddressingMode::ZeroPage => 1,
                AddressingMode::Absolute => 2,
                AddressingMode::Relative => 1,
                AddressingMode::Indirect => 1,
            };
            if self.operands.len() == num_of_operands {
                let address = match inst.addr_mode {
                    AddressingMode::Illegal => 0,
                    AddressingMode::ZeroPageIndexedX => {
                        self.operands[0].wrapping_add(self.reg_x) as u16
                    }
                    AddressingMode::ZeroPageIndexedY => {
                        self.operands[0].wrapping_add(self.reg_y) as u16
                    }
                    AddressingMode::AbsoluteIndexedX => {
                        (self.operands[1] as u16) << 8 + self.operands[0] + self.reg_x
                    }
                    AddressingMode::AbsoluteIndexedY => {
                        (self.operands[1] as u16) << 8 + self.operands[0] + self.reg_y
                    }
                    AddressingMode::IndexedIndirect => 1,
                    AddressingMode::IndirectIndexed => 1,
                    AddressingMode::Implicit => 0,
                    AddressingMode::Accumulator => self.reg_a as u16,
                    AddressingMode::Immediate => 1,
                    AddressingMode::ZeroPage => 1,
                    AddressingMode::Absolute => (self.operands[1] as u16) << 8 + self.operands[0],
                    AddressingMode::Relative => {
                        (self.reg_pc as i32 + self.operands[0] as i8 as i32) as u16
                    }
                    AddressingMode::Indirect => 1, // TODO implement
                };

                match inst.inst_type {
                    InstructionType::Illegal => {
                        println!("Illegal instruction {:?}", inst)
                    }
                    InstructionType::ADC => {}
                    InstructionType::AND => {}
                    InstructionType::ASL => {}
                    InstructionType::BCC => {}
                    InstructionType::BCS => {}
                    InstructionType::BEQ => {}
                    InstructionType::BIT => {}
                    InstructionType::BMI => {}
                    InstructionType::BNE => {}
                    InstructionType::BPL => {}
                    InstructionType::BRK => {}
                    InstructionType::BVC => {}
                    InstructionType::BVS => {}
                    InstructionType::CLC => {}
                    InstructionType::CLD => {}
                    InstructionType::CLI => {}
                    InstructionType::CLV => {}
                    InstructionType::CMP => {}
                    InstructionType::CPX => {}
                    InstructionType::CPY => {}
                    InstructionType::DEC => {}
                    InstructionType::DEX => {}
                    InstructionType::DEY => {}
                    InstructionType::EOR => {}
                    InstructionType::INC => {}
                    InstructionType::INX => {}
                    InstructionType::INY => {}
                    InstructionType::JMP => {}
                    InstructionType::JSR => {}
                    InstructionType::LDA => {}
                    InstructionType::LDX => {}
                    InstructionType::LDY => {}
                    InstructionType::LSR => {}
                    InstructionType::NOP => {}
                    InstructionType::ORA => {}
                    InstructionType::PHA => {}
                    InstructionType::PHP => {}
                    InstructionType::PLA => {}
                    InstructionType::PLP => {}
                    InstructionType::ROL => {}
                    InstructionType::ROR => {}
                    InstructionType::RTI => {}
                    InstructionType::RTS => {}
                    InstructionType::SBC => {}
                    InstructionType::SEC => {}
                    InstructionType::SED => {}
                    InstructionType::SEI => {}
                    InstructionType::STA => {}
                    InstructionType::STX => {}
                    InstructionType::STY => {}
                    InstructionType::TAX => {}
                    InstructionType::TAY => {}
                    InstructionType::TSX => {}
                    InstructionType::TXA => {}
                    InstructionType::TXS => {}
                    InstructionType::TYA => {}
                }
                if self.operands.len() == 0 {
                    // Add extra cycle because the minimum is 2
                    self.cycle += 1;
                }
                // TODO add cycles depending on instruction/addressing mode

                self.curr_inst_byte = None;
                self.curr_inst = None;
            }
        }

        self.reg_pc += 1;
    }
}

impl Powerable for CPU {
    fn power_on(&mut self) {
        self.interconnect.power_on();

        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.reg_pc = 0xFFFC;
        self.reg_s = 0xFD;
        self.reg_c = 0;
        self.reg_z = 0;
        self.reg_i = 1;
        self.reg_d = 0;
        self.reg_v = 0;
        self.reg_n = 0;

        self.operands = vec![];
        self.cycle = 0;
    }
    fn reset(&mut self) {
        self.interconnect.reset();

        self.reg_pc = 0xFFFC;
        self.reg_s -= 3;
        self.reg_i = 1;

        self.operands = vec![];
        self.cycle = 0;
    }
}
