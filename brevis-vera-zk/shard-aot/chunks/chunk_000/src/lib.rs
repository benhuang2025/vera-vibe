pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2099200u32;
pub const PC_MAX: u32 = 2102996u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00200800,
        block_0x00200824,
        block_0x00200830,
        block_0x00200848,
        block_0x00200874,
        block_0x0020087c,
        block_0x00200890,
        block_0x002008b8,
        block_0x002008c0,
        block_0x002008d4,
        block_0x002008fc,
        block_0x00200904,
        block_0x00200914,
        block_0x00200920,
        block_0x0020092c,
        block_0x0020094c,
        block_0x00200964,
        block_0x002009fc,
        block_0x00200a14,
        block_0x00200a2c,
        block_0x00200a40,
        block_0x00200a48,
        block_0x00200a4c,
        block_0x00200a60,
        block_0x00200a6c,
        block_0x00200a8c,
        block_0x00200aa8,
        block_0x00200ae4,
        block_0x00200b00,
        block_0x00200b04,
        block_0x00200b20,
        block_0x00200b40,
        block_0x00200b5c,
        block_0x00200b9c,
        block_0x00200ba4,
        block_0x00200bbc,
        block_0x00200bd4,
        block_0x00200bfc,
        block_0x00200c00,
        block_0x00200c24,
        block_0x00200c38,
        block_0x00200c58,
        block_0x00200c68,
        block_0x00200c6c,
        block_0x00200cd8,
        block_0x00200ce4,
        block_0x00200cf8,
        block_0x00200d08,
        block_0x00200d14,
        block_0x00200d34,
        block_0x00200d3c,
        block_0x00200d4c,
        block_0x00200d6c,
        block_0x00200d7c,
        block_0x00200d80,
        block_0x00200dec,
        block_0x00200df8,
        block_0x00200e08,
        block_0x00200e18,
        block_0x00200e28,
        block_0x00200e40,
        block_0x00200e48,
        block_0x00200e68,
        block_0x00200e80,
        block_0x00200e88,
        block_0x00200ecc,
        block_0x00200ef0,
        block_0x00200f00,
        block_0x00200f28,
        block_0x00200f98,
        block_0x00200fbc,
        block_0x00200fd0,
        block_0x00200fd4,
        block_0x00200fe8,
        block_0x00200fec,
        block_0x00201004,
        block_0x00201090,
        block_0x002010a4,
        block_0x002010b0,
        block_0x002010c4,
        block_0x002010d8,
        block_0x00201120,
        block_0x00201134,
        block_0x0020113c,
        block_0x00201168,
        block_0x0020117c,
        block_0x00201184,
        block_0x0020118c,
        block_0x00201194,
        block_0x002011ac,
        block_0x002011b8,
        block_0x002011d0,
        block_0x00201240,
        block_0x00201264,
        block_0x00201270,
        block_0x00201278,
        block_0x00201284,
        block_0x0020129c,
        block_0x00201328,
        block_0x0020133c,
        block_0x00201348,
        block_0x0020135c,
        block_0x00201370,
        block_0x002013b8,
        block_0x002013cc,
        block_0x002013d4,
        block_0x00201698,
        block_0x002016d4,
    ];
    const IDX: [u16; 950usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16,
        12u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 21u16, 0u16, 22u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16,
        25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 30u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 44u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16,
        0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 48u16, 0u16,
        0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 51u16, 0u16,
        0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16,
        0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16,
        0u16, 59u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 0u16,
        74u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 0u16,
        0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        82u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 87u16,
        0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 91u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 97u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16,
        0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16,
        0u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 108u16,
    ];
    if pc < 2099200u32 || pc > 2102996u32 {
        return None;
    }
    let word_offset = ((pc - 2099200u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00200800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2099204u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2099208u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2099212u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2099216u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2099220u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2099224u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2099228u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2099232u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020094c));
    } else {
        emu.pc = 2099236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200824));
    }
}
#[inline(always)]
pub fn block_0x00200824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2099240u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2099244u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2099272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200848));
    } else {
        emu.pc = 2099248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200830));
    }
}
#[inline(always)]
pub fn block_0x00200830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2099252u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099256u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2099260u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2099264u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2099268u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2099272u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099476u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200914));
}
#[inline]
pub fn block_0x00200848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2099276u32)?;
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2099280u32);
    emu.adi_no_count(6usize, 10usize, 4u32, 2099284u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2099288u32);
    emu.lbu_no_count(5usize, 10usize, 1u32, 2099292u32);
    emu.lbu_no_count(16usize, 10usize, 2u32, 2099296u32);
    emu.lbu_no_count(17usize, 10usize, 3u32, 2099300u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2099304u32);
    emu.sw_no_count(6usize, 11usize, 0u32, 2099308u32)?;
    emu.sw_no_count(15usize, 11usize, 4u32, 2099312u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002009fc));
    } else {
        emu.pc = 2099316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200874));
    }
}
#[inline(always)]
pub fn block_0x00200874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 4u32, 2099320u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2099344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200890));
    } else {
        emu.pc = 2099324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020087c));
    }
}
#[inline(always)]
pub fn block_0x0020087c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2099328u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2099332u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2099336u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2099340u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2099344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099476u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200914));
}
#[inline]
pub fn block_0x00200890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 10usize, 8u32, 2099348u32);
    emu.adi_no_count(6usize, 12usize, 4294967288u32, 2099352u32);
    emu.lbu_no_count(15usize, 10usize, 4u32, 2099356u32);
    emu.lbu_no_count(29usize, 10usize, 5u32, 2099360u32);
    emu.lbu_no_count(7usize, 10usize, 6u32, 2099364u32);
    emu.lbu_no_count(28usize, 10usize, 7u32, 2099368u32);
    emu.adi_no_count(31usize, 0usize, 2u32, 2099372u32);
    emu.sw_no_count(30usize, 11usize, 0u32, 2099376u32)?;
    emu.sw_no_count(6usize, 11usize, 4u32, 2099380u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a14));
    } else {
        emu.pc = 2099384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008b8));
    }
}
#[inline(always)]
pub fn block_0x002008b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 0usize, 4u32, 2099388u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2099412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008d4));
    } else {
        emu.pc = 2099392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008c0));
    }
}
#[inline(always)]
pub fn block_0x002008c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2099396u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2099400u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2099404u32)?;
    emu.sw_no_count(30usize, 2usize, 4u32, 2099408u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2099412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099476u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200914));
}
#[inline]
pub fn block_0x002008d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 12u32, 2099416u32);
    emu.adi_no_count(18usize, 12usize, 4294967284u32, 2099420u32);
    emu.lbu_no_count(6usize, 10usize, 8u32, 2099424u32);
    emu.lbu_no_count(9usize, 10usize, 9u32, 2099428u32);
    emu.lbu_no_count(30usize, 10usize, 10u32, 2099432u32);
    emu.lbu_no_count(31usize, 10usize, 11u32, 2099436u32);
    emu.adi_no_count(20usize, 0usize, 3u32, 2099440u32);
    emu.sw_no_count(19usize, 11usize, 0u32, 2099444u32)?;
    emu.sw_no_count(18usize, 11usize, 4u32, 2099448u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a2c));
    } else {
        emu.pc = 2099452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008fc));
    }
}
#[inline(always)]
pub fn block_0x002008fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 4u32, 2099456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2099556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200964));
    } else {
        emu.pc = 2099460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200904));
    }
}
#[inline(always)]
pub fn block_0x00200904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2099464u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2099468u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2099472u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2099476u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2099476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200914));
}
#[inline(always)]
pub fn block_0x00200914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2099480u32);
    emu.apc_no_count(1usize, 2099480u32, 12288u32, 2099484u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099488u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2099492u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2099496u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2099500u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2099500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020092c));
}
#[inline(always)]
pub fn block_0x0020092c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2099504u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099508u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2099512u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2099516u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2099520u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2099524u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099528u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099532u32;
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
pub fn block_0x0020094c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099536u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 548u32, 2099540u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099544u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 556u32, 2099548u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2099552u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2099556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a40));
}
#[inline(never)]
pub fn block_0x00200964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 8u32, 2099560u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2099564u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2099568u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2099572u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2099576u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2099580u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2099584u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2099588u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2099592u32);
    emu.orr_no_count(13usize, 5usize, 14usize, 2099596u32);
    emu.adi_no_count(14usize, 10usize, 16u32, 2099600u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2099604u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2099608u32);
    emu.orr_no_count(15usize, 29usize, 15usize, 2099612u32);
    emu.orr_no_count(17usize, 28usize, 7usize, 2099616u32);
    emu.lbu_no_count(5usize, 10usize, 12u32, 2099620u32);
    emu.lbu_no_count(7usize, 10usize, 13u32, 2099624u32);
    emu.lbu_no_count(28usize, 10usize, 14u32, 2099628u32);
    emu.lbu_no_count(10usize, 10usize, 15u32, 2099632u32);
    emu.orr_no_count(6usize, 9usize, 6usize, 2099636u32);
    emu.orr_no_count(29usize, 31usize, 30usize, 2099640u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2099644u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2099648u32)?;
    emu.orr_no_count(11usize, 16usize, 13usize, 2099652u32);
    emu.orr_no_count(12usize, 17usize, 15usize, 2099656u32);
    emu.orr_no_count(13usize, 29usize, 6usize, 2099660u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2099664u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2099668u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2099672u32);
    emu.orr_no_count(14usize, 7usize, 5usize, 2099676u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2099680u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2099684u32);
    emu.sh_no_count(0usize, 8usize, 0u32, 2099688u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2099692u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2099696u32)?;
    emu.sw_no_count(13usize, 8usize, 12u32, 2099700u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2099704u32)?;
    emu.add_memory_rw_events(38usize);
    let return_addr = 2099708u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020092c));
}
#[inline(always)]
pub fn block_0x002009fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099712u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 548u32, 2099716u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099720u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 556u32, 2099724u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2099728u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2099732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a40));
}
#[inline(always)]
pub fn block_0x00200a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099736u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 548u32, 2099740u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099744u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 556u32, 2099748u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2099752u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2099756u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a40));
}
#[inline(always)]
pub fn block_0x00200a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099760u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 548u32, 2099764u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099768u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 556u32, 2099772u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2099776u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2099776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a40));
}
#[inline(always)]
pub fn block_0x00200a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2099776u32, 4096u32, 2099780u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099784u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200920));
}
#[inline(always)]
pub fn block_0x00200a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2099792u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2099796u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2099800u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2099804u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ae4));
    } else {
        emu.pc = 2099808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a60));
    }
}
#[inline(always)]
pub fn block_0x00200a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2099812u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2099816u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2099880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aa8));
    } else {
        emu.pc = 2099820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a6c));
    }
}
#[inline(always)]
pub fn block_0x00200a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2099824u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099828u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2099832u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2099836u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2099840u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2099844u32);
    emu.apc_no_count(1usize, 2099844u32, 12288u32, 2099848u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2099856u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2099860u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2099864u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2099868u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2099872u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2099876u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099880u32;
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
pub fn block_0x00200aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 11usize, 0u32, 2099884u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2099888u32);
    emu.lbu_no_count(14usize, 13usize, 0u32, 2099892u32);
    emu.lbu_no_count(15usize, 13usize, 1u32, 2099896u32);
    emu.adi_no_count(13usize, 13usize, 2u32, 2099900u32);
    emu.sw_no_count(13usize, 11usize, 0u32, 2099904u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2099908u32)?;
    emu.sli_no_count(15usize, 15usize, 8u32, 2099912u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2099916u32);
    emu.sh_no_count(10usize, 8usize, 0u32, 2099920u32)?;
    emu.sh_no_count(14usize, 8usize, 2u32, 2099924u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2099928u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2099932u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2099936u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099940u32;
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
pub fn block_0x00200ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099944u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 636u32, 2099948u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099952u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 556u32, 2099956u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2099960u32);
    emu.apc_no_count(1usize, 2099960u32, 4096u32, 2099964u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099972u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a8c));
}
#[inline(always)]
pub fn block_0x00200b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2099976u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2099980u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2099984u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2099988u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2099992u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2099996u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2100060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b5c));
    } else {
        emu.pc = 2100000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b20));
    }
}
#[inline(always)]
pub fn block_0x00200b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2100004u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100008u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2100012u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2100016u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2100020u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2100024u32);
    emu.apc_no_count(1usize, 2100024u32, 12288u32, 2100028u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2100036u32);
    emu.sb_no_count(11usize, 8usize, 0u32, 2100040u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2100044u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2100048u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2100052u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2100056u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100060u32;
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
pub fn block_0x00200b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2100064u32)?;
    emu.lbu_no_count(13usize, 10usize, 1u32, 2100068u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2100072u32);
    emu.lbu_no_count(15usize, 10usize, 2u32, 2100076u32);
    emu.lbu_no_count(16usize, 10usize, 3u32, 2100080u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2100084u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2100088u32);
    emu.adi_no_count(14usize, 12usize, 4294967292u32, 2100092u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2100096u32);
    emu.sli_no_count(15usize, 15usize, 16u32, 2100100u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2100104u32);
    emu.orr_no_count(12usize, 16usize, 15usize, 2100108u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2100112u32);
    emu.sw_no_count(10usize, 11usize, 0u32, 2100116u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2100120u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bbc));
    } else {
        emu.pc = 2100124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b9c));
    }
}
#[inline(always)]
pub fn block_0x00200b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2100128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2100180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bd4));
    } else {
        emu.pc = 2100132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ba4));
    }
}
#[inline(always)]
pub fn block_0x00200ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 0u32, 2100136u32);
    emu.sw_no_count(11usize, 8usize, 4u32, 2100140u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2100144u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2100148u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2100152u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100156u32;
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
pub fn block_0x00200bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2100160u32);
    emu.sw_no_count(11usize, 8usize, 4u32, 2100164u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2100168u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2100172u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2100176u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100180u32;
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
pub fn block_0x00200bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 2usize, 8u32, 2100184u32);
    emu.sw_no_count(12usize, 2usize, 16u32, 2100188u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2100192u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100196u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966732u32, 2100200u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100204u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966740u32, 2100208u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2100212u32);
    emu.apc_no_count(1usize, 2100212u32, 4096u32, 2100216u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2100224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200b40));
}
#[inline]
pub fn block_0x00200c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2100228u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2100232u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2100236u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2100240u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2100244u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2100248u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2100252u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2100256u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e28));
    } else {
        emu.pc = 2100260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c24));
    }
}
#[inline(always)]
pub fn block_0x00200c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 12usize, 0u32, 2100264u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2100268u32);
    emu.lw_no_count(10usize, 11usize, 4u32, 2100272u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2100276u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    } else {
        emu.pc = 2100280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c38));
    }
}
#[inline(always)]
pub fn block_0x00200c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2100284u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100288u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2100292u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2100296u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2100300u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2100304u32);
    emu.apc_no_count(1usize, 2100304u32, 12288u32, 2100308u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100312u32;
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
pub fn block_0x00200c58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2100316u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2100320u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2100324u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ce4));
    } else {
        emu.pc = 2100328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c68));
    }
}
#[inline(always)]
pub fn block_0x00200c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2100332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200d08));
}
#[inline(never)]
pub fn block_0x00200c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2100336u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2100340u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2100344u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2100348u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2100352u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2100356u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2100360u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2100364u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2100368u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2100372u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2100376u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2100380u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2100384u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2100388u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2100392u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2100396u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2100400u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2100404u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2100408u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2100412u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2100416u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2100420u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2100424u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2100428u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2100432u32)?;
    emu.apc_no_count(1usize, 2100432u32, 12288u32, 2100436u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 11usize, 0u32, 2100444u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2100448u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d08));
    } else {
        emu.pc = 2100452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ce4));
    }
}
#[inline(always)]
pub fn block_0x00200ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2100456u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2100460u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2100464u32);
    emu.apc_no_count(1usize, 2100464u32, 4096u32, 2100468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 2usize, 4u32, 2100476u32)?;
    emu.lw_no_count(18usize, 2usize, 8u32, 2100480u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100484u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2100532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d34));
    } else {
        emu.pc = 2100488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d08));
    }
}
#[inline(always)]
pub fn block_0x00200d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100492u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2100496u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2100500u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2100500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200d14));
}
#[inline(always)]
pub fn block_0x00200d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2100504u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2100508u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2100512u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2100516u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2100520u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2100524u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2100528u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100532u32;
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
pub fn block_0x00200d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2100536u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2100840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e68));
    } else {
        emu.pc = 2100540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d3c));
    }
}
#[inline(always)]
pub fn block_0x00200d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2100544u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2100548u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2100552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2100608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d80));
    } else {
        emu.pc = 2100556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d4c));
    }
}
#[inline(always)]
pub fn block_0x00200d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2100560u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100564u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2100568u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2100572u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2100576u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2100580u32);
    emu.apc_no_count(1usize, 2100580u32, 12288u32, 2100584u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2100592u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2100596u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2100600u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200df8));
    } else {
        emu.pc = 2100604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d7c));
    }
}
#[inline(always)]
pub fn block_0x00200d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2100608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200e18));
}
#[inline(never)]
pub fn block_0x00200d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2100612u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2100616u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2100620u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2100624u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2100628u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2100632u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2100636u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2100640u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2100644u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2100648u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2100652u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2100656u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2100660u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2100664u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2100668u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2100672u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2100676u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2100680u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2100684u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2100688u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2100692u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2100696u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2100700u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2100704u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2100708u32)?;
    emu.apc_no_count(1usize, 2100708u32, 8192u32, 2100712u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100716u32;
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
pub fn block_0x00200dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2100720u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2100724u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e18));
    } else {
        emu.pc = 2100728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200df8));
    }
}
#[inline(always)]
pub fn block_0x00200df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2100732u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2100736u32);
    emu.apc_no_count(1usize, 2100736u32, 4096u32, 2100740u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2100748u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2100752u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100756u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e48));
    } else {
        emu.pc = 2100760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e18));
    }
}
#[inline(always)]
pub fn block_0x00200e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100764u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2100768u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2100772u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200d14));
}
#[inline(always)]
pub fn block_0x00200e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100780u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 480u32, 2100784u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2100788u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2100792u32);
    emu.apc_no_count(1usize, 2100792u32, 4096u32, 2100796u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2100804u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200d08));
}
#[inline(always)]
pub fn block_0x00200e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2100812u32)?;
    emu.sw_no_count(20usize, 8usize, 0u32, 2100816u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2100820u32)?;
    emu.sw_no_count(19usize, 8usize, 8u32, 2100824u32)?;
    emu.sw_no_count(10usize, 8usize, 12u32, 2100828u32)?;
    emu.sw_no_count(12usize, 8usize, 16u32, 2100832u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2100836u32)?;
    emu.add_memory_rw_events(8usize);
    let return_addr = 2100840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200d14));
}
#[inline(always)]
pub fn block_0x00200e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100844u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 480u32, 2100848u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2100852u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2100856u32);
    emu.apc_no_count(1usize, 2100856u32, 4096u32, 2100860u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100864u32;
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
pub fn block_0x00200e80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2100868u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100872u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200e18));
}
#[inline]
pub fn block_0x00200e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966832u32, 2100876u32);
    emu.sw_no_count(1usize, 2usize, 460u32, 2100880u32)?;
    emu.sw_no_count(8usize, 2usize, 456u32, 2100884u32)?;
    emu.sw_no_count(9usize, 2usize, 452u32, 2100888u32)?;
    emu.sw_no_count(18usize, 2usize, 448u32, 2100892u32)?;
    emu.sw_no_count(19usize, 2usize, 444u32, 2100896u32)?;
    emu.sw_no_count(20usize, 2usize, 440u32, 2100900u32)?;
    emu.sw_no_count(21usize, 2usize, 436u32, 2100904u32)?;
    emu.sw_no_count(22usize, 2usize, 432u32, 2100908u32)?;
    emu.sw_no_count(23usize, 2usize, 428u32, 2100912u32)?;
    emu.sw_no_count(24usize, 2usize, 424u32, 2100916u32)?;
    emu.sw_no_count(25usize, 2usize, 420u32, 2100920u32)?;
    emu.sw_no_count(26usize, 2usize, 416u32, 2100924u32)?;
    emu.sw_no_count(27usize, 2usize, 412u32, 2100928u32)?;
    emu.adi_no_count(10usize, 2usize, 120u32, 2100932u32);
    emu.apc_no_count(1usize, 2100932u32, 8192u32, 2100936u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00200ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 124u32, 2100944u32)?;
    emu.lw_no_count(11usize, 2usize, 128u32, 2100948u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2100952u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2100956u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2100960u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2100964u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2100968u32);
    emu.apc_no_count(1usize, 2100968u32, 0u32, 2100972u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 232u32, 2100980u32)?;
    emu.lw_no_count(18usize, 2usize, 236u32, 2100984u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100988u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016d4));
    } else {
        emu.pc = 2100992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f00));
    }
}
#[inline]
pub fn block_0x00200f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 240u32, 2100996u32)?;
    emu.lw_no_count(21usize, 2usize, 248u32, 2101000u32)?;
    emu.lw_no_count(10usize, 2usize, 252u32, 2101004u32)?;
    emu.sw_no_count(10usize, 2usize, 0u32, 2101008u32)?;
    emu.adi_no_count(8usize, 2usize, 48u32, 2101012u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2101016u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2101020u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2101024u32);
    emu.apc_no_count(1usize, 2101024u32, 8192u32, 2101028u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00200f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101036u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2101040u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101044u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2101048u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2101052u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2101056u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2101060u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2101064u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2101068u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2101072u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2101076u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2101080u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2101084u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2101088u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2101092u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101096u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2101100u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2101104u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2101108u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2101112u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2101116u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2101120u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2101124u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2101128u32)?;
    emu.sri_no_count(12usize, 9usize, 6u32, 2101132u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2101136u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2101140u32)?;
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2101204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fd4));
    } else {
        emu.pc = 2101144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f98));
    }
}
#[inline]
pub fn block_0x00200f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 9usize, 4294967232u32, 2101148u32);
    emu.ani_no_count(19usize, 9usize, 63u32, 2101152u32);
    emu.adr_no_count(20usize, 18usize, 20usize, 2101156u32);
    emu.sw_no_count(12usize, 2usize, 40u32, 2101160u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2101164u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2101168u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2101172u32);
    emu.apc_no_count(1usize, 2101172u32, 8192u32, 2101176u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2101184u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2101188u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2101192u32);
    emu.apc_no_count(1usize, 2101192u32, 8192u32, 2101196u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200fec));
}
#[inline(always)]
pub fn block_0x00200fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2101208u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2101212u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2101216u32);
    emu.apc_no_count(1usize, 2101216u32, 8192u32, 2101220u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 9usize, 0u32, 2101228u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200fec));
}
#[inline(always)]
pub fn block_0x00200fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(19usize, 2usize, 112u32, 2101232u32);
    emu.adi_no_count(10usize, 2usize, 232u32, 2101236u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2101240u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2101244u32);
    emu.apc_no_count(1usize, 2101244u32, 8192u32, 2101248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00201004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 2usize, 272u32, 2101256u32);
    emu.lbu_no_count(8usize, 2usize, 336u32, 2101260u32);
    emu.lw_no_count(10usize, 2usize, 264u32, 2101264u32)?;
    emu.lw_no_count(11usize, 2usize, 268u32, 2101268u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101272u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2101276u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2101280u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2101284u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2101288u32);
    emu.sli_no_count(16usize, 8usize, 3u32, 2101292u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2101296u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2101300u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2101304u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2101308u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2101312u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2101316u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2101320u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2101324u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2101328u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2101332u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2101336u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2101340u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2101344u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2101348u32);
    emu.sli_no_count(10usize, 8usize, 27u32, 2101352u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2101356u32);
    emu.adr_no_count(10usize, 6usize, 8usize, 2101360u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2101364u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2101368u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2101372u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2101376u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2101380u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2101384u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2101388u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2101424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010b0));
    } else {
        emu.pc = 2101392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201090));
    }
}
#[inline(always)]
pub fn block_0x00201090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2101396u32);
    emu.xri_no_count(12usize, 8usize, 63u32, 2101400u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2101404u32);
    emu.apc_no_count(1usize, 2101404u32, 8192u32, 2101408u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002010a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 8usize, 56u32, 2101416u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2101420u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2101536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201120));
    } else {
        emu.pc = 2101424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010b0));
    }
}
#[inline(always)]
pub fn block_0x002010b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 232u32, 2101428u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2101432u32);
    emu.adi_no_count(11usize, 2usize, 272u32, 2101436u32);
    emu.apc_no_count(1usize, 2101436u32, 8192u32, 2101440u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002010c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2101448u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2101452u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2101456u32);
    emu.apc_no_count(1usize, 2101456u32, 8192u32, 2101460u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002010d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2101468u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2101472u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2101476u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2101480u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2101484u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2101488u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2101492u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2101496u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2101500u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2101504u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2101508u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2101512u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2101516u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2101520u32);
    emu.adi_no_count(10usize, 2usize, 232u32, 2101524u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2101528u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2101532u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2101536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201134));
}
#[inline(always)]
pub fn block_0x00201120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 328u32, 2101540u32)?;
    emu.sw_no_count(20usize, 2usize, 332u32, 2101544u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2101548u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2101552u32);
    emu.adi_no_count(11usize, 2usize, 272u32, 2101556u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2101556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201134));
}
#[inline(always)]
pub fn block_0x00201134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2101556u32, 8192u32, 2101560u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020113c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 232u32, 2101568u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2101572u32)?;
    emu.lw_no_count(22usize, 2usize, 236u32, 2101576u32)?;
    emu.lw_no_count(27usize, 2usize, 240u32, 2101580u32)?;
    emu.lw_no_count(24usize, 2usize, 244u32, 2101584u32)?;
    emu.lw_no_count(25usize, 2usize, 248u32, 2101588u32)?;
    emu.lw_no_count(26usize, 2usize, 252u32, 2101592u32)?;
    emu.lw_no_count(23usize, 2usize, 256u32, 2101596u32)?;
    emu.lw_no_count(8usize, 2usize, 260u32, 2101600u32)?;
    emu.lw_no_count(11usize, 2usize, 0u32, 2101604u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2101688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011b8));
    } else {
        emu.pc = 2101608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201168));
    }
}
#[inline(always)]
pub fn block_0x00201168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2101612u32);
    emu.mul_no_count(19usize, 11usize, 10usize, 2101616u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2101620u32);
    emu.adi_no_count(20usize, 0usize, 2u32, 2101624u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2101628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201184));
}
#[inline(always)]
pub fn block_0x0020117c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 21usize, 20u32, 2101632u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2101688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011b8));
    } else {
        emu.pc = 2101636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201184));
    }
}
#[inline(always)]
pub fn block_0x00201184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 21usize, 0u32, 2101640u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011b8));
    } else {
        emu.pc = 2101644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020118c));
    }
}
#[inline(always)]
pub fn block_0x0020118c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2101648u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020117c));
    } else {
        emu.pc = 2101652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201194));
    }
}
#[inline(always)]
pub fn block_0x00201194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(13usize, 21usize, 2u32, 2101656u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2101660u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2101664u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2101668u32);
    emu.apc_no_count(1usize, 2101668u32, 24576u32, 2101672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002011ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 236u32, 2101680u32)?;
    emu.lw_no_count(9usize, 2usize, 240u32, 2101684u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2101688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020117c));
}
#[inline(always)]
pub fn block_0x002011b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 2usize, 160u32, 2101692u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2101696u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2101700u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2101704u32);
    emu.apc_no_count(1usize, 2101704u32, 8192u32, 2101708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002011d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101716u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2101720u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101724u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2101728u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2101732u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2101736u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2101740u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2101744u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2101748u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2101752u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2101756u32);
    emu.sw_no_count(10usize, 2usize, 120u32, 2101760u32)?;
    emu.sw_no_count(11usize, 2usize, 124u32, 2101764u32)?;
    emu.sw_no_count(12usize, 2usize, 128u32, 2101768u32)?;
    emu.sw_no_count(13usize, 2usize, 132u32, 2101772u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101776u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2101780u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2101784u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2101788u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2101792u32);
    emu.sw_no_count(11usize, 2usize, 136u32, 2101796u32)?;
    emu.sw_no_count(12usize, 2usize, 140u32, 2101800u32)?;
    emu.sw_no_count(13usize, 2usize, 144u32, 2101804u32)?;
    emu.sw_no_count(10usize, 2usize, 148u32, 2101808u32)?;
    emu.sri_no_count(12usize, 9usize, 6u32, 2101812u32);
    emu.sw_no_count(0usize, 2usize, 152u32, 2101816u32)?;
    emu.sw_no_count(0usize, 2usize, 156u32, 2101820u32)?;
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2101872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201270));
    } else {
        emu.pc = 2101824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201240));
    }
}
#[inline]
pub fn block_0x00201240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 9usize, 4294967232u32, 2101828u32);
    emu.ani_no_count(9usize, 9usize, 63u32, 2101832u32);
    emu.adr_no_count(20usize, 18usize, 20usize, 2101836u32);
    emu.sw_no_count(12usize, 2usize, 152u32, 2101840u32)?;
    emu.sw_no_count(0usize, 2usize, 156u32, 2101844u32)?;
    emu.adi_no_count(10usize, 2usize, 120u32, 2101848u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2101852u32);
    emu.apc_no_count(1usize, 2101852u32, 8192u32, 2101856u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101860u32;
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
pub fn block_0x00201264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2101864u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2101868u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2101872u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201278));
}
#[inline(always)]
pub fn block_0x00201270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2101876u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2101880u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2101880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201278));
}
#[inline(always)]
pub fn block_0x00201278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 9usize, 0u32, 2101884u32);
    emu.apc_no_count(1usize, 2101884u32, 8192u32, 2101888u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101892u32;
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
pub fn block_0x00201284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 2usize, 224u32, 2101896u32);
    emu.adi_no_count(10usize, 2usize, 232u32, 2101900u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2101904u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2101908u32);
    emu.apc_no_count(1usize, 2101908u32, 8192u32, 2101912u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020129c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 336u32, 2101920u32);
    emu.lw_no_count(10usize, 2usize, 264u32, 2101924u32)?;
    emu.lw_no_count(11usize, 2usize, 268u32, 2101928u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101932u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2101936u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2101940u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2101944u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2101948u32);
    emu.sli_no_count(16usize, 9usize, 3u32, 2101952u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2101956u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2101960u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2101964u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2101968u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2101972u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2101976u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2101980u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2101984u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2101988u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2101992u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2101996u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2102000u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2102004u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2102008u32);
    emu.sli_no_count(10usize, 9usize, 27u32, 2102012u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2102016u32);
    emu.adi_no_count(10usize, 2usize, 272u32, 2102020u32);
    emu.adr_no_count(10usize, 10usize, 9usize, 2102024u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2102028u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2102032u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2102036u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2102040u32);
    emu.orr_no_count(19usize, 15usize, 14usize, 2102044u32);
    emu.orr_no_count(18usize, 11usize, 12usize, 2102048u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2102052u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2102088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201348));
    } else {
        emu.pc = 2102056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201328));
    }
}
#[inline(always)]
pub fn block_0x00201328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2102060u32);
    emu.xri_no_count(12usize, 9usize, 63u32, 2102064u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2102068u32);
    emu.apc_no_count(1usize, 2102068u32, 8192u32, 2102072u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020133c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 9usize, 56u32, 2102080u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2102084u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2102200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013b8));
    } else {
        emu.pc = 2102088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201348));
    }
}
#[inline(always)]
pub fn block_0x00201348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 232u32, 2102092u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2102096u32);
    emu.adi_no_count(11usize, 2usize, 272u32, 2102100u32);
    emu.apc_no_count(1usize, 2102100u32, 8192u32, 2102104u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020135c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 348u32, 2102112u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2102116u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2102120u32);
    emu.apc_no_count(1usize, 2102120u32, 8192u32, 2102124u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102128u32;
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
#[inline]
pub fn block_0x00201370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 19usize, 24u32, 2102132u32);
    emu.sri_no_count(11usize, 19usize, 16u32, 2102136u32);
    emu.sri_no_count(12usize, 19usize, 8u32, 2102140u32);
    emu.sri_no_count(13usize, 18usize, 24u32, 2102144u32);
    emu.sri_no_count(14usize, 18usize, 16u32, 2102148u32);
    emu.sb_no_count(19usize, 2usize, 408u32, 2102152u32);
    emu.sb_no_count(12usize, 2usize, 409u32, 2102156u32);
    emu.sb_no_count(11usize, 2usize, 410u32, 2102160u32);
    emu.sb_no_count(10usize, 2usize, 411u32, 2102164u32);
    emu.sri_no_count(10usize, 18usize, 8u32, 2102168u32);
    emu.sb_no_count(18usize, 2usize, 404u32, 2102172u32);
    emu.sb_no_count(10usize, 2usize, 405u32, 2102176u32);
    emu.sb_no_count(14usize, 2usize, 406u32, 2102180u32);
    emu.sb_no_count(13usize, 2usize, 407u32, 2102184u32);
    emu.adi_no_count(10usize, 2usize, 232u32, 2102188u32);
    emu.adi_no_count(11usize, 2usize, 348u32, 2102192u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2102196u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2102200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002013cc));
}
#[inline(always)]
pub fn block_0x002013b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 328u32, 2102204u32)?;
    emu.sw_no_count(19usize, 2usize, 332u32, 2102208u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2102212u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2102216u32);
    emu.adi_no_count(11usize, 2usize, 272u32, 2102220u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2102220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002013cc));
}
#[inline(always)]
pub fn block_0x002013cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2102220u32, 8192u32, 2102224u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002013d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 177u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 8usize, 8u32, 2102232u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102236u32;
    emu.update_insn_clock();
    emu.sri_no_count(12usize, 8usize, 24u32, 2102240u32);
    emu.sli_no_count(29usize, 8usize, 24u32, 2102244u32);
    emu.sri_no_count(13usize, 23usize, 8u32, 2102248u32);
    emu.sri_no_count(14usize, 23usize, 24u32, 2102252u32);
    emu.sli_no_count(30usize, 23usize, 24u32, 2102256u32);
    emu.sri_no_count(15usize, 26usize, 8u32, 2102260u32);
    emu.sri_no_count(16usize, 26usize, 24u32, 2102264u32);
    emu.sli_no_count(9usize, 26usize, 24u32, 2102268u32);
    emu.sri_no_count(17usize, 25usize, 8u32, 2102272u32);
    emu.sri_no_count(5usize, 25usize, 24u32, 2102276u32);
    emu.sli_no_count(6usize, 25usize, 24u32, 2102280u32);
    emu.sri_no_count(7usize, 24usize, 8u32, 2102284u32);
    emu.sri_no_count(31usize, 24usize, 24u32, 2102288u32);
    emu.sli_no_count(28usize, 24usize, 24u32, 2102292u32);
    emu.sri_no_count(19usize, 27usize, 8u32, 2102296u32);
    emu.sri_no_count(20usize, 27usize, 24u32, 2102300u32);
    emu.adi_no_count(10usize, 10usize, 4294967040u32, 2102304u32);
    emu.anr_no_count(11usize, 11usize, 10usize, 2102308u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2102312u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2102316u32)?;
    emu.sli_no_count(18usize, 27usize, 24u32, 2102320u32);
    emu.anr_no_count(12usize, 13usize, 10usize, 2102324u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2102328u32);
    emu.sri_no_count(1usize, 22usize, 8u32, 2102332u32);
    emu.anr_no_count(15usize, 15usize, 10usize, 2102336u32);
    emu.orr_no_count(13usize, 15usize, 16usize, 2102340u32);
    emu.adi_no_count(21usize, 22usize, 0u32, 2102344u32);
    emu.sri_no_count(22usize, 22usize, 24u32, 2102348u32);
    emu.anr_no_count(14usize, 17usize, 10usize, 2102352u32);
    emu.orr_no_count(14usize, 14usize, 5usize, 2102356u32);
    emu.sli_no_count(11usize, 21usize, 24u32, 2102360u32);
    emu.anr_no_count(15usize, 7usize, 10usize, 2102364u32);
    emu.orr_no_count(15usize, 15usize, 31usize, 2102368u32);
    emu.lw_no_count(7usize, 2usize, 4u32, 2102372u32)?;
    emu.sri_no_count(5usize, 7usize, 8u32, 2102376u32);
    emu.anr_no_count(16usize, 19usize, 10usize, 2102380u32);
    emu.orr_no_count(16usize, 16usize, 20usize, 2102384u32);
    emu.sri_no_count(31usize, 7usize, 24u32, 2102388u32);
    emu.anr_no_count(17usize, 1usize, 10usize, 2102392u32);
    emu.orr_no_count(17usize, 17usize, 22usize, 2102396u32);
    emu.sli_no_count(1usize, 7usize, 24u32, 2102400u32);
    emu.anr_no_count(8usize, 8usize, 10usize, 2102404u32);
    emu.anr_no_count(19usize, 23usize, 10usize, 2102408u32);
    emu.anr_no_count(20usize, 26usize, 10usize, 2102412u32);
    emu.anr_no_count(5usize, 5usize, 10usize, 2102416u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2102420u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2102424u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2102428u32);
    emu.orr_no_count(5usize, 5usize, 31usize, 2102432u32);
    emu.orr_no_count(31usize, 29usize, 8usize, 2102436u32);
    emu.orr_no_count(30usize, 30usize, 19usize, 2102440u32);
    emu.orr_no_count(29usize, 9usize, 20usize, 2102444u32);
    emu.lw_no_count(8usize, 2usize, 232u32, 2102448u32)?;
    emu.lw_no_count(9usize, 2usize, 236u32, 2102452u32)?;
    emu.lw_no_count(19usize, 2usize, 240u32, 2102456u32)?;
    emu.lw_no_count(20usize, 2usize, 244u32, 2102460u32)?;
    emu.anr_no_count(22usize, 25usize, 10usize, 2102464u32);
    emu.anr_no_count(24usize, 24usize, 10usize, 2102468u32);
    emu.anr_no_count(23usize, 27usize, 10usize, 2102472u32);
    emu.anr_no_count(25usize, 21usize, 10usize, 2102476u32);
    emu.sli_no_count(22usize, 22usize, 8u32, 2102480u32);
    emu.sli_no_count(24usize, 24usize, 8u32, 2102484u32);
    emu.sli_no_count(23usize, 23usize, 8u32, 2102488u32);
    emu.sli_no_count(25usize, 25usize, 8u32, 2102492u32);
    emu.orr_no_count(26usize, 6usize, 22usize, 2102496u32);
    emu.orr_no_count(24usize, 28usize, 24usize, 2102500u32);
    emu.orr_no_count(23usize, 18usize, 23usize, 2102504u32);
    emu.orr_no_count(22usize, 11usize, 25usize, 2102508u32);
    emu.lw_no_count(28usize, 2usize, 248u32, 2102512u32)?;
    emu.lw_no_count(18usize, 2usize, 252u32, 2102516u32)?;
    emu.lw_no_count(21usize, 2usize, 256u32, 2102520u32)?;
    emu.lw_no_count(6usize, 2usize, 260u32, 2102524u32)?;
    emu.anr_no_count(25usize, 7usize, 10usize, 2102528u32);
    emu.sli_no_count(25usize, 25usize, 8u32, 2102532u32);
    emu.orr_no_count(25usize, 1usize, 25usize, 2102536u32);
    emu.sri_no_count(27usize, 8usize, 8u32, 2102540u32);
    emu.lw_no_count(11usize, 2usize, 0u32, 2102544u32)?;
    emu.orr_no_count(11usize, 31usize, 11usize, 2102548u32);
    emu.sri_no_count(31usize, 8usize, 24u32, 2102552u32);
    emu.orr_no_count(12usize, 30usize, 12usize, 2102556u32);
    emu.anr_no_count(30usize, 8usize, 10usize, 2102560u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2102564u32);
    emu.orr_no_count(13usize, 29usize, 13usize, 2102568u32);
    emu.sri_no_count(1usize, 9usize, 8u32, 2102572u32);
    emu.orr_no_count(14usize, 26usize, 14usize, 2102576u32);
    emu.sri_no_count(26usize, 9usize, 24u32, 2102580u32);
    emu.orr_no_count(15usize, 24usize, 15usize, 2102584u32);
    emu.anr_no_count(24usize, 9usize, 10usize, 2102588u32);
    emu.sli_no_count(9usize, 9usize, 24u32, 2102592u32);
    emu.orr_no_count(16usize, 23usize, 16usize, 2102596u32);
    emu.sri_no_count(23usize, 19usize, 8u32, 2102600u32);
    emu.orr_no_count(7usize, 22usize, 17usize, 2102604u32);
    emu.sri_no_count(22usize, 19usize, 24u32, 2102608u32);
    emu.orr_no_count(29usize, 25usize, 5usize, 2102612u32);
    emu.anr_no_count(25usize, 19usize, 10usize, 2102616u32);
    emu.sli_no_count(19usize, 19usize, 24u32, 2102620u32);
    emu.anr_no_count(17usize, 27usize, 10usize, 2102624u32);
    emu.orr_no_count(17usize, 17usize, 31usize, 2102628u32);
    emu.sri_no_count(31usize, 20usize, 8u32, 2102632u32);
    emu.sli_no_count(30usize, 30usize, 8u32, 2102636u32);
    emu.orr_no_count(5usize, 8usize, 30usize, 2102640u32);
    emu.sri_no_count(8usize, 20usize, 24u32, 2102644u32);
    emu.anr_no_count(30usize, 1usize, 10usize, 2102648u32);
    emu.orr_no_count(30usize, 30usize, 26usize, 2102652u32);
    emu.anr_no_count(26usize, 20usize, 10usize, 2102656u32);
    emu.sli_no_count(20usize, 20usize, 24u32, 2102660u32);
    emu.sli_no_count(24usize, 24usize, 8u32, 2102664u32);
    emu.orr_no_count(9usize, 9usize, 24usize, 2102668u32);
    emu.sri_no_count(24usize, 28usize, 8u32, 2102672u32);
    emu.anr_no_count(23usize, 23usize, 10usize, 2102676u32);
    emu.orr_no_count(22usize, 23usize, 22usize, 2102680u32);
    emu.sri_no_count(23usize, 28usize, 24u32, 2102684u32);
    emu.sli_no_count(25usize, 25usize, 8u32, 2102688u32);
    emu.orr_no_count(19usize, 19usize, 25usize, 2102692u32);
    emu.anr_no_count(25usize, 28usize, 10usize, 2102696u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2102700u32);
    emu.anr_no_count(31usize, 31usize, 10usize, 2102704u32);
    emu.orr_no_count(31usize, 31usize, 8usize, 2102708u32);
    emu.sri_no_count(8usize, 18usize, 8u32, 2102712u32);
    emu.sli_no_count(26usize, 26usize, 8u32, 2102716u32);
    emu.orr_no_count(20usize, 20usize, 26usize, 2102720u32);
    emu.sri_no_count(26usize, 18usize, 24u32, 2102724u32);
    emu.anr_no_count(24usize, 24usize, 10usize, 2102728u32);
    emu.orr_no_count(23usize, 24usize, 23usize, 2102732u32);
    emu.anr_no_count(24usize, 18usize, 10usize, 2102736u32);
    emu.sli_no_count(18usize, 18usize, 24u32, 2102740u32);
    emu.sli_no_count(25usize, 25usize, 8u32, 2102744u32);
    emu.orr_no_count(28usize, 28usize, 25usize, 2102748u32);
    emu.sri_no_count(25usize, 21usize, 8u32, 2102752u32);
    emu.anr_no_count(8usize, 8usize, 10usize, 2102756u32);
    emu.orr_no_count(8usize, 8usize, 26usize, 2102760u32);
    emu.sri_no_count(26usize, 21usize, 24u32, 2102764u32);
    emu.sli_no_count(24usize, 24usize, 8u32, 2102768u32);
    emu.orr_no_count(18usize, 18usize, 24usize, 2102772u32);
    emu.anr_no_count(24usize, 21usize, 10usize, 2102776u32);
    emu.sli_no_count(21usize, 21usize, 24u32, 2102780u32);
    emu.anr_no_count(25usize, 25usize, 10usize, 2102784u32);
    emu.orr_no_count(25usize, 25usize, 26usize, 2102788u32);
    emu.sri_no_count(26usize, 6usize, 8u32, 2102792u32);
    emu.sli_no_count(24usize, 24usize, 8u32, 2102796u32);
    emu.orr_no_count(21usize, 21usize, 24usize, 2102800u32);
    emu.sri_no_count(24usize, 6usize, 24u32, 2102804u32);
    emu.anr_no_count(26usize, 26usize, 10usize, 2102808u32);
    emu.orr_no_count(24usize, 26usize, 24usize, 2102812u32);
    emu.anr_no_count(10usize, 6usize, 10usize, 2102816u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2102820u32);
    emu.sli_no_count(10usize, 10usize, 8u32, 2102824u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2102828u32);
    emu.sw_no_count(29usize, 2usize, 232u32, 2102832u32)?;
    emu.sw_no_count(7usize, 2usize, 236u32, 2102836u32)?;
    emu.sw_no_count(16usize, 2usize, 240u32, 2102840u32)?;
    emu.sw_no_count(15usize, 2usize, 244u32, 2102844u32)?;
    emu.sw_no_count(14usize, 2usize, 248u32, 2102848u32)?;
    emu.sw_no_count(13usize, 2usize, 252u32, 2102852u32)?;
    emu.sw_no_count(12usize, 2usize, 256u32, 2102856u32)?;
    emu.sw_no_count(11usize, 2usize, 260u32, 2102860u32)?;
    emu.orr_no_count(11usize, 5usize, 17usize, 2102864u32);
    emu.orr_no_count(12usize, 9usize, 30usize, 2102868u32);
    emu.orr_no_count(13usize, 19usize, 22usize, 2102872u32);
    emu.orr_no_count(14usize, 20usize, 31usize, 2102876u32);
    emu.orr_no_count(15usize, 28usize, 23usize, 2102880u32);
    emu.orr_no_count(16usize, 18usize, 8usize, 2102884u32);
    emu.orr_no_count(17usize, 21usize, 25usize, 2102888u32);
    emu.orr_no_count(10usize, 10usize, 24usize, 2102892u32);
    emu.sw_no_count(11usize, 2usize, 264u32, 2102896u32)?;
    emu.sw_no_count(12usize, 2usize, 268u32, 2102900u32)?;
    emu.sw_no_count(13usize, 2usize, 272u32, 2102904u32)?;
    emu.sw_no_count(14usize, 2usize, 276u32, 2102908u32)?;
    emu.sw_no_count(15usize, 2usize, 280u32, 2102912u32)?;
    emu.sw_no_count(16usize, 2usize, 284u32, 2102916u32)?;
    emu.sw_no_count(17usize, 2usize, 288u32, 2102920u32)?;
    emu.sw_no_count(10usize, 2usize, 292u32, 2102924u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2102928u32);
    emu.apc_no_count(1usize, 2102928u32, 0u32, 2102932u32);
    emu.add_memory_rw_events(177usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 460u32, 2102940u32)?;
    emu.lw_no_count(8usize, 2usize, 456u32, 2102944u32)?;
    emu.lw_no_count(9usize, 2usize, 452u32, 2102948u32)?;
    emu.lw_no_count(18usize, 2usize, 448u32, 2102952u32)?;
    emu.lw_no_count(19usize, 2usize, 444u32, 2102956u32)?;
    emu.lw_no_count(20usize, 2usize, 440u32, 2102960u32)?;
    emu.lw_no_count(21usize, 2usize, 436u32, 2102964u32)?;
    emu.lw_no_count(22usize, 2usize, 432u32, 2102968u32)?;
    emu.lw_no_count(23usize, 2usize, 428u32, 2102972u32)?;
    emu.lw_no_count(24usize, 2usize, 424u32, 2102976u32)?;
    emu.lw_no_count(25usize, 2usize, 420u32, 2102980u32)?;
    emu.lw_no_count(26usize, 2usize, 416u32, 2102984u32)?;
    emu.lw_no_count(27usize, 2usize, 412u32, 2102988u32)?;
    emu.adi_no_count(2usize, 2usize, 464u32, 2102992u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102996u32;
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
pub fn block_0x002016d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 232u32, 2103000u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103004u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966872u32, 2103008u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2103012u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966820u32, 2103016u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2103020u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966896u32, 2103024u32);
    emu.adi_no_count(11usize, 0usize, 22u32, 2103028u32);
    emu.adi_no_count(12usize, 2usize, 232u32, 2103032u32);
    emu.apc_no_count(1usize, 2103032u32, 57344u32, 2103036u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
