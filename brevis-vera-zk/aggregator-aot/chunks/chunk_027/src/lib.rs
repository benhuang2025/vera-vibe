pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2218928u32;
pub const PC_MAX: u32 = 2220780u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0021dbb0,
        block_0x0021dc10,
        block_0x0021dc38,
        block_0x0021dc54,
        block_0x0021dc64,
        block_0x0021dc6c,
        block_0x0021dc7c,
        block_0x0021dc98,
        block_0x0021dcb0,
        block_0x0021dcb4,
        block_0x0021dce8,
        block_0x0021dd24,
        block_0x0021dd50,
        block_0x0021dd9c,
        block_0x0021dda4,
        block_0x0021ddd4,
        block_0x0021dde0,
        block_0x0021ddf8,
        block_0x0021ddfc,
        block_0x0021de0c,
        block_0x0021de10,
        block_0x0021de14,
        block_0x0021de28,
        block_0x0021de2c,
        block_0x0021de78,
        block_0x0021de7c,
        block_0x0021de98,
        block_0x0021de9c,
        block_0x0021dea0,
        block_0x0021deac,
        block_0x0021decc,
        block_0x0021ded0,
        block_0x0021deec,
        block_0x0021def4,
        block_0x0021df00,
        block_0x0021df14,
        block_0x0021df28,
        block_0x0021df80,
        block_0x0021df88,
        block_0x0021df90,
        block_0x0021dfd0,
        block_0x0021dfd4,
        block_0x0021dfe4,
        block_0x0021dff0,
        block_0x0021e000,
        block_0x0021e004,
        block_0x0021e008,
        block_0x0021e014,
        block_0x0021e018,
        block_0x0021e028,
        block_0x0021e040,
        block_0x0021e05c,
        block_0x0021e080,
        block_0x0021e08c,
        block_0x0021e098,
        block_0x0021e0ac,
        block_0x0021e0c0,
        block_0x0021e0c8,
        block_0x0021e0d0,
        block_0x0021e0d4,
        block_0x0021e0e0,
        block_0x0021e0e4,
        block_0x0021e0e8,
        block_0x0021e0ec,
        block_0x0021e0f0,
        block_0x0021e104,
        block_0x0021e108,
        block_0x0021e114,
        block_0x0021e134,
        block_0x0021e138,
        block_0x0021e144,
        block_0x0021e154,
        block_0x0021e158,
        block_0x0021e15c,
        block_0x0021e174,
        block_0x0021e178,
        block_0x0021e180,
        block_0x0021e18c,
        block_0x0021e198,
        block_0x0021e1a0,
        block_0x0021e1ac,
        block_0x0021e1b8,
        block_0x0021e1c0,
        block_0x0021e1c8,
        block_0x0021e1d0,
        block_0x0021e1e0,
        block_0x0021e1f4,
        block_0x0021e1f8,
        block_0x0021e1fc,
        block_0x0021e20c,
        block_0x0021e210,
        block_0x0021e218,
        block_0x0021e21c,
        block_0x0021e220,
        block_0x0021e224,
        block_0x0021e234,
        block_0x0021e23c,
        block_0x0021e244,
        block_0x0021e248,
        block_0x0021e260,
        block_0x0021e264,
        block_0x0021e270,
        block_0x0021e274,
        block_0x0021e2b4,
        block_0x0021e2bc,
        block_0x0021e2d0,
        block_0x0021e2ec,
    ];
    const IDX: [u16; 464usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 0u16, 0u16, 7u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16,
        10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 17u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 0u16, 0u16, 20u16, 21u16, 22u16, 0u16,
        0u16, 0u16, 0u16, 23u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 28u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        33u16, 0u16, 34u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16,
        39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16,
        44u16, 0u16, 0u16, 0u16, 45u16, 46u16, 47u16, 0u16, 0u16, 48u16, 49u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        0u16, 54u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16,
        0u16, 57u16, 0u16, 58u16, 0u16, 59u16, 60u16, 0u16, 0u16, 61u16, 62u16, 63u16,
        64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 68u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 71u16, 0u16, 0u16,
        0u16, 72u16, 73u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16,
        77u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 0u16, 80u16, 0u16, 0u16, 81u16,
        0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 84u16, 0u16, 85u16, 0u16, 0u16, 0u16,
        86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 89u16, 0u16, 0u16, 0u16, 90u16,
        91u16, 0u16, 92u16, 93u16, 94u16, 95u16, 0u16, 0u16, 0u16, 96u16, 0u16, 97u16,
        0u16, 98u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 101u16, 0u16, 0u16,
        102u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2218928u32 || pc > 2220780u32 {
        return None;
    }
    let word_offset = ((pc - 2218928u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021dbb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2218932u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2218936u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2218940u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2218944u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2218948u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2218952u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2218956u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2218960u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2218964u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2218968u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2218972u32)?;
    emu.adi_no_count(8usize, 17usize, 0u32, 2218976u32);
    emu.adi_no_count(9usize, 16usize, 0u32, 2218980u32);
    emu.adi_no_count(18usize, 15usize, 0u32, 2218984u32);
    emu.adi_no_count(19usize, 14usize, 0u32, 2218988u32);
    emu.adi_no_count(20usize, 13usize, 0u32, 2218992u32);
    emu.adi_no_count(21usize, 10usize, 0u32, 2218996u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2219000u32)?;
    emu.lw_no_count(13usize, 21usize, 4u32, 2219004u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2219008u32)?;
    emu.lw_no_count(23usize, 2usize, 52u32, 2219012u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2219016u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2219020u32)?;
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219024u32;
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
#[inline]
pub fn block_0x0021dc10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 0u32, 2219028u32)?;
    emu.sb_no_count(10usize, 2usize, 4u32, 2219032u32);
    emu.sb_no_count(0usize, 2usize, 5u32, 2219036u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2219040u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2219044u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2219048u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2219052u32);
    emu.adi_no_count(14usize, 9usize, 0u32, 2219056u32);
    emu.apc_no_count(1usize, 2219056u32, 4294959104u32, 2219060u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021dc38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2219068u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2219072u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2219076u32);
    emu.adi_no_count(13usize, 23usize, 0u32, 2219080u32);
    emu.adi_no_count(14usize, 24usize, 0u32, 2219084u32);
    emu.apc_no_count(1usize, 2219084u32, 4294959104u32, 2219088u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219092u32;
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
pub fn block_0x0021dc54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 4u32, 2219096u32);
    emu.lbu_no_count(12usize, 2usize, 5u32, 2219100u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2219104u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2219188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcb4));
    } else {
        emu.pc = 2219108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc64));
    }
}
#[inline(always)]
pub fn block_0x0021dc64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 11usize, 1u32, 2219112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2219188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcb4));
    } else {
        emu.pc = 2219116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc6c));
    }
}
#[inline(always)]
pub fn block_0x0021dc6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2219120u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2219124u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2219128u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2219160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc98));
    } else {
        emu.pc = 2219132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc7c));
    }
}
#[inline(always)]
pub fn block_0x0021dc7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2219136u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2219140u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2219144u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966605u32, 2219152u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2219156u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2219160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dcb0));
}
#[inline(always)]
pub fn block_0x0021dc98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2219164u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2219168u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2219172u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966604u32, 2219180u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2219184u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2219184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dcb0));
}
#[inline(always)]
pub fn block_0x0021dcb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219188u32;
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
#[inline]
pub fn block_0x0021dcb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2219192u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2219196u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2219200u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2219204u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2219208u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2219212u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2219216u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2219220u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2219224u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2219228u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2219232u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2219236u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219240u32;
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
pub fn block_0x0021dce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2219244u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2219248u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2219252u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2219256u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2219260u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2219264u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2219268u32);
    emu.lw_no_count(13usize, 11usize, 4u32, 2219272u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2219276u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2219280u32)?;
    emu.adi_no_count(18usize, 10usize, 0u32, 2219284u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2219288u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2219292u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2219296u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219300u32;
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
#[inline]
pub fn block_0x0021dd24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(11usize, 8usize, 1u32, 2219304u32);
    emu.sw_no_count(0usize, 18usize, 0u32, 2219308u32)?;
    emu.sw_no_count(9usize, 18usize, 4u32, 2219312u32)?;
    emu.sb_no_count(10usize, 18usize, 8u32, 2219316u32);
    emu.sb_no_count(11usize, 18usize, 9u32, 2219320u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2219324u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2219328u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2219332u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2219336u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2219340u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219344u32;
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
pub fn block_0x0021dd50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2219348u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2219352u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2219356u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2219360u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2219364u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2219368u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2219372u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2219376u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2219380u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2219384u32)?;
    emu.adi_no_count(20usize, 14usize, 0u32, 2219388u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2219392u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2219396u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2219400u32);
    emu.lw_no_count(22usize, 10usize, 4u32, 2219404u32)?;
    emu.lw_no_count(21usize, 10usize, 0u32, 2219408u32)?;
    emu.lw_no_count(23usize, 22usize, 12u32, 2219412u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2219416u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2219420u32;
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
pub fn block_0x0021dd9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2219424u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2219476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ddd4));
    } else {
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    }
}
#[inline]
pub fn block_0x0021dda4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2219432u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2219436u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2219440u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2219444u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2219448u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2219452u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2219456u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2219460u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2219464u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2219468u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2219472u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219476u32;
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
pub fn block_0x0021ddd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2219480u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2219484u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2219540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de14));
    } else {
        emu.pc = 2219488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dde0));
    }
}
#[inline(always)]
pub fn block_0x0021dde0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219492u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966607u32, 2219496u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2219500u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2219504u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2219508u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2219512u32;
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
pub fn block_0x0021ddf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    } else {
        emu.pc = 2219516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ddfc));
    }
}
#[inline(always)]
pub fn block_0x0021ddfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 12u32, 2219520u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2219524u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2219528u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2219532u32;
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
pub fn block_0x0021de0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    } else {
        emu.pc = 2219536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de10));
    }
}
#[inline(always)]
pub fn block_0x0021de10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021de9c));
}
#[inline(always)]
pub fn block_0x0021de14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219544u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966608u32, 2219548u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2219552u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2219556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2219560u32;
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
pub fn block_0x0021de28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    } else {
        emu.pc = 2219564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de2c));
    }
}
#[inline]
pub fn block_0x0021de2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2219568u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2219572u32);
    emu.lw_no_count(11usize, 8usize, 8u32, 2219576u32)?;
    emu.lw_no_count(12usize, 8usize, 12u32, 2219580u32)?;
    emu.adi_no_count(13usize, 2usize, 12u32, 2219584u32);
    emu.sw_no_count(21usize, 2usize, 12u32, 2219588u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2219592u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2219596u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2219600u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966568u32, 2219604u32);
    emu.lw_no_count(14usize, 20usize, 12u32, 2219608u32)?;
    emu.sb_no_count(18usize, 2usize, 27u32, 2219612u32);
    emu.sw_no_count(13usize, 2usize, 28u32, 2219616u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2219620u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2219624u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2219628u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2219632u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2219636u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2219640u32;
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
pub fn block_0x0021de78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    } else {
        emu.pc = 2219644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de7c));
    }
}
#[inline(always)]
pub fn block_0x0021de7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2219648u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2219652u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2219656u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219660u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966602u32, 2219664u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2219668u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219672u32;
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
pub fn block_0x0021de98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    } else {
        emu.pc = 2219676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de9c));
    }
}
#[inline(always)]
pub fn block_0x0021de9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2219728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ded0));
    } else {
        emu.pc = 2219680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dea0));
    }
}
#[inline(always)]
pub fn block_0x0021dea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2219684u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2219688u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2219728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ded0));
    } else {
        emu.pc = 2219692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021deac));
    }
}
#[inline(always)]
pub fn block_0x0021deac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2219696u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2219700u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2219704u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219708u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966611u32, 2219712u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2219716u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2219720u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219724u32;
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
pub fn block_0x0021decc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    } else {
        emu.pc = 2219728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ded0));
    }
}
#[inline(always)]
pub fn block_0x0021ded0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2219732u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2219736u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2219740u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219744u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966610u32, 2219748u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2219752u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219756u32;
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
pub fn block_0x0021deec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2219760u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219764u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dda4));
}
#[inline(always)]
pub fn block_0x0021def4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 0u32, 2219768u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2219772u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2219796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df14));
    } else {
        emu.pc = 2219776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df00));
    }
}
#[inline(always)]
pub fn block_0x0021df00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967072u32, 2219784u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2219788u32);
    emu.apc_no_count(6usize, 2219788u32, 0u32, 2219792u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2219796u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021df14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219800u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 428u32, 2219804u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2219808u32);
    emu.apc_no_count(6usize, 2219808u32, 0u32, 2219812u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2219816u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021df28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2219820u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2219824u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2219828u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2219832u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2219836u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2219840u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2219844u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2219848u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2219852u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2219856u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2219860u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2219864u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2219868u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2219872u32)?;
    emu.adi_no_count(19usize, 11usize, 0u32, 2219876u32);
    emu.lw_no_count(21usize, 12usize, 4u32, 2219880u32)?;
    emu.lw_no_count(8usize, 12usize, 0u32, 2219884u32)?;
    emu.lw_no_count(9usize, 21usize, 16u32, 2219888u32)?;
    emu.adi_no_count(23usize, 10usize, 0u32, 2219892u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2219896u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2219900u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2219904u32;
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
pub fn block_0x0021df80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2219908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2220660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e274));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 24u32, 2219916u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2220560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e210));
    } else {
        emu.pc = 2219920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df90));
    }
}
#[inline]
pub fn block_0x0021df90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 16u32, 2219924u32)?;
    emu.sw_no_count(9usize, 2usize, 12u32, 2219928u32)?;
    emu.adi_no_count(20usize, 0usize, 0u32, 2219932u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2219936u32);
    emu.sbr_no_count(10usize, 0usize, 19usize, 2219940u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2219944u32)?;
    emu.adi_no_count(27usize, 0usize, 4294967201u32, 2219948u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2219952u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 0usize, 1u32, 2219956u32);
    emu.adi_no_count(26usize, 0usize, 34u32, 2219960u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2219964u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2219968u32)?;
    emu.adi_no_count(21usize, 0usize, 92u32, 2219972u32);
    emu.adi_no_count(18usize, 23usize, 0u32, 2219976u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2219980u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2219984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dfe4));
}
#[inline(always)]
pub fn block_0x0021dfd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2219988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2219988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dfd4));
}
#[inline(always)]
pub fn block_0x0021dfd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 9usize, 2219992u32);
    emu.sbr_no_count(13usize, 8usize, 18usize, 2219996u32);
    emu.adr_no_count(9usize, 10usize, 25usize, 2220000u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2220732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2bc));
    } else {
        emu.pc = 2220004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dfe4));
    }
}
#[inline(always)]
pub fn block_0x0021dfe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2220008u32);
    emu.adr_no_count(8usize, 18usize, 13usize, 2220012u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2220016u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dff0));
}
#[inline(always)]
pub fn block_0x0021dff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 25usize, 2220020u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2220024u32);
    emu.adi_no_count(14usize, 12usize, 4294967169u32, 2220028u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2220056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e018));
    } else {
        emu.pc = 2220032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e000));
    }
}
#[inline(always)]
pub fn block_0x0021e000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2220056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e018));
    } else {
        emu.pc = 2220036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e004));
    }
}
#[inline(always)]
pub fn block_0x0021e004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2220056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e018));
    } else {
        emu.pc = 2220040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e008));
    }
}
#[inline(always)]
pub fn block_0x0021e008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2220044u32);
    emu.adr_no_count(10usize, 11usize, 25usize, 2220048u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2220016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dff0));
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
#[inline(always)]
pub fn block_0x0021e014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e1e0));
}
#[inline(always)]
pub fn block_0x0021e018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 10usize, 0u32, 2220060u32);
    emu.adi_no_count(18usize, 10usize, 1u32, 2220064u32);
    emu.ani_no_count(22usize, 11usize, 255u32, 2220068u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e098));
    } else {
        emu.pc = 2220072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e028));
    }
}
#[inline(always)]
pub fn block_0x0021e028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 18usize, 0u32, 2220076u32);
    emu.ani_no_count(11usize, 22usize, 31u32, 2220080u32);
    emu.adi_no_count(18usize, 10usize, 2u32, 2220084u32);
    emu.ani_no_count(13usize, 12usize, 63u32, 2220088u32);
    emu.adi_no_count(12usize, 0usize, 224u32, 2220092u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2220160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e080));
    } else {
        emu.pc = 2220096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e040));
    }
}
#[inline(always)]
pub fn block_0x0021e040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 18usize, 0u32, 2220100u32);
    emu.adi_no_count(12usize, 10usize, 3u32, 2220104u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2220108u32);
    emu.ani_no_count(14usize, 14usize, 63u32, 2220112u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2220116u32);
    emu.adi_no_count(14usize, 0usize, 240u32, 2220120u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2220172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e08c));
    } else {
        emu.pc = 2220124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e05c));
    }
}
#[inline]
pub fn block_0x0021e05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 4u32, 2220128u32);
    emu.lbu_no_count(10usize, 12usize, 0u32, 2220132u32);
    emu.sli_no_count(11usize, 11usize, 29u32, 2220136u32);
    emu.sri_no_count(11usize, 11usize, 11u32, 2220140u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2220144u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2220148u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2220152u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2220156u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2220160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e098));
}
#[inline(always)]
pub fn block_0x0021e080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 6u32, 2220164u32);
    emu.orr_no_count(22usize, 11usize, 13usize, 2220168u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2220172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e098));
}
#[inline(always)]
pub fn block_0x0021e08c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 12u32, 2220176u32);
    emu.orr_no_count(22usize, 13usize, 11usize, 2220180u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2220184u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e098));
}
#[inline(always)]
pub fn block_0x0021e098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2220188u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2220192u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2220196u32)?;
    emu.apc_no_count(1usize, 2220196u32, 4294963200u32, 2220200u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e0ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 40u32, 2220208u32);
    emu.lbu_no_count(11usize, 2usize, 41u32, 2220212u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2220216u32);
    emu.ani_no_count(10usize, 11usize, 255u32, 2220220u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2220472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1b8));
    } else {
        emu.pc = 2220224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0c0));
    }
}
#[inline(always)]
pub fn block_0x0021e0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 9usize, 25usize, 2220228u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2220780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2ec));
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
    emu.adr_no_count(11usize, 23usize, 20usize, 2220236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2220264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0e8));
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
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2220260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0e4));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(10usize, 11usize, 0u32, 2220248u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2220252u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0e8));
    } else {
        emu.pc = 2220256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0e0));
    }
}
#[inline(always)]
pub fn block_0x0021e0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e2ec));
}
#[inline(always)]
pub fn block_0x0021e0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2220780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2ec));
    } else {
        emu.pc = 2220264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0e8));
    }
}
#[inline(always)]
pub fn block_0x0021e0e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2220308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e114));
    } else {
        emu.pc = 2220268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0ec));
    }
}
#[inline(always)]
pub fn block_0x0021e0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2220296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e108));
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 9usize, 2220276u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2220280u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2220284u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2220288u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e114));
    } else {
        emu.pc = 2220292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e104));
    }
}
#[inline(always)]
pub fn block_0x0021e104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220296u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e2ec));
}
#[inline(always)]
pub fn block_0x0021e108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2220300u32)?;
    emu.adr_no_count(10usize, 13usize, 10usize, 2220304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2220780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2ec));
    } else {
        emu.pc = 2220308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e114));
    }
}
#[inline(always)]
pub fn block_0x0021e114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 23usize, 0u32, 2220312u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2220316u32)?;
    emu.lw_no_count(23usize, 10usize, 12u32, 2220320u32)?;
    emu.sbr_no_count(12usize, 9usize, 20usize, 2220324u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2220328u32);
    emu.lw_no_count(20usize, 2usize, 24u32, 2220332u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2220336u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2220340u32;
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
pub fn block_0x0021e134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2220724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2b4));
    } else {
        emu.pc = 2220344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e138));
    }
}
#[inline(always)]
pub fn block_0x0021e138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 41u32, 2220348u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2220352u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2220380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e15c));
    } else {
        emu.pc = 2220356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e144));
    }
}
#[inline(always)]
pub fn block_0x0021e144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 28u32, 2220360u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2220364u32);
    emu.lw_no_count(12usize, 2usize, 12u32, 2220368u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2220372u32;
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
pub fn block_0x0021e154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2220408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e178));
    } else {
        emu.pc = 2220376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e158));
    }
}
#[inline(always)]
pub fn block_0x0021e158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220724u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e2b4));
}
#[inline(always)]
pub fn block_0x0021e15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 40u32, 2220384u32);
    emu.sbr_no_count(12usize, 10usize, 11usize, 2220388u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2220392u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2220396u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2220400u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2220404u32;
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
pub fn block_0x0021e174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2220724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2b4));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2220412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2220428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e18c));
    } else {
        emu.pc = 2220416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e180));
    }
}
#[inline(always)]
pub fn block_0x0021e180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2220420u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2220424u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2220428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e1ac));
}
#[inline(always)]
pub fn block_0x0021e18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2220432u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2220436u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2220448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1a0));
    } else {
        emu.pc = 2220440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e198));
    }
}
#[inline(always)]
pub fn block_0x0021e198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2220444u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e1ac));
}
#[inline(always)]
pub fn block_0x0021e1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2220452u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2220456u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2220460u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220460u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e1ac));
}
#[inline(always)]
pub fn block_0x0021e1ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 1u32, 2220464u32);
    emu.adr_no_count(10usize, 9usize, 25usize, 2220468u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2220472u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220472u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e1b8));
}
#[inline(always)]
pub fn block_0x0021e1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2220476u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2219984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dfd0));
    } else {
        emu.pc = 2220480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1c0));
    }
}
#[inline(always)]
pub fn block_0x0021e1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2220484u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2220496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1d0));
    } else {
        emu.pc = 2220488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1c8));
    }
}
#[inline(always)]
pub fn block_0x0021e1c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2220492u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dfd4));
}
#[inline(always)]
pub fn block_0x0021e1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2220500u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2220504u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2220508u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2220512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dfd4));
}
#[inline(always)]
pub fn block_0x0021e1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 9usize, 2220516u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2220520u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2220524u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2220528u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2220752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2d0));
    } else {
        emu.pc = 2220532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1f4));
    }
}
#[inline(always)]
pub fn block_0x0021e1f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2220572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e21c));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2220568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e218));
    } else {
        emu.pc = 2220540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1fc));
    }
}
#[inline(always)]
pub fn block_0x0021e1fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 20usize, 2220544u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2220548u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2220552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2220572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e21c));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220752u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e2d0));
}
#[inline(always)]
pub fn block_0x0021e210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2220564u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e248));
}
#[inline(always)]
pub fn block_0x0021e218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2220752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2d0));
    } else {
        emu.pc = 2220572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e21c));
    }
}
#[inline(always)]
pub fn block_0x0021e21c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2220604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e23c));
    } else {
        emu.pc = 2220576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e220));
    }
}
#[inline(always)]
pub fn block_0x0021e220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2220612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e244));
    } else {
        emu.pc = 2220580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e224));
    }
}
#[inline(always)]
pub fn block_0x0021e224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 13usize, 2220584u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2220588u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2220592u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2220752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2d0));
    } else {
        emu.pc = 2220596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e234));
    }
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
    emu.adi_no_count(19usize, 13usize, 0u32, 2220600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e248));
}
#[inline(always)]
pub fn block_0x0021e23c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2220608u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220612u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e248));
}
#[inline(always)]
pub fn block_0x0021e244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2220752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e2d0));
    } else {
        emu.pc = 2220616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e248));
    }
}
#[inline(always)]
pub fn block_0x0021e248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 21usize, 12u32, 2220620u32)?;
    emu.sbr_no_count(12usize, 19usize, 20usize, 2220624u32);
    emu.adr_no_count(11usize, 23usize, 20usize, 2220628u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2220632u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2220636u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2220640u32;
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
pub fn block_0x0021e260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2220660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e274));
    } else {
        emu.pc = 2220644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e264));
    }
}
#[inline(always)]
pub fn block_0x0021e264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 34u32, 2220648u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2220652u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2220656u32;
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
pub fn block_0x0021e270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2220660u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2220660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e274));
}
#[inline]
pub fn block_0x0021e274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2220664u32);
    emu.lw_no_count(1usize, 2usize, 92u32, 2220668u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2220672u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2220676u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2220680u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2220684u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2220688u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2220692u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2220696u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2220700u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2220704u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2220708u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2220712u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2220716u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2220720u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220724u32;
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
pub fn block_0x0021e2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2220728u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e274));
}
#[inline(always)]
pub fn block_0x0021e2bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 10usize, 25usize, 2220736u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2220740u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2220744u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2220748u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2220532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1f4));
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
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2220756u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 452u32, 2220760u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2220764u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2220768u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2220772u32);
    emu.apc_no_count(1usize, 2220772u32, 4294959104u32, 2220776u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965928u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2220784u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 436u32, 2220788u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2220792u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2220796u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2220800u32);
    emu.apc_no_count(1usize, 2220800u32, 4294959104u32, 2220804u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
