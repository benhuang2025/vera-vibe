pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2219740u32;
pub const PC_MAX: u32 = 2221840u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 130usize] = [
        block_0x0021dedc,
        block_0x0021def4,
        block_0x0021df28,
        block_0x0021df2c,
        block_0x0021df34,
        block_0x0021df4c,
        block_0x0021df58,
        block_0x0021df64,
        block_0x0021df68,
        block_0x0021df6c,
        block_0x0021df84,
        block_0x0021df88,
        block_0x0021df94,
        block_0x0021dfb0,
        block_0x0021dfc0,
        block_0x0021dfd8,
        block_0x0021e014,
        block_0x0021e088,
        block_0x0021e090,
        block_0x0021e09c,
        block_0x0021e0a8,
        block_0x0021e0ac,
        block_0x0021e0c8,
        block_0x0021e0d0,
        block_0x0021e0d4,
        block_0x0021e0dc,
        block_0x0021e0e4,
        block_0x0021e0f0,
        block_0x0021e0f4,
        block_0x0021e108,
        block_0x0021e10c,
        block_0x0021e110,
        block_0x0021e118,
        block_0x0021e128,
        block_0x0021e12c,
        block_0x0021e134,
        block_0x0021e140,
        block_0x0021e148,
        block_0x0021e150,
        block_0x0021e154,
        block_0x0021e15c,
        block_0x0021e168,
        block_0x0021e170,
        block_0x0021e178,
        block_0x0021e17c,
        block_0x0021e180,
        block_0x0021e184,
        block_0x0021e188,
        block_0x0021e190,
        block_0x0021e194,
        block_0x0021e19c,
        block_0x0021e1a0,
        block_0x0021e1a8,
        block_0x0021e1b4,
        block_0x0021e1bc,
        block_0x0021e1c4,
        block_0x0021e1d0,
        block_0x0021e1d4,
        block_0x0021e1dc,
        block_0x0021e1e4,
        block_0x0021e1ec,
        block_0x0021e1f8,
        block_0x0021e200,
        block_0x0021e20c,
        block_0x0021e214,
        block_0x0021e224,
        block_0x0021e22c,
        block_0x0021e234,
        block_0x0021e23c,
        block_0x0021e240,
        block_0x0021e258,
        block_0x0021e298,
        block_0x0021e2ac,
        block_0x0021e2c0,
        block_0x0021e2c4,
        block_0x0021e2c8,
        block_0x0021e2cc,
        block_0x0021e2d0,
        block_0x0021e2e0,
        block_0x0021e2e4,
        block_0x0021e2e8,
        block_0x0021e2f0,
        block_0x0021e2f8,
        block_0x0021e308,
        block_0x0021e314,
        block_0x0021e318,
        block_0x0021e31c,
        block_0x0021e338,
        block_0x0021e340,
        block_0x0021e348,
        block_0x0021e350,
        block_0x0021e368,
        block_0x0021e380,
        block_0x0021e390,
        block_0x0021e398,
        block_0x0021e3a0,
        block_0x0021e3a8,
        block_0x0021e3b0,
        block_0x0021e3b8,
        block_0x0021e3ec,
        block_0x0021e3f4,
        block_0x0021e428,
        block_0x0021e508,
        block_0x0021e510,
        block_0x0021e518,
        block_0x0021e520,
        block_0x0021e534,
        block_0x0021e53c,
        block_0x0021e540,
        block_0x0021e558,
        block_0x0021e55c,
        block_0x0021e568,
        block_0x0021e574,
        block_0x0021e57c,
        block_0x0021e580,
        block_0x0021e598,
        block_0x0021e5a0,
        block_0x0021e5a8,
        block_0x0021e5c4,
        block_0x0021e5e0,
        block_0x0021e614,
        block_0x0021e618,
        block_0x0021e62c,
        block_0x0021e630,
        block_0x0021e638,
        block_0x0021e644,
        block_0x0021e6c0,
        block_0x0021e6c4,
        block_0x0021e6e8,
        block_0x0021e710,
    ];
    const IDX: [u16; 526usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 4u16, 0u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 9u16, 10u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 14u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 19u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16,
        22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 24u16, 25u16, 0u16,
        26u16, 0u16, 27u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 30u16,
        31u16, 32u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16, 36u16, 0u16,
        0u16, 37u16, 0u16, 38u16, 0u16, 39u16, 40u16, 0u16, 41u16, 0u16, 0u16, 42u16,
        0u16, 43u16, 0u16, 44u16, 45u16, 46u16, 47u16, 48u16, 0u16, 49u16, 50u16, 0u16,
        51u16, 52u16, 0u16, 53u16, 0u16, 0u16, 54u16, 0u16, 55u16, 0u16, 56u16, 0u16,
        0u16, 57u16, 58u16, 0u16, 59u16, 0u16, 60u16, 0u16, 61u16, 0u16, 0u16, 62u16,
        0u16, 63u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 66u16, 0u16,
        67u16, 0u16, 68u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16,
        75u16, 76u16, 77u16, 78u16, 0u16, 0u16, 0u16, 79u16, 80u16, 81u16, 0u16, 82u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 85u16, 86u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 0u16, 90u16, 0u16, 91u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16,
        0u16, 94u16, 0u16, 95u16, 0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 99u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16,
        0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 104u16, 0u16, 105u16, 0u16, 106u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 110u16, 111u16, 0u16, 0u16, 112u16, 0u16, 0u16, 113u16, 0u16, 114u16,
        115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 117u16, 0u16, 118u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 120u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 121u16,
        122u16, 0u16, 0u16, 0u16, 0u16, 123u16, 124u16, 0u16, 125u16, 0u16, 0u16, 126u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 127u16, 128u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 129u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 130u16,
    ];
    if pc < 2219740u32 || pc > 2221840u32 {
        return None;
    }
    let word_offset = ((pc - 2219740u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021dedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2219744u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2219748u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2219752u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2219756u32);
    emu.apc_no_count(6usize, 2219756u32, 4294963200u32, 2219760u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2219764u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021def4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2219768u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2219772u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2219776u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2219780u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2219784u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2219788u32)?;
    emu.lw_no_count(19usize, 11usize, 4u32, 2219792u32)?;
    emu.lw_no_count(8usize, 11usize, 0u32, 2219796u32)?;
    emu.lw_no_count(18usize, 19usize, 16u32, 2219800u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2219804u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2219808u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2219812u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2219816u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021df28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df34));
    } else {
        emu.pc = 2219820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df2c));
    }
}
#[inline(always)]
pub fn block_0x0021df2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2219824u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021df94));
}
#[inline(always)]
pub fn block_0x0021df34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2219832u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2219836u32);
    emu.adi_no_count(12usize, 0usize, 257u32, 2219840u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2219844u32);
    emu.apc_no_count(1usize, 2219844u32, 4294963200u32, 2219848u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021df4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 25u32, 2219856u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2219860u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2219884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df6c));
    } else {
        emu.pc = 2219864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df58));
    }
}
#[inline(always)]
pub fn block_0x0021df58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2219868u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2219872u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2219876u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021df64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df2c));
    } else {
        emu.pc = 2219880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df68));
    }
}
#[inline(always)]
pub fn block_0x0021df68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021df88));
}
#[inline(always)]
pub fn block_0x0021df6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 24u32, 2219888u32);
    emu.lw_no_count(13usize, 19usize, 12u32, 2219892u32)?;
    emu.sbr_no_count(12usize, 10usize, 11usize, 2219896u32);
    emu.adr_no_count(11usize, 9usize, 11usize, 2219900u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2219904u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219908u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021df84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df2c));
    } else {
        emu.pc = 2219912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df88));
    }
}
#[inline(always)]
pub fn block_0x0021df88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2219916u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2219920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2219924u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021df94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2219928u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2219932u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2219936u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2219940u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2219944u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2219948u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219952u32;
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
pub fn block_0x0021dfb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2219956u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2219960u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2219964u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2219968u32;
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
pub fn block_0x0021dfc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2219972u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2219976u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2219980u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2219984u32);
    emu.apc_no_count(6usize, 2219984u32, 4294963200u32, 2219988u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2219992u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021dfd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2219996u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2220000u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2220004u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2220008u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2220012u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2220016u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2220020u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2220024u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2220028u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2220032u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2220036u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2220040u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2220044u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2220048u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2220564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e214));
    } else {
        emu.pc = 2220052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e014));
    }
}
#[inline(never)]
pub fn block_0x0021e014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2220056u32);
    emu.adi_no_count(22usize, 12usize, 4294967289u32, 2220060u32);
    emu.adi_no_count(25usize, 11usize, 3u32, 2220064u32);
    emu.adi_no_count(14usize, 11usize, 4u32, 2220068u32);
    emu.sbr_no_count(15usize, 0usize, 12usize, 2220072u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2220076u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966296u32, 2220080u32);
    emu.adi_no_count(17usize, 0usize, 4u32, 2220084u32);
    emu.adi_no_count(5usize, 0usize, 240u32, 2220088u32);
    emu.adi_no_count(6usize, 0usize, 48u32, 2220092u32);
    emu.adi_no_count(7usize, 0usize, 4294967231u32, 2220096u32);
    emu.adi_no_count(28usize, 0usize, 4294967232u32, 2220100u32);
    emu.adi_no_count(29usize, 0usize, 244u32, 2220104u32);
    emu.adi_no_count(30usize, 0usize, 4294967183u32, 2220108u32);
    emu.adi_no_count(31usize, 0usize, 2u32, 2220112u32);
    emu.adi_no_count(8usize, 0usize, 3u32, 2220116u32);
    emu.adi_no_count(9usize, 0usize, 224u32, 2220120u32);
    emu.adi_no_count(18usize, 0usize, 160u32, 2220124u32);
    emu.adi_no_count(19usize, 0usize, 237u32, 2220128u32);
    emu.adi_no_count(20usize, 0usize, 4294967199u32, 2220132u32);
    emu.adi_no_count(21usize, 0usize, 12u32, 2220136u32);
    emu.sltru_no_count(24usize, 12usize, 22usize, 2220140u32);
    emu.adi_no_count(24usize, 24usize, 4294967295u32, 2220144u32);
    emu.anr_no_count(24usize, 24usize, 22usize, 2220148u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2220152u32;
    emu.update_insn_clock();
    emu.ani_no_count(25usize, 25usize, 4294967292u32, 2220156u32);
    emu.sbr_no_count(25usize, 25usize, 11usize, 2220160u32);
    emu.adi_no_count(26usize, 22usize, 128u32, 2220164u32);
    emu.add_memory_rw_events(29usize);
    let return_addr = 2220168u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e090));
}
#[inline(always)]
pub fn block_0x0021e088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2220172u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2220564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e214));
    } else {
        emu.pc = 2220176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e090));
    }
}
#[inline(always)]
pub fn block_0x0021e090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 11usize, 13usize, 2220180u32);
    emu.lb_no_count(22usize, 22usize, 0u32, 2220184u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0f4));
    } else {
        emu.pc = 2220188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e09c));
    }
}
#[inline(always)]
pub fn block_0x0021e09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(22usize, 25usize, 13usize, 2220192u32);
    emu.ani_no_count(22usize, 22usize, 3u32, 2220196u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a != b {
        emu.pc = 2220168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e088));
    } else {
        emu.pc = 2220200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0a8));
    }
}
#[inline(always)]
pub fn block_0x0021e0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2220240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0d0));
    } else {
        emu.pc = 2220204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0ac));
    }
}
#[inline(always)]
pub fn block_0x0021e0ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 11usize, 13usize, 2220208u32);
    emu.adr_no_count(27usize, 14usize, 13usize, 2220212u32);
    emu.lw_no_count(22usize, 22usize, 0u32, 2220216u32)?;
    emu.lw_no_count(27usize, 27usize, 0u32, 2220220u32)?;
    emu.orr_no_count(22usize, 27usize, 22usize, 2220224u32);
    emu.anr_no_count(22usize, 22usize, 26usize, 2220228u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a != b {
        emu.pc = 2220240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0d0));
    } else {
        emu.pc = 2220232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0c8));
    }
}
#[inline(always)]
pub fn block_0x0021e0c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 8u32, 2220236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2220204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0ac));
    } else {
        emu.pc = 2220240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0d0));
    }
}
#[inline(always)]
pub fn block_0x0021e0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2220416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e180));
    } else {
        emu.pc = 2220244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0d4));
    }
}
#[inline(always)]
pub fn block_0x0021e0d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(22usize, 0usize, 13usize, 2220248u32);
    emu.adr_no_count(13usize, 11usize, 13usize, 2220252u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2220252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e0dc));
}
#[inline(always)]
pub fn block_0x0021e0dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(27usize, 13usize, 0u32, 2220256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(27usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e17c));
    } else {
        emu.pc = 2220260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0e4));
    }
}
#[inline(always)]
pub fn block_0x0021e0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 4294967295u32, 2220264u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2220268u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2220252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0dc));
    } else {
        emu.pc = 2220272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0f0));
    }
}
#[inline(always)]
pub fn block_0x0021e0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220564u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e214));
}
#[inline(always)]
pub fn block_0x0021e0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(27usize, 22usize, 255u32, 2220280u32);
    emu.adr_no_count(22usize, 16usize, 27usize, 2220284u32);
    emu.lbu_no_count(1usize, 22usize, 0u32, 2220288u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2220292u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a == b {
        emu.pc = 2220372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e154));
    } else {
        emu.pc = 2220296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e108));
    }
}
#[inline(always)]
pub fn block_0x0021e108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a == b {
        emu.pc = 2220332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e12c));
    } else {
        emu.pc = 2220300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e10c));
    }
}
#[inline(always)]
pub fn block_0x0021e10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a != b {
        emu.pc = 2220588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e22c));
    } else {
        emu.pc = 2220304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e110));
    }
}
#[inline(always)]
pub fn block_0x0021e110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 1u32, 2220308u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2220580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e224));
    } else {
        emu.pc = 2220312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e118));
    }
}
#[inline(always)]
pub fn block_0x0021e118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 11usize, 1usize, 2220316u32);
    emu.lb_no_count(23usize, 27usize, 0u32, 2220320u32);
    emu.adi_no_count(27usize, 0usize, 1u32, 2220324u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e20c));
    } else {
        emu.pc = 2220328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e128));
    }
}
#[inline(always)]
pub fn block_0x0021e128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e240));
}
#[inline(always)]
pub fn block_0x0021e12c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 1u32, 2220336u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2220580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e224));
    } else {
        emu.pc = 2220340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e134));
    }
}
#[inline(always)]
pub fn block_0x0021e134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(1usize, 11usize, 1usize, 2220344u32);
    emu.lbu_no_count(1usize, 1usize, 0u32, 2220348u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2220424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e188));
    } else {
        emu.pc = 2220352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e140));
    }
}
#[inline(always)]
pub fn block_0x0021e140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 1usize, 24u32, 2220356u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2220448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1a0));
    } else {
        emu.pc = 2220360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e148));
    }
}
#[inline(always)]
pub fn block_0x0021e148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2220364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1bc));
    } else {
        emu.pc = 2220368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e150));
    }
}
#[inline(always)]
pub fn block_0x0021e150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e22c));
}
#[inline(always)]
pub fn block_0x0021e154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 1u32, 2220376u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2220580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e224));
    } else {
        emu.pc = 2220380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e15c));
    }
}
#[inline(always)]
pub fn block_0x0021e15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(1usize, 11usize, 1usize, 2220384u32);
    emu.lbu_no_count(1usize, 1usize, 0u32, 2220388u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2220436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e194));
    } else {
        emu.pc = 2220392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e168));
    }
}
#[inline(always)]
pub fn block_0x0021e168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 1usize, 24u32, 2220396u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2220500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1d4));
    } else {
        emu.pc = 2220400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e170));
    }
}
#[inline(always)]
pub fn block_0x0021e170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2220404u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(30usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1e4));
    } else {
        emu.pc = 2220408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e178));
    }
}
#[inline(always)]
pub fn block_0x0021e178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e22c));
}
#[inline(always)]
pub fn block_0x0021e17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 22usize, 2220416u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2220416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e180));
}
#[inline(always)]
pub fn block_0x0021e180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2220176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e090));
    } else {
        emu.pc = 2220420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e184));
    }
}
#[inline(always)]
pub fn block_0x0021e184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220424u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220564u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e214));
}
#[inline(always)]
pub fn block_0x0021e188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(23usize, 1usize, 224u32, 2220428u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2220476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1bc));
    } else {
        emu.pc = 2220432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e190));
    }
}
#[inline(always)]
pub fn block_0x0021e190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e22c));
}
#[inline(always)]
pub fn block_0x0021e194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 1usize, 4294967152u32, 2220440u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2220516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1e4));
    } else {
        emu.pc = 2220444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e19c));
    }
}
#[inline(always)]
pub fn block_0x0021e19c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e22c));
}
#[inline(always)]
pub fn block_0x0021e1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 27usize, 4294967071u32, 2220452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2220468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1b4));
    } else {
        emu.pc = 2220456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1a8));
    }
}
#[inline(always)]
pub fn block_0x0021e1a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(23usize, 27usize, 254u32, 2220460u32);
    emu.adi_no_count(27usize, 0usize, 238u32, 2220464u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2220588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e22c));
    } else {
        emu.pc = 2220468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1b4));
    }
}
#[inline(always)]
pub fn block_0x0021e1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2220472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(23usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e22c));
    } else {
        emu.pc = 2220476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1bc));
    }
}
#[inline(always)]
pub fn block_0x0021e1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 2u32, 2220480u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2220580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e224));
    } else {
        emu.pc = 2220484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1c4));
    }
}
#[inline(always)]
pub fn block_0x0021e1c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 11usize, 1usize, 2220488u32);
    emu.lb_no_count(23usize, 23usize, 0u32, 2220492u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e20c));
    } else {
        emu.pc = 2220496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1d0));
    }
}
#[inline(always)]
pub fn block_0x0021e1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e234));
}
#[inline(always)]
pub fn block_0x0021e1d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 27usize, 4294967055u32, 2220504u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a < b {
        emu.pc = 2220588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e22c));
    } else {
        emu.pc = 2220508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1dc));
    }
}
#[inline(always)]
pub fn block_0x0021e1dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2220512u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(23usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e22c));
    } else {
        emu.pc = 2220516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1e4));
    }
}
#[inline(always)]
pub fn block_0x0021e1e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 13usize, 2u32, 2220520u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2220580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e224));
    } else {
        emu.pc = 2220524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1ec));
    }
}
#[inline(always)]
pub fn block_0x0021e1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 11usize, 27usize, 2220528u32);
    emu.lb_no_count(23usize, 27usize, 0u32, 2220532u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e234));
    } else {
        emu.pc = 2220536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1f8));
    }
}
#[inline(always)]
pub fn block_0x0021e1f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 3u32, 2220540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2220580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e224));
    } else {
        emu.pc = 2220544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e200));
    }
}
#[inline(always)]
pub fn block_0x0021e200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 11usize, 1usize, 2220548u32);
    emu.lb_no_count(23usize, 23usize, 0u32, 2220552u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(23usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e23c));
    } else {
        emu.pc = 2220556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e20c));
    }
}
#[inline(always)]
pub fn block_0x0021e20c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 1usize, 1u32, 2220560u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2220176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e090));
    } else {
        emu.pc = 2220564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e214));
    }
}
#[inline(always)]
pub fn block_0x0021e214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2220568u32);
    emu.sw_no_count(11usize, 10usize, 4u32, 2220572u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2220576u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2220580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220632u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e258));
}
#[inline(always)]
pub fn block_0x0021e224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 0u32, 2220584u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e240));
}
#[inline(always)]
pub fn block_0x0021e22c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 1u32, 2220592u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e240));
}
#[inline(always)]
pub fn block_0x0021e234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 2u32, 2220600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e240));
}
#[inline(always)]
pub fn block_0x0021e23c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 3u32, 2220608u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2220608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e240));
}
#[inline(always)]
pub fn block_0x0021e240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 27usize, 255u32, 2220612u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2220616u32);
    emu.orr_no_count(11usize, 22usize, 11usize, 2220620u32);
    emu.sw_no_count(13usize, 10usize, 4u32, 2220624u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2220628u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2220632u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2220632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e258));
}
#[inline]
pub fn block_0x0021e258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 10usize, 0u32, 2220636u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2220640u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2220644u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2220648u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2220652u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2220656u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2220660u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2220664u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2220668u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2220672u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2220676u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2220680u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2220684u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2220688u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2220692u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220696u32;
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
pub fn block_0x0021e298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2220700u32);
    emu.sri_no_count(6usize, 10usize, 8u32, 2220704u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2220708u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2220712u32);
    emu.ani_no_count(7usize, 10usize, 255u32, 2220716u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2220716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e2ac));
}
#[inline(always)]
pub fn block_0x0021e2ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(29usize, 11usize, 0u32, 2220720u32);
    emu.lbu_no_count(28usize, 11usize, 1u32, 2220724u32);
    emu.adi_no_count(11usize, 11usize, 2u32, 2220728u32);
    emu.adr_no_count(5usize, 17usize, 28usize, 2220732u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2220772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2e4));
    } else {
        emu.pc = 2220736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2c0));
    }
}
#[inline(always)]
pub fn block_0x0021e2c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2220880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e350));
    } else {
        emu.pc = 2220740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2c4));
    }
}
#[inline(always)]
pub fn block_0x0021e2c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2220904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e368));
    } else {
        emu.pc = 2220744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2c8));
    }
}
#[inline(always)]
pub fn block_0x0021e2c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 13usize, 17usize, 2220748u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2220748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e2cc));
}
#[inline(always)]
pub fn block_0x0021e2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2220776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2e8));
    } else {
        emu.pc = 2220752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2d0));
    }
}
#[inline(always)]
pub fn block_0x0021e2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(29usize, 17usize, 0u32, 2220756u32);
    emu.adi_no_count(17usize, 17usize, 1u32, 2220760u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2220764u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2220748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2cc));
    } else {
        emu.pc = 2220768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2e0));
    }
}
#[inline(always)]
pub fn block_0x0021e2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220772u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e348));
}
#[inline(always)]
pub fn block_0x0021e2e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2220784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2f0));
    } else {
        emu.pc = 2220776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2e8));
    }
}
#[inline(always)]
pub fn block_0x0021e2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 5usize, 0u32, 2220780u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2220716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2ac));
    } else {
        emu.pc = 2220784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2f0));
    }
}
#[inline(always)]
pub fn block_0x0021e2f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 15usize, 16usize, 2220788u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2220792u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2220792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e2f8));
}
#[inline(always)]
pub fn block_0x0021e2f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 15usize, 0u32, 2220796u32);
    emu.adi_no_count(13usize, 15usize, 1u32, 2220800u32);
    emu.ani_no_count(12usize, 14usize, 255u32, 2220804u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e318));
    } else {
        emu.pc = 2220808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e308));
    }
}
#[inline(always)]
pub fn block_0x0021e308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2220812u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2220816u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e338));
    } else {
        emu.pc = 2220820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e314));
    }
}
#[inline(always)]
pub fn block_0x0021e314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220864u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e340));
}
#[inline(always)]
pub fn block_0x0021e318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2220928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e380));
    } else {
        emu.pc = 2220828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e31c));
    }
}
#[inline(always)]
pub fn block_0x0021e31c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 15usize, 1u32, 2220832u32);
    emu.adi_no_count(15usize, 15usize, 2u32, 2220836u32);
    emu.ani_no_count(12usize, 12usize, 127u32, 2220840u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2220844u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2220848u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2220852u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e340));
    } else {
        emu.pc = 2220856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e338));
    }
}
#[inline(always)]
pub fn block_0x0021e338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(11usize, 11usize, 1u32, 2220860u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2220792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2f8));
    } else {
        emu.pc = 2220864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e340));
    }
}
#[inline(always)]
pub fn block_0x0021e340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2220868u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220872u32;
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
pub fn block_0x0021e348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 0usize, 1u32, 2220876u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220880u32;
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
pub fn block_0x0021e350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220884u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966552u32, 2220888u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2220892u32);
    emu.adi_no_count(11usize, 5usize, 0u32, 2220896u32);
    emu.apc_no_count(1usize, 2220896u32, 0u32, 2220900u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220908u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966552u32, 2220912u32);
    emu.adi_no_count(10usize, 5usize, 0u32, 2220916u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2220920u32);
    emu.apc_no_count(1usize, 2220920u32, 0u32, 2220924u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(408u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2220932u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966568u32, 2220936u32);
    emu.apc_no_count(1usize, 2220936u32, 4294959104u32, 2220940u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 32u32, 2220948u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2220960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3a0));
    } else {
        emu.pc = 2220952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e398));
    }
}
#[inline(always)]
pub fn block_0x0021e398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2220956u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220960u32;
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
pub fn block_0x0021e3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 127u32, 2220964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2220976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3b0));
    } else {
        emu.pc = 2220968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3a8));
    }
}
#[inline(always)]
pub fn block_0x0021e3a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2220972u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220976u32;
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
pub fn block_0x0021e3b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 16u32, 2220980u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2221036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3ec));
    } else {
        emu.pc = 2220984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3b8));
    }
}
#[inline]
pub fn block_0x0021e3b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2220988u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2220992u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 70u32, 2220996u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2221000u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 150u32, 2221004u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2221008u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 440u32, 2221012u32);
    emu.adi_no_count(12usize, 0usize, 40u32, 2221016u32);
    emu.adi_no_count(14usize, 0usize, 290u32, 2221020u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2221024u32);
    emu.adi_no_count(16usize, 0usize, 297u32, 2221028u32);
    emu.apc_no_count(6usize, 2221028u32, 0u32, 2221032u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2221036u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e3ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 17u32, 2221040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2221096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e428));
    } else {
        emu.pc = 2221044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3f4));
    }
}
#[inline]
pub fn block_0x0021e3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2221048u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221052u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966584u32, 2221056u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2221060u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966672u32, 2221064u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2221068u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966880u32, 2221072u32);
    emu.adi_no_count(12usize, 0usize, 44u32, 2221076u32);
    emu.adi_no_count(14usize, 0usize, 208u32, 2221080u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2221084u32);
    emu.adi_no_count(16usize, 0usize, 486u32, 2221088u32);
    emu.apc_no_count(6usize, 2221088u32, 0u32, 2221092u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2221096u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021e428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221100u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(172032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2221104u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294791168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2221108u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294782976u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2221112u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294774784u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2221116u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294770688u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2221120u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294766592u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2221124u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294049792u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2221128u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2221132u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967264u32, 2221136u32);
    emu.adi_no_count(12usize, 12usize, 1760u32, 2221140u32);
    emu.adi_no_count(13usize, 13usize, 4294965440u32, 2221144u32);
    emu.adi_no_count(14usize, 14usize, 336u32, 2221148u32);
    emu.anr_no_count(7usize, 10usize, 11usize, 2221152u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2221156u32);
    emu.adi_no_count(7usize, 15usize, 1040u32, 2221160u32);
    emu.adi_no_count(15usize, 15usize, 4294965248u32, 2221164u32);
    emu.adr_no_count(16usize, 10usize, 16usize, 2221168u32);
    emu.adi_no_count(17usize, 17usize, 4294966448u32, 2221172u32);
    emu.adi_no_count(5usize, 5usize, 4294967040u32, 2221176u32);
    emu.adi_no_count(6usize, 6usize, 496u32, 2221180u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2221184u32);
    emu.adi_no_count(11usize, 11usize, 30u32, 2221188u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2221192u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2221196u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2221200u32);
    emu.adr_no_count(17usize, 10usize, 17usize, 2221204u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2221208u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2221212u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2221216u32);
    let a = 0u32.wrapping_add(4294963200u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221220u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1630u32, 2221224u32);
    emu.sltru_no_count(11usize, 15usize, 11usize, 2221228u32);
    let a = 0u32.wrapping_add(4294254592u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2221232u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 688u32, 2221236u32);
    emu.sltru_no_count(15usize, 5usize, 15usize, 2221240u32);
    let a = 0u32.wrapping_add(180224u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2221244u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294965278u32, 2221248u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2221252u32);
    emu.sltiu_no_count(14usize, 14usize, 4294967282u32, 2221256u32);
    emu.sltiu_no_count(5usize, 7usize, 4294967281u32, 2221260u32);
    emu.anr_no_count(14usize, 14usize, 5usize, 2221264u32);
    emu.sltiu_no_count(17usize, 17usize, 4294967291u32, 2221268u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2221272u32);
    emu.sltiu_no_count(13usize, 13usize, 4294967290u32, 2221276u32);
    emu.sltru_no_count(12usize, 0usize, 12usize, 2221280u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2221284u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2221288u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2221292u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2221296u32);
    emu.sltiu_no_count(12usize, 16usize, 4294965790u32, 2221300u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2221304u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2221308u32);
    emu.anr_no_count(11usize, 15usize, 6usize, 2221312u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2221316u32);
    emu.add_memory_rw_events(56usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221320u32;
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
pub fn block_0x0021e508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2221320u32, 0u32, 2221324u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2221328u32, 0u32, 2221332u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2221336u32, 0u32, 2221340u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2221348u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2221352u32);
    emu.ani_no_count(10usize, 10usize, 4294967292u32, 2221356u32);
    emu.sbr_no_count(5usize, 10usize, 12usize, 2221360u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2221404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e55c));
    } else {
        emu.pc = 2221364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e534));
    }
}
#[inline(always)]
pub fn block_0x0021e534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2221368u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2221400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e558));
    } else {
        emu.pc = 2221372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e53c));
    }
}
#[inline(always)]
pub fn block_0x0021e53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 12usize, 11usize, 2221376u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2221376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e540));
}
#[inline(always)]
pub fn block_0x0021e540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2221380u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2221384u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2221388u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2221392u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2221396u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2221376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e540));
    } else {
        emu.pc = 2221400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e558));
    }
}
#[inline(always)]
pub fn block_0x0021e558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221404u32;
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
pub fn block_0x0021e55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 11usize, 5usize, 2221408u32);
    emu.sri_no_count(17usize, 13usize, 2u32, 2221412u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2221364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e534));
    } else {
        emu.pc = 2221416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e568));
    }
}
#[inline(always)]
pub fn block_0x0021e568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 12usize, 5usize, 2221420u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2221424u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e57c));
    } else {
        emu.pc = 2221428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e574));
    }
}
#[inline(always)]
pub fn block_0x0021e574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2221432u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2221436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e598));
}
#[inline(always)]
pub fn block_0x0021e57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2221440u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2221440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e580));
}
#[inline(always)]
pub fn block_0x0021e580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 12usize, 0u32, 2221444u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2221448u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2221452u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2221456u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2221460u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2221440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e580));
    } else {
        emu.pc = 2221464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e598));
    }
}
#[inline(always)]
pub fn block_0x0021e598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2221468u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2221508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e5c4));
    } else {
        emu.pc = 2221472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e5a0));
    }
}
#[inline(always)]
pub fn block_0x0021e5a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2221476u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2221480u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2221480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e5a8));
}
#[inline(always)]
pub fn block_0x0021e5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2221484u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2221488u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2221492u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2221496u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2221500u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2221504u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2221480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e5a8));
    } else {
        emu.pc = 2221508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e5c4));
    }
}
#[inline(always)]
pub fn block_0x0021e5c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221512u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16711680u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2221516u32;
    emu.update_insn_clock();
    emu.adr_no_count(10usize, 12usize, 10usize, 2221520u32);
    emu.adi_no_count(12usize, 11usize, 257u32, 2221524u32);
    emu.adi_no_count(11usize, 13usize, 255u32, 2221528u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2221532u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2221536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e614));
}
#[inline]
pub fn block_0x0021e5e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(6usize, 16usize, 2u32, 2221540u32);
    emu.sbr_no_count(17usize, 13usize, 16usize, 2221544u32);
    emu.ani_no_count(7usize, 16usize, 3u32, 2221548u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2221552u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2221556u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2221560u32);
    emu.anr_no_count(6usize, 29usize, 11usize, 2221564u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2221568u32);
    emu.sli_no_count(28usize, 6usize, 16u32, 2221572u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2221576u32);
    emu.sri_no_count(6usize, 6usize, 16u32, 2221580u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2221584u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2221764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e6c4));
    } else {
        emu.pc = 2221588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e614));
    }
}
#[inline(always)]
pub fn block_0x0021e614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2221400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e558));
    } else {
        emu.pc = 2221592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e618));
    }
}
#[inline(always)]
pub fn block_0x0021e618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 17usize, 0u32, 2221596u32);
    emu.adi_no_count(15usize, 5usize, 0u32, 2221600u32);
    emu.adi_no_count(17usize, 0usize, 192u32, 2221604u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2221608u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2221616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e630));
    } else {
        emu.pc = 2221612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e62c));
    }
}
#[inline(always)]
pub fn block_0x0021e62c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 192u32, 2221616u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2221616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e630));
}
#[inline(always)]
pub fn block_0x0021e630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2221620u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2221536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e5e0));
    } else {
        emu.pc = 2221624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e638));
    }
}
#[inline(always)]
pub fn block_0x0021e638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 2u32, 2221628u32);
    emu.sli_no_count(17usize, 17usize, 4u32, 2221632u32);
    emu.adi_no_count(6usize, 15usize, 0u32, 2221636u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2221636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e644));
}
#[inline(never)]
pub fn block_0x0021e644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 6usize, 0u32, 2221640u32)?;
    emu.lw_no_count(28usize, 6usize, 4u32, 2221644u32)?;
    emu.lw_no_count(29usize, 6usize, 8u32, 2221648u32)?;
    emu.lw_no_count(30usize, 6usize, 12u32, 2221652u32)?;
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2221656u32);
    emu.sri_no_count(7usize, 7usize, 6u32, 2221660u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2221664u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2221668u32);
    emu.xri_no_count(31usize, 28usize, 4294967295u32, 2221672u32);
    emu.sri_no_count(28usize, 28usize, 6u32, 2221676u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2221680u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2221684u32);
    emu.xri_no_count(31usize, 29usize, 4294967295u32, 2221688u32);
    emu.sri_no_count(29usize, 29usize, 6u32, 2221692u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2221696u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2221700u32);
    emu.xri_no_count(31usize, 30usize, 4294967295u32, 2221704u32);
    emu.sri_no_count(30usize, 30usize, 6u32, 2221708u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2221712u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2221716u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2221720u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2221724u32);
    emu.adi_no_count(17usize, 17usize, 4294967280u32, 2221728u32);
    emu.anr_no_count(7usize, 28usize, 12usize, 2221732u32);
    emu.anr_no_count(28usize, 29usize, 12usize, 2221736u32);
    emu.anr_no_count(29usize, 30usize, 12usize, 2221740u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2221744u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2221748u32);
    emu.adr_no_count(5usize, 5usize, 29usize, 2221752u32);
    emu.adi_no_count(6usize, 6usize, 16u32, 2221756u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2221636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e644));
    } else {
        emu.pc = 2221760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e6c0));
    }
}
#[inline(always)]
pub fn block_0x0021e6c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2221764u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e5e0));
}
#[inline]
pub fn block_0x0021e6c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2221768u32);
    emu.ani_no_count(16usize, 16usize, 252u32, 2221772u32);
    emu.sli_no_count(16usize, 16usize, 2u32, 2221776u32);
    emu.adr_no_count(15usize, 15usize, 16usize, 2221780u32);
    emu.sltiu_no_count(16usize, 13usize, 192u32, 2221784u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2221788u32);
    emu.anr_no_count(13usize, 13usize, 16usize, 2221792u32);
    emu.ani_no_count(13usize, 13usize, 3u32, 2221796u32);
    emu.sli_no_count(13usize, 13usize, 2u32, 2221800u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2221800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e6e8));
}
#[inline]
pub fn block_0x0021e6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2221804u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2221808u32);
    emu.xri_no_count(17usize, 16usize, 4294967295u32, 2221812u32);
    emu.sri_no_count(16usize, 16usize, 6u32, 2221816u32);
    emu.sri_no_count(17usize, 17usize, 7u32, 2221820u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2221824u32);
    emu.anr_no_count(16usize, 16usize, 12usize, 2221828u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2221832u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2221836u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2221800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e6e8));
    } else {
        emu.pc = 2221840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e710));
    }
}
#[inline]
pub fn block_0x0021e710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(12usize, 14usize, 11usize, 2221844u32);
    emu.sri_no_count(14usize, 14usize, 8u32, 2221848u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2221852u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2221856u32);
    emu.sli_no_count(12usize, 11usize, 16u32, 2221860u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2221864u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2221868u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2221872u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221876u32;
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
