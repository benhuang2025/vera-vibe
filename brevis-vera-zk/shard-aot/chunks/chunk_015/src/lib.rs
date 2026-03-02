pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2154424u32;
pub const PC_MAX: u32 = 2156944u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 104usize] = [
        block_0x0020dfb8,
        block_0x0020dfd0,
        block_0x0020e0c4,
        block_0x0020e0d0,
        block_0x0020e0dc,
        block_0x0020e0e0,
        block_0x0020e0e4,
        block_0x0020e0fc,
        block_0x0020e110,
        block_0x0020e120,
        block_0x0020e128,
        block_0x0020e130,
        block_0x0020e17c,
        block_0x0020e190,
        block_0x0020e1a0,
        block_0x0020e1b4,
        block_0x0020e1b8,
        block_0x0020e1bc,
        block_0x0020e1c0,
        block_0x0020e1c8,
        block_0x0020e1cc,
        block_0x0020e1d0,
        block_0x0020e1d8,
        block_0x0020e1e8,
        block_0x0020e210,
        block_0x0020e280,
        block_0x0020e290,
        block_0x0020e300,
        block_0x0020e304,
        block_0x0020e328,
        block_0x0020e338,
        block_0x0020e34c,
        block_0x0020e368,
        block_0x0020e36c,
        block_0x0020e38c,
        block_0x0020e3a0,
        block_0x0020e3b0,
        block_0x0020e3c4,
        block_0x0020e3e0,
        block_0x0020e3e4,
        block_0x0020e400,
        block_0x0020e440,
        block_0x0020e57c,
        block_0x0020e584,
        block_0x0020e594,
        block_0x0020e5e8,
        block_0x0020e5f0,
        block_0x0020e5fc,
        block_0x0020e604,
        block_0x0020e610,
        block_0x0020e620,
        block_0x0020e628,
        block_0x0020e630,
        block_0x0020e63c,
        block_0x0020e640,
        block_0x0020e648,
        block_0x0020e654,
        block_0x0020e664,
        block_0x0020e66c,
        block_0x0020e674,
        block_0x0020e678,
        block_0x0020e680,
        block_0x0020e690,
        block_0x0020e694,
        block_0x0020e698,
        block_0x0020e708,
        block_0x0020e710,
        block_0x0020e730,
        block_0x0020e748,
        block_0x0020e750,
        block_0x0020e754,
        block_0x0020e774,
        block_0x0020e77c,
        block_0x0020e784,
        block_0x0020e798,
        block_0x0020e7a8,
        block_0x0020e7b0,
        block_0x0020e7b8,
        block_0x0020e7d0,
        block_0x0020e7ec,
        block_0x0020e7f0,
        block_0x0020e80c,
        block_0x0020e814,
        block_0x0020e824,
        block_0x0020e84c,
        block_0x0020e868,
        block_0x0020e8b4,
        block_0x0020e8bc,
        block_0x0020e8c4,
        block_0x0020e8f8,
        block_0x0020e900,
        block_0x0020e904,
        block_0x0020e920,
        block_0x0020e92c,
        block_0x0020e930,
        block_0x0020e93c,
        block_0x0020e940,
        block_0x0020e948,
        block_0x0020e954,
        block_0x0020e960,
        block_0x0020e964,
        block_0x0020e968,
        block_0x0020e984,
        block_0x0020e990,
    ];
    const IDX: [u16; 631usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 3u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 6u16, 7u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 10u16,
        0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16,
        0u16, 14u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16,
        18u16, 19u16, 0u16, 20u16, 21u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16,
        0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16, 0u16, 0u16,
        48u16, 0u16, 49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16, 52u16,
        0u16, 53u16, 0u16, 0u16, 54u16, 55u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16,
        0u16, 0u16, 58u16, 0u16, 59u16, 0u16, 60u16, 61u16, 0u16, 62u16, 0u16, 0u16,
        0u16, 63u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 70u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 74u16, 0u16, 0u16,
        0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 81u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 94u16,
        95u16, 0u16, 0u16, 96u16, 97u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16, 0u16,
        100u16, 101u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16,
        104u16,
    ];
    if pc < 2154424u32 || pc > 2156944u32 {
        return None;
    }
    let word_offset = ((pc - 2154424u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020dfb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2154428u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154432u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1355u32, 2154436u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2154440u32);
    emu.apc_no_count(1usize, 2154440u32, 4294959104u32, 2154444u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020dfd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 61u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(73728u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154452u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2154456u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1400u32, 2154460u32);
    emu.adi_no_count(11usize, 11usize, 4294965295u32, 2154464u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2154468u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2154472u32);
    emu.ani_no_count(11usize, 11usize, 17u32, 2154476u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154480u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2154484u32);
    emu.lw_no_count(12usize, 12usize, 32u32, 2154488u32)?;
    emu.sli_no_count(14usize, 10usize, 11u32, 2154492u32);
    emu.sli_no_count(12usize, 12usize, 11u32, 2154496u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154500u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154504u32);
    emu.sli_no_count(12usize, 12usize, 3u32, 2154508u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2154512u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154516u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2154520u32);
    emu.lw_no_count(12usize, 12usize, 16u32, 2154524u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154528u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154532u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154536u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2154540u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2154544u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154548u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2154552u32);
    emu.lw_no_count(12usize, 12usize, 8u32, 2154556u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154560u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154564u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154568u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2154572u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2154576u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154580u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2154584u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2154588u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154592u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154596u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154600u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2154604u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154608u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2154612u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2154616u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154620u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154624u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154628u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2154632u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154636u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2154640u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2154644u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154648u32);
    emu.xrr_no_count(15usize, 12usize, 14usize, 2154652u32);
    emu.sltru_no_count(12usize, 12usize, 14usize, 2154656u32);
    emu.sltiu_no_count(14usize, 15usize, 1u32, 2154660u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2154664u32);
    emu.adr_no_count(14usize, 14usize, 11usize, 2154668u32);
    emu.sli_no_count(11usize, 14usize, 2u32, 2154672u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2154676u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2154680u32)?;
    emu.adi_no_count(12usize, 0usize, 32u32, 2154684u32);
    emu.sri_no_count(11usize, 11usize, 21u32, 2154688u32);
    emu.add_memory_rw_events(60usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2154720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0e0));
    } else {
        emu.pc = 2154692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0c4));
    }
}
#[inline(always)]
pub fn block_0x0020e0c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2154696u32)?;
    emu.sri_no_count(12usize, 12usize, 21u32, 2154700u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2154724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0e4));
    } else {
        emu.pc = 2154704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0d0));
    }
}
#[inline(always)]
pub fn block_0x0020e0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2154708u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2154712u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2154748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0fc));
    } else {
        emu.pc = 2154716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0dc));
    }
}
#[inline(always)]
pub fn block_0x0020e0dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2154720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e128));
}
#[inline(always)]
pub fn block_0x0020e0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 751u32, 2154724u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0e4));
}
#[inline(always)]
pub fn block_0x0020e0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 13usize, 4294967292u32, 2154728u32)?;
    emu.sli_no_count(13usize, 13usize, 11u32, 2154732u32);
    emu.sri_no_count(14usize, 13usize, 11u32, 2154736u32);
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2154740u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2154744u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2154792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e128));
    } else {
        emu.pc = 2154748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0fc));
    }
}
#[inline(always)]
pub fn block_0x0020e0fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2154752u32);
    emu.sbr_no_count(10usize, 10usize, 14usize, 2154756u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2154760u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2154764u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 604u32, 2154768u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2154768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e110));
}
#[inline(always)]
pub fn block_0x0020e110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 11usize, 2154772u32);
    emu.lbu_no_count(15usize, 15usize, 0u32, 2154776u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2154780u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2154792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e128));
    } else {
        emu.pc = 2154784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e120));
    }
}
#[inline(always)]
pub fn block_0x0020e120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 1u32, 2154788u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2154768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e110));
    } else {
        emu.pc = 2154792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e128));
    }
}
#[inline(always)]
pub fn block_0x0020e128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2154796u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154800u32;
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
pub fn block_0x0020e130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2154804u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2154808u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2154812u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2154816u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2154820u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2154824u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2154828u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2154832u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2154836u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2154840u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2154844u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2154848u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2154852u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2154856u32)?;
    emu.adi_no_count(26usize, 12usize, 0u32, 2154860u32);
    emu.lw_no_count(14usize, 11usize, 0u32, 2154864u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2154868u32)?;
    emu.orr_no_count(12usize, 14usize, 16usize, 2154872u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2157984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157984u32));
    } else {
        emu.pc = 2154876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e17c));
    }
}
#[inline(always)]
pub fn block_0x0020e17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2154880u32);
    emu.lw_no_count(17usize, 11usize, 8u32, 2154884u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2154888u32)?;
    emu.orr_no_count(10usize, 17usize, 6usize, 2154892u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158012u32));
    } else {
        emu.pc = 2154896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e190));
    }
}
#[inline(always)]
pub fn block_0x0020e190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 16u32, 2154900u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2154904u32)?;
    emu.orr_no_count(12usize, 10usize, 5usize, 2154908u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158040u32));
    } else {
        emu.pc = 2154912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1a0));
    }
}
#[inline(always)]
pub fn block_0x0020e1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 10usize, 2154916u32);
    emu.sltru_no_count(7usize, 15usize, 14usize, 2154920u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2154924u32);
    emu.adr_no_count(12usize, 5usize, 7usize, 2154928u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2154936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1b8));
    } else {
        emu.pc = 2154932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1b4));
    }
}
#[inline(always)]
pub fn block_0x0020e1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 16usize, 2154936u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1b8));
}
#[inline(always)]
pub fn block_0x0020e1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2158068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158068u32));
    } else {
        emu.pc = 2154940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1bc));
    }
}
#[inline(always)]
pub fn block_0x0020e1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2154952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1c8));
    } else {
        emu.pc = 2154944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1c0));
    }
}
#[inline(always)]
pub fn block_0x0020e1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 16usize, 6usize, 2154948u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154956u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1cc));
}
#[inline(always)]
pub fn block_0x0020e1c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 17usize, 2154956u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1cc));
}
#[inline(always)]
pub fn block_0x0020e1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2158096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158096u32));
    } else {
        emu.pc = 2154960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1d0));
    }
}
#[inline(always)]
pub fn block_0x0020e1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 16u32, 2154964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2158124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158124u32));
    } else {
        emu.pc = 2154968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1d8));
    }
}
#[inline(always)]
pub fn block_0x0020e1d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 10usize, 2154972u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2154976u32);
    emu.sri_no_count(12usize, 10usize, 29u32, 2154980u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158152u32));
    } else {
        emu.pc = 2154984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1e8));
    }
}
#[inline]
pub fn block_0x0020e1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 15usize, 1u32, 2154988u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2154992u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2154996u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2155000u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2155004u32;
    emu.update_insn_clock();
    emu.adi_no_count(30usize, 12usize, 1365u32, 2155008u32);
    emu.adi_no_count(29usize, 7usize, 819u32, 2155012u32);
    emu.adi_no_count(28usize, 28usize, 4294967055u32, 2155016u32);
    emu.adi_no_count(7usize, 31usize, 257u32, 2155020u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2155152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e290));
    } else {
        emu.pc = 2155024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e210));
    }
}
#[inline(never)]
pub fn block_0x0020e210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(12usize, 15usize, 5usize, 2155028u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2155032u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155036u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2155040u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155044u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2155048u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155052u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2155056u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155060u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2155064u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2155068u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2155072u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2155076u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2155080u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2155084u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2155088u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2155092u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2155096u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2155100u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2155104u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2155108u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2155112u32);
    emu.adi_no_count(7usize, 12usize, 32u32, 2155116u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2155120u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2155124u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2155128u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2155132u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2155264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e300));
    } else {
        emu.pc = 2155136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e280));
    }
}
#[inline(always)]
pub fn block_0x0020e280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 10usize, 7usize, 2155140u32);
    emu.srr_no_count(11usize, 5usize, 8usize, 2155144u32);
    emu.orr_no_count(11usize, 10usize, 11usize, 2155148u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2155152u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e304));
}
#[inline(never)]
pub fn block_0x0020e290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 10usize, 1u32, 2155156u32);
    emu.orr_no_count(12usize, 10usize, 12usize, 2155160u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2155164u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155168u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2155172u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155176u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2155180u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155184u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2155188u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155192u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2155196u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2155200u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2155204u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2155208u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2155212u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2155216u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2155220u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2155224u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2155228u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2155232u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2155236u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2155240u32);
    emu.sri_no_count(7usize, 12usize, 24u32, 2155244u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2155248u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2155252u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2155256u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2155260u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e280));
    } else {
        emu.pc = 2155264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e300));
    }
}
#[inline(always)]
pub fn block_0x0020e300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(11usize, 15usize, 31usize, 2155268u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e304));
}
#[inline]
pub fn block_0x0020e304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 9usize, 1055u32, 2155272u32);
    emu.sltru_no_count(10usize, 14usize, 17usize, 2155276u32);
    emu.sbr_no_count(12usize, 16usize, 6usize, 2155280u32);
    emu.sbr_no_count(28usize, 14usize, 17usize, 2155284u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2155288u32);
    emu.sw_no_count(28usize, 2usize, 24u32, 2155292u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2155296u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2155300u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e34c));
    } else {
        emu.pc = 2155304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e328));
    }
}
#[inline(always)]
pub fn block_0x0020e328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(17usize, 28usize, 31usize, 2155308u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2155312u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2155316u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2155368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e368));
    } else {
        emu.pc = 2155320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e338));
    }
}
#[inline(always)]
pub fn block_0x0020e338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 6usize, 7usize, 2155324u32);
    emu.sli_no_count(29usize, 17usize, 1u32, 2155328u32);
    emu.slr_no_count(29usize, 29usize, 8usize, 2155332u32);
    emu.orr_no_count(29usize, 12usize, 29usize, 2155336u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2155340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e36c));
}
#[inline(always)]
pub fn block_0x0020e34c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 10usize, 7usize, 2155344u32);
    emu.sri_no_count(17usize, 28usize, 1u32, 2155348u32);
    emu.srr_no_count(17usize, 17usize, 8usize, 2155352u32);
    emu.orr_no_count(17usize, 12usize, 17usize, 2155356u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2155360u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2155364u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e338));
    } else {
        emu.pc = 2155368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e368));
    }
}
#[inline(always)]
pub fn block_0x0020e368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(29usize, 17usize, 31usize, 2155372u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e36c));
}
#[inline(always)]
pub fn block_0x0020e36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 17usize, 7usize, 2155376u32);
    emu.xrr_no_count(28usize, 29usize, 28usize, 2155380u32);
    emu.anr_no_count(12usize, 5usize, 12usize, 2155384u32);
    emu.xrr_no_count(10usize, 12usize, 10usize, 2155388u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2155392u32);
    emu.sw_no_count(29usize, 2usize, 40u32, 2155396u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2155400u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157948u32));
    } else {
        emu.pc = 2155404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e38c));
    }
}
#[inline(always)]
pub fn block_0x0020e38c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 24u32, 2155408u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2155412u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2155416u32)?;
    emu.slr_no_count(10usize, 14usize, 31usize, 2155420u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3c4));
    } else {
        emu.pc = 2155424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3a0));
    }
}
#[inline(always)]
pub fn block_0x0020e3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 10usize, 0u32, 2155428u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2155432u32);
    emu.srr_no_count(10usize, 10usize, 31usize, 2155436u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2155488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3e0));
    } else {
        emu.pc = 2155440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3b0));
    }
}
#[inline(always)]
pub fn block_0x0020e3b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 28usize, 1u32, 2155444u32);
    emu.slr_no_count(12usize, 12usize, 8usize, 2155448u32);
    emu.srr_no_count(31usize, 29usize, 31usize, 2155452u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2155456u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2155460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155492u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e3e4));
}
#[inline(always)]
pub fn block_0x0020e3c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 14usize, 1u32, 2155464u32);
    emu.srr_no_count(12usize, 12usize, 8usize, 2155468u32);
    emu.slr_no_count(28usize, 16usize, 31usize, 2155472u32);
    emu.orr_no_count(28usize, 28usize, 12usize, 2155476u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2155480u32);
    emu.srr_no_count(10usize, 28usize, 31usize, 2155484u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3b0));
    } else {
        emu.pc = 2155488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3e0));
    }
}
#[inline(always)]
pub fn block_0x0020e3e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 10usize, 0u32, 2155492u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155492u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e3e4));
}
#[inline(always)]
pub fn block_0x0020e3e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 5usize, 10usize, 2155496u32);
    emu.xrr_no_count(12usize, 31usize, 14usize, 2155500u32);
    emu.xrr_no_count(14usize, 10usize, 16usize, 2155504u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2155508u32);
    emu.sw_no_count(31usize, 2usize, 40u32, 2155512u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2155516u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2157948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157948u32));
    } else {
        emu.pc = 2155520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e400));
    }
}
#[inline]
pub fn block_0x0020e400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 16u32, 2155524u32)?;
    emu.sbr_no_count(10usize, 30usize, 7usize, 2155528u32);
    emu.adi_no_count(12usize, 0usize, 4294967200u32, 2155532u32);
    emu.adi_no_count(16usize, 0usize, 80u32, 2155536u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2155540u32;
    emu.update_insn_clock();
    emu.sbr_no_count(12usize, 12usize, 10usize, 2155544u32);
    emu.adi_no_count(14usize, 14usize, 4294965651u32, 2155548u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2155552u32);
    emu.sai_no_count(12usize, 12usize, 1040u32, 2155556u32);
    emu.adi_no_count(12usize, 12usize, 1087u32, 2155560u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2155564u32);
    emu.mulh_no_count(12usize, 12usize, 14usize, 2155568u32);
    emu.sri_no_count(14usize, 12usize, 31u32, 2155572u32);
    emu.sai_no_count(12usize, 12usize, 1034u32, 2155576u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2155580u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2158228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158228u32));
    } else {
        emu.pc = 2155584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e440));
    }
}
#[inline(never)]
pub fn block_0x0020e440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 15usize, 7usize, 2155588u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2155592u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2155596u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1536u32, 2155600u32);
    emu.sbr_no_count(15usize, 0usize, 10usize, 2155604u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2155608u32);
    emu.lw_no_count(14usize, 16usize, 0u32, 2155612u32)?;
    emu.lw_no_count(7usize, 16usize, 4u32, 2155616u32)?;
    emu.anr_no_count(10usize, 5usize, 12usize, 2155620u32);
    emu.lh_no_count(12usize, 16usize, 8u32, 2155624u32)?;
    emu.mulhu_no_count(30usize, 14usize, 10usize, 2155628u32);
    emu.mul_no_count(31usize, 7usize, 10usize, 2155632u32);
    emu.mulhu_no_count(8usize, 7usize, 10usize, 2155636u32);
    emu.mul_no_count(9usize, 14usize, 11usize, 2155640u32);
    emu.mulhu_no_count(18usize, 14usize, 11usize, 2155644u32);
    emu.mul_no_count(10usize, 7usize, 11usize, 2155648u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2155652u32);
    emu.sbr_no_count(15usize, 15usize, 12usize, 2155656u32);
    emu.mulhu_no_count(12usize, 14usize, 6usize, 2155660u32);
    emu.mul_no_count(19usize, 7usize, 6usize, 2155664u32);
    emu.mulhu_no_count(6usize, 7usize, 6usize, 2155668u32);
    emu.mul_no_count(20usize, 14usize, 17usize, 2155672u32);
    emu.mulhu_no_count(21usize, 14usize, 17usize, 2155676u32);
    emu.mul_no_count(5usize, 7usize, 17usize, 2155680u32);
    emu.mulhu_no_count(17usize, 7usize, 17usize, 2155684u32);
    emu.mulhu_no_count(22usize, 14usize, 29usize, 2155688u32);
    emu.mul_no_count(23usize, 7usize, 29usize, 2155692u32);
    emu.mulhu_no_count(29usize, 7usize, 29usize, 2155696u32);
    emu.mul_no_count(24usize, 14usize, 28usize, 2155700u32);
    emu.mulhu_no_count(14usize, 14usize, 28usize, 2155704u32);
    emu.mul_no_count(25usize, 7usize, 28usize, 2155708u32);
    emu.mulhu_no_count(28usize, 7usize, 28usize, 2155712u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2155716u32);
    emu.sltru_no_count(7usize, 30usize, 31usize, 2155720u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2155724u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2155728u32);
    emu.sltru_no_count(31usize, 12usize, 19usize, 2155732u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2155736u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2155740u32);
    emu.sltru_no_count(6usize, 22usize, 23usize, 2155744u32);
    emu.adr_no_count(19usize, 29usize, 6usize, 2155748u32);
    emu.adr_no_count(30usize, 9usize, 30usize, 2155752u32);
    emu.sltru_no_count(6usize, 30usize, 9usize, 2155756u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2155760u32);
    emu.adr_no_count(6usize, 20usize, 12usize, 2155764u32);
    emu.sltru_no_count(12usize, 6usize, 20usize, 2155768u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2155772u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2155776u32);
    emu.sltru_no_count(29usize, 22usize, 24usize, 2155780u32);
    emu.adr_no_count(14usize, 14usize, 29usize, 2155784u32);
    emu.adr_no_count(18usize, 7usize, 18usize, 2155788u32);
    emu.sltru_no_count(7usize, 18usize, 7usize, 2155792u32);
    emu.adr_no_count(9usize, 11usize, 7usize, 2155796u32);
    emu.ani_no_count(7usize, 15usize, 63u32, 2155800u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2155804u32);
    emu.sltru_no_count(8usize, 12usize, 31usize, 2155808u32);
    emu.adr_no_count(8usize, 17usize, 8usize, 2155812u32);
    emu.adi_no_count(29usize, 7usize, 4294967264u32, 2155816u32);
    emu.sri_no_count(17usize, 30usize, 31u32, 2155820u32);
    emu.sri_no_count(30usize, 22usize, 31u32, 2155824u32);
    emu.adr_no_count(11usize, 19usize, 14usize, 2155828u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2155832u32);
    emu.adr_no_count(14usize, 5usize, 12usize, 2155836u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2155840u32);
    emu.adr_no_count(31usize, 25usize, 11usize, 2155844u32);
    emu.sltru_no_count(19usize, 18usize, 10usize, 2155848u32);
    emu.adr_no_count(11usize, 17usize, 18usize, 2155852u32);
    emu.sltru_no_count(10usize, 14usize, 5usize, 2155856u32);
    emu.sltru_no_count(24usize, 31usize, 25usize, 2155860u32);
    emu.adr_no_count(5usize, 28usize, 12usize, 2155864u32);
    emu.adr_no_count(18usize, 30usize, 31usize, 2155868u32);
    emu.adr_no_count(9usize, 9usize, 19usize, 2155872u32);
    emu.sltru_no_count(17usize, 11usize, 17usize, 2155876u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2155880u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2155884u32);
    emu.sltiu_no_count(28usize, 12usize, 1u32, 2155888u32);
    emu.adr_no_count(21usize, 17usize, 28usize, 2155892u32);
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2155896u32);
    emu.add_memory_rw_events(78usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e584));
    } else {
        emu.pc = 2155900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e57c));
    }
}
#[inline(always)]
pub fn block_0x0020e57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(22usize, 21usize, 7usize, 2155904u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2155908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e594));
}
#[inline(always)]
pub fn block_0x0020e584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 21usize, 1u32, 2155912u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2155916u32);
    emu.srr_no_count(9usize, 12usize, 15usize, 2155920u32);
    emu.orr_no_count(22usize, 9usize, 28usize, 2155924u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2155924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e594));
}
#[inline]
pub fn block_0x0020e594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 5usize, 24usize, 2155928u32);
    emu.sw_no_count(18usize, 2usize, 12u32, 2155932u32)?;
    emu.sltru_no_count(18usize, 18usize, 30usize, 2155936u32);
    emu.lhu_no_count(9usize, 16usize, 10u32, 2155940u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2155944u32);
    emu.sai_no_count(5usize, 6usize, 1055u32, 2155948u32);
    emu.slti_no_count(10usize, 29usize, 0u32, 2155952u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2155956u32);
    emu.sri_no_count(6usize, 22usize, 4u32, 2155960u32);
    emu.adi_no_count(28usize, 0usize, 625u32, 2155964u32);
    emu.slr_no_count(30usize, 16usize, 7usize, 2155968u32);
    emu.slr_no_count(16usize, 16usize, 15usize, 2155972u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2155976u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2155980u32);
    emu.anr_no_count(15usize, 15usize, 30usize, 2155984u32);
    emu.anr_no_count(16usize, 10usize, 16usize, 2155988u32);
    emu.sltiu_no_count(10usize, 16usize, 1u32, 2155992u32);
    emu.sbr_no_count(19usize, 15usize, 10usize, 2155996u32);
    emu.adi_no_count(20usize, 16usize, 4294967295u32, 2156000u32);
    emu.sw_no_count(26usize, 2usize, 20u32, 2156004u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2156036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e604));
    } else {
        emu.pc = 2156008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e5e8));
    }
}
#[inline(always)]
pub fn block_0x0020e5e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 100u32, 2156012u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e630));
    } else {
        emu.pc = 2156016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e5f0));
    }
}
#[inline(always)]
pub fn block_0x0020e5f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 9u32, 2156020u32);
    emu.sltiu_no_count(10usize, 22usize, 10u32, 2156024u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2156148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e674));
    } else {
        emu.pc = 2156028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e5fc));
    }
}
#[inline(always)]
pub fn block_0x0020e5fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1u32, 2156032u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156036u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e678));
}
#[inline(always)]
pub fn block_0x0020e604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156040u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 576u32, 2156044u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e648));
    } else {
        emu.pc = 2156048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e610));
    }
}
#[inline(always)]
pub fn block_0x0020e610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156052u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1696u32, 2156056u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2156060u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e628));
    } else {
        emu.pc = 2156064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e620));
    }
}
#[inline(always)]
pub fn block_0x0020e620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2156068u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 1808u32, 2156072u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2156072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e628));
}
#[inline(always)]
pub fn block_0x0020e628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 5u32, 2156076u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156080u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e698));
}
#[inline(always)]
pub fn block_0x0020e630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 1000u32, 2156084u32);
    emu.sltiu_no_count(10usize, 22usize, 1000u32, 2156088u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2156096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e640));
    } else {
        emu.pc = 2156092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e63c));
    }
}
#[inline(always)]
pub fn block_0x0020e63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1000u32, 2156096u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156096u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e640));
}
#[inline(always)]
pub fn block_0x0020e640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 3u32, 2156100u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156104u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e698));
}
#[inline(always)]
pub fn block_0x0020e648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156108u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 256u32, 2156112u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e680));
    } else {
        emu.pc = 2156116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e654));
    }
}
#[inline(always)]
pub fn block_0x0020e654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156120u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1664u32, 2156124u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2156128u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e66c));
    } else {
        emu.pc = 2156132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e664));
    }
}
#[inline(always)]
pub fn block_0x0020e664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2156136u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 576u32, 2156140u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2156140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e66c));
}
#[inline(always)]
pub fn block_0x0020e66c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 7u32, 2156144u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e698));
}
#[inline(always)]
pub fn block_0x0020e674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 10u32, 2156152u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e678));
}
#[inline(always)]
pub fn block_0x0020e678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 1u32, 2156156u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e698));
}
#[inline(always)]
pub fn block_0x0020e680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156164u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 10usize, 4294965760u32, 2156168u32);
    emu.sltru_no_count(10usize, 22usize, 6usize, 2156172u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2156180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e694));
    } else {
        emu.pc = 2156176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e690));
    }
}
#[inline(always)]
pub fn block_0x0020e690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 6usize, 0u32, 2156180u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e694));
}
#[inline(always)]
pub fn block_0x0020e694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 9u32, 2156184u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e698));
}
#[inline(never)]
pub fn block_0x0020e698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 24usize, 18usize, 2156188u32);
    emu.sw_no_count(18usize, 2usize, 4u32, 2156192u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2156196u32)?;
    emu.anr_no_count(18usize, 19usize, 21usize, 2156200u32);
    emu.anr_no_count(21usize, 20usize, 12usize, 2156204u32);
    emu.sbr_no_count(6usize, 1usize, 9usize, 2156208u32);
    emu.sltru_no_count(28usize, 5usize, 14usize, 2156212u32);
    emu.sbr_no_count(30usize, 5usize, 8usize, 2156216u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2156220u32);
    emu.adi_no_count(24usize, 0usize, 4294967295u32, 2156224u32);
    emu.sai_no_count(14usize, 29usize, 1055u32, 2156228u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2156232u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 0usize, 10u32, 2156236u32);
    emu.adi_no_count(6usize, 6usize, 1u32, 2156240u32);
    emu.sw_no_count(6usize, 2usize, 0u32, 2156244u32)?;
    emu.sbr_no_count(28usize, 30usize, 28usize, 2156248u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2156252u32);
    emu.adi_no_count(6usize, 8usize, 4294966477u32, 2156256u32);
    emu.sltru_no_count(5usize, 11usize, 5usize, 2156260u32);
    emu.adi_no_count(8usize, 11usize, 2u32, 2156264u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2156268u32);
    emu.sltru_no_count(9usize, 8usize, 11usize, 2156272u32);
    emu.anr_no_count(17usize, 8usize, 20usize, 2156276u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2156280u32);
    emu.adr_no_count(9usize, 5usize, 9usize, 2156284u32);
    emu.anr_no_count(5usize, 9usize, 19usize, 2156288u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2156292u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2156296u32)?;
    emu.add_memory_rw_events(28usize);
    emu.pc = 2156296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e708));
}
#[inline(always)]
pub fn block_0x0020e708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 13usize, 11usize, 2156300u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2158180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158180u32));
    } else {
        emu.pc = 2156304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e710));
    }
}
#[inline(always)]
pub fn block_0x0020e710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 26usize, 0u32, 2156308u32);
    emu.divu_no_count(28usize, 22usize, 26usize, 2156312u32);
    emu.mul_no_count(26usize, 28usize, 26usize, 2156316u32);
    emu.adi_no_count(30usize, 28usize, 48u32, 2156320u32);
    emu.sbr_no_count(22usize, 22usize, 26usize, 2156324u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2156328u32);
    emu.slr_no_count(26usize, 22usize, 7usize, 2156332u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e754));
    } else {
        emu.pc = 2156336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e730));
    }
}
#[inline(always)]
pub fn block_0x0020e730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 14usize, 26usize, 2156340u32);
    emu.adr_no_count(27usize, 26usize, 18usize, 2156344u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2156348u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2156352u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2156356u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2156404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e774));
    } else {
        emu.pc = 2156360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e748));
    }
}
#[inline(always)]
pub fn block_0x0020e748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 26usize, 8usize, 2156364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2156412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e77c));
    } else {
        emu.pc = 2156368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e750));
    }
}
#[inline(always)]
pub fn block_0x0020e750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e7a8));
}
#[inline(always)]
pub fn block_0x0020e754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(28usize, 22usize, 1u32, 2156376u32);
    emu.srr_no_count(27usize, 28usize, 31usize, 2156380u32);
    emu.anr_no_count(28usize, 14usize, 26usize, 2156384u32);
    emu.adr_no_count(27usize, 27usize, 18usize, 2156388u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2156392u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2156396u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2156400u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2156360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e748));
    } else {
        emu.pc = 2156404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e774));
    }
}
#[inline(always)]
pub fn block_0x0020e774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 27usize, 9usize, 2156408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2156456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7a8));
    } else {
        emu.pc = 2156412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e77c));
    }
}
#[inline(always)]
pub fn block_0x0020e77c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 1usize, 11usize, 2156416u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2156472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7b8));
    } else {
        emu.pc = 2156420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e784));
    }
}
#[inline(always)]
pub fn block_0x0020e784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(28usize, 25usize, 6usize, 2156424u32);
    emu.adi_no_count(23usize, 23usize, 1u32, 2156428u32);
    emu.sri_no_count(26usize, 28usize, 3u32, 2156432u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2156436u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2156296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e708));
    } else {
        emu.pc = 2156440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e798));
    }
}
#[inline(always)]
pub fn block_0x0020e798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156444u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966312u32, 2156448u32);
    emu.apc_no_count(1usize, 2156448u32, 4096u32, 2156452u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e7a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 25usize, 7usize, 2156460u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8bc));
    } else {
        emu.pc = 2156464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7b0));
    }
}
#[inline(always)]
pub fn block_0x0020e7b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2156468u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156472u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156740u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e8c4));
}
#[inline(always)]
pub fn block_0x0020e7b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2156476u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2156480u32);
    emu.adi_no_count(6usize, 0usize, 1u32, 2156484u32);
    emu.adi_no_count(10usize, 0usize, 10u32, 2156488u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2156492u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2156496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e7ec));
}
#[inline(always)]
pub fn block_0x0020e7d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 18usize, 5usize, 2156500u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2156504u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2156508u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2156512u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2156516u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2156520u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a != b {
        emu.pc = 2156648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e868));
    } else {
        emu.pc = 2156524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7ec));
    }
}
#[inline(always)]
pub fn block_0x0020e7ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2158204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158204u32));
    } else {
        emu.pc = 2156528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7f0));
    }
}
#[inline(always)]
pub fn block_0x0020e7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 14usize, 0u32, 2156532u32);
    emu.adi_no_count(8usize, 6usize, 0u32, 2156536u32);
    emu.mulhu_no_count(14usize, 21usize, 10usize, 2156540u32);
    emu.mul_no_count(6usize, 18usize, 10usize, 2156544u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2156548u32);
    emu.mul_no_count(6usize, 21usize, 10usize, 2156552u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e814));
    } else {
        emu.pc = 2156556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e80c));
    }
}
#[inline(always)]
pub fn block_0x0020e80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(30usize, 14usize, 7usize, 2156560u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156580u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e824));
}
#[inline(always)]
pub fn block_0x0020e814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 14usize, 1u32, 2156568u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2156572u32);
    emu.srr_no_count(30usize, 6usize, 7usize, 2156576u32);
    emu.orr_no_count(30usize, 30usize, 28usize, 2156580u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2156580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e824));
}
#[inline]
pub fn block_0x0020e824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(18usize, 14usize, 19usize, 2156584u32);
    emu.anr_no_count(21usize, 6usize, 20usize, 2156588u32);
    emu.mulhu_no_count(14usize, 17usize, 10usize, 2156592u32);
    emu.mul_no_count(5usize, 5usize, 10usize, 2156596u32);
    emu.mul_no_count(17usize, 17usize, 10usize, 2156600u32);
    emu.adi_no_count(22usize, 30usize, 48u32, 2156604u32);
    emu.adr_no_count(5usize, 14usize, 5usize, 2156608u32);
    emu.adr_no_count(14usize, 23usize, 11usize, 2156612u32);
    emu.sb_no_count(22usize, 14usize, 0u32, 2156616u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2156496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7d0));
    } else {
        emu.pc = 2156620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e84c));
    }
}
#[inline(always)]
pub fn block_0x0020e84c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 21usize, 17usize, 2156624u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2156628u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2156632u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2156636u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2156640u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2156644u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2156524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7ec));
    } else {
        emu.pc = 2156648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e868));
    }
}
#[inline]
pub fn block_0x0020e868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 2usize, 12u32, 2156652u32)?;
    emu.sltru_no_count(10usize, 12usize, 7usize, 2156656u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2156660u32)?;
    emu.lw_no_count(28usize, 2usize, 4u32, 2156664u32)?;
    emu.sbr_no_count(13usize, 13usize, 28usize, 2156668u32);
    emu.sbr_no_count(12usize, 12usize, 7usize, 2156672u32);
    emu.sbr_no_count(13usize, 13usize, 10usize, 2156676u32);
    emu.mulhu_no_count(10usize, 6usize, 12usize, 2156680u32);
    emu.mul_no_count(31usize, 14usize, 12usize, 2156684u32);
    emu.mul_no_count(7usize, 6usize, 12usize, 2156688u32);
    emu.mul_no_count(12usize, 6usize, 13usize, 2156692u32);
    emu.adr_no_count(13usize, 7usize, 6usize, 2156696u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2156700u32);
    emu.sltru_no_count(12usize, 7usize, 6usize, 2156704u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2156708u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2156712u32);
    emu.sbr_no_count(29usize, 31usize, 12usize, 2156716u32);
    emu.sbr_no_count(30usize, 7usize, 6usize, 2156720u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2156900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e964));
    } else {
        emu.pc = 2156724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8b4));
    }
}
#[inline(always)]
pub fn block_0x0020e8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 29usize, 2156728u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e968));
}
#[inline(always)]
pub fn block_0x0020e8bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 1u32, 2156736u32);
    emu.srr_no_count(15usize, 13usize, 31usize, 2156740u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2156740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e8c4));
}
#[inline]
pub fn block_0x0020e8c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2156744u32)?;
    emu.lw_no_count(16usize, 2usize, 8u32, 2156748u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2156752u32)?;
    emu.sltru_no_count(13usize, 12usize, 17usize, 2156756u32);
    emu.lw_no_count(5usize, 2usize, 4u32, 2156760u32)?;
    emu.sbr_no_count(16usize, 16usize, 5usize, 2156764u32);
    emu.sbr_no_count(28usize, 12usize, 17usize, 2156768u32);
    emu.sbr_no_count(17usize, 16usize, 13usize, 2156772u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2156776u32);
    emu.sltiu_no_count(12usize, 28usize, 1u32, 2156780u32);
    emu.sbr_no_count(7usize, 17usize, 12usize, 2156784u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2156788u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2156800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e900));
    } else {
        emu.pc = 2156792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8f8));
    }
}
#[inline(always)]
pub fn block_0x0020e8f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 27usize, 7usize, 2156796u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e904));
}
#[inline(always)]
pub fn block_0x0020e900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 26usize, 28usize, 2156804u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156804u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e904));
}
#[inline(always)]
pub fn block_0x0020e904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(6usize, 14usize, 10usize, 2156808u32);
    emu.sltiu_no_count(29usize, 13usize, 1u32, 2156812u32);
    emu.sltru_no_count(10usize, 8usize, 26usize, 2156816u32);
    emu.sbr_no_count(12usize, 9usize, 27usize, 2156820u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2156824u32);
    emu.sbr_no_count(5usize, 8usize, 26usize, 2156828u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2156848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e930));
    } else {
        emu.pc = 2156832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e920));
    }
}
#[inline(always)]
pub fn block_0x0020e920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 10usize, 15usize, 2156836u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2156840u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2156860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e93c));
    } else {
        emu.pc = 2156844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e92c));
    }
}
#[inline(always)]
pub fn block_0x0020e92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e948));
}
#[inline(always)]
pub fn block_0x0020e930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 5usize, 6usize, 2156852u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2156856u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2156872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e948));
    } else {
        emu.pc = 2156860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e93c));
    }
}
#[inline(always)]
pub fn block_0x0020e93c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157000u32));
    } else {
        emu.pc = 2156864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e940));
    }
}
#[inline(always)]
pub fn block_0x0020e940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 15usize, 2156868u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157028u32));
    } else {
        emu.pc = 2156872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e948));
    }
}
#[inline(always)]
pub fn block_0x0020e948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 26usize, 0u32, 2156876u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2156880u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2157216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157216u32));
    } else {
        emu.pc = 2156884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e954));
    }
}
#[inline(always)]
pub fn block_0x0020e954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 13usize, 2156888u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2156892u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157228u32));
    } else {
        emu.pc = 2156896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e960));
    }
}
#[inline(always)]
pub fn block_0x0020e960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2157584u32));
}
#[inline(always)]
pub fn block_0x0020e964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 30usize, 2156904u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e968));
}
#[inline(always)]
pub fn block_0x0020e968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 7usize, 2156908u32);
    emu.adr_no_count(7usize, 31usize, 14usize, 2156912u32);
    emu.sltru_no_count(12usize, 17usize, 21usize, 2156916u32);
    emu.sbr_no_count(14usize, 5usize, 18usize, 2156920u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2156924u32);
    emu.sbr_no_count(6usize, 17usize, 21usize, 2156928u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2156948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2156948u32));
    } else {
        emu.pc = 2156932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e984));
    }
}
#[inline(always)]
pub fn block_0x0020e984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 14usize, 15usize, 2156936u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2156940u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2156960u32));
    } else {
        emu.pc = 2156944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e990));
    }
}
#[inline(always)]
pub fn block_0x0020e990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156972u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2156972u32));
}
