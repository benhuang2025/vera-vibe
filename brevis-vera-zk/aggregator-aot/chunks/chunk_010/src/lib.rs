pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2137200u32;
pub const PC_MAX: u32 = 2139920u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 113usize] = [
        block_0x00209c70,
        block_0x00209c8c,
        block_0x00209c98,
        block_0x00209ca0,
        block_0x00209cc4,
        block_0x00209cd0,
        block_0x00209cdc,
        block_0x00209d18,
        block_0x00209db4,
        block_0x00209de0,
        block_0x00209dec,
        block_0x00209e08,
        block_0x00209e38,
        block_0x00209e58,
        block_0x00209e7c,
        block_0x00209ea4,
        block_0x00209ea8,
        block_0x00209eb0,
        block_0x00209ec0,
        block_0x00209ed0,
        block_0x00209ef8,
        block_0x00209f20,
        block_0x00209f34,
        block_0x00209f4c,
        block_0x00209fb0,
        block_0x00209fb8,
        block_0x00209fc8,
        block_0x00209fcc,
        block_0x00209ffc,
        block_0x0020a00c,
        block_0x0020a02c,
        block_0x0020a03c,
        block_0x0020a040,
        block_0x0020a064,
        block_0x0020a074,
        block_0x0020a09c,
        block_0x0020a0a0,
        block_0x0020a0b8,
        block_0x0020a0c4,
        block_0x0020a0d0,
        block_0x0020a0e4,
        block_0x0020a0e8,
        block_0x0020a0f4,
        block_0x0020a0f8,
        block_0x0020a10c,
        block_0x0020a114,
        block_0x0020a124,
        block_0x0020a128,
        block_0x0020a150,
        block_0x0020a174,
        block_0x0020a184,
        block_0x0020a18c,
        block_0x0020a19c,
        block_0x0020a1a0,
        block_0x0020a1a8,
        block_0x0020a1ac,
        block_0x0020a1e0,
        block_0x0020a1e8,
        block_0x0020a218,
        block_0x0020a228,
        block_0x0020a230,
        block_0x0020a238,
        block_0x0020a240,
        block_0x0020a248,
        block_0x0020a27c,
        block_0x0020a280,
        block_0x0020a2a4,
        block_0x0020a2a8,
        block_0x0020a2e0,
        block_0x0020a2e4,
        block_0x0020a308,
        block_0x0020a30c,
        block_0x0020a328,
        block_0x0020a32c,
        block_0x0020a340,
        block_0x0020a344,
        block_0x0020a370,
        block_0x0020a374,
        block_0x0020a39c,
        block_0x0020a3a4,
        block_0x0020a3a8,
        block_0x0020a3d8,
        block_0x0020a3dc,
        block_0x0020a404,
        block_0x0020a408,
        block_0x0020a428,
        block_0x0020a430,
        block_0x0020a434,
        block_0x0020a464,
        block_0x0020a468,
        block_0x0020a488,
        block_0x0020a48c,
        block_0x0020a4d4,
        block_0x0020a4d8,
        block_0x0020a51c,
        block_0x0020a548,
        block_0x0020a54c,
        block_0x0020a578,
        block_0x0020a57c,
        block_0x0020a5b8,
        block_0x0020a5bc,
        block_0x0020a5d8,
        block_0x0020a5dc,
        block_0x0020a610,
        block_0x0020a614,
        block_0x0020a640,
        block_0x0020a644,
        block_0x0020a68c,
        block_0x0020a690,
        block_0x0020a6c0,
        block_0x0020a6c4,
        block_0x0020a70c,
        block_0x0020a710,
    ];
    const IDX: [u16; 681usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 4u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 6u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16,
        0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        25u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 37u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16,
        42u16, 0u16, 0u16, 43u16, 44u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 51u16, 0u16, 52u16, 0u16, 0u16, 0u16, 53u16, 54u16, 0u16, 55u16, 56u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16,
        0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        59u16, 0u16, 0u16, 0u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 63u16, 0u16,
        64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        65u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 68u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16,
        70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 72u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 81u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 86u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 97u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 99u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 103u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 105u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 107u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 110u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 113u16,
    ];
    if pc < 2137200u32 || pc > 2139920u32 {
        return None;
    }
    let word_offset = ((pc - 2137200u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00209c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(3usize, 2137200u32, 4292898816u32, 2137204u32);
    emu.adi_no_count(3usize, 3usize, 4294966160u32, 2137208u32);
    emu.apc_no_count(2usize, 2137208u32, 98304u32, 2137212u32);
    emu.adi_no_count(2usize, 2usize, 4294967176u32, 2137216u32);
    emu.lw_no_count(2usize, 2usize, 0u32, 2137220u32)?;
    emu.apc_no_count(1usize, 2137220u32, 0u32, 2137224u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137232u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294965632u32, 2137236u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2137248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ca0));
    } else {
        emu.pc = 2137240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c98));
    }
}
#[inline(always)]
pub fn block_0x00209c98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137244u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965868u32, 2137248u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2137248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ca0));
}
#[inline]
pub fn block_0x00209ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967295u32, 2137252u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2137256u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2137260u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2137264u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2137268u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2137272u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2137276u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2137280u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2137308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cdc));
    } else {
        emu.pc = 2137284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cc4));
    }
}
#[inline(always)]
pub fn block_0x00209cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2013265920u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137288u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2137292u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2137308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cdc));
    } else {
        emu.pc = 2137296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cd0));
    }
}
#[inline(always)]
pub fn block_0x00209cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137300u32;
    emu.update_insn_clock();
    emu.sw_no_count(12usize, 11usize, 4294965632u32, 2137304u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137308u32;
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
pub fn block_0x00209cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2137312u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137316u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966248u32, 2137320u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2137324u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2137328u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2137332u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2137336u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2137340u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2137344u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2137348u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137352u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966256u32, 2137356u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2137360u32);
    emu.apc_no_count(1usize, 2137360u32, 73728u32, 2137364u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137368u32;
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
#[inline(never)]
pub fn block_0x00209d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2137372u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2137376u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2137380u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2137384u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2137388u32)?;
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2137392u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 0usize, 1u32, 2137396u32);
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137400u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137404u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137408u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2137412u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2137416u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2137420u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 8usize, 4294965640u32, 2137424u32);
    emu.adi_no_count(10usize, 10usize, 1639u32, 2137428u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2137432u32);
    emu.sw_no_count(9usize, 18usize, 40u32, 2137436u32)?;
    emu.sw_no_count(0usize, 18usize, 44u32, 2137440u32)?;
    emu.sw_no_count(10usize, 18usize, 48u32, 2137444u32)?;
    emu.sw_no_count(11usize, 18usize, 52u32, 2137448u32)?;
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137452u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 12usize, 882u32, 2137456u32);
    emu.adi_no_count(12usize, 13usize, 1338u32, 2137460u32);
    emu.adi_no_count(13usize, 14usize, 639u32, 2137464u32);
    emu.adi_no_count(14usize, 15usize, 4294965388u32, 2137468u32);
    emu.sw_no_count(11usize, 18usize, 56u32, 2137472u32)?;
    emu.sw_no_count(12usize, 18usize, 60u32, 2137476u32)?;
    emu.sw_no_count(13usize, 18usize, 64u32, 2137480u32)?;
    emu.sw_no_count(14usize, 18usize, 68u32, 2137484u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137488u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965675u32, 2137492u32);
    emu.adi_no_count(11usize, 11usize, 4294966553u32, 2137496u32);
    emu.sw_no_count(10usize, 18usize, 72u32, 2137500u32)?;
    emu.sw_no_count(11usize, 18usize, 76u32, 2137504u32)?;
    emu.adi_no_count(10usize, 18usize, 80u32, 2137508u32);
    emu.adi_no_count(12usize, 0usize, 73u32, 2137512u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2137516u32);
    emu.apc_no_count(1usize, 2137516u32, 0u32, 2137520u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 4294965640u32, 2137528u32)?;
    emu.sw_no_count(0usize, 8usize, 4294965644u32, 2137532u32)?;
    emu.sw_no_count(0usize, 18usize, 8u32, 2137536u32)?;
    emu.sw_no_count(0usize, 18usize, 12u32, 2137540u32)?;
    emu.sw_no_count(0usize, 18usize, 16u32, 2137544u32)?;
    emu.sw_no_count(0usize, 18usize, 20u32, 2137548u32)?;
    emu.sw_no_count(0usize, 18usize, 24u32, 2137552u32)?;
    emu.sw_no_count(0usize, 18usize, 28u32, 2137556u32)?;
    emu.sw_no_count(0usize, 18usize, 32u32, 2137560u32)?;
    emu.apc_no_count(1usize, 2137560u32, 4294955008u32, 2137564u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2137572u32);
    emu.apc_no_count(1usize, 2137572u32, 4294963200u32, 2137576u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2137584u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966276u32, 2137588u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2137592u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2137596u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2137600u32);
    emu.apc_no_count(6usize, 2137600u32, 81920u32, 2137604u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2137608u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2137612u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2137616u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2137620u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2137624u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2137628u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2137632u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2137636u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2137640u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2137644u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2137648u32);
    emu.apc_no_count(1usize, 2137648u32, 4294963200u32, 2137652u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2137660u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2137664u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2137668u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2137672u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2137676u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2137680u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2137684u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137688u32;
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
pub fn block_0x00209e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2137692u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2137696u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2137700u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2137704u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2137708u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2137712u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2137716u32);
    emu.apc_no_count(1usize, 2137716u32, 0u32, 2137720u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209e7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2137728u32);
    emu.ani_no_count(10usize, 10usize, 3u32, 2137732u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2137736u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2137740u32);
    emu.adr_no_count(10usize, 10usize, 9usize, 2137744u32);
    emu.ani_no_count(18usize, 10usize, 4294967292u32, 2137748u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2137752u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2137756u32);
    emu.apc_no_count(1usize, 2137756u32, 69632u32, 2137760u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209ea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2137848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ef8));
    } else {
        emu.pc = 2137768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ea8));
    }
}
#[inline(always)]
pub fn block_0x00209ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2137768u32, 4294963200u32, 2137772u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(92u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2137780u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2137784u32);
    emu.apc_no_count(1usize, 2137784u32, 4294963200u32, 2137788u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209ec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2137796u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2137800u32);
    emu.apc_no_count(1usize, 2137800u32, 0u32, 2137804u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 0u32, 2137812u32)?;
    emu.sw_no_count(19usize, 8usize, 4u32, 2137816u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2137820u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2137824u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2137828u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2137832u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2137836u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2137840u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2137844u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137848u32;
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
pub fn block_0x00209ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137852u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 574u32, 2137856u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2137860u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966288u32, 2137864u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2137868u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966304u32, 2137872u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2137876u32);
    emu.adi_no_count(12usize, 2usize, 11u32, 2137880u32);
    emu.apc_no_count(1usize, 2137880u32, 77824u32, 2137884u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2137892u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2137896u32)?;
    emu.sw_no_count(10usize, 2usize, 0u32, 2137900u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2137904u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f4c));
    } else {
        emu.pc = 2137908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f34));
    }
}
#[inline(always)]
pub fn block_0x00209f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2137912u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2137916u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2137920u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2137924u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2137928u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137932u32;
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
#[inline(never)]
pub fn block_0x00209f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2137936u32);
    let a = 0u32.wrapping_add(2211840u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965448u32, 2137944u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137948u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967044u32, 2137952u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2137956u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1064u32, 2137960u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2137964u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966368u32, 2137968u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2137972u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2137976u32)?;
    emu.adi_no_count(16usize, 2usize, 44u32, 2137980u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2137984u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2137988u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2137992u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2137996u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2138000u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2138004u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2138008u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2138012u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2138016u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2138020u32);
    emu.adi_no_count(11usize, 2usize, 20u32, 2138024u32);
    emu.apc_no_count(1usize, 2138024u32, 69632u32, 2138028u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2138032u32, 4294963200u32, 2138036u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2138044u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138048u32);
    emu.apc_no_count(1usize, 2138048u32, 4294959104u32, 2138052u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ffc));
    } else {
        emu.pc = 2138060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fcc));
    }
}
#[inline]
pub fn block_0x00209fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2138064u32);
    emu.lw_no_count(10usize, 2usize, 8u32, 2138068u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2138072u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2138076u32)?;
    emu.sw_no_count(10usize, 12usize, 0u32, 2138080u32)?;
    emu.sw_no_count(11usize, 12usize, 4u32, 2138084u32)?;
    emu.sw_no_count(13usize, 12usize, 8u32, 2138088u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2138092u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2138096u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2138100u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2138104u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138108u32;
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
pub fn block_0x00209ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138112u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2138116u32);
    emu.apc_no_count(1usize, 2138116u32, 69632u32, 2138120u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138128u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2138132u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2138136u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2138140u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2138144u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2138148u32)?;
    emu.apc_no_count(1usize, 2138148u32, 4294963200u32, 2138152u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2138160u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138164u32);
    emu.apc_no_count(1usize, 2138164u32, 4294959104u32, 2138168u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a03c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a064));
    } else {
        emu.pc = 2138176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a040));
    }
}
#[inline]
pub fn block_0x0020a040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138180u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 10usize, 0u32, 2138184u32)?;
    emu.sw_no_count(8usize, 10usize, 4u32, 2138188u32)?;
    emu.sw_no_count(9usize, 10usize, 8u32, 2138192u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2138196u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2138200u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2138204u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2138208u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138212u32;
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
pub fn block_0x0020a064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138216u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2138220u32);
    emu.apc_no_count(1usize, 2138220u32, 69632u32, 2138224u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2138232u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2138236u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2138240u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2138244u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2138248u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2138252u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2138256u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2138260u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2138264u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2138296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0b8));
    } else {
        emu.pc = 2138268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a09c));
    }
}
#[inline(always)]
pub fn block_0x0020a09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2138272u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0a0));
}
#[inline(always)]
pub fn block_0x0020a0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138276u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966452u32, 2138280u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2138284u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2138288u32);
    emu.apc_no_count(1usize, 2138288u32, 69632u32, 2138292u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2138300u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2138304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2138356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0f4));
    } else {
        emu.pc = 2138308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0c4));
    }
}
#[inline(always)]
pub fn block_0x0020a0c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 12usize, 0u32, 2138312u32);
    emu.apc_no_count(1usize, 2138312u32, 4294963200u32, 2138316u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2138324u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2138328u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2138332u32);
    emu.apc_no_count(1usize, 2138332u32, 4294959104u32, 2138336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0a0));
    } else {
        emu.pc = 2138344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0e8));
    }
}
#[inline(always)]
pub fn block_0x0020a0e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2138348u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2138352u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2138356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0f8));
}
#[inline(always)]
pub fn block_0x0020a0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2138360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0f8));
}
#[inline(always)]
pub fn block_0x0020a0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2138364u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2138368u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2138372u32);
    emu.apc_no_count(1usize, 2138372u32, 4294963200u32, 2138376u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2138380u32, 4294963200u32, 2138384u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2138392u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138396u32);
    emu.apc_no_count(1usize, 2138396u32, 4294959104u32, 2138400u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a174));
    } else {
        emu.pc = 2138408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a128));
    }
}
#[inline]
pub fn block_0x0020a128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2138412u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2138416u32)?;
    emu.sw_no_count(19usize, 10usize, 4u32, 2138420u32)?;
    emu.sw_no_count(8usize, 10usize, 8u32, 2138424u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2138428u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966408u32, 2138432u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2138436u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2138440u32);
    emu.apc_no_count(1usize, 2138440u32, 65536u32, 2138444u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2138452u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2138456u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2138460u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2138464u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2138468u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2138472u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2138476u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2138480u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138484u32;
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
pub fn block_0x0020a174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138488u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2138492u32);
    emu.apc_no_count(1usize, 2138492u32, 69632u32, 2138496u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138500u32;
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
pub fn block_0x0020a184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2138504u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2138524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a19c));
    } else {
        emu.pc = 2138508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a18c));
    }
}
#[inline(always)]
pub fn block_0x0020a18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2138512u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2138516u32);
    emu.apc_no_count(6usize, 2138516u32, 4294959104u32, 2138520u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138524u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a19c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138528u32;
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
pub fn block_0x0020a1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2138532u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138536u32;
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
pub fn block_0x0020a1a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138540u32;
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
pub fn block_0x0020a1ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1059942400u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138544u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2722881536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138548u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2725687296u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2138552u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2304393216u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138556u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966095u32, 2138560u32);
    emu.adi_no_count(12usize, 12usize, 1479u32, 2138564u32);
    emu.adi_no_count(13usize, 13usize, 4294966961u32, 2138568u32);
    emu.adi_no_count(14usize, 14usize, 4294965480u32, 2138572u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2138576u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2138580u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2138584u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2138588u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138592u32;
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
pub fn block_0x0020a1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2138596u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138600u32;
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
pub fn block_0x0020a1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138604u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2138608u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2138612u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2138616u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2138620u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138624u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966468u32, 2138628u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2138632u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2138636u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2138640u32);
    emu.apc_no_count(1usize, 2138640u32, 73728u32, 2138644u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2138652u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2138656u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2138660u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2138680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a238));
    } else {
        emu.pc = 2138664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a228));
    }
}
#[inline(always)]
pub fn block_0x0020a228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2138668u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2138688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a240));
    } else {
        emu.pc = 2138672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a230));
    }
}
#[inline(always)]
pub fn block_0x0020a230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2138672u32, 69632u32, 2138676u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138680u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2138680u32, 69632u32, 2138684u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138688u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2138688u32, 69632u32, 2138692u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138696u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 12usize, 0u32, 2138700u32)?;
    emu.lw_no_count(15usize, 12usize, 4u32, 2138704u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2138708u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2138712u32)?;
    emu.lw_no_count(17usize, 11usize, 8u32, 2138716u32)?;
    emu.lw_no_count(30usize, 11usize, 12u32, 2138720u32)?;
    emu.lw_no_count(29usize, 12usize, 8u32, 2138724u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2138728u32)?;
    emu.adr_no_count(5usize, 15usize, 5usize, 2138732u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2138736u32);
    emu.sltru_no_count(28usize, 13usize, 14usize, 2138740u32);
    emu.adr_no_count(14usize, 5usize, 28usize, 2138744u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2138752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a280));
    } else {
        emu.pc = 2138748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a27c));
    }
}
#[inline(always)]
pub fn block_0x0020a27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 15usize, 2138752u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138752u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a280));
}
#[inline]
pub fn block_0x0020a280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 11usize, 16u32, 2138756u32)?;
    emu.lw_no_count(7usize, 11usize, 20u32, 2138760u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2138764u32)?;
    emu.lw_no_count(15usize, 12usize, 20u32, 2138768u32)?;
    emu.adr_no_count(30usize, 16usize, 30usize, 2138772u32);
    emu.adr_no_count(31usize, 29usize, 17usize, 2138776u32);
    emu.sltru_no_count(17usize, 31usize, 29usize, 2138780u32);
    emu.adr_no_count(30usize, 30usize, 17usize, 2138784u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2138792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2a8));
    } else {
        emu.pc = 2138788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2a4));
    }
}
#[inline(always)]
pub fn block_0x0020a2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 30usize, 16usize, 2138792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a2a8));
}
#[inline]
pub fn block_0x0020a2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138796u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2138800u32)?;
    emu.adr_no_count(28usize, 31usize, 28usize, 2138804u32);
    emu.adr_no_count(7usize, 15usize, 7usize, 2138808u32);
    emu.adr_no_count(16usize, 6usize, 5usize, 2138812u32);
    emu.sltru_no_count(31usize, 28usize, 31usize, 2138816u32);
    emu.sltru_no_count(5usize, 16usize, 6usize, 2138820u32);
    emu.adr_no_count(29usize, 30usize, 31usize, 2138824u32);
    emu.sltru_no_count(6usize, 29usize, 30usize, 2138828u32);
    emu.anr_no_count(31usize, 31usize, 6usize, 2138832u32);
    emu.adr_no_count(31usize, 17usize, 31usize, 2138836u32);
    emu.adr_no_count(6usize, 7usize, 5usize, 2138840u32);
    emu.sltru_no_count(8usize, 31usize, 17usize, 2138844u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2138852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2e4));
    } else {
        emu.pc = 2138848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2e0));
    }
}
#[inline(always)]
pub fn block_0x0020a2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 15usize, 2138852u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a2e4));
}
#[inline]
pub fn block_0x0020a2e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 11usize, 24u32, 2138856u32)?;
    emu.lw_no_count(30usize, 11usize, 28u32, 2138860u32)?;
    emu.lw_no_count(7usize, 12usize, 24u32, 2138864u32)?;
    emu.lw_no_count(11usize, 12usize, 28u32, 2138868u32)?;
    emu.adr_no_count(15usize, 16usize, 31usize, 2138872u32);
    emu.sltru_no_count(12usize, 15usize, 16usize, 2138876u32);
    emu.adr_no_count(16usize, 6usize, 8usize, 2138880u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2138884u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2138892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a30c));
    } else {
        emu.pc = 2138888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a308));
    }
}
#[inline(always)]
pub fn block_0x0020a308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 6usize, 2138892u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a30c));
}
#[inline(always)]
pub fn block_0x0020a30c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 5usize, 12usize, 2138896u32);
    emu.adr_no_count(12usize, 11usize, 30usize, 2138900u32);
    emu.adr_no_count(30usize, 7usize, 17usize, 2138904u32);
    emu.sltru_no_count(17usize, 30usize, 7usize, 2138908u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2138912u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2138916u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2138924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a32c));
    } else {
        emu.pc = 2138920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a328));
    }
}
#[inline(always)]
pub fn block_0x0020a328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 12usize, 11usize, 2138924u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a32c));
}
#[inline(always)]
pub fn block_0x0020a32c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 30usize, 6usize, 2138928u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2138932u32);
    emu.adr_no_count(5usize, 12usize, 5usize, 2138936u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2138940u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2138948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a344));
    } else {
        emu.pc = 2138944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a340));
    }
}
#[inline(always)]
pub fn block_0x0020a340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 12usize, 2138948u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138948u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a344));
}
#[inline]
pub fn block_0x0020a344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 1u32, 2138952u32);
    emu.sltiu_no_count(11usize, 12usize, 1u32, 2138956u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2138960u32);
    emu.orr_no_count(13usize, 12usize, 11usize, 2138964u32);
    emu.sltru_no_count(30usize, 0usize, 13usize, 2138968u32);
    emu.sbr_no_count(13usize, 0usize, 30usize, 2138972u32);
    emu.sbr_no_count(28usize, 28usize, 30usize, 2138976u32);
    emu.sltru_no_count(14usize, 28usize, 13usize, 2138980u32);
    emu.sbr_no_count(31usize, 29usize, 30usize, 2138984u32);
    emu.adr_no_count(31usize, 31usize, 14usize, 2138988u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2138996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a374));
    } else {
        emu.pc = 2138992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a370));
    }
}
#[inline(always)]
pub fn block_0x0020a370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 13usize, 2138996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a374));
}
#[inline]
pub fn block_0x0020a374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(29usize, 14usize, 30usize, 2139000u32);
    emu.adi_no_count(14usize, 28usize, 1u32, 2139004u32);
    emu.sltru_no_count(8usize, 29usize, 13usize, 2139008u32);
    emu.sltiu_no_count(13usize, 14usize, 1u32, 2139012u32);
    emu.adr_no_count(13usize, 31usize, 13usize, 2139016u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2139020u32);
    emu.sbr_no_count(30usize, 8usize, 30usize, 2139024u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2139028u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2139032u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2139044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3a4));
    } else {
        emu.pc = 2139036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a39c));
    }
}
#[inline(always)]
pub fn block_0x0020a39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 31usize, 2139040u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2139044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139048u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3a8));
}
#[inline(always)]
pub fn block_0x0020a3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 28usize, 2139048u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139048u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3a8));
}
#[inline]
pub fn block_0x0020a3a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 29usize, 4294967295u32, 2139052u32);
    emu.sltiu_no_count(29usize, 29usize, 1u32, 2139056u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2139060u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2139064u32);
    emu.sltru_no_count(28usize, 28usize, 31usize, 2139068u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2139072u32);
    emu.sai_no_count(28usize, 28usize, 1055u32, 2139076u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2139080u32);
    emu.sltru_no_count(29usize, 15usize, 28usize, 2139084u32);
    emu.adr_no_count(16usize, 28usize, 16usize, 2139088u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2139092u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2139100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3dc));
    } else {
        emu.pc = 2139096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3d8));
    }
}
#[inline(always)]
pub fn block_0x0020a3d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 16usize, 28usize, 2139100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3dc));
}
#[inline]
pub fn block_0x0020a3dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 28usize, 29usize, 2139104u32);
    emu.sltru_no_count(29usize, 29usize, 28usize, 2139108u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2139112u32);
    emu.sai_no_count(30usize, 28usize, 1055u32, 2139116u32);
    emu.adr_no_count(31usize, 5usize, 30usize, 2139120u32);
    emu.adr_no_count(28usize, 6usize, 30usize, 2139124u32);
    emu.sltru_no_count(29usize, 28usize, 6usize, 2139128u32);
    emu.adr_no_count(31usize, 31usize, 29usize, 2139132u32);
    emu.adr_no_count(7usize, 17usize, 7usize, 2139136u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2139144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a408));
    } else {
        emu.pc = 2139140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a404));
    }
}
#[inline(always)]
pub fn block_0x0020a404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 31usize, 5usize, 2139144u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139144u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a408));
}
#[inline(always)]
pub fn block_0x0020a408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 7usize, 17usize, 2139148u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2139152u32);
    emu.sltru_no_count(5usize, 0usize, 28usize, 2139156u32);
    emu.sltru_no_count(17usize, 29usize, 30usize, 2139160u32);
    emu.adr_no_count(5usize, 31usize, 5usize, 2139164u32);
    emu.adr_no_count(30usize, 30usize, 17usize, 2139168u32);
    emu.adi_no_count(17usize, 28usize, 4294967295u32, 2139172u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2139184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a430));
    } else {
        emu.pc = 2139176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a428));
    }
}
#[inline(always)]
pub fn block_0x0020a428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 5usize, 31usize, 2139180u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2139184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a434));
}
#[inline(always)]
pub fn block_0x0020a430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 17usize, 28usize, 2139188u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a434));
}
#[inline]
pub fn block_0x0020a434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 29usize, 4294967295u32, 2139192u32);
    emu.sltiu_no_count(29usize, 29usize, 1u32, 2139196u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2139200u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2139204u32);
    emu.sltru_no_count(28usize, 28usize, 31usize, 2139208u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2139212u32);
    emu.sai_no_count(28usize, 28usize, 1055u32, 2139216u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2139220u32);
    emu.sltru_no_count(7usize, 7usize, 28usize, 2139224u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2139228u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2139232u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2139240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a468));
    } else {
        emu.pc = 2139236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a464));
    }
}
#[inline(always)]
pub fn block_0x0020a464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 6usize, 28usize, 2139240u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a468));
}
#[inline(always)]
pub fn block_0x0020a468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 28usize, 7usize, 2139244u32);
    emu.sltru_no_count(7usize, 6usize, 28usize, 2139248u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2139252u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2139256u32);
    emu.sltru_no_count(28usize, 12usize, 6usize, 2139260u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2139264u32);
    emu.adr_no_count(11usize, 11usize, 28usize, 2139268u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2139276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a48c));
    } else {
        emu.pc = 2139272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a488));
    }
}
#[inline(always)]
pub fn block_0x0020a488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 11usize, 7usize, 2139276u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a48c));
}
#[inline]
pub fn block_0x0020a48c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 6usize, 14usize, 2139280u32);
    emu.sltru_no_count(30usize, 29usize, 6usize, 2139284u32);
    emu.adr_no_count(14usize, 29usize, 28usize, 2139288u32);
    emu.adr_no_count(28usize, 13usize, 30usize, 2139292u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2139296u32);
    emu.sltru_no_count(13usize, 0usize, 28usize, 2139300u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2139304u32);
    emu.anr_no_count(30usize, 13usize, 30usize, 2139308u32);
    emu.adr_no_count(13usize, 28usize, 29usize, 2139312u32);
    emu.sltru_no_count(28usize, 13usize, 28usize, 2139316u32);
    emu.anr_no_count(28usize, 29usize, 28usize, 2139320u32);
    emu.adr_no_count(28usize, 30usize, 28usize, 2139324u32);
    emu.sltru_no_count(29usize, 28usize, 30usize, 2139328u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2139332u32);
    emu.sltru_no_count(28usize, 15usize, 28usize, 2139336u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2139340u32);
    emu.adr_no_count(16usize, 16usize, 28usize, 2139344u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d4));
    }
}
#[inline(always)]
pub fn block_0x0020a4d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 16usize, 29usize, 2139352u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a4d8));
}
#[inline]
pub fn block_0x0020a4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(6usize, 6usize, 1u32, 2139356u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2139360u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2139364u32)?;
    emu.sw_no_count(11usize, 10usize, 4u32, 2139368u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2139372u32)?;
    emu.sw_no_count(13usize, 10usize, 12u32, 2139376u32)?;
    emu.adr_no_count(17usize, 6usize, 17usize, 2139380u32);
    emu.sltru_no_count(11usize, 17usize, 6usize, 2139384u32);
    emu.adr_no_count(28usize, 17usize, 28usize, 2139388u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2139392u32);
    emu.sltru_no_count(12usize, 28usize, 17usize, 2139396u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2139400u32);
    emu.sw_no_count(15usize, 10usize, 16u32, 2139404u32)?;
    emu.sw_no_count(16usize, 10usize, 20u32, 2139408u32)?;
    emu.sw_no_count(28usize, 10usize, 24u32, 2139412u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2139416u32)?;
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139420u32;
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
pub fn block_0x0020a51c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 11usize, 0u32, 2139424u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2139428u32)?;
    emu.lw_no_count(13usize, 11usize, 8u32, 2139432u32)?;
    emu.lw_no_count(7usize, 11usize, 12u32, 2139436u32)?;
    emu.lw_no_count(16usize, 12usize, 0u32, 2139440u32)?;
    emu.lw_no_count(6usize, 12usize, 4u32, 2139444u32)?;
    emu.lw_no_count(30usize, 12usize, 8u32, 2139448u32)?;
    emu.lw_no_count(29usize, 12usize, 12u32, 2139452u32)?;
    emu.sltru_no_count(17usize, 14usize, 16usize, 2139456u32);
    emu.adi_no_count(15usize, 17usize, 0u32, 2139460u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2139468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a54c));
    } else {
        emu.pc = 2139464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a548));
    }
}
#[inline(always)]
pub fn block_0x0020a548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 5usize, 6usize, 2139468u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139468u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a54c));
}
#[inline]
pub fn block_0x0020a54c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2139472u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2139476u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2139480u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2139484u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2139488u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2139492u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2139496u32)?;
    emu.sltru_no_count(8usize, 13usize, 30usize, 2139500u32);
    emu.sbr_no_count(28usize, 7usize, 29usize, 2139504u32);
    emu.sbr_no_count(28usize, 28usize, 8usize, 2139508u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2139516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a57c));
    } else {
        emu.pc = 2139512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a578));
    }
}
#[inline(always)]
pub fn block_0x0020a578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 7usize, 29usize, 2139516u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a57c));
}
#[inline]
pub fn block_0x0020a57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 11usize, 16u32, 2139520u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2139524u32)?;
    emu.lw_no_count(29usize, 12usize, 16u32, 2139528u32)?;
    emu.lw_no_count(9usize, 12usize, 20u32, 2139532u32)?;
    emu.sbr_no_count(18usize, 0usize, 8usize, 2139536u32);
    emu.sbr_no_count(13usize, 13usize, 30usize, 2139540u32);
    emu.adr_no_count(30usize, 15usize, 8usize, 2139544u32);
    emu.sbr_no_count(20usize, 28usize, 15usize, 2139548u32);
    emu.sbr_no_count(8usize, 0usize, 30usize, 2139552u32);
    emu.sbr_no_count(15usize, 13usize, 15usize, 2139556u32);
    emu.sltru_no_count(18usize, 8usize, 18usize, 2139560u32);
    emu.sltru_no_count(19usize, 15usize, 13usize, 2139564u32);
    emu.adr_no_count(13usize, 20usize, 19usize, 2139568u32);
    emu.sbr_no_count(18usize, 18usize, 30usize, 2139572u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2139580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5bc));
    } else {
        emu.pc = 2139576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5b8));
    }
}
#[inline(always)]
pub fn block_0x0020a5b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(19usize, 13usize, 28usize, 2139580u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5bc));
}
#[inline(always)]
pub fn block_0x0020a5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(28usize, 19usize, 30usize, 2139584u32);
    emu.sltru_no_count(28usize, 28usize, 8usize, 2139588u32);
    emu.adr_no_count(28usize, 18usize, 28usize, 2139592u32);
    emu.sltru_no_count(21usize, 7usize, 29usize, 2139596u32);
    emu.sai_no_count(19usize, 28usize, 1055u32, 2139600u32);
    emu.adi_no_count(20usize, 21usize, 0u32, 2139604u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2139612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5dc));
    } else {
        emu.pc = 2139608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5d8));
    }
}
#[inline(always)]
pub fn block_0x0020a5d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 31usize, 9usize, 2139612u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5dc));
}
#[inline]
pub fn block_0x0020a5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(28usize, 11usize, 24u32, 2139616u32)?;
    emu.lw_no_count(8usize, 11usize, 28u32, 2139620u32)?;
    emu.lw_no_count(30usize, 12usize, 24u32, 2139624u32)?;
    emu.lw_no_count(18usize, 12usize, 28u32, 2139628u32)?;
    emu.sbr_no_count(11usize, 31usize, 9usize, 2139632u32);
    emu.sbr_no_count(12usize, 7usize, 29usize, 2139636u32);
    emu.sbr_no_count(31usize, 11usize, 21usize, 2139640u32);
    emu.adr_no_count(11usize, 12usize, 19usize, 2139644u32);
    emu.adr_no_count(7usize, 31usize, 19usize, 2139648u32);
    emu.sltru_no_count(29usize, 11usize, 12usize, 2139652u32);
    emu.adr_no_count(12usize, 7usize, 29usize, 2139656u32);
    emu.sbr_no_count(7usize, 0usize, 20usize, 2139660u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2139668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a614));
    } else {
        emu.pc = 2139664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a610));
    }
}
#[inline(always)]
pub fn block_0x0020a610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 12usize, 31usize, 2139668u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a614));
}
#[inline]
pub fn block_0x0020a614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 5usize, 6usize, 2139672u32);
    emu.sbr_no_count(6usize, 19usize, 20usize, 2139676u32);
    emu.adr_no_count(29usize, 6usize, 29usize, 2139680u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2139684u32);
    emu.sltru_no_count(29usize, 29usize, 6usize, 2139688u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2139692u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2139696u32);
    emu.sltru_no_count(29usize, 28usize, 30usize, 2139700u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2139704u32);
    emu.adi_no_count(7usize, 29usize, 0u32, 2139708u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2139716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a644));
    } else {
        emu.pc = 2139712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a640));
    }
}
#[inline(always)]
pub fn block_0x0020a640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 8usize, 18usize, 2139716u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a644));
}
#[inline]
pub fn block_0x0020a644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 5usize, 17usize, 2139720u32);
    emu.sbr_no_count(17usize, 14usize, 16usize, 2139724u32);
    emu.sbr_no_count(14usize, 8usize, 18usize, 2139728u32);
    emu.sbr_no_count(16usize, 28usize, 30usize, 2139732u32);
    emu.sbr_no_count(29usize, 14usize, 29usize, 2139736u32);
    emu.adr_no_count(14usize, 16usize, 6usize, 2139740u32);
    emu.adr_no_count(28usize, 29usize, 6usize, 2139744u32);
    emu.sltru_no_count(30usize, 14usize, 16usize, 2139748u32);
    emu.adr_no_count(16usize, 28usize, 30usize, 2139752u32);
    emu.sbr_no_count(28usize, 0usize, 7usize, 2139756u32);
    emu.lw_no_count(8usize, 2usize, 28u32, 2139760u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2139764u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2139768u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2139772u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2139776u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2139780u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139784u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2139792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a690));
    } else {
        emu.pc = 2139788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a68c));
    }
}
#[inline(always)]
pub fn block_0x0020a68c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 16usize, 29usize, 2139792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a690));
}
#[inline]
pub fn block_0x0020a690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(6usize, 6usize, 7usize, 2139796u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2139800u32);
    emu.sltru_no_count(7usize, 6usize, 28usize, 2139804u32);
    emu.sltru_no_count(28usize, 30usize, 6usize, 2139808u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2139812u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2139816u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2139820u32);
    emu.adr_no_count(17usize, 6usize, 17usize, 2139824u32);
    emu.sltru_no_count(7usize, 17usize, 6usize, 2139828u32);
    emu.adr_no_count(5usize, 6usize, 5usize, 2139832u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2139836u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2139844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6c4));
    } else {
        emu.pc = 2139840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6c0));
    }
}
#[inline(always)]
pub fn block_0x0020a6c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 6usize, 2139844u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139844u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6c4));
}
#[inline]
pub fn block_0x0020a6c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 6usize, 15usize, 2139848u32);
    emu.sltru_no_count(29usize, 28usize, 6usize, 2139852u32);
    emu.adr_no_count(15usize, 28usize, 7usize, 2139856u32);
    emu.adr_no_count(7usize, 13usize, 29usize, 2139860u32);
    emu.sltru_no_count(28usize, 15usize, 28usize, 2139864u32);
    emu.sltru_no_count(13usize, 0usize, 7usize, 2139868u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2139872u32);
    emu.anr_no_count(29usize, 13usize, 29usize, 2139876u32);
    emu.adr_no_count(13usize, 7usize, 28usize, 2139880u32);
    emu.sltru_no_count(7usize, 13usize, 7usize, 2139884u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2139888u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2139892u32);
    emu.sltru_no_count(28usize, 7usize, 29usize, 2139896u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2139900u32);
    emu.sltru_no_count(7usize, 11usize, 7usize, 2139904u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2139908u32);
    emu.adr_no_count(12usize, 12usize, 7usize, 2139912u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2139920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a710));
    } else {
        emu.pc = 2139916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a70c));
    }
}
#[inline(always)]
pub fn block_0x0020a70c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 28usize, 2139920u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139920u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a710));
}
#[inline]
pub fn block_0x0020a710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(28usize, 6usize, 1u32, 2139924u32);
    emu.sbr_no_count(14usize, 14usize, 6usize, 2139928u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2139932u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2139936u32)?;
    emu.sw_no_count(5usize, 10usize, 4u32, 2139940u32)?;
    emu.sw_no_count(15usize, 10usize, 8u32, 2139944u32)?;
    emu.sw_no_count(13usize, 10usize, 12u32, 2139948u32)?;
    emu.sltru_no_count(13usize, 14usize, 28usize, 2139952u32);
    emu.adr_no_count(7usize, 14usize, 7usize, 2139956u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2139960u32);
    emu.sltru_no_count(14usize, 7usize, 14usize, 2139964u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2139968u32);
    emu.sw_no_count(11usize, 10usize, 16u32, 2139972u32)?;
    emu.sw_no_count(12usize, 10usize, 20u32, 2139976u32)?;
    emu.sw_no_count(7usize, 10usize, 24u32, 2139980u32)?;
    emu.sw_no_count(13usize, 10usize, 28u32, 2139984u32)?;
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139988u32;
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
