use crate::interconnect::Interconnect;
use crate::nes::Powerable;

enum InstructionType {
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

enum AddressingMode {
    None,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    IndexedIndirect,
    IndirectIndexed,
}

struct Instruction {
    inst_type: InstructionType,
    add_mode: AddressingMode,
}

#[derive(Default)]
pub struct CPU {
    cycle: u64,

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
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.interconnect.load_rom(rom)
    }

    pub fn run(&mut self) {
        for _ in 0..10 {
            self.do_cycle();
        }
    }

    fn do_cycle(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
    }

    fn fetch(&self) -> Instruction {}

    fn decode(&self) {}

    fn execute(&self) {}
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
    }
    fn reset(&mut self) {
        self.interconnect.reset();

        self.reg_pc = 0xFFFC;
        self.reg_s -= 3;
        self.reg_i = 1;
    }
}
