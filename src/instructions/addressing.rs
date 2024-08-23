use crate::{cpu::CPU, utils::build_u16};

use super::{AddressingMode, InstructionType};

pub enum MemoryOp {
    Read,
    Write,
}

pub fn read_mem(cpu: &mut CPU) {
    cpu.ic.read_mem(cpu.addr);
}

pub fn write_mem(cpu: &mut CPU) {
    cpu.ic.write_mem(cpu.addr, cpu.write);
}

pub fn queue_push_memory_op(cpu: &mut CPU, op: MemoryOp) {
    let Some(inst) = cpu.curr_inst.as_mut() else {
        panic!("No instruction");
    };

    match inst.addr_mode {
        AddressingMode::Illegal => panic!("Illegal addressing mode"),
        AddressingMode::ZeroPageIndexedX => {
            cpu.addr = cpu.operands[0].wrapping_add(cpu.reg_x) as u16;
        }
        AddressingMode::ZeroPageIndexedY => {
            cpu.addr = cpu.operands[0].wrapping_add(cpu.reg_y) as u16;
        }
        AddressingMode::AbsoluteIndexedX => {
            cpu.addr = build_u16(cpu.operands[1], cpu.operands[0] + cpu.reg_x);
        }
        AddressingMode::AbsoluteIndexedY => {
            cpu.addr = build_u16(cpu.operands[1], cpu.operands[0] + cpu.reg_y);
        }
        AddressingMode::IndexedIndirect => {
            cpu.addr = build_u16(
                cpu.ic
                    .read_mem(cpu.operands[0].wrapping_add(cpu.reg_x).wrapping_add(1) as u16),
                cpu.ic
                    .read_mem(cpu.operands[0].wrapping_add(cpu.reg_x) as u16),
            )
        }
        AddressingMode::IndirectIndexed => {
            cpu.addr = cpu.ic.read_mem(cpu.operands[0] as u16) as u16
                + cpu.ic.read_mem(cpu.operands[0].wrapping_add(1) as u16) as u16 * 256
                + cpu.reg_y as u16
        }
        AddressingMode::Implicit => {}
        AddressingMode::Accumulator => {
            cpu.value = cpu.reg_a;
        }
        AddressingMode::Immediate => {
            cpu.value = cpu.operands[0];
        }
        AddressingMode::ZeroPage => {
            cpu.addr = cpu.operands[0] as u16;
        }
        AddressingMode::Absolute => {
            cpu.addr = build_u16(cpu.operands[1], cpu.operands[0]);
        }
        AddressingMode::Relative => {
            cpu.addr = (cpu.reg_pc as i32 + cpu.operands[0] as i8 as i32) as u16;
        }
        AddressingMode::Indirect => {
            cpu.addr = cpu
                .ic
                .read_mem_word(build_u16(cpu.operands[1], cpu.operands[0]));
        }
    };
    if matches!(op, MemoryOp::Read) {
        match inst.addr_mode {
            AddressingMode::Immediate
            | AddressingMode::Accumulator
            | AddressingMode::Implicit
            | AddressingMode::Relative => {}
            _ => {
                if !matches!(inst.inst_type, InstructionType::JMP) {
                    cpu.inst_queue.push_back((read_mem, 1));
                }
            }
        };
    } else {
        cpu.inst_queue.push_back((write_mem, 1));
    }
}
