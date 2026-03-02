pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2206268u32;
pub const PC_MAX: u32 = 2208240u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0021aa3c,
        block_0x0021aa58,
        block_0x0021aa68,
        block_0x0021aa70,
        block_0x0021aa84,
        block_0x0021aa88,
        block_0x0021aa98,
        block_0x0021aa9c,
        block_0x0021aaa4,
        block_0x0021aab4,
        block_0x0021aab8,
        block_0x0021aac4,
        block_0x0021aac8,
        block_0x0021aacc,
        block_0x0021aae8,
        block_0x0021aaf0,
        block_0x0021aaf4,
        block_0x0021aaf8,
        block_0x0021ab04,
        block_0x0021ab0c,
        block_0x0021ab20,
        block_0x0021ab34,
        block_0x0021ab3c,
        block_0x0021ab4c,
        block_0x0021ab50,
        block_0x0021ab80,
        block_0x0021ab94,
        block_0x0021abbc,
        block_0x0021abd4,
        block_0x0021abe0,
        block_0x0021ac0c,
        block_0x0021ac14,
        block_0x0021ac18,
        block_0x0021ac20,
        block_0x0021ac2c,
        block_0x0021ac3c,
        block_0x0021ac4c,
        block_0x0021ac54,
        block_0x0021ac6c,
        block_0x0021ac88,
        block_0x0021ac8c,
        block_0x0021aca8,
        block_0x0021acb0,
        block_0x0021acdc,
        block_0x0021ad14,
        block_0x0021ad40,
        block_0x0021ad48,
        block_0x0021ad58,
        block_0x0021ad5c,
        block_0x0021ad78,
        block_0x0021ada0,
        block_0x0021adac,
        block_0x0021adc0,
        block_0x0021adc8,
        block_0x0021add0,
        block_0x0021add8,
        block_0x0021addc,
        block_0x0021ade4,
        block_0x0021ade8,
        block_0x0021adf8,
        block_0x0021adfc,
        block_0x0021ae00,
        block_0x0021ae18,
        block_0x0021ae1c,
        block_0x0021ae24,
        block_0x0021ae28,
        block_0x0021ae30,
        block_0x0021ae44,
        block_0x0021ae48,
        block_0x0021ae6c,
        block_0x0021ae70,
        block_0x0021aea4,
        block_0x0021aecc,
        block_0x0021aefc,
        block_0x0021af10,
        block_0x0021af38,
        block_0x0021af50,
        block_0x0021af5c,
        block_0x0021af88,
        block_0x0021af90,
        block_0x0021af94,
        block_0x0021af9c,
        block_0x0021afa8,
        block_0x0021afb8,
        block_0x0021afc8,
        block_0x0021afd0,
        block_0x0021afe8,
        block_0x0021b004,
        block_0x0021b008,
        block_0x0021b024,
        block_0x0021b02c,
        block_0x0021b058,
        block_0x0021b090,
        block_0x0021b0bc,
        block_0x0021b0cc,
        block_0x0021b0ec,
        block_0x0021b10c,
        block_0x0021b12c,
        block_0x0021b144,
        block_0x0021b14c,
        block_0x0021b164,
        block_0x0021b18c,
        block_0x0021b198,
        block_0x0021b1b8,
        block_0x0021b1d0,
        block_0x0021b1d8,
        block_0x0021b1f0,
    ];
    const IDX: [u16; 494usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16, 0u16,
        4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 6u16, 0u16, 0u16, 0u16, 7u16, 8u16, 0u16,
        9u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16, 12u16, 13u16, 14u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 17u16, 18u16, 0u16, 0u16,
        19u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 22u16,
        0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        29u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 31u16, 0u16, 32u16, 33u16, 0u16, 34u16, 0u16, 0u16, 35u16, 0u16, 0u16,
        0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 41u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 46u16, 0u16, 47u16, 0u16, 0u16, 0u16, 48u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16,
        0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 55u16, 0u16,
        56u16, 57u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 60u16, 61u16, 62u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 63u16, 64u16, 0u16, 65u16, 66u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        70u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16,
        0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 81u16, 0u16, 82u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16,
        106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2206268u32 || pc > 2208240u32 {
        return None;
    }
    let word_offset = ((pc - 2206268u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021aa3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2206272u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2206276u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2206280u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2206284u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2206288u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2206292u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2206404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aac4));
    } else {
        emu.pc = 2206296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa58));
    }
}
#[inline(always)]
pub fn block_0x0021aa58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2206300u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2206304u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2206308u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2206360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa98));
    } else {
        emu.pc = 2206312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa68));
    }
}
#[inline(always)]
pub fn block_0x0021aa68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2206316u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2206360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa98));
    } else {
        emu.pc = 2206320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa70));
    }
}
#[inline(always)]
pub fn block_0x0021aa70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2206324u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2206328u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2206332u32);
    emu.apc_no_count(1usize, 2206332u32, 4294893568u32, 2206336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021aa84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2206392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aab8));
    } else {
        emu.pc = 2206344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa88));
    }
}
#[inline(always)]
pub fn block_0x0021aa88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2206348u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2206352u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2206356u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2206360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aacc));
}
#[inline(always)]
pub fn block_0x0021aa98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2206440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aae8));
    } else {
        emu.pc = 2206364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa9c));
    }
}
#[inline(always)]
pub fn block_0x0021aa9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2206364u32, 4294893568u32, 2206368u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021aaa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2206376u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2206380u32);
    emu.apc_no_count(1usize, 2206380u32, 4294893568u32, 2206384u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021aab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa88));
    } else {
        emu.pc = 2206392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aab8));
    }
}
#[inline(always)]
pub fn block_0x0021aab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2206396u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2206400u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2206404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206408u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aac8));
}
#[inline(always)]
pub fn block_0x0021aac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2206408u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2206408u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aac8));
}
#[inline(always)]
pub fn block_0x0021aac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2206412u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2206412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aacc));
}
#[inline(always)]
pub fn block_0x0021aacc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2206416u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2206420u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2206424u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2206428u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2206432u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2206436u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206440u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021aae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2206444u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2206344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa88));
    } else {
        emu.pc = 2206448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aaf0));
    }
}
#[inline(always)]
pub fn block_0x0021aaf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2206452u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206392u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aab8));
}
#[inline(always)]
pub fn block_0x0021aaf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab04));
    } else {
        emu.pc = 2206456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aaf8));
    }
}
#[inline(always)]
pub fn block_0x0021aaf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2206460u32);
    emu.apc_no_count(1usize, 2206460u32, 0u32, 2206464u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ab04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2206468u32, 0u32, 2206472u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ab0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2206480u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2206484u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2206488u32);
    emu.apc_no_count(1usize, 2206488u32, 4294893568u32, 2206492u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ab20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2206500u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2206504u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965792u32, 2206508u32);
    emu.apc_no_count(6usize, 2206508u32, 8192u32, 2206512u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2206516u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ab34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2206520u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2206540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab4c));
    } else {
        emu.pc = 2206524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab3c));
    }
}
#[inline(always)]
pub fn block_0x0021ab3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2206528u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2206532u32);
    emu.apc_no_count(6usize, 2206532u32, 4294893568u32, 2206536u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2206540u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ab4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206544u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021ab50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2206548u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2206552u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2206556u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2206560u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2206564u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2206568u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2206572u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2206576u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2206580u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2206584u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2206588u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2206652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021abbc));
    } else {
        emu.pc = 2206592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab80));
    }
}
#[inline(always)]
pub fn block_0x0021ab80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2206596u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2206600u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2206604u32);
    emu.apc_no_count(1usize, 2206604u32, 4294897664u32, 2206608u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021ab94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2206616u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2206620u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2206624u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2206628u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2206632u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2206636u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2206640u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2206644u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2206648u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206652u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021abbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2206656u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2206660u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2206664u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2206668u32);
    emu.apc_no_count(1usize, 2206668u32, 0u32, 2206672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021abd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2206680u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2206684u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2206688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206592u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ab80));
}
#[inline]
pub fn block_0x0021abe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2206692u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2206696u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2206700u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2206704u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2206708u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2206712u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2206716u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2206720u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2206724u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2206728u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2206740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac14));
    } else {
        emu.pc = 2206732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac0c));
    }
}
#[inline(always)]
pub fn block_0x0021ac0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2206736u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2206740u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ac2c));
}
#[inline(always)]
pub fn block_0x0021ac14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2206752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac20));
    } else {
        emu.pc = 2206744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac18));
    }
}
#[inline(always)]
pub fn block_0x0021ac18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2206748u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2206752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ac2c));
}
#[inline(always)]
pub fn block_0x0021ac20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2206756u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2206760u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2206764u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2206764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ac2c));
}
#[inline(always)]
pub fn block_0x0021ac2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2206768u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2206772u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2206776u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2206804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac54));
    } else {
        emu.pc = 2206780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac3c));
    }
}
#[inline(always)]
pub fn block_0x0021ac3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2206784u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2206788u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2206792u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2206856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac88));
    } else {
        emu.pc = 2206796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac4c));
    }
}
#[inline(always)]
pub fn block_0x0021ac4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2206800u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2206804u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad14));
}
#[inline(always)]
pub fn block_0x0021ac54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2206808u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2206812u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2206816u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2206820u32);
    emu.apc_no_count(1usize, 2206820u32, 0u32, 2206824u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ac6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2206832u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2206836u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2206840u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2206844u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2206848u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2206852u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2206796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac4c));
    } else {
        emu.pc = 2206856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac88));
    }
}
#[inline(always)]
pub fn block_0x0021ac88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2206888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aca8));
    } else {
        emu.pc = 2206860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac8c));
    }
}
#[inline(always)]
pub fn block_0x0021ac8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2206864u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2206868u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2206872u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2206876u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2206880u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2206884u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2206888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad14));
}
#[inline(always)]
pub fn block_0x0021aca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2206892u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2206940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021acdc));
    } else {
        emu.pc = 2206896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021acb0));
    }
}
#[inline]
pub fn block_0x0021acb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2206900u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2206904u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2206908u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2206912u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2206916u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2206920u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2206924u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2206928u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2206932u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2206936u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2206940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad14));
}
#[inline]
pub fn block_0x0021acdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2206944u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2206948u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2206952u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2206956u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2206960u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2206964u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2206968u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2206972u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2206976u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2206980u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2206984u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2206988u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2206992u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2206996u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2206996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad14));
}
#[inline]
pub fn block_0x0021ad14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2207000u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2207004u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2207008u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2207012u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2207016u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2207020u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2207024u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2207028u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2207032u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2207036u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207040u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ad40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2207044u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2207064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad58));
    } else {
        emu.pc = 2207048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad48));
    }
}
#[inline(always)]
pub fn block_0x0021ad48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2207052u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2207056u32);
    emu.apc_no_count(6usize, 2207056u32, 4294893568u32, 2207060u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2207064u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ad58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207068u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ad5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2207072u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965816u32, 2207076u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2207080u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2207084u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2207088u32);
    emu.apc_no_count(6usize, 2207088u32, 12288u32, 2207092u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2207096u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021ad78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2207100u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2207104u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2207108u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2207112u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2207116u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2207120u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2207124u32);
    emu.lw_no_count(11usize, 11usize, 4u32, 2207128u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2207132u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2207196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021addc));
    } else {
        emu.pc = 2207136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ada0));
    }
}
#[inline(always)]
pub fn block_0x0021ada0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2207140u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2207144u32)?;
    emu.adi_no_count(12usize, 10usize, 4u32, 2207148u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2207148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021adac));
}
#[inline(always)]
pub fn block_0x0021adac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2207152u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2207156u32);
    emu.adr_no_count(18usize, 13usize, 18usize, 2207160u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2207164u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2207148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adac));
    } else {
        emu.pc = 2207168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adc0));
    }
}
#[inline(always)]
pub fn block_0x0021adc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 12u32, 2207172u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2207224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adf8));
    } else {
        emu.pc = 2207176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adc8));
    }
}
#[inline(always)]
pub fn block_0x0021adc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2207180u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2207208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ade8));
    } else {
        emu.pc = 2207184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021add0));
    }
}
#[inline(always)]
pub fn block_0x0021add0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2207188u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2207256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae18));
    } else {
        emu.pc = 2207192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021add8));
    }
}
#[inline(always)]
pub fn block_0x0021add8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2207196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ade8));
}
#[inline(always)]
pub fn block_0x0021addc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 12u32, 2207200u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2207256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae18));
    } else {
        emu.pc = 2207204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ade4));
    }
}
#[inline(always)]
pub fn block_0x0021ade4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2207208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2207208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ade8));
}
#[inline(always)]
pub fn block_0x0021ade8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltr_no_count(10usize, 0usize, 18usize, 2207212u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2207216u32);
    emu.anr_no_count(18usize, 10usize, 18usize, 2207220u32);
    emu.sli_no_count(18usize, 18usize, 1u32, 2207224u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2207224u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021adf8));
}
#[inline(always)]
pub fn block_0x0021adf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2207268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae24));
    } else {
        emu.pc = 2207228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adfc));
    }
}
#[inline(always)]
pub fn block_0x0021adfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2207232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2207232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae00));
}
#[inline(always)]
pub fn block_0x0021ae00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2207236u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965848u32, 2207240u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2207244u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2207248u32);
    emu.apc_no_count(1usize, 2207248u32, 0u32, 2207252u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207256u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ae18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2207260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2207260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae1c));
}
#[inline(always)]
pub fn block_0x0021ae1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2207264u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207268u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207304u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae48));
}
#[inline(always)]
pub fn block_0x0021ae24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2207260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae1c));
    } else {
        emu.pc = 2207272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae28));
    }
}
#[inline(always)]
pub fn block_0x0021ae28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2207272u32, 4294893568u32, 2207276u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ae30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2207284u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2207288u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2207292u32);
    emu.apc_no_count(1usize, 2207292u32, 4294893568u32, 2207296u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ae44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2207232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae00));
    } else {
        emu.pc = 2207304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae48));
    }
}
#[inline]
pub fn block_0x0021ae48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 12u32, 2207308u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2207312u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2207316u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2207320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965824u32, 2207324u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2207328u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2207332u32);
    emu.apc_no_count(1usize, 2207332u32, 8192u32, 2207336u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ae6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2207396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aea4));
    } else {
        emu.pc = 2207344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae70));
    }
}
#[inline]
pub fn block_0x0021ae70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2207348u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2207352u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2207356u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2207360u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2207364u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2207368u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2207372u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2207376u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2207380u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2207384u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2207388u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2207392u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207396u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021aea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2207400u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965880u32, 2207404u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2207408u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965864u32, 2207412u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2207416u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965968u32, 2207420u32);
    emu.adi_no_count(11usize, 0usize, 86u32, 2207424u32);
    emu.adi_no_count(12usize, 2usize, 27u32, 2207428u32);
    emu.apc_no_count(1usize, 2207428u32, 8192u32, 2207432u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207436u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021aecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2207440u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2207444u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2207448u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2207452u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2207456u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2207460u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2207464u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2207468u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2207472u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2207476u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2207480u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2207544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af38));
    } else {
        emu.pc = 2207484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aefc));
    }
}
#[inline(always)]
pub fn block_0x0021aefc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2207488u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2207492u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2207496u32);
    emu.apc_no_count(1usize, 2207496u32, 4294897664u32, 2207500u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021af10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2207508u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2207512u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2207516u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2207520u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2207524u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2207528u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2207532u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2207536u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2207540u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207544u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021af38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2207548u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2207552u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2207556u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2207560u32);
    emu.apc_no_count(1usize, 2207560u32, 0u32, 2207564u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965800u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021af50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2207572u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2207576u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2207580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207484u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aefc));
}
#[inline]
pub fn block_0x0021af5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2207584u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2207588u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2207592u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2207596u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2207600u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2207604u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2207608u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2207612u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2207616u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2207620u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2207632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af90));
    } else {
        emu.pc = 2207624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af88));
    }
}
#[inline(always)]
pub fn block_0x0021af88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2207628u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021afa8));
}
#[inline(always)]
pub fn block_0x0021af90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2207644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af9c));
    } else {
        emu.pc = 2207636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af94));
    }
}
#[inline(always)]
pub fn block_0x0021af94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2207640u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207644u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021afa8));
}
#[inline(always)]
pub fn block_0x0021af9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2207648u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2207652u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2207656u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2207656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021afa8));
}
#[inline(always)]
pub fn block_0x0021afa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2207660u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2207664u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2207668u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2207696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afd0));
    } else {
        emu.pc = 2207672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afb8));
    }
}
#[inline(always)]
pub fn block_0x0021afb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2207676u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2207680u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2207684u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2207748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b004));
    } else {
        emu.pc = 2207688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afc8));
    }
}
#[inline(always)]
pub fn block_0x0021afc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2207692u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b090));
}
#[inline(always)]
pub fn block_0x0021afd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2207700u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2207704u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2207708u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2207712u32);
    emu.apc_no_count(1usize, 2207712u32, 0u32, 2207716u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021afe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2207724u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2207728u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2207732u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2207736u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2207740u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2207744u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2207688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afc8));
    } else {
        emu.pc = 2207748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b004));
    }
}
#[inline(always)]
pub fn block_0x0021b004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2207780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b024));
    } else {
        emu.pc = 2207752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b008));
    }
}
#[inline(always)]
pub fn block_0x0021b008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2207756u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2207760u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2207764u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2207768u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2207772u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2207776u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2207780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b090));
}
#[inline(always)]
pub fn block_0x0021b024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2207784u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2207832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b058));
    } else {
        emu.pc = 2207788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b02c));
    }
}
#[inline]
pub fn block_0x0021b02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2207792u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2207796u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2207800u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2207804u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2207808u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2207812u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2207816u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2207820u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2207824u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2207828u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2207832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b090));
}
#[inline]
pub fn block_0x0021b058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2207836u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2207840u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2207844u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2207848u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2207852u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2207856u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2207860u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2207864u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2207868u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2207872u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2207876u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2207880u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2207884u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2207888u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2207888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b090));
}
#[inline]
pub fn block_0x0021b090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2207892u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2207896u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2207900u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2207904u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2207908u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2207912u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2207916u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2207920u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2207924u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2207928u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207932u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2207936u32)?;
    emu.lw_no_count(11usize, 10usize, 8u32, 2207940u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2207944u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207948u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b0cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2207952u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2207956u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2207960u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2207964u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2207968u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2207972u32);
    emu.apc_no_count(6usize, 2207972u32, 12288u32, 2207976u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2207980u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2207984u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2207988u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2207992u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2207996u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2208000u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2208004u32);
    emu.apc_no_count(6usize, 2208004u32, 12288u32, 2208008u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2208012u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2208016u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2208020u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2208024u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2208028u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2208032u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2208036u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2208040u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2208044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208068u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b144));
}
#[inline(always)]
pub fn block_0x0021b12c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2208048u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208052u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208056u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208060u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208064u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2208100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b164));
    } else {
        emu.pc = 2208068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b144));
    }
}
#[inline(always)]
pub fn block_0x0021b144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2208072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2208044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b12c));
    } else {
        emu.pc = 2208076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b14c));
    }
}
#[inline(always)]
pub fn block_0x0021b14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2208080u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208084u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208088u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208092u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208096u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2208068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b144));
    } else {
        emu.pc = 2208100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b164));
    }
}
#[inline]
pub fn block_0x0021b164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2208104u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2208108u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2208112u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208116u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965984u32, 2208120u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2208124u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2208128u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2208132u32);
    emu.apc_no_count(1usize, 2208132u32, 8192u32, 2208136u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2208144u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2208148u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208152u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2208156u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2208160u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2208164u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2208168u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2208172u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2208176u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2208180u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2208184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b1d0));
}
#[inline(always)]
pub fn block_0x0021b1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2208188u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208192u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208196u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208200u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208204u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2208240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1f0));
    } else {
        emu.pc = 2208208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1d0));
    }
}
#[inline(always)]
pub fn block_0x0021b1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2208212u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2208184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1b8));
    } else {
        emu.pc = 2208216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1d8));
    }
}
#[inline(always)]
pub fn block_0x0021b1d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2208220u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208224u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208228u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208232u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208236u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2208208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1d0));
    } else {
        emu.pc = 2208240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1f0));
    }
}
#[inline]
pub fn block_0x0021b1f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2208244u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2208248u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2208252u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208256u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965984u32, 2208260u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2208264u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2208268u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2208272u32);
    emu.apc_no_count(1usize, 2208272u32, 8192u32, 2208276u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
