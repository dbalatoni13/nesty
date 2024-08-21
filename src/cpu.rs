use crate::interconnect::Interconnect;
use crate::nes::Powerable;
use crate::utils::{build_u16, get_lsb, get_msb};
use bitfield_struct::bitfield;
//use enum_map::{enum_map, Enum, EnumMap};
//use once_cell::sync::Lazy;

const RESET_VECTOR_ADDR: u16 = 0xFFFC;
const IRQ_VECTOR_ADDR: u16 = 0xFFFE;

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

fn set_register_with_flags(reg: &mut u8, status: &mut Status, value: u8) {
    // TODO maybe extract the first line so that this function becomes more universal
    *reg = value;
    status.set_zero(*reg == 0);
    status.set_negative(*reg & 0b10000000 != 0);
}

#[bitfield(u8)]
struct Status {
    #[bits(1)]
    carry: bool,
    #[bits(1)]
    zero: bool,
    #[bits(1)]
    interrupt_disable: bool,
    #[bits(1)]
    decimal: bool,
    #[bits(1)]
    b: bool,
    #[bits(1)]
    one: bool,
    #[bits(1)]
    overflow: bool,
    #[bits(1)]
    negative: bool,
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
    status: Status,

    ic: Interconnect,
}

impl CPU {
    pub fn load_rom(&mut self, ines: Vec<u8>) {
        self.ic.load_rom(ines)
    }

    pub fn run(&mut self) {
        for _ in 0..500 {
            self.do_cycle();
        }
    }

    fn do_cycle(&mut self) {
        if self.cycle > 0 {
            self.fetch();
            self.decode();
            self.execute();
        } else {
            //self.reg_pc = self.ic.read_mem_word(self.reg_pc); // jump to start of code
            self.reg_pc = 0xC000;
            println!("Started execution at {:#02X}", self.reg_pc);
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
                    0xC0 => InstructionType::DEC,
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

    fn get_addr_mode(&self, byte: u8) -> AddressingMode {
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

    fn push_to_stack(&mut self, value: u8) {
        self.ic.write_mem(0x100 + self.reg_s as u16, value);
        self.reg_s -= 1;
    }

    fn pull_from_stack(&mut self) -> u8 {
        self.reg_s += 1;
        self.ic.read_mem(0x100 + self.reg_s as u16)
    }

    fn push_to_stack_word(&mut self, value: u16) {
        self.push_to_stack(get_msb(value));
        self.push_to_stack(get_lsb(value));
    }

    fn pull_from_stack_word(&mut self) -> u16 {
        let lsb = self.pull_from_stack();
        let msb = self.pull_from_stack();

        build_u16(msb, lsb)
    }

    fn fetch(&mut self) {
        let byte = self.ic.read_mem(self.reg_pc);

        if self.curr_inst.is_some() {
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
            self.curr_inst_byte = None;
        }
    }

    fn execute(&mut self) {
        let Some(inst) = self.curr_inst.as_mut() else {
            self.reg_pc += 1;
            return;
        };

        let num_of_operands = match inst.addr_mode {
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
        };
        if self.operands.len() != num_of_operands {
            self.reg_pc += 1;
            return;
        }

        let mut jump_occured = false;

        let operand = match inst.addr_mode {
            AddressingMode::Illegal => 0,
            AddressingMode::ZeroPageIndexedX => self.operands[0].wrapping_add(self.reg_x) as u16,
            AddressingMode::ZeroPageIndexedY => self.operands[0].wrapping_add(self.reg_y) as u16,
            AddressingMode::AbsoluteIndexedX => {
                build_u16(self.operands[1], self.operands[0] + self.reg_x)
            }
            AddressingMode::AbsoluteIndexedY => {
                build_u16(self.operands[1], self.operands[0] + self.reg_y)
            }
            AddressingMode::IndexedIndirect => build_u16(
                // TODO replace with read_mem_word function?
                self.ic
                    .read_mem(self.operands[0].wrapping_add(self.reg_x).wrapping_add(1) as u16),
                self.ic
                    .read_mem(self.operands[0].wrapping_add(self.reg_x) as u16),
            ),
            AddressingMode::IndirectIndexed => {
                self.ic.read_mem(self.operands[0] as u16) as u16
                    + self.ic.read_mem(self.operands[0].wrapping_add(1) as u16) as u16 * 256
                    + self.reg_y as u16
            }
            AddressingMode::Implicit => 0,
            AddressingMode::Accumulator => self.reg_a as u16,
            AddressingMode::Immediate => self.operands[0] as u16,
            AddressingMode::ZeroPage => self.operands[0] as u16,
            AddressingMode::Absolute => build_u16(self.operands[1], self.operands[0]),
            AddressingMode::Relative => (self.reg_pc as i32 + self.operands[0] as i8 as i32) as u16,
            AddressingMode::Indirect => self
                .ic
                .read_mem_word(build_u16(self.operands[1], self.operands[0])),
        };
        let value = match inst.addr_mode {
            AddressingMode::Immediate => operand as u8,
            _ => self.ic.read_mem(operand),
        };
        println!(
            "{:04X} {:?} val: {:02X} addr: {:02X}                A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
            self.reg_pc - num_of_operands as u16,
            inst,
            value,
            operand,
            self.reg_a,
            self.reg_x,
            self.reg_y,
            self.status.into_bits(),
            self.reg_s,
            self.cycle
        );
        match inst.inst_type {
            InstructionType::Illegal => {
                //println!("Illegal instruction {:?}", inst);
            }
            InstructionType::ADC => {
                // TODO does this work? we need to consider 'decimal mode' too
                let old_bit_7 = self.reg_a & 0b10000000 != 0;
                let (sum, overflow) =
                    (self.reg_a as i8).overflowing_add(value as i8 + self.status.carry() as i8);
                self.reg_a = sum as u8;
                self.status.set_carry(overflow);
                let new_bit_7 = self.reg_a & 0b10000000 != 0;
                self.status.set_overflow(old_bit_7 ^ new_bit_7);
                self.status.set_zero(self.reg_a == 0);
                self.status.set_negative(new_bit_7);
            }
            InstructionType::AND => {
                let anded = self.reg_a & value;
                set_register_with_flags(&mut self.reg_a, &mut self.status, anded);
            }
            InstructionType::ASL => match inst.addr_mode {
                AddressingMode::Accumulator => {
                    let old_bit_7 = self.reg_a & 0b10000000 != 0;
                    self.reg_a = self.reg_a.wrapping_shl(1);
                    self.status.set_carry(old_bit_7);
                    self.status.set_zero(self.reg_a == 0);
                    self.status.set_negative(self.reg_a & 0b10000000 != 0);
                }
                _ => {
                    let old_bit_7 = value & 0b10000000 != 0;
                    let rot = value.wrapping_shl(1);
                    self.ic.write_mem(operand, rot);
                    self.status.set_carry(old_bit_7);
                    self.status.set_zero(self.reg_a == 0); // TODO is this right?
                    self.status.set_negative(rot & 0b10000000 != 0);
                }
            },
            InstructionType::BCC => {
                if !self.status.carry() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::BCS => {
                if self.status.carry() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::BEQ => {
                if self.status.zero() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::BIT => {
                self.status.set_zero(self.reg_a & value == 0);
                self.status.set_overflow(value & 0b01000000 != 0);
                self.status.set_negative(value & 0b10000000 != 0);
            }
            InstructionType::BMI => {
                if self.status.negative() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::BNE => {
                if !self.status.zero() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::BPL => {
                if !self.status.negative() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::BRK => {
                self.push_to_stack_word(self.reg_pc);
                self.push_to_stack(self.status.into_bits());
                self.reg_pc = self.ic.read_mem_word(RESET_VECTOR_ADDR);
                self.status.set_b(true);
                self.cycle += 6;
            }
            InstructionType::BVC => {
                if !self.status.overflow() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::BVS => {
                if self.status.overflow() {
                    self.reg_pc = operand;
                    jump_occured = true;
                }
            }
            InstructionType::CLC => {
                self.status.set_carry(false);
            }
            InstructionType::CLD => {
                self.status.set_decimal(false);
            }
            InstructionType::CLI => {
                self.status.set_decimal(false);
            }
            InstructionType::CLV => {
                self.status.set_overflow(false);
            }
            InstructionType::CMP => {
                let res = self.reg_a.wrapping_sub(value);
                self.status.set_carry(self.reg_a >= value);
                self.status.set_zero(res == 0);
                self.status.set_negative(res & 0b10000000 != 0);
            }
            InstructionType::CPX => {
                let res = self.reg_x.wrapping_sub(value);
                self.status.set_carry(self.reg_x >= value);
                self.status.set_zero(res == 0);
                self.status.set_negative(res & 0b10000000 != 0);
            }
            InstructionType::CPY => {
                let res = self.reg_y.wrapping_sub(value);
                self.status.set_carry(self.reg_y >= value);
                self.status.set_zero(res == 0);
                self.status.set_negative(res & 0b10000000 != 0);
            }
            InstructionType::DEC => {
                let sub = value.wrapping_sub(1);
                self.ic.write_mem(operand, sub);
                self.status.set_zero(sub == 0);
                self.status.set_negative(sub & 0b10000000 != 0);
            }
            InstructionType::DEX => {
                let sub = self.reg_y.wrapping_sub(1);
                set_register_with_flags(&mut self.reg_x, &mut self.status, sub);
            }
            InstructionType::DEY => {
                let sub = self.reg_y.wrapping_sub(1);
                set_register_with_flags(&mut self.reg_y, &mut self.status, sub);
            }
            InstructionType::EOR => {
                let eorred = self.reg_a ^ value;
                set_register_with_flags(&mut self.reg_a, &mut self.status, eorred);
            }
            InstructionType::INC => {
                let inc = value.wrapping_add(1);
                self.ic.write_mem(operand, value.wrapping_add(1));
                self.status.set_zero(inc == 0);
                self.status.set_negative(inc & 0b10000000 != 0);
            }
            InstructionType::INX => {
                let inc = self.reg_x.wrapping_add(1);
                set_register_with_flags(&mut self.reg_x, &mut self.status, inc);
            }
            InstructionType::INY => {
                let inc = self.reg_y.wrapping_add(1);
                set_register_with_flags(&mut self.reg_y, &mut self.status, inc);
            }
            InstructionType::JMP => {
                self.reg_pc = operand;
                jump_occured = true;
                println!("Jumped to {:#02X}", operand);
            }
            InstructionType::JSR => {
                self.push_to_stack_word(self.reg_pc);
                self.reg_pc = operand;
                jump_occured = true;
                println!("Jumped to {:#02X}", operand);
            }
            InstructionType::LDA => {
                set_register_with_flags(&mut self.reg_a, &mut self.status, value);
            }
            InstructionType::LDX => {
                set_register_with_flags(&mut self.reg_x, &mut self.status, value);
            }
            InstructionType::LDY => {
                set_register_with_flags(&mut self.reg_y, &mut self.status, value);
            }
            InstructionType::LSR => match inst.addr_mode {
                AddressingMode::Accumulator => {
                    let old_bit_0 = self.reg_a & 1 != 0;
                    self.reg_a = self.reg_a.wrapping_shr(1);
                    self.status.set_carry(old_bit_0);
                    self.status.set_zero(self.reg_a == 0);
                    self.status.set_negative(self.reg_a & 0b10000000 != 0);
                }
                _ => {
                    let old_bit_0 = value & 1 != 0;
                    let rot = value.wrapping_shr(1);
                    self.ic.write_mem(operand, rot);
                    self.status.set_carry(old_bit_0);
                    self.status.set_zero(rot == 0); // TODO is this right?
                    self.status.set_negative(rot & 0b10000000 != 0);
                }
            },
            InstructionType::NOP => {}
            InstructionType::ORA => {
                let orred = self.reg_a | value;
                set_register_with_flags(&mut self.reg_a, &mut self.status, orred);
            }
            InstructionType::PHA => {
                self.push_to_stack(self.reg_a);
            }
            InstructionType::PHP => {
                self.push_to_stack(self.status.into_bits());
            }
            InstructionType::PLA => {
                let s = self.pull_from_stack();
                set_register_with_flags(&mut self.reg_a, &mut self.status, s);
            }
            InstructionType::PLP => {
                let s = self.pull_from_stack();
                self.status = Status::from_bits(s);
            }
            InstructionType::ROL => match inst.addr_mode {
                AddressingMode::Accumulator => {
                    let old_bit_7 = self.reg_a & 0b10000000 != 0;
                    self.reg_a = self.reg_a.wrapping_shl(1) | self.status.carry() as u8;
                    self.status.set_carry(old_bit_7);
                    self.status.set_zero(self.reg_a == 0);
                    self.status.set_negative(self.reg_a & 0b10000000 != 0);
                }
                _ => {
                    let old_bit_7 = value & 0b10000000 != 0;
                    let rot = value.wrapping_shl(1) | self.status.carry() as u8;
                    self.ic.write_mem(operand, rot);
                    self.status.set_carry(old_bit_7);
                    self.status.set_zero(self.reg_a == 0); // TODO is this right?
                    self.status.set_negative(rot & 0b10000000 != 0);
                }
            },
            InstructionType::ROR => match inst.addr_mode {
                AddressingMode::Accumulator => {
                    let old_bit_0 = self.reg_a & 1 != 0;
                    self.reg_a =
                        self.reg_a.wrapping_shl(1) | (self.status.carry() as u8).wrapping_shl(7);
                    self.status.set_carry(old_bit_0);
                    self.status.set_zero(self.reg_a == 0);
                    self.status.set_negative(self.reg_a & 0b10000000 != 0);
                }
                _ => {
                    let old_bit_0 = value & 1 != 0;
                    let rot = value.wrapping_shr(1) | (self.status.carry() as u8).wrapping_shl(7);
                    self.ic.write_mem(operand, rot);
                    self.status.set_carry(old_bit_0);
                    self.status.set_zero(self.reg_a == 0); // TODO is this right?
                    self.status.set_negative(rot & 0b10000000 != 0);
                }
            },
            InstructionType::RTI => {
                self.status = Status::from_bits(self.pull_from_stack());
                self.reg_pc = self.pull_from_stack_word();
                self.cycle += 5;
            }
            InstructionType::RTS => {
                self.reg_pc = self.pull_from_stack_word() + 1;
                jump_occured = true;
                println!("Jumped to {:#02X}", self.reg_pc);
                self.cycle += 5;
            }
            InstructionType::SBC => {
                // TODO does this work? we need to consider 'decimal mode' too
                let old_bit_7 = self.reg_a & 0b10000000 != 0;
                let (sum, overflow) =
                    (self.reg_a as i8).overflowing_sub(value as i8 + self.status.carry() as i8);
                self.reg_a = sum as u8;
                self.status.set_carry(!overflow);
                let new_bit_7 = self.reg_a & 0b10000000 != 0;
                self.status.set_overflow(old_bit_7 ^ new_bit_7);
                self.status.set_zero(self.reg_a == 0);
                self.status.set_negative(new_bit_7);
            }
            InstructionType::SEC => {
                self.status.set_carry(true);
            }
            InstructionType::SED => {
                self.status.set_decimal(true);
            }
            InstructionType::SEI => {
                self.status.set_interrupt_disable(true);
            }
            InstructionType::STA => {
                self.ic.write_mem(operand, self.reg_a);
            }
            InstructionType::STX => {
                self.ic.write_mem(operand, self.reg_x);
            }
            InstructionType::STY => {
                self.ic.write_mem(operand, self.reg_y);
            }
            InstructionType::TAX => {
                set_register_with_flags(&mut self.reg_x, &mut self.status, self.reg_a);
            }
            InstructionType::TAY => {
                set_register_with_flags(&mut self.reg_y, &mut self.status, self.reg_a);
            }
            InstructionType::TSX => {
                set_register_with_flags(&mut self.reg_x, &mut self.status, self.reg_s);
            }
            InstructionType::TXA => {
                set_register_with_flags(&mut self.reg_a, &mut self.status, self.reg_x);
            }
            InstructionType::TXS => {
                set_register_with_flags(&mut self.reg_s, &mut self.status, self.reg_x);
            }
            InstructionType::TYA => {
                set_register_with_flags(&mut self.reg_a, &mut self.status, self.reg_y);
            }
        }
        if self.operands.len() == 0 {
            // Add extra cycle because the minimum is 2
            self.cycle += 1;
        }
        // TODO add cycles depending on instruction/addressing mode

        self.curr_inst = None;
        self.operands.clear();

        if !jump_occured {
            self.reg_pc += 1;
        }
    }
}

impl Powerable for CPU {
    fn power_on(&mut self) {
        self.ic.power_on();

        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.reg_pc = RESET_VECTOR_ADDR;
        self.reg_s = 0xFD;
        self.status.set_carry(false);
        self.status.set_zero(false);
        self.status.set_interrupt_disable(true);
        self.status.set_decimal(false);
        self.status.set_one(true);
        self.status.set_overflow(false);
        self.status.set_negative(false);

        self.operands = vec![];
        self.cycle = 0;
    }
    fn reset(&mut self) {
        self.ic.reset();

        self.reg_pc = RESET_VECTOR_ADDR;
        self.reg_s -= 3;
        self.status.set_interrupt_disable(true);

        self.operands = vec![];
        self.cycle = 0;
    }
}
