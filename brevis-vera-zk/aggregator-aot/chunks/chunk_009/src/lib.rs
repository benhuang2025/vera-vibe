pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2133808u32;
pub const PC_MAX: u32 = 2137196u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x00208f30,
        block_0x00208f80,
        block_0x00208f94,
        block_0x00208f98,
        block_0x00208fa4,
        block_0x00208ff4,
        block_0x00209008,
        block_0x0020901c,
        block_0x00209028,
        block_0x0020903c,
        block_0x00209044,
        block_0x00209054,
        block_0x00209060,
        block_0x00209098,
        block_0x002090b0,
        block_0x0020913c,
        block_0x00209150,
        block_0x0020915c,
        block_0x00209170,
        block_0x00209184,
        block_0x002091cc,
        block_0x002091e0,
        block_0x002091e8,
        block_0x0020933c,
        block_0x0020934c,
        block_0x0020935c,
        block_0x0020936c,
        block_0x0020937c,
        block_0x0020938c,
        block_0x0020939c,
        block_0x002093ac,
        block_0x002093b8,
        block_0x00209408,
        block_0x00209440,
        block_0x00209478,
        block_0x002094b0,
        block_0x002094e8,
        block_0x00209520,
        block_0x00209558,
        block_0x00209590,
        block_0x0020959c,
        block_0x002095b8,
        block_0x002095c8,
        block_0x002095d8,
        block_0x00209600,
        block_0x00209608,
        block_0x00209618,
        block_0x00209630,
        block_0x00209644,
        block_0x0020964c,
        block_0x00209650,
        block_0x00209668,
        block_0x00209698,
        block_0x0020969c,
        block_0x002096b0,
        block_0x002096d8,
        block_0x002096ec,
        block_0x002096f0,
        block_0x00209710,
        block_0x00209720,
        block_0x00209728,
        block_0x00209730,
        block_0x00209738,
        block_0x0020973c,
        block_0x00209758,
        block_0x00209764,
        block_0x0020976c,
        block_0x00209770,
        block_0x00209784,
        block_0x0020979c,
        block_0x002097ac,
        block_0x002097e0,
        block_0x002097f8,
        block_0x0020982c,
        block_0x00209834,
        block_0x00209860,
        block_0x00209864,
        block_0x00209878,
        block_0x00209880,
        block_0x002098b4,
        block_0x002098bc,
        block_0x002098c4,
        block_0x002098cc,
        block_0x002098d4,
        block_0x002098dc,
        block_0x00209904,
        block_0x00209964,
        block_0x0020996c,
        block_0x0020997c,
        block_0x00209984,
        block_0x00209988,
        block_0x002099b8,
        block_0x002099c0,
        block_0x002099d8,
        block_0x002099e0,
        block_0x002099f4,
        block_0x00209a0c,
        block_0x00209a6c,
        block_0x00209a74,
        block_0x00209a94,
        block_0x00209af4,
        block_0x00209af8,
        block_0x00209afc,
        block_0x00209b04,
        block_0x00209b0c,
        block_0x00209b14,
        block_0x00209b40,
        block_0x00209b48,
        block_0x00209b50,
        block_0x00209b54,
        block_0x00209b78,
        block_0x00209b84,
        block_0x00209c18,
        block_0x00209c6c,
    ];
    const IDX: [u16; 848usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        4u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 13u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16,
        0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16,
        0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16,
        0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 0u16,
        0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16,
        0u16, 31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 0u16, 0u16, 0u16,
        47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16,
        50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 54u16, 0u16, 0u16, 0u16, 0u16, 55u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16,
        0u16, 57u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16,
        0u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 63u16, 64u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 67u16, 68u16, 0u16, 0u16, 0u16,
        0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 79u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16,
        0u16, 81u16, 0u16, 82u16, 0u16, 83u16, 0u16, 84u16, 0u16, 85u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16,
        90u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 95u16, 0u16, 0u16,
        0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 102u16, 103u16, 0u16, 104u16, 0u16, 105u16, 0u16, 106u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 108u16, 0u16,
        109u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16,
        0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16,
    ];
    if pc < 2133808u32 || pc > 2137196u32 {
        return None;
    }
    let word_offset = ((pc - 2133808u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00208f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2133812u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2133816u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2133820u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2133824u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133828u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2133832u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2133836u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133840u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2133844u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2133848u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133852u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2133856u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2133860u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133864u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2133868u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2133872u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2133876u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2133880u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2133884u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2133912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f98));
    } else {
        emu.pc = 2133888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f80));
    }
}
#[inline(always)]
pub fn block_0x00208f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2133892u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2133896u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2133900u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2133904u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209008));
    } else {
        emu.pc = 2133908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f94));
    }
}
#[inline(always)]
pub fn block_0x00208f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2133912u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020901c));
}
#[inline(always)]
pub fn block_0x00208f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2133916u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2133920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2133924u32;
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
pub fn block_0x00208fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2133928u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2133932u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2133936u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2133940u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133944u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2133948u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2133952u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133956u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2133960u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2133964u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133968u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2133972u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2133976u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133980u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2133984u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2133988u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2133992u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2133996u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2134000u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2134024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209008));
    } else {
        emu.pc = 2134004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ff4));
    }
}
#[inline(always)]
pub fn block_0x00208ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2134008u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2134012u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2134016u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2134020u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2134044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020901c));
    } else {
        emu.pc = 2134024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209008));
    }
}
#[inline(always)]
pub fn block_0x00209008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2134028u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2134032u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2134036u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2134040u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134044u32;
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
pub fn block_0x0020901c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2134048u32)?;
    emu.apc_no_count(1usize, 2134048u32, 0u32, 2134052u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2134060u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2134064u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2134068u32)?;
    emu.apc_no_count(1usize, 2134068u32, 0u32, 2134072u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134076u32;
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
#[inline(always)]
pub fn block_0x0020903c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2134076u32, 69632u32, 2134080u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2134088u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2134092u32)?;
    emu.apc_no_count(1usize, 2134092u32, 4096u32, 2134096u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2134104u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2134108u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134112u32;
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
pub fn block_0x00209060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2134116u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2134120u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2134124u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2134128u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2134132u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2134136u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2134140u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2134144u32);
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134148u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294965680u32, 2134152u32)?;
    emu.sw_no_count(0usize, 10usize, 4294965684u32, 2134156u32)?;
    emu.ani_no_count(11usize, 11usize, 1u32, 2134160u32);
    emu.sw_no_count(0usize, 10usize, 4294965680u32, 2134164u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2135480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095b8));
    } else {
        emu.pc = 2134168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209098));
    }
}
#[inline(always)]
pub fn block_0x00209098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134172u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965688u32, 2134176u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2134180u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2134184u32);
    emu.apc_no_count(1usize, 2134184u32, 0u32, 2134188u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002090b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 48u32, 2134196u32);
    emu.lbu_no_count(18usize, 2usize, 112u32, 2134200u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2134204u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2134208u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134212u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2134216u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2134220u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2134224u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2134228u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2134232u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2134236u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2134240u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2134244u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2134248u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2134252u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2134256u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2134260u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2134264u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2134268u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2134272u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2134276u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2134280u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2134284u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2134288u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2134292u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2134296u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2134300u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2134304u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2134308u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2134312u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2134316u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2134320u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2134324u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2134328u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2134364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020915c));
    } else {
        emu.pc = 2134332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020913c));
    }
}
#[inline(always)]
pub fn block_0x0020913c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2134336u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2134340u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2134344u32);
    emu.apc_no_count(1usize, 2134344u32, 0u32, 2134348u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2134356u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2134360u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2134476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091cc));
    } else {
        emu.pc = 2134364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020915c));
    }
}
#[inline(always)]
pub fn block_0x0020915c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2134368u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2134372u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2134376u32);
    emu.apc_no_count(1usize, 2134376u32, 36864u32, 2134380u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2134388u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2134392u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2134396u32);
    emu.apc_no_count(1usize, 2134396u32, 0u32, 2134400u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134404u32;
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
#[inline]
pub fn block_0x00209184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2134408u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2134412u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2134416u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2134420u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2134424u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2134428u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2134432u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2134436u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2134440u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2134444u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2134448u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2134452u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2134456u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2134460u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2134464u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2134468u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2134472u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2134476u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002091e0));
}
#[inline(always)]
pub fn block_0x002091cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 104u32, 2134480u32)?;
    emu.sw_no_count(20usize, 2usize, 108u32, 2134484u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2134488u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2134492u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2134496u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2134496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002091e0));
}
#[inline(always)]
pub fn block_0x002091e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2134496u32, 36864u32, 2134500u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002091e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2134508u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134512u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 8u32, 2134516u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2134520u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2134524u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2134528u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2134532u32)?;
    emu.lw_no_count(5usize, 2usize, 28u32, 2134536u32)?;
    emu.lw_no_count(6usize, 2usize, 32u32, 2134540u32)?;
    emu.lw_no_count(14usize, 2usize, 36u32, 2134544u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2134548u32);
    emu.sri_no_count(7usize, 11usize, 8u32, 2134552u32);
    emu.sri_no_count(28usize, 11usize, 24u32, 2134556u32);
    emu.anr_no_count(29usize, 11usize, 12usize, 2134560u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2134564u32);
    emu.sri_no_count(30usize, 13usize, 8u32, 2134568u32);
    emu.sri_no_count(31usize, 13usize, 24u32, 2134572u32);
    emu.anr_no_count(9usize, 13usize, 12usize, 2134576u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2134580u32);
    emu.sri_no_count(18usize, 15usize, 8u32, 2134584u32);
    emu.sri_no_count(19usize, 15usize, 24u32, 2134588u32);
    emu.anr_no_count(20usize, 15usize, 12usize, 2134592u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2134596u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2134600u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2134604u32);
    emu.sri_no_count(28usize, 16usize, 8u32, 2134608u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2134612u32);
    emu.orr_no_count(11usize, 11usize, 29usize, 2134616u32);
    emu.sri_no_count(29usize, 16usize, 24u32, 2134620u32);
    emu.anr_no_count(30usize, 30usize, 12usize, 2134624u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2134628u32);
    emu.anr_no_count(31usize, 16usize, 12usize, 2134632u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2134636u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2134640u32);
    emu.orr_no_count(13usize, 13usize, 9usize, 2134644u32);
    emu.sri_no_count(9usize, 17usize, 8u32, 2134648u32);
    emu.anr_no_count(18usize, 18usize, 12usize, 2134652u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2134656u32);
    emu.sri_no_count(19usize, 17usize, 24u32, 2134660u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2134664u32);
    emu.orr_no_count(15usize, 15usize, 20usize, 2134668u32);
    emu.anr_no_count(20usize, 17usize, 12usize, 2134672u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2134676u32);
    emu.anr_no_count(28usize, 28usize, 12usize, 2134680u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2134684u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2134688u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2134692u32);
    emu.orr_no_count(16usize, 16usize, 31usize, 2134696u32);
    emu.sri_no_count(31usize, 5usize, 24u32, 2134700u32);
    emu.anr_no_count(9usize, 9usize, 12usize, 2134704u32);
    emu.orr_no_count(9usize, 9usize, 19usize, 2134708u32);
    emu.anr_no_count(19usize, 5usize, 12usize, 2134712u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2134716u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2134720u32);
    emu.orr_no_count(17usize, 17usize, 20usize, 2134724u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2134728u32);
    emu.anr_no_count(29usize, 29usize, 12usize, 2134732u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2134736u32);
    emu.sri_no_count(31usize, 6usize, 24u32, 2134740u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2134744u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2134748u32);
    emu.anr_no_count(19usize, 6usize, 12usize, 2134752u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2134756u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2134760u32);
    emu.orr_no_count(31usize, 20usize, 31usize, 2134764u32);
    emu.sri_no_count(20usize, 14usize, 8u32, 2134768u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2134772u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2134776u32);
    emu.sri_no_count(19usize, 14usize, 24u32, 2134780u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2134784u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2134788u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2134792u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2134796u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2134800u32);
    emu.orr_no_count(20usize, 14usize, 12usize, 2134804u32);
    emu.orr_no_count(11usize, 11usize, 7usize, 2134808u32);
    emu.orr_no_count(12usize, 13usize, 30usize, 2134812u32);
    emu.orr_no_count(13usize, 15usize, 18usize, 2134816u32);
    emu.orr_no_count(14usize, 16usize, 28usize, 2134820u32);
    emu.orr_no_count(15usize, 17usize, 9usize, 2134824u32);
    emu.orr_no_count(16usize, 5usize, 29usize, 2134828u32);
    emu.orr_no_count(17usize, 6usize, 31usize, 2134832u32);
    emu.adi_no_count(5usize, 0usize, 16u32, 2134836u32);
    emu.orr_no_count(6usize, 20usize, 19usize, 2134840u32);
    emu.add_memory_rw_events(85usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020933c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2134848u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2134852u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2134856u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020934c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2134864u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2134868u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2134872u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020935c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2134880u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2134884u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2134888u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020936c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2134896u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2134900u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2134904u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020937c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2134912u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2134916u32);
    emu.adi_no_count(11usize, 16usize, 0u32, 2134920u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020938c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2134928u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2134932u32);
    emu.adi_no_count(11usize, 17usize, 0u32, 2134936u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020939c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2134944u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2134948u32);
    emu.adi_no_count(11usize, 6usize, 0u32, 2134952u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002093ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134960u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294965640u32, 2134964u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c8));
    } else {
        emu.pc = 2134968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002093b8));
    }
}
#[inline]
pub fn block_0x002093b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2134972u32);
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134976u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965640u32, 2134980u32);
    let a = 0u32.wrapping_add(2164260864u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2134984u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 14usize, 4u32, 2134988u32)?;
    let a = 0u32.wrapping_add(2130706432u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134992u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1u32, 2134996u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2135000u32);
    emu.mul_no_count(15usize, 11usize, 13usize, 2135004u32);
    emu.mulhu_no_count(16usize, 15usize, 12usize, 2135008u32);
    emu.mul_no_count(15usize, 15usize, 12usize, 2135012u32);
    emu.sltru_no_count(11usize, 11usize, 15usize, 2135016u32);
    emu.sltru_no_count(15usize, 0usize, 16usize, 2135020u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2135024u32);
    emu.orr_no_count(11usize, 11usize, 15usize, 2135028u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2135032u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2135036u32);
    emu.sbr_no_count(11usize, 11usize, 16usize, 2135040u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2135044u32);
    emu.add_memory_rw_events(20usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00209408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 8u32, 2135052u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2135056u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2135060u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2135064u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2135068u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2135072u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2135076u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2135080u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2135084u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2135088u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2135092u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2135096u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2135100u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00209440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 12u32, 2135108u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2135112u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2135116u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2135120u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2135124u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2135128u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2135132u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2135136u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2135140u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2135144u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2135148u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2135152u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2135156u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00209478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 16u32, 2135164u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2135168u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2135172u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2135176u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2135180u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2135184u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2135188u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2135192u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2135196u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2135200u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2135204u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2135208u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2135212u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x002094b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 20u32, 2135220u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2135224u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2135228u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2135232u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2135236u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2135240u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2135244u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2135248u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2135252u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2135256u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2135260u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2135264u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2135268u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x002094e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 24u32, 2135276u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2135280u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2135284u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2135288u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2135292u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2135296u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2135300u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2135304u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2135308u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2135312u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2135316u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2135320u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2135324u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00209520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 28u32, 2135332u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2135336u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2135340u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2135344u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2135348u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2135352u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2135356u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2135360u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2135364u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2135368u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2135372u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2135376u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2135380u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00209558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 32u32, 2135388u32)?;
    emu.adi_no_count(5usize, 0usize, 26u32, 2135392u32);
    emu.mul_no_count(11usize, 10usize, 13usize, 2135396u32);
    emu.mulhu_no_count(13usize, 11usize, 12usize, 2135400u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2135404u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2135408u32);
    emu.sltru_no_count(11usize, 0usize, 13usize, 2135412u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2135416u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2135420u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2135424u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2135428u32);
    emu.sbr_no_count(11usize, 10usize, 13usize, 2135432u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2135436u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2135444u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2135448u32);
    emu.add_memory_rw_events(3usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020959c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135456u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2135460u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135464u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966180u32, 2135468u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2135472u32);
    emu.apc_no_count(1usize, 2135472u32, 73728u32, 2135476u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135484u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966148u32, 2135488u32);
    emu.apc_no_count(1usize, 2135488u32, 77824u32, 2135492u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135500u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966164u32, 2135504u32);
    emu.apc_no_count(1usize, 2135504u32, 77824u32, 2135508u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135512u32;
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
pub fn block_0x002095d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2135516u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2135520u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2135524u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2135528u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2135532u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2135536u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2135540u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2135544u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2135548u32);
    emu.add_memory_rw_events(10usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2135556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096f0));
    } else {
        emu.pc = 2135560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209608));
    }
}
#[inline(always)]
pub fn block_0x00209608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135564u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294965680u32, 2135568u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2135572u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209710));
    } else {
        emu.pc = 2135576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209618));
    }
}
#[inline(always)]
pub fn block_0x00209618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2135580u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294965680u32, 2135584u32);
    emu.lbu_no_count(19usize, 18usize, 112u32, 2135588u32);
    emu.adi_no_count(10usize, 0usize, 64u32, 2135592u32);
    emu.sbr_no_count(12usize, 10usize, 19usize, 2135596u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2135628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020964c));
    } else {
        emu.pc = 2135600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209630));
    }
}
#[inline(always)]
pub fn block_0x00209630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 19usize, 2135604u32);
    emu.adi_no_count(10usize, 10usize, 48u32, 2135608u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2135612u32);
    emu.apc_no_count(1usize, 2135612u32, 0u32, 2135616u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 8usize, 19usize, 2135624u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2135628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2135788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002096ec));
}
#[inline(always)]
pub fn block_0x0020964c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2135708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020969c));
    } else {
        emu.pc = 2135632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209650));
    }
}
#[inline(always)]
pub fn block_0x00209650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 12usize, 2135636u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2135640u32);
    emu.adi_no_count(9usize, 18usize, 48u32, 2135644u32);
    emu.adr_no_count(10usize, 9usize, 19usize, 2135648u32);
    emu.apc_no_count(1usize, 2135648u32, 0u32, 2135652u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2135660u32)?;
    emu.lw_no_count(11usize, 18usize, 44u32, 2135664u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2135668u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2135672u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2135676u32);
    emu.sw_no_count(10usize, 18usize, 40u32, 2135680u32)?;
    emu.sw_no_count(11usize, 18usize, 44u32, 2135684u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2135688u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2135692u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2135696u32);
    emu.apc_no_count(1usize, 2135696u32, 36864u32, 2135700u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2135708u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2135708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020969c));
}
#[inline(always)]
pub fn block_0x0020969c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 4294967232u32, 2135712u32);
    emu.ani_no_count(9usize, 8usize, 63u32, 2135716u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2135720u32);
    emu.adr_no_count(8usize, 11usize, 10usize, 2135724u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2135768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096d8));
    } else {
        emu.pc = 2135728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096b0));
    }
}
#[inline]
pub fn block_0x002096b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2135732u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2135736u32)?;
    emu.adr_no_count(14usize, 10usize, 12usize, 2135740u32);
    emu.sltru_no_count(10usize, 14usize, 10usize, 2135744u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2135748u32);
    emu.sw_no_count(14usize, 18usize, 40u32, 2135752u32)?;
    emu.sw_no_count(10usize, 18usize, 44u32, 2135756u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2135760u32);
    emu.apc_no_count(1usize, 2135760u32, 36864u32, 2135764u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135768u32;
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
pub fn block_0x002096d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 48u32, 2135772u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2135776u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2135780u32);
    emu.apc_no_count(1usize, 2135780u32, 0u32, 2135784u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135788u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002096ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 18usize, 112u32, 2135792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2135792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002096f0));
}
#[inline(always)]
pub fn block_0x002096f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2135796u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2135800u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2135804u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2135808u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2135812u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2135816u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2135820u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135824u32;
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
pub fn block_0x00209710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135828u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966196u32, 2135832u32);
    emu.apc_no_count(1usize, 2135832u32, 77824u32, 2135836u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 240u32, 2135844u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 5usize, 0u32, 2135852u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135856u32;
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
pub fn block_0x00209730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 241u32, 2135860u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135868u32;
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
pub fn block_0x0020973c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2135872u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2135876u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2135880u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2135884u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2135888u32);
    emu.apc_no_count(1usize, 2135888u32, 0u32, 2135892u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(20u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2135900u32);
    emu.apc_no_count(1usize, 2135900u32, 0u32, 2135904u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2135908u32, 0u32, 2135912u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135916u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020976c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2135920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209770));
    }
}
#[inline(always)]
pub fn block_0x00209770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 0u32, 2135924u32);
    emu.adr_no_count(13usize, 12usize, 10usize, 2135928u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2135932u32);
    emu.sb_no_count(11usize, 13usize, 4294967295u32, 2135936u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2135940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209784));
    }
}
#[inline(always)]
pub fn block_0x00209784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 1u32, 2135944u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2135948u32);
    emu.sb_no_count(11usize, 13usize, 4294967294u32, 2135952u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2135956u32);
    emu.sb_no_count(11usize, 13usize, 4294967293u32, 2135960u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2135964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020979c));
    }
}
#[inline(always)]
pub fn block_0x0020979c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 3u32, 2135968u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2135972u32);
    emu.sb_no_count(11usize, 13usize, 4294967292u32, 2135976u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2135980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097ac));
    }
}
#[inline]
pub fn block_0x002097ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2135984u32);
    emu.ani_no_count(14usize, 13usize, 3u32, 2135988u32);
    emu.adr_no_count(13usize, 10usize, 14usize, 2135992u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2135996u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2136000u32);
    emu.ani_no_count(11usize, 11usize, 255u32, 2136004u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136008u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 257u32, 2136012u32);
    emu.mul_no_count(11usize, 11usize, 14usize, 2136016u32);
    emu.sw_no_count(11usize, 13usize, 0u32, 2136020u32)?;
    emu.adr_no_count(14usize, 13usize, 12usize, 2136024u32);
    emu.sw_no_count(11usize, 14usize, 4294967292u32, 2136028u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2136032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097e0));
    }
}
#[inline(always)]
pub fn block_0x002097e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 4u32, 2136036u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2136040u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967284u32, 2136044u32)?;
    emu.adi_no_count(15usize, 0usize, 25u32, 2136048u32);
    emu.sw_no_count(11usize, 14usize, 4294967288u32, 2136052u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2136056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097f8));
    }
}
#[inline]
pub fn block_0x002097f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 12u32, 2136060u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2136064u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2136068u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2136072u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967268u32, 2136076u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967272u32, 2136080u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967276u32, 2136084u32)?;
    emu.ani_no_count(15usize, 13usize, 4u32, 2136088u32);
    emu.ori_no_count(15usize, 15usize, 24u32, 2136092u32);
    emu.sbr_no_count(12usize, 12usize, 15usize, 2136096u32);
    emu.adi_no_count(16usize, 0usize, 32u32, 2136100u32);
    emu.sw_no_count(11usize, 14usize, 4294967280u32, 2136104u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2136108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020982c));
    }
}
#[inline(always)]
pub fn block_0x0020982c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 15usize, 2136112u32);
    emu.adi_no_count(14usize, 0usize, 31u32, 2136116u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2136116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209834));
}
#[inline]
pub fn block_0x00209834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 0u32, 2136120u32)?;
    emu.sw_no_count(11usize, 13usize, 4u32, 2136124u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2136128u32)?;
    emu.sw_no_count(11usize, 13usize, 12u32, 2136132u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2136136u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2136140u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2136144u32)?;
    emu.sw_no_count(11usize, 13usize, 28u32, 2136148u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967264u32, 2136152u32);
    emu.adi_no_count(13usize, 13usize, 32u32, 2136156u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2136116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209834));
    } else {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    }
}
#[inline(always)]
pub fn block_0x00209860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136164u32;
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
pub fn block_0x00209864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 11usize, 3u32, 2136168u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2136172u32);
    emu.sltiu_no_count(14usize, 12usize, 1u32, 2136176u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2136180u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2136428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020996c));
    } else {
        emu.pc = 2136184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209878));
    }
}
#[inline(always)]
pub fn block_0x00209878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 1u32, 2136188u32);
    emu.adi_no_count(16usize, 10usize, 0u32, 2136192u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2136192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209880));
}
#[inline]
pub fn block_0x00209880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 11usize, 0u32, 2136196u32);
    emu.adi_no_count(14usize, 11usize, 1u32, 2136200u32);
    emu.adi_no_count(13usize, 16usize, 1u32, 2136204u32);
    emu.sb_no_count(17usize, 16usize, 0u32, 2136208u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2136212u32);
    emu.ani_no_count(11usize, 15usize, 3u32, 2136216u32);
    emu.sltru_no_count(11usize, 0usize, 11usize, 2136220u32);
    emu.sltru_no_count(16usize, 0usize, 12usize, 2136224u32);
    emu.anr_no_count(17usize, 11usize, 16usize, 2136228u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2136232u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2136236u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2136240u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2136192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209880));
    } else {
        emu.pc = 2136244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098b4));
    }
}
#[inline(always)]
pub fn block_0x002098b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 13usize, 3u32, 2136248u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020997c));
    } else {
        emu.pc = 2136252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098bc));
    }
}
#[inline(always)]
pub fn block_0x002098bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 32u32, 2136256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209afc));
    } else {
        emu.pc = 2136260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c4));
    }
}
#[inline(always)]
pub fn block_0x002098c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2136264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099f4));
    } else {
        emu.pc = 2136268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098cc));
    }
}
#[inline(always)]
pub fn block_0x002098cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2136272u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a74));
    } else {
        emu.pc = 2136276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098d4));
    }
}
#[inline(always)]
pub fn block_0x002098d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2136280u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209afc));
    } else {
        emu.pc = 2136284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098dc));
    }
}
#[inline]
pub fn block_0x002098dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2136288u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2136292u32);
    emu.sri_no_count(11usize, 15usize, 8u32, 2136296u32);
    emu.sb_no_count(11usize, 13usize, 1u32, 2136300u32);
    emu.sri_no_count(16usize, 15usize, 16u32, 2136304u32);
    emu.adi_no_count(11usize, 13usize, 3u32, 2136308u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2136312u32);
    emu.adi_no_count(12usize, 12usize, 4294967293u32, 2136316u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2136320u32);
    emu.adi_no_count(14usize, 0usize, 16u32, 2136324u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2136324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209904));
}
#[inline]
pub fn block_0x00209904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2136328u32)?;
    emu.sri_no_count(15usize, 15usize, 24u32, 2136332u32);
    emu.sli_no_count(17usize, 16usize, 8u32, 2136336u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2136340u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2136344u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2136348u32)?;
    emu.sri_no_count(15usize, 16usize, 24u32, 2136352u32);
    emu.sli_no_count(16usize, 5usize, 8u32, 2136356u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2136360u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2136364u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2136368u32)?;
    emu.sri_no_count(16usize, 5usize, 24u32, 2136372u32);
    emu.sli_no_count(5usize, 17usize, 8u32, 2136376u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2136380u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2136384u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2136388u32)?;
    emu.sri_no_count(16usize, 17usize, 24u32, 2136392u32);
    emu.sli_no_count(17usize, 15usize, 8u32, 2136396u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2136400u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2136404u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2136408u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2136412u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2136416u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2136324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209904));
    } else {
        emu.pc = 2136420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209964));
    }
}
#[inline(always)]
pub fn block_0x00209964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967283u32, 2136424u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209af8));
}
#[inline(always)]
pub fn block_0x0020996c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2136432u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2136436u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2136440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098bc));
    } else {
        emu.pc = 2136444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020997c));
    }
}
#[inline(always)]
pub fn block_0x0020997c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2136448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099b8));
    } else {
        emu.pc = 2136452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209984));
    }
}
#[inline(always)]
pub fn block_0x00209984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2136456u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209988));
}
#[inline]
pub fn block_0x00209988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2136460u32)?;
    emu.lw_no_count(16usize, 14usize, 4u32, 2136464u32)?;
    emu.lw_no_count(17usize, 14usize, 8u32, 2136468u32)?;
    emu.lw_no_count(5usize, 14usize, 12u32, 2136472u32)?;
    emu.sw_no_count(15usize, 13usize, 0u32, 2136476u32)?;
    emu.sw_no_count(16usize, 13usize, 4u32, 2136480u32)?;
    emu.sw_no_count(17usize, 13usize, 8u32, 2136484u32)?;
    emu.sw_no_count(5usize, 13usize, 12u32, 2136488u32)?;
    emu.adi_no_count(14usize, 14usize, 16u32, 2136492u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2136496u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2136500u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2136456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209988));
    } else {
        emu.pc = 2136504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099b8));
    }
}
#[inline(always)]
pub fn block_0x002099b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2136508u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099d8));
    } else {
        emu.pc = 2136512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099c0));
    }
}
#[inline(always)]
pub fn block_0x002099c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2136516u32)?;
    emu.lw_no_count(15usize, 14usize, 4u32, 2136520u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2136524u32)?;
    emu.sw_no_count(15usize, 13usize, 4u32, 2136528u32)?;
    emu.adi_no_count(13usize, 13usize, 8u32, 2136532u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2136536u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2136536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002099d8));
}
#[inline(always)]
pub fn block_0x002099d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2136540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b40));
    } else {
        emu.pc = 2136544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099e0));
    }
}
#[inline(always)]
pub fn block_0x002099e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2136548u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2136552u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2136556u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2136560u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2136564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b40));
}
#[inline(always)]
pub fn block_0x002099f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2136568u32)?;
    emu.adi_no_count(11usize, 13usize, 1u32, 2136572u32);
    emu.sb_no_count(15usize, 13usize, 0u32, 2136576u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2136580u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2136584u32);
    emu.adi_no_count(14usize, 0usize, 18u32, 2136588u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2136588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a0c));
}
#[inline]
pub fn block_0x00209a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2136592u32)?;
    emu.sri_no_count(15usize, 15usize, 8u32, 2136596u32);
    emu.sli_no_count(17usize, 16usize, 24u32, 2136600u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2136604u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2136608u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2136612u32)?;
    emu.sri_no_count(15usize, 16usize, 8u32, 2136616u32);
    emu.sli_no_count(16usize, 5usize, 24u32, 2136620u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2136624u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2136628u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2136632u32)?;
    emu.sri_no_count(16usize, 5usize, 8u32, 2136636u32);
    emu.sli_no_count(5usize, 17usize, 24u32, 2136640u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2136644u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2136648u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2136652u32)?;
    emu.sri_no_count(16usize, 17usize, 8u32, 2136656u32);
    emu.sli_no_count(17usize, 15usize, 24u32, 2136660u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2136664u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2136668u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2136672u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2136676u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2136680u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2136588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a0c));
    } else {
        emu.pc = 2136684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a6c));
    }
}
#[inline(always)]
pub fn block_0x00209a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967281u32, 2136688u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136692u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209af8));
}
#[inline(always)]
pub fn block_0x00209a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2136696u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2136700u32);
    emu.sri_no_count(16usize, 15usize, 8u32, 2136704u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2136708u32);
    emu.sb_no_count(16usize, 13usize, 1u32, 2136712u32);
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2136716u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2136720u32);
    emu.adi_no_count(14usize, 0usize, 17u32, 2136724u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2136724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a94));
}
#[inline]
pub fn block_0x00209a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2136728u32)?;
    emu.sri_no_count(15usize, 15usize, 16u32, 2136732u32);
    emu.sli_no_count(17usize, 16usize, 16u32, 2136736u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2136740u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2136744u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2136748u32)?;
    emu.sri_no_count(15usize, 16usize, 16u32, 2136752u32);
    emu.sli_no_count(16usize, 5usize, 16u32, 2136756u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2136760u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2136764u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2136768u32)?;
    emu.sri_no_count(16usize, 5usize, 16u32, 2136772u32);
    emu.sli_no_count(5usize, 17usize, 16u32, 2136776u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2136780u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2136784u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2136788u32)?;
    emu.sri_no_count(16usize, 17usize, 16u32, 2136792u32);
    emu.sli_no_count(17usize, 15usize, 16u32, 2136796u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2136800u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2136804u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2136808u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2136812u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2136816u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2136724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a94));
    } else {
        emu.pc = 2136820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209af4));
    }
}
#[inline(always)]
pub fn block_0x00209af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967282u32, 2136824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209af8));
}
#[inline(always)]
pub fn block_0x00209af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2136828u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209afc));
}
#[inline(always)]
pub fn block_0x00209afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 16u32, 2136832u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b84));
    } else {
        emu.pc = 2136836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b04));
    }
}
#[inline(always)]
pub fn block_0x00209b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2136840u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c18));
    } else {
        emu.pc = 2136844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b0c));
    }
}
#[inline(always)]
pub fn block_0x00209b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2136848u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b40));
    } else {
        emu.pc = 2136852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b14));
    }
}
#[inline]
pub fn block_0x00209b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2136856u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2136860u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2136864u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2136868u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2136872u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2136876u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2136880u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2136884u32);
    emu.adi_no_count(15usize, 13usize, 4u32, 2136888u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2136892u32);
    emu.adi_no_count(13usize, 15usize, 0u32, 2136896u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2136896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b40));
}
#[inline(always)]
pub fn block_0x00209b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 2u32, 2136900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b54));
    } else {
        emu.pc = 2136904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b48));
    }
}
#[inline(always)]
pub fn block_0x00209b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 1u32, 2136908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b78));
    } else {
        emu.pc = 2136912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b50));
    }
}
#[inline(always)]
pub fn block_0x00209b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136916u32;
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
pub fn block_0x00209b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2136920u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2136924u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2136928u32);
    emu.adi_no_count(14usize, 14usize, 2u32, 2136932u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2136936u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2136940u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2136944u32);
    emu.ani_no_count(11usize, 12usize, 1u32, 2136948u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b50));
    } else {
        emu.pc = 2136952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b78));
    }
}
#[inline(always)]
pub fn block_0x00209b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2136956u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2136960u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136964u32;
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
pub fn block_0x00209b84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2136968u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2136972u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2136976u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2136980u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2136984u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2136988u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2136992u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2136996u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2137000u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2137004u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2137008u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2137012u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2137016u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2137020u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2137024u32);
    emu.lb_no_count(11usize, 14usize, 8u32, 2137028u32);
    emu.lb_no_count(16usize, 14usize, 9u32, 2137032u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2137036u32);
    emu.lb_no_count(15usize, 14usize, 10u32, 2137040u32);
    emu.sb_no_count(11usize, 13usize, 8u32, 2137044u32);
    emu.sb_no_count(16usize, 13usize, 9u32, 2137048u32);
    emu.lb_no_count(11usize, 14usize, 11u32, 2137052u32);
    emu.sb_no_count(15usize, 13usize, 10u32, 2137056u32);
    emu.lb_no_count(15usize, 14usize, 12u32, 2137060u32);
    emu.lb_no_count(16usize, 14usize, 13u32, 2137064u32);
    emu.sb_no_count(11usize, 13usize, 11u32, 2137068u32);
    emu.lb_no_count(11usize, 14usize, 14u32, 2137072u32);
    emu.sb_no_count(15usize, 13usize, 12u32, 2137076u32);
    emu.sb_no_count(16usize, 13usize, 13u32, 2137080u32);
    emu.lb_no_count(15usize, 14usize, 15u32, 2137084u32);
    emu.sb_no_count(11usize, 13usize, 14u32, 2137088u32);
    emu.adi_no_count(14usize, 14usize, 16u32, 2137092u32);
    emu.adi_no_count(11usize, 13usize, 16u32, 2137096u32);
    emu.sb_no_count(15usize, 13usize, 15u32, 2137100u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2137104u32);
    emu.ani_no_count(11usize, 12usize, 8u32, 2137108u32);
    emu.add_memory_rw_events(36usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b0c));
    } else {
        emu.pc = 2137112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c18));
    }
}
#[inline]
pub fn block_0x00209c18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2137116u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2137120u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2137124u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2137128u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2137132u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2137136u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2137140u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2137144u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2137148u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2137152u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2137156u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2137160u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2137164u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2137168u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2137172u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2137176u32);
    emu.adi_no_count(11usize, 13usize, 8u32, 2137180u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2137184u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2137188u32);
    emu.ani_no_count(11usize, 12usize, 4u32, 2137192u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b14));
    } else {
        emu.pc = 2137196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c6c));
    }
}
#[inline(always)]
pub fn block_0x00209c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2137200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b40));
}
