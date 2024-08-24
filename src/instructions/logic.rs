use crate::{
    cpu::{Status, CPU, IRQ_VECTOR_ADDR},
    utils::{get_lsb, get_msb},
};

use super::AddressingMode;

fn set_register_with_flags(reg: &mut u8, status: &mut Status, value: u8) {
    // TODO maybe extract the first line so that this function becomes more universal
    *reg = value;
    status.set_zero(*reg == 0);
    status.set_negative(*reg & 0b10000000 != 0);
}

fn handle_successful_branching(cpu: &mut CPU) {
    cpu.inst_queue.push_back((nop, 1));
    if cpu.reg_pc & 0xFF00 != cpu.addr & 0xFF00 {
        // new page
        cpu.inst_queue.push_back((nop, 1));
    }
    cpu.reg_pc = cpu.addr;
}

pub fn adc_1(cpu: &mut CPU) {
    let carry_6 =
        (((cpu.reg_a & 0b01111111) + (cpu.value & 0b01111111) + cpu.status.carry() as u8)
            & 0b10000000)
            != 0;
    let (sum1, overflow1) = cpu.reg_a.overflowing_add(cpu.value);
    let (sum, overflow2) = sum1.overflowing_add(cpu.status.carry() as u8);
    let overflow = overflow1 | overflow2;

    cpu.status.set_carry(overflow);
    cpu.status.set_zero(sum == 0);
    cpu.status.set_overflow(carry_6 ^ overflow);
    cpu.status.set_negative(sum & 0b10000000 != 0);
    cpu.reg_a = sum as u8;
}

pub fn and_1(cpu: &mut CPU) {
    let anded = cpu.reg_a & cpu.value;
    set_register_with_flags(&mut cpu.reg_a, &mut cpu.status, anded);
}

pub fn asl_1(cpu: &mut CPU) {
    match cpu.curr_inst.as_mut().expect("No instruction").addr_mode {
        AddressingMode::Accumulator => {
            let old_bit_7 = cpu.reg_a & 0b10000000 != 0;
            cpu.reg_a = cpu.reg_a.wrapping_shl(1);
            cpu.status.set_carry(old_bit_7);
            cpu.status.set_zero(cpu.reg_a == 0);
            cpu.status.set_negative(cpu.reg_a & 0b10000000 != 0);
        }
        _ => {
            let old_bit_7 = cpu.value & 0b10000000 != 0;
            let rot = cpu.value.wrapping_shl(1);
            // TODO fix for cycle accuracy
            cpu.ic.write_mem(cpu.addr, rot);
            cpu.status.set_carry(old_bit_7);
            cpu.status.set_zero(cpu.reg_a == 0); // TODO is this right?
            cpu.status.set_negative(rot & 0b10000000 != 0);
        }
    }
}

pub fn bcc_1(cpu: &mut CPU) {
    if !cpu.status.carry() {
        handle_successful_branching(cpu);
    }
}

pub fn bcs_1(cpu: &mut CPU) {
    if cpu.status.carry() {
        handle_successful_branching(cpu);
    }
}

pub fn beq_1(cpu: &mut CPU) {
    if cpu.status.zero() {
        handle_successful_branching(cpu);
    }
}

pub fn bit_1(cpu: &mut CPU) {
    cpu.status.set_zero(cpu.reg_a & cpu.value == 0);
    cpu.status.set_overflow(cpu.value & 0b01000000 != 0);
    cpu.status.set_negative(cpu.value & 0b10000000 != 0);
}

pub fn bmi_1(cpu: &mut CPU) {
    if cpu.status.negative() {
        handle_successful_branching(cpu);
    }
}

pub fn bne_1(cpu: &mut CPU) {
    if !cpu.status.zero() {
        handle_successful_branching(cpu);
    }
}

pub fn bpl_1(cpu: &mut CPU) {
    if !cpu.status.negative() {
        handle_successful_branching(cpu);
    }
}

pub fn brk_1(cpu: &mut CPU) {
    cpu.status.set_b(true);
    cpu.push_to_stack(get_msb(cpu.reg_pc));
}

pub fn brk_2(cpu: &mut CPU) {
    cpu.push_to_stack(get_lsb(cpu.reg_pc));
}

pub fn brk_3(cpu: &mut CPU) {
    cpu.push_to_stack(cpu.status.into_bits());
}

pub fn brk_4(cpu: &mut CPU) {
    cpu.reg_pc = cpu.ic.read_mem_word(IRQ_VECTOR_ADDR);
}

pub fn bvc_1(cpu: &mut CPU) {
    if !cpu.status.overflow() {
        handle_successful_branching(cpu);
    }
}

pub fn bvs_1(cpu: &mut CPU) {
    if cpu.status.overflow() {
        handle_successful_branching(cpu);
    }
}

pub fn clc_1(cpu: &mut CPU) {
    cpu.status.set_carry(false);
}

pub fn cld_1(cpu: &mut CPU) {
    cpu.status.set_decimal(false);
}

pub fn cli_1(cpu: &mut CPU) {
    cpu.status.set_decimal(false);
}

pub fn clv_1(cpu: &mut CPU) {
    cpu.status.set_overflow(false);
}

pub fn cmp_1(cpu: &mut CPU) {
    let res = cpu.reg_a.wrapping_sub(cpu.value);
    cpu.status.set_carry(cpu.reg_a >= cpu.value);
    cpu.status.set_zero(res == 0);
    cpu.status.set_negative(res & 0b10000000 != 0);
}

pub fn cpx_1(cpu: &mut CPU) {
    let res = cpu.reg_x.wrapping_sub(cpu.value);
    cpu.status.set_carry(cpu.reg_x >= cpu.value);
    cpu.status.set_zero(res == 0);
    cpu.status.set_negative(res & 0b10000000 != 0);
}

pub fn cpy_1(cpu: &mut CPU) {
    let res = cpu.reg_y.wrapping_sub(cpu.value);
    cpu.status.set_carry(cpu.reg_y >= cpu.value);
    cpu.status.set_zero(res == 0);
    cpu.status.set_negative(res & 0b10000000 != 0);
}

pub fn dec_1(cpu: &mut CPU) {
    // Dummy write
    cpu.ic.write_mem(cpu.addr, cpu.value);
}

pub fn dec_2(cpu: &mut CPU) {
    let sub = cpu.value.wrapping_sub(1);
    cpu.ic.write_mem(cpu.addr, sub);
    cpu.status.set_zero(sub == 0);
    cpu.status.set_negative(sub & 0b10000000 != 0);
}

pub fn dex_1(cpu: &mut CPU) {
    let sub = cpu.reg_x.wrapping_sub(1);
    set_register_with_flags(&mut cpu.reg_x, &mut cpu.status, sub);
}

pub fn dey_1(cpu: &mut CPU) {
    let sub = cpu.reg_y.wrapping_sub(1);
    set_register_with_flags(&mut cpu.reg_y, &mut cpu.status, sub);
}

pub fn eor_1(cpu: &mut CPU) {
    let eorred = cpu.reg_a ^ cpu.value;
    set_register_with_flags(&mut cpu.reg_a, &mut cpu.status, eorred);
}

pub fn inc_1(cpu: &mut CPU) {
    // Dummy write
    cpu.ic.write_mem(cpu.addr, cpu.value);
}

pub fn inc_2(cpu: &mut CPU) {
    let inc = cpu.value.wrapping_add(1);
    cpu.ic.write_mem(cpu.addr, inc);
    cpu.status.set_zero(inc == 0);
    cpu.status.set_negative(inc & 0b10000000 != 0);
}

pub fn inx_1(cpu: &mut CPU) {
    let inc = cpu.reg_x.wrapping_add(1);
    set_register_with_flags(&mut cpu.reg_x, &mut cpu.status, inc);
}

pub fn iny_1(cpu: &mut CPU) {
    let inc = cpu.reg_y.wrapping_add(1);
    set_register_with_flags(&mut cpu.reg_y, &mut cpu.status, inc);
}

pub fn jmp_1(cpu: &mut CPU) {
    cpu.reg_pc = cpu.addr;
    // println!("Jumped to {:#02X}", cpu.addr);
}

pub fn jsr_1(cpu: &mut CPU) {
    cpu.push_to_stack(get_msb(cpu.reg_pc - 1));
}

pub fn jsr_2(cpu: &mut CPU) {
    cpu.push_to_stack(get_lsb(cpu.reg_pc - 1));
    cpu.reg_pc = cpu.addr;
    // println!("Jumped to {:#02X}", cpu.addr);
}

pub fn lda_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_a, &mut cpu.status, cpu.value);
}

pub fn ldx_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_x, &mut cpu.status, cpu.value);
}

pub fn ldy_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_y, &mut cpu.status, cpu.value);
}

pub fn lsr_1(cpu: &mut CPU) {
    // TODO
    match cpu.curr_inst.as_mut().expect("No instruction").addr_mode {
        AddressingMode::Accumulator => {
            let old_bit_0 = cpu.reg_a & 1 != 0;
            cpu.reg_a = cpu.reg_a.wrapping_shr(1);
            cpu.status.set_carry(old_bit_0);
            cpu.status.set_zero(cpu.reg_a == 0);
            cpu.status.set_negative(cpu.reg_a & 0b10000000 != 0);
        }
        _ => {
            let old_bit_0 = cpu.value & 1 != 0;
            let rot = cpu.value.wrapping_shr(1);
            cpu.ic.write_mem(cpu.addr, rot); // TODO
            cpu.status.set_carry(old_bit_0);
            cpu.status.set_zero(rot == 0); // TODO is this right?
            cpu.status.set_negative(rot & 0b10000000 != 0);
        }
    }
}

pub fn nop(cpu: &mut CPU) {}

pub fn ora_1(cpu: &mut CPU) {
    let orred = cpu.reg_a | cpu.value;
    set_register_with_flags(&mut cpu.reg_a, &mut cpu.status, orred);
}

pub fn pha_1(cpu: &mut CPU) {
    cpu.push_to_stack(cpu.reg_a);
}

pub fn php_1(cpu: &mut CPU) {
    cpu.push_to_stack(cpu.status.with_b(true).into_bits());
}

pub fn pla_1(cpu: &mut CPU) {
    let s = cpu.pull_from_stack();
    set_register_with_flags(&mut cpu.reg_a, &mut cpu.status, s);
}

pub fn plp_1(cpu: &mut CPU) {
    let s = Status::from_bits(cpu.pull_from_stack())
        .with_b(false)
        .with_one(true);
    cpu.status = Status::from_bits(s.into_bits());
}

pub fn rol_1(cpu: &mut CPU) {
    match cpu.curr_inst.as_mut().expect("No instruction").addr_mode {
        AddressingMode::Accumulator => {
            let old_bit_7 = cpu.reg_a & 0b10000000 != 0;
            cpu.reg_a = cpu.reg_a.wrapping_shl(1) | cpu.status.carry() as u8;
            cpu.status.set_carry(old_bit_7);
            cpu.status.set_zero(cpu.reg_a == 0);
            cpu.status.set_negative(cpu.reg_a & 0b10000000 != 0);
        }
        _ => {
            let old_bit_7 = cpu.value & 0b10000000 != 0;
            let rot = cpu.value.wrapping_shl(1) | cpu.status.carry() as u8;
            cpu.ic.write_mem(cpu.addr, rot); // TODO
            cpu.status.set_carry(old_bit_7);
            cpu.status.set_zero(cpu.reg_a == 0); // TODO is this right?
            cpu.status.set_negative(rot & 0b10000000 != 0);
        }
    }
}

pub fn ror_1(cpu: &mut CPU) {
    match cpu.curr_inst.as_mut().expect("No instruction").addr_mode {
        AddressingMode::Accumulator => {
            let old_bit_0 = cpu.reg_a & 1 != 0;
            cpu.reg_a = cpu.reg_a.wrapping_shl(1) | (cpu.status.carry() as u8).wrapping_shl(7);
            cpu.status.set_carry(old_bit_0);
            cpu.status.set_zero(cpu.reg_a == 0);
            cpu.status.set_negative(cpu.reg_a & 0b10000000 != 0);
        }
        _ => {
            let old_bit_0 = cpu.value & 1 != 0;
            let rot = cpu.value.wrapping_shr(1) | (cpu.status.carry() as u8).wrapping_shl(7);
            cpu.ic.write_mem(cpu.addr, rot); // TODO
            cpu.status.set_carry(old_bit_0);
            cpu.status.set_zero(cpu.reg_a == 0); // TODO is this right?
            cpu.status.set_negative(rot & 0b10000000 != 0);
        }
    }
}

pub fn rti_1(cpu: &mut CPU) {
    cpu.status = Status::from_bits(cpu.pull_from_stack());
}

pub fn rti_2(cpu: &mut CPU) {
    cpu.reg_pc = cpu.pull_from_stack() as u16;
}

pub fn rti_3(cpu: &mut CPU) {
    cpu.reg_pc = (cpu.pull_from_stack() as u16) << 8;
}

pub fn rts_1(cpu: &mut CPU) {
    let lsb = cpu.pull_from_stack();
    cpu.reg_pc = lsb as u16;
}

pub fn rts_2(cpu: &mut CPU) {
    let msb = cpu.pull_from_stack();
    cpu.reg_pc += ((msb as u16) << 8) + 1;
    // println!("Jumped to {:#02X}", cpu.reg_pc);
}

pub fn sbc_1(cpu: &mut CPU) {
    let carry_6 = ((cpu.reg_a & 0b01111111)
        .wrapping_sub(cpu.value & 0b01111111)
        .wrapping_sub(1 - cpu.status.carry() as u8)
        & 0b10000000)
        != 0;
    let (sub1, overflow1) = cpu.reg_a.overflowing_sub(cpu.value);
    let (sub, overflow2) = sub1.overflowing_sub(1 - cpu.status.carry() as u8);
    let overflow = overflow1 | overflow2;

    cpu.status.set_carry(!overflow);
    cpu.status.set_zero(sub == 0);
    cpu.status.set_overflow(carry_6 ^ overflow);
    cpu.status.set_negative(sub & 0b10000000 != 0);
    cpu.reg_a = sub as u8;
}

pub fn sec_1(cpu: &mut CPU) {
    cpu.status.set_carry(true);
}

pub fn sed_1(cpu: &mut CPU) {
    cpu.status.set_decimal(true);
}

pub fn sei_1(cpu: &mut CPU) {
    cpu.status.set_interrupt_disable(true);
}

pub fn tax_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_x, &mut cpu.status, cpu.reg_a);
}

pub fn tay_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_y, &mut cpu.status, cpu.reg_a);
}

pub fn tsx_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_x, &mut cpu.status, cpu.reg_s);
}

pub fn txa_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_a, &mut cpu.status, cpu.reg_x);
}

pub fn txs_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_s, &mut cpu.status, cpu.reg_x);
}

pub fn tya_1(cpu: &mut CPU) {
    set_register_with_flags(&mut cpu.reg_a, &mut cpu.status, cpu.reg_y);
}
