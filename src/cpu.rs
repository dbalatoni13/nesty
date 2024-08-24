use crate::instructions::addressing::MemoryOp;
use crate::instructions::{
    addressing, get_addr_mode, get_inst_type, get_num_of_operands, logic, AddressingMode,
    Instruction, InstructionType,
};
use crate::interconnect::Interconnect;
use crate::nes::Powerable;
use crate::utils::{build_u16, get_lsb, get_msb};
use bitfield_struct::bitfield;
use std::collections::VecDeque;

pub const RESET_VECTOR_ADDR: u16 = 0xFFFC;
pub const IRQ_VECTOR_ADDR: u16 = 0xFFFE;

#[bitfield(u8)]
pub struct Status {
    #[bits(1)]
    pub carry: bool,
    #[bits(1)]
    pub zero: bool,
    #[bits(1)]
    pub interrupt_disable: bool,
    #[bits(1)]
    pub decimal: bool,
    #[bits(1)]
    pub b: bool,
    #[bits(1)]
    pub one: bool,
    #[bits(1)]
    pub overflow: bool,
    #[bits(1)]
    pub negative: bool,
}

#[derive(Default)]
pub struct CPU {
    pub cycle: u64,
    cycle_debug: u64,
    curr_inst_byte: Option<u8>,
    pub curr_inst: Option<Instruction>,
    pub operands: Vec<u8>,
    pub num_operands: usize,
    pub value: u8,
    pub addr: u16,
    pub write: u8,
    pub inst_queue: VecDeque<(fn(&mut CPU), u32)>,
    printed: bool,

    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub reg_pc: u16,
    pub reg_s: u8,
    pub status: Status,

    pub ic: Interconnect,
}

impl CPU {
    pub fn load_rom(&mut self, ines: Vec<u8>) {
        self.ic.load_rom(ines)
    }

    pub fn run(&mut self) {
        for _ in 0..2000 {
            self.do_cycle();
        }
    }

    fn do_cycle(&mut self) {
        if self.cycle > 0 {
            self.fetch();
            let only_free = self.decode();
            self.execute(only_free);
        } else {
            //self.reg_pc = self.ic.read_mem_word(self.reg_pc); // jump to start of code
            self.reg_pc = 0xC000;
            println!("Started execution at {:#02X}", self.reg_pc);
            self.cycle = 6;
        }

        self.cycle += 1;
    }

    pub fn push_to_stack(&mut self, value: u8) {
        self.ic.write_mem(0x100 + self.reg_s as u16, value);
        self.reg_s -= 1;
    }

    pub fn pull_from_stack(&mut self) -> u8 {
        self.reg_s += 1;
        self.ic.read_mem(0x100 + self.reg_s as u16)
    }

    fn fetch(&mut self) {
        if self.inst_queue.len() > 0 {
            return;
        }
        let byte = self.ic.read_mem(self.reg_pc);
        if self.curr_inst.is_some() && self.operands.len() < self.num_operands {
            self.operands.push(byte);
            self.reg_pc += 1;
        } else {
            self.curr_inst_byte = Some(byte);
            self.cycle_debug = self.cycle;
            self.reg_pc += 1;
        }
    }

    fn decode(&mut self) -> bool {
        let Some(inst_byte) = self.curr_inst_byte else {
            return false;
        };

        if self.operands.len() == 0 {
            let inst_type = get_inst_type(inst_byte);
            let addr_mode = get_addr_mode(inst_byte);
            let inst = Instruction {
                inst_type: inst_type,
                addr_mode: addr_mode,
            };
            self.num_operands = get_num_of_operands(&inst.addr_mode);
            self.curr_inst = Some(inst);
            if self.num_operands > 0 {
                return false;
            }
        }

        if self.operands.len() < self.num_operands {
            return false;
        }

        self.curr_inst_byte = None;

        match self.curr_inst.as_mut().unwrap().inst_type {
            InstructionType::Illegal => {
                //println!("Illegal instruction {:?}", inst);
            }
            InstructionType::ADC => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::adc_1, 0));
            }
            InstructionType::AND => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::and_1, 0));
            }
            InstructionType::ASL => {
                self.inst_queue.push_back((logic::asl_1, 0));
            }
            InstructionType::BCC => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bcc_1, 0));
            }
            InstructionType::BCS => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bcs_1, 0));
            }
            InstructionType::BEQ => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::beq_1, 0));
            }
            InstructionType::BIT => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bit_1, 0));
            }
            InstructionType::BMI => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bmi_1, 0));
            }
            InstructionType::BNE => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bne_1, 0));
            }
            InstructionType::BPL => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bpl_1, 0));
            }
            InstructionType::BRK => {
                // TODO why only 5 cycles instead of 7?
                self.inst_queue.push_back((logic::brk_1, 1));
                self.inst_queue.push_back((logic::brk_2, 1));
                self.inst_queue.push_back((logic::brk_3, 1));
                self.inst_queue.push_back((logic::brk_4, 1));
            }
            InstructionType::BVC => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bvc_1, 0));
            }
            InstructionType::BVS => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::bvs_1, 0));
            }
            InstructionType::CLC => {
                self.inst_queue.push_back((logic::clc_1, 0));
            }
            InstructionType::CLD => {
                self.inst_queue.push_back((logic::cld_1, 0));
            }
            InstructionType::CLI => {
                self.inst_queue.push_back((logic::cli_1, 0));
            }
            InstructionType::CLV => {
                self.inst_queue.push_back((logic::clv_1, 0));
            }
            InstructionType::CMP => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::cmp_1, 0));
            }
            InstructionType::CPX => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::cpx_1, 0));
            }
            InstructionType::CPY => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::cpy_1, 0));
            }
            InstructionType::DEC => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::dec_1, 1));
                self.inst_queue.push_back((logic::dec_2, 1));
            }
            InstructionType::DEX => {
                self.inst_queue.push_back((logic::dex_1, 0));
            }
            InstructionType::DEY => {
                self.inst_queue.push_back((logic::dey_1, 0));
            }
            InstructionType::EOR => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::eor_1, 0));
            }
            InstructionType::INC => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::inc_1, 1));
                self.inst_queue.push_back((logic::inc_2, 1));
            }
            InstructionType::INX => {
                self.inst_queue.push_back((logic::inx_1, 0));
            }
            InstructionType::INY => {
                self.inst_queue.push_back((logic::iny_1, 0));
            }
            InstructionType::JMP => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::jmp_1, 0));
            }
            InstructionType::JSR => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::jsr_1, 1));
                self.inst_queue.push_back((logic::jsr_2, 1));
            }
            InstructionType::LDA => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::lda_1, 0));
            }
            InstructionType::LDX => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::ldx_1, 0));
            }
            InstructionType::LDY => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::ldy_1, 0));
            }
            InstructionType::LSR => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::lsr_1, 0));
            }
            InstructionType::NOP => {
                // this already gets handled by the fact that the instruction has no operands
            }
            InstructionType::ORA => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::ora_1, 0));
            }
            InstructionType::PHA => {
                self.inst_queue.push_back((logic::pha_1, 1));
            }
            InstructionType::PHP => {
                self.inst_queue.push_back((logic::php_1, 1));
            }
            InstructionType::PLA => {
                self.inst_queue.push_back((logic::pla_1, 1));
                self.inst_queue.push_back((logic::nop, 1));
            }
            InstructionType::PLP => {
                self.inst_queue.push_back((logic::plp_1, 1));
                self.inst_queue.push_back((logic::nop, 1));
            }
            InstructionType::ROL => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::rol_1, 0));
            }
            InstructionType::ROR => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::ror_1, 0));
            }
            InstructionType::RTI => {
                self.inst_queue.push_back((logic::rti_1, 1));
                self.inst_queue.push_back((logic::rti_2, 1));
                self.inst_queue.push_back((logic::rti_3, 1));
            }
            InstructionType::RTS => {
                self.inst_queue.push_back((logic::rts_1, 1));
                self.inst_queue.push_back((logic::nop, 1));
                self.inst_queue.push_back((logic::rts_2, 1));
                self.inst_queue.push_back((logic::nop, 1));
            }
            InstructionType::SBC => {
                addressing::queue_push_memory_op(self, MemoryOp::Read);
                self.inst_queue.push_back((logic::sbc_1, 0));
            }
            InstructionType::SEC => {
                self.inst_queue.push_back((logic::sec_1, 0));
            }
            InstructionType::SED => {
                self.inst_queue.push_back((logic::sed_1, 0));
            }
            InstructionType::SEI => {
                self.inst_queue.push_back((logic::sei_1, 0));
            }
            InstructionType::STA => {
                self.write = self.reg_a;
                addressing::queue_push_memory_op(self, MemoryOp::Write);
            }
            InstructionType::STX => {
                self.write = self.reg_x;
                addressing::queue_push_memory_op(self, MemoryOp::Write);
            }
            InstructionType::STY => {
                self.write = self.reg_y;
                addressing::queue_push_memory_op(self, MemoryOp::Write);
            }
            InstructionType::TAX => {
                self.inst_queue.push_back((logic::tax_1, 0));
            }
            InstructionType::TAY => {
                self.inst_queue.push_back((logic::tay_1, 0));
            }
            InstructionType::TSX => {
                self.inst_queue.push_back((logic::tsx_1, 0));
            }
            InstructionType::TXA => {
                self.inst_queue.push_back((logic::txa_1, 0));
            }
            InstructionType::TXS => {
                self.inst_queue.push_back((logic::txs_1, 0));
            }
            InstructionType::TYA => {
                self.inst_queue.push_back((logic::tya_1, 0));
            }
        }
        if self.num_operands == 0 {
            self.inst_queue.push_back((logic::nop, 1));
        }
        true
    }

    fn execute(&mut self, only_free: bool) {
        if self.operands.len() < self.num_operands {
            return;
        }
        if !self.printed {
            println!(
                "{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
                self.reg_pc - 1 - self.num_operands as u16,
                self.reg_a,
                self.reg_x,
                self.reg_y,
                self.status.into_bits(),
                self.reg_s,
                self.cycle_debug
            );
            self.printed = true;

            // println!(
            // "{:04X} {:?} val: {:02X} addr: {:02X}                A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
            // self.reg_pc - 1 - self.num_operands as u16,
            // self.curr_inst.as_mut().unwrap(),
            // self.value,
            // self.addr,
            // self.reg_a,
            // self.reg_x,
            // self.reg_y,
            // self.status.into_bits(),
            // self.reg_s,
            // self.cycle_debug
            // );
        }
        let mut total_cost = 0;
        let max = match only_free {
            true => 1,
            false => 2,
        };
        while self.inst_queue.front().is_some()
            && total_cost + self.inst_queue.front().unwrap().1 < max
        {
            let (func, cost) = self
                .inst_queue
                .pop_front()
                .expect("Queue empty after fetch");
            func(self);
            total_cost += cost;
        }

        if self.inst_queue.len() == 0 {
            self.printed = false;
            self.curr_inst = None;
            self.operands.clear();
            return;
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
        self.inst_queue = VecDeque::new();
        self.cycle = 0;
    }
    fn reset(&mut self) {
        self.ic.reset();

        self.reg_pc = RESET_VECTOR_ADDR;
        self.reg_s -= 3;
        self.status.set_interrupt_disable(true);

        self.operands = vec![];
        self.inst_queue = VecDeque::new();
        self.cycle = 0;
    }
}
