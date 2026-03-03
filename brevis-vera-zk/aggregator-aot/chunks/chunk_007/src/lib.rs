pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2124824u32;
pub const PC_MAX: u32 = 2127572u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x00206c18,
        block_0x00206c30,
        block_0x00206c4c,
        block_0x00206c6c,
        block_0x00206c94,
        block_0x00206ca0,
        block_0x00206cc8,
        block_0x00206cd4,
        block_0x00206cfc,
        block_0x00206d08,
        block_0x00206d24,
        block_0x00206d40,
        block_0x00206d68,
        block_0x00206d74,
        block_0x00206d90,
        block_0x00206dac,
        block_0x00206dd0,
        block_0x00206ddc,
        block_0x00206e28,
        block_0x00206e2c,
        block_0x00206e30,
        block_0x00206e3c,
        block_0x00206e54,
        block_0x00206e58,
        block_0x00206e78,
        block_0x00206ec4,
        block_0x00206ed4,
        block_0x00206ee8,
        block_0x00206ef0,
        block_0x00206f78,
        block_0x00206f84,
        block_0x00206f8c,
        block_0x00206f9c,
        block_0x00206fb8,
        block_0x00206fc8,
        block_0x00207000,
        block_0x00207018,
        block_0x0020705c,
        block_0x00207060,
        block_0x00207064,
        block_0x0020706c,
        block_0x00207084,
        block_0x00207088,
        block_0x002070b4,
        block_0x002070d0,
        block_0x002070d8,
        block_0x002070f4,
        block_0x00207104,
        block_0x00207108,
        block_0x00207118,
        block_0x00207134,
        block_0x00207154,
        block_0x00207160,
        block_0x00207190,
        block_0x002071a8,
        block_0x002071b8,
        block_0x002071cc,
        block_0x002071e0,
        block_0x00207200,
        block_0x00207220,
        block_0x0020723c,
        block_0x0020724c,
        block_0x00207258,
        block_0x00207268,
        block_0x00207270,
        block_0x00207274,
        block_0x00207294,
        block_0x002072b4,
        block_0x002072d0,
        block_0x002072ec,
        block_0x00207308,
        block_0x00207324,
        block_0x00207340,
        block_0x0020735c,
        block_0x00207378,
        block_0x00207394,
        block_0x002073b0,
        block_0x002073cc,
        block_0x002073e8,
        block_0x00207404,
        block_0x00207420,
        block_0x0020743c,
        block_0x00207458,
        block_0x00207474,
        block_0x00207490,
        block_0x002074ac,
        block_0x002074c8,
        block_0x002074e4,
        block_0x00207500,
        block_0x0020751c,
        block_0x00207538,
        block_0x00207554,
        block_0x00207570,
        block_0x0020758c,
        block_0x002075a8,
        block_0x002075c4,
        block_0x002075e0,
        block_0x002075fc,
        block_0x00207618,
        block_0x00207624,
        block_0x00207638,
        block_0x00207664,
        block_0x0020766c,
        block_0x00207688,
        block_0x0020768c,
        block_0x002076a0,
        block_0x002076a8,
        block_0x002076ac,
        block_0x002076b4,
        block_0x002076cc,
        block_0x002076d0,
        block_0x002076d4,
    ];
    const IDX: [u16; 688usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 19u16, 20u16, 21u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        23u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16,
        29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 31u16, 0u16,
        32u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        38u16, 39u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 43u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        47u16, 0u16, 0u16, 0u16, 48u16, 49u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16,
        0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16,
        0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 64u16, 0u16, 65u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16,
        0u16, 106u16, 0u16, 107u16, 108u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        110u16, 111u16, 112u16,
    ];
    if pc < 2124824u32 || pc > 2127572u32 {
        return None;
    }
    let word_offset = ((pc - 2124824u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00206c18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2124828u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2124832u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2124836u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2124840u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2124844u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c4c));
    } else {
        emu.pc = 2124848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c30));
    }
}
#[inline(always)]
pub fn block_0x00206c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2124852u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2124856u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2124860u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967276u32, 2124864u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2124868u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2124872u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2124876u32;
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
pub fn block_0x00206c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124880u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 13usize, 11usize, 2124884u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2124888u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2124892u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967276u32, 2124896u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2124900u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2124904u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2124908u32;
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
pub fn block_0x00206c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2124912u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2124916u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124920u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2124924u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2124928u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 332u32, 2124932u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2124936u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2124940u32);
    emu.apc_no_count(1usize, 2124940u32, 94208u32, 2124944u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2124952u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124956u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124960u32;
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
pub fn block_0x00206ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2124964u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2124968u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 444u32, 2124976u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2124980u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2124984u32);
    emu.adi_no_count(12usize, 0usize, 18u32, 2124988u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2124992u32);
    emu.apc_no_count(1usize, 2124992u32, 94208u32, 2124996u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966040u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125004u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125008u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125012u32;
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
pub fn block_0x00206cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2125016u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2125020u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125024u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 387u32, 2125028u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2125032u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 60u32, 2125036u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2125040u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2125044u32);
    emu.apc_no_count(1usize, 2125044u32, 94208u32, 2125048u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125056u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125060u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125064u32;
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
pub fn block_0x00206d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125068u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 406u32, 2125072u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2125076u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2125080u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125084u32);
    emu.apc_no_count(6usize, 2125084u32, 90112u32, 2125088u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125092u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125096u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 497u32, 2125100u32);
    emu.adi_no_count(12usize, 0usize, 22u32, 2125104u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2125108u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125112u32);
    emu.apc_no_count(6usize, 2125112u32, 90112u32, 2125116u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125120u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2125124u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2125128u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125132u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 368u32, 2125136u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2125140u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 352u32, 2125144u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2125148u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2125152u32);
    emu.apc_no_count(1usize, 2125152u32, 94208u32, 2125156u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125164u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125172u32;
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
pub fn block_0x00206d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 462u32, 2125180u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2125184u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2125188u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125192u32);
    emu.apc_no_count(6usize, 2125192u32, 90112u32, 2125196u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125200u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00206d90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125204u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 488u32, 2125208u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2125212u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2125216u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125220u32);
    emu.apc_no_count(6usize, 2125220u32, 90112u32, 2125224u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125228u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 8u32, 2125232u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125236u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 536u32, 2125240u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2125244u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 520u32, 2125248u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2125252u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2125256u32);
    emu.apc_no_count(1usize, 2125256u32, 94208u32, 2125260u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125264u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125268u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125272u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125276u32;
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
pub fn block_0x00206ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967136u32, 2125280u32);
    emu.sw_no_count(1usize, 2usize, 156u32, 2125284u32)?;
    emu.sw_no_count(8usize, 2usize, 152u32, 2125288u32)?;
    emu.sw_no_count(9usize, 2usize, 148u32, 2125292u32)?;
    emu.sw_no_count(18usize, 2usize, 144u32, 2125296u32)?;
    emu.sw_no_count(19usize, 2usize, 140u32, 2125300u32)?;
    emu.sw_no_count(20usize, 2usize, 136u32, 2125304u32)?;
    emu.sw_no_count(21usize, 2usize, 132u32, 2125308u32)?;
    emu.sw_no_count(22usize, 2usize, 128u32, 2125312u32)?;
    emu.sw_no_count(23usize, 2usize, 124u32, 2125316u32)?;
    emu.sw_no_count(24usize, 2usize, 120u32, 2125320u32)?;
    emu.sw_no_count(25usize, 2usize, 116u32, 2125324u32)?;
    emu.sw_no_count(26usize, 2usize, 112u32, 2125328u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2125332u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2125336u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2125340u32);
    let a = 0u32.wrapping_add(32768u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2125344u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 12usize, 0u32, 2125348u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2125356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e2c));
    } else {
        emu.pc = 2125352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e28));
    }
}
#[inline(always)]
pub fn block_0x00206e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(32768u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2125356u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2125356u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206e2c));
}
#[inline(always)]
pub fn block_0x00206e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2125708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f8c));
    } else {
        emu.pc = 2125360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e30));
    }
}
#[inline(always)]
pub fn block_0x00206e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 22usize, 5u32, 2125364u32);
    emu.apc_no_count(1usize, 2125364u32, 4096u32, 2125368u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2125376u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1212u32, 2125380u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2125384u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2125388u32);
    emu.apc_no_count(1usize, 2125388u32, 8192u32, 2125392u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2125824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207000));
    } else {
        emu.pc = 2125400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e58));
    }
}
#[inline(always)]
pub fn block_0x00206e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2125404u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2125408u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2125412u32);
    emu.sw_no_count(22usize, 2usize, 8u32, 2125416u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2125420u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2125424u32)?;
    emu.adi_no_count(22usize, 2usize, 84u32, 2125428u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2125432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2125524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206ed4));
}
#[inline]
pub fn block_0x00206e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(26usize, 26usize, 8u32, 2125436u32);
    emu.adr_no_count(10usize, 19usize, 20usize, 2125440u32);
    emu.sri_no_count(11usize, 24usize, 24u32, 2125444u32);
    emu.sri_no_count(12usize, 24usize, 16u32, 2125448u32);
    emu.orr_no_count(13usize, 26usize, 25usize, 2125452u32);
    emu.sri_no_count(14usize, 24usize, 8u32, 2125456u32);
    emu.sb_no_count(14usize, 10usize, 4u32, 2125460u32);
    emu.sb_no_count(12usize, 10usize, 5u32, 2125464u32);
    emu.sb_no_count(11usize, 10usize, 6u32, 2125468u32);
    emu.sri_no_count(11usize, 13usize, 8u32, 2125472u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2125476u32);
    emu.sb_no_count(11usize, 10usize, 1u32, 2125480u32);
    emu.sb_no_count(23usize, 10usize, 2u32, 2125484u32);
    emu.sb_no_count(24usize, 10usize, 3u32, 2125488u32);
    emu.adi_no_count(10usize, 10usize, 7u32, 2125492u32);
    emu.adi_no_count(11usize, 2usize, 20u32, 2125496u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2125500u32);
    emu.apc_no_count(1usize, 2125500u32, 8192u32, 2125504u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966884u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 21usize, 1u32, 2125512u32);
    emu.sw_no_count(21usize, 2usize, 16u32, 2125516u32)?;
    emu.adi_no_count(20usize, 20usize, 32u32, 2125520u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2125724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f9c));
    } else {
        emu.pc = 2125524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ed4));
    }
}
#[inline(always)]
pub fn block_0x00206ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 76u32, 2125528u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2125532u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2125536u32);
    emu.apc_no_count(1usize, 2125536u32, 4294942720u32, 2125540u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 76u32, 2125548u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2125752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206fb8));
    } else {
        emu.pc = 2125552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ef0));
    }
}
#[inline(never)]
pub fn block_0x00206ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 34u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 22usize, 0u32, 2125556u32)?;
    emu.lw_no_count(11usize, 22usize, 4u32, 2125560u32)?;
    emu.lw_no_count(12usize, 22usize, 8u32, 2125564u32)?;
    emu.lw_no_count(13usize, 22usize, 12u32, 2125568u32)?;
    emu.lw_no_count(14usize, 22usize, 16u32, 2125572u32)?;
    emu.lw_no_count(15usize, 22usize, 20u32, 2125576u32)?;
    emu.lbu_no_count(16usize, 22usize, 24u32, 2125580u32);
    emu.lw_no_count(17usize, 2usize, 8u32, 2125584u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2125588u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2125592u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2125596u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2125600u32)?;
    emu.sw_no_count(14usize, 2usize, 64u32, 2125604u32)?;
    emu.sw_no_count(15usize, 2usize, 68u32, 2125608u32)?;
    emu.sb_no_count(16usize, 2usize, 72u32, 2125612u32);
    emu.lw_no_count(10usize, 2usize, 48u32, 2125616u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2125620u32)?;
    emu.lw_no_count(12usize, 2usize, 56u32, 2125624u32)?;
    emu.lw_no_count(13usize, 2usize, 60u32, 2125628u32)?;
    emu.lw_no_count(14usize, 2usize, 64u32, 2125632u32)?;
    emu.lw_no_count(15usize, 2usize, 68u32, 2125636u32)?;
    emu.lbu_no_count(16usize, 2usize, 72u32, 2125640u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2125644u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2125648u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2125652u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2125656u32)?;
    emu.lbu_no_count(25usize, 2usize, 77u32, 2125660u32);
    emu.lbu_no_count(26usize, 2usize, 78u32, 2125664u32);
    emu.lbu_no_count(23usize, 2usize, 79u32, 2125668u32);
    emu.lw_no_count(24usize, 2usize, 80u32, 2125672u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2125676u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2125680u32)?;
    emu.sb_no_count(16usize, 2usize, 44u32, 2125684u32);
    emu.add_memory_rw_events(33usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2125432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e78));
    } else {
        emu.pc = 2125688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f78));
    }
}
#[inline(always)]
pub fn block_0x00206f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2125692u32);
    emu.apc_no_count(1usize, 2125692u32, 0u32, 2125696u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 12u32, 2125704u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2125708u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2125432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206e78));
}
#[inline(always)]
pub fn block_0x00206f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2125712u32);
    emu.sw_no_count(22usize, 2usize, 8u32, 2125716u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2125720u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2125724u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2125724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206f9c));
}
#[inline(always)]
pub fn block_0x00206f9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2125728u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2125732u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2125736u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2125740u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2125744u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2125748u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2125752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2125768u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206fc8));
}
#[inline(always)]
pub fn block_0x00206fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 80u32, 2125756u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125760u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2125764u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2125768u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2125768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206fc8));
}
#[inline]
pub fn block_0x00206fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 156u32, 2125772u32)?;
    emu.lw_no_count(8usize, 2usize, 152u32, 2125776u32)?;
    emu.lw_no_count(9usize, 2usize, 148u32, 2125780u32)?;
    emu.lw_no_count(18usize, 2usize, 144u32, 2125784u32)?;
    emu.lw_no_count(19usize, 2usize, 140u32, 2125788u32)?;
    emu.lw_no_count(20usize, 2usize, 136u32, 2125792u32)?;
    emu.lw_no_count(21usize, 2usize, 132u32, 2125796u32)?;
    emu.lw_no_count(22usize, 2usize, 128u32, 2125800u32)?;
    emu.lw_no_count(23usize, 2usize, 124u32, 2125804u32)?;
    emu.lw_no_count(24usize, 2usize, 120u32, 2125808u32)?;
    emu.lw_no_count(25usize, 2usize, 116u32, 2125812u32)?;
    emu.lw_no_count(26usize, 2usize, 112u32, 2125816u32)?;
    emu.adi_no_count(2usize, 2usize, 160u32, 2125820u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125824u32;
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
pub fn block_0x00207000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2125828u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 880u32, 2125832u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2125836u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2125840u32);
    emu.apc_no_count(1usize, 2125840u32, 77824u32, 2125844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125848u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2125852u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2125856u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2125860u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2125864u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2125868u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2125872u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2125876u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2125880u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2125884u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2125888u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2125892u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2125896u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2125900u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2125904u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2125908u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 12usize, 0u32, 2125912u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2125920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207060));
    } else {
        emu.pc = 2125916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020705c));
    }
}
#[inline(always)]
pub fn block_0x0020705c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2125920u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2125920u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207060));
}
#[inline(always)]
pub fn block_0x00207060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2126088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207108));
    } else {
        emu.pc = 2125924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207064));
    }
}
#[inline(always)]
pub fn block_0x00207064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2125924u32, 4096u32, 2125928u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020706c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2125936u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1212u32, 2125940u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2125944u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2125948u32);
    emu.apc_no_count(1usize, 2125948u32, 8192u32, 2125952u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2126224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207190));
    } else {
        emu.pc = 2125960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207088));
    }
}
#[inline]
pub fn block_0x00207088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2125964u32);
    emu.lw_no_count(21usize, 18usize, 0u32, 2125968u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2125972u32)?;
    emu.sw_no_count(19usize, 2usize, 4u32, 2125976u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2125980u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2125984u32)?;
    emu.sbr_no_count(22usize, 0usize, 23usize, 2125988u32);
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2125992u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2125996u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 896u32, 2126000u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2126004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2126032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002070d0));
}
#[inline(always)]
pub fn block_0x002070b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2126008u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2126012u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2126016u32);
    emu.sb_no_count(24usize, 10usize, 0u32, 2126020u32);
    emu.sw_no_count(20usize, 2usize, 12u32, 2126024u32)?;
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2126028u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2126104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207118));
    } else {
        emu.pc = 2126032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002070d0));
    }
}
#[inline(always)]
pub fn block_0x002070d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 22usize, 20usize, 2126036u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2126132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207134));
    } else {
        emu.pc = 2126040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002070d8));
    }
}
#[inline(always)]
pub fn block_0x002070d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 20usize, 2126044u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2126048u32)?;
    emu.lbu_no_count(24usize, 10usize, 0u32, 2126052u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2126056u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2126060u32)?;
    emu.sw_no_count(23usize, 18usize, 4u32, 2126064u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2126004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002070b4));
    } else {
        emu.pc = 2126068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002070f4));
    }
}
#[inline(always)]
pub fn block_0x002070f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2126072u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2126076u32);
    emu.apc_no_count(1usize, 2126076u32, 77824u32, 2126080u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2126088u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2126004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002070b4));
}
#[inline(always)]
pub fn block_0x00207108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2126092u32);
    emu.sw_no_count(19usize, 2usize, 4u32, 2126096u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2126100u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2126104u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2126104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207118));
}
#[inline(always)]
pub fn block_0x00207118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2126108u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2126112u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2126116u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2126120u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2126124u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2126128u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2126132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2126176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207160));
}
#[inline(always)]
pub fn block_0x00207134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2126136u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126140u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2126144u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2126148u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2126152u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2126156u32);
    emu.apc_no_count(1usize, 2126156u32, 8192u32, 2126160u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126168u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2126172u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2126176u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2126176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207160));
}
#[inline]
pub fn block_0x00207160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2126180u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2126184u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2126188u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2126192u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2126196u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2126200u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2126204u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2126208u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2126212u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2126216u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2126220u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126224u32;
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
pub fn block_0x00207190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126228u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 880u32, 2126232u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2126236u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2126240u32);
    emu.apc_no_count(1usize, 2126240u32, 77824u32, 2126244u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2126252u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2126256u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2126260u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2126284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002071cc));
    } else {
        emu.pc = 2126264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002071b8));
    }
}
#[inline(always)]
pub fn block_0x002071b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 15u32, 2126268u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126272u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 912u32, 2126276u32);
    emu.apc_no_count(6usize, 2126276u32, 90112u32, 2126280u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126284u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 13u32, 2126288u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126292u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 927u32, 2126296u32);
    emu.apc_no_count(6usize, 2126296u32, 90112u32, 2126300u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126304u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2126308u32)?;
    emu.lw_no_count(10usize, 12usize, 4u32, 2126312u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2126316u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2126320u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2126324u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2126328u32);
    emu.apc_no_count(6usize, 2126328u32, 94208u32, 2126332u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126336u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2126340u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2126344u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2126348u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2126352u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2126356u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2126360u32);
    emu.apc_no_count(6usize, 2126360u32, 65536u32, 2126364u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126368u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(88u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2126372u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 856u32, 2126376u32);
    emu.adi_no_count(12usize, 0usize, 21u32, 2126380u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2126384u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2126388u32);
    emu.apc_no_count(6usize, 2126388u32, 90112u32, 2126392u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126396u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020723c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2126400u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2126404u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126408u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2126448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207270));
    } else {
        emu.pc = 2126412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020724c));
    }
}
#[inline(always)]
pub fn block_0x0020724c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 4u32, 2126416u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2126420u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2126448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207270));
    } else {
        emu.pc = 2126424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207258));
    }
}
#[inline(always)]
pub fn block_0x00207258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 8u32, 2126428u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2126432u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2126436u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2126448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207270));
    } else {
        emu.pc = 2126440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207268));
    }
}
#[inline(always)]
pub fn block_0x00207268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2126444u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126448u32;
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
pub fn block_0x00207270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126452u32;
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
pub fn block_0x00207274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2126456u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2126460u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2126464u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2126468u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2126472u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2126476u32);
    emu.apc_no_count(1usize, 2126476u32, 90112u32, 2126480u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 16u32, 2126488u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2126492u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 9usize, 60u32, 2126496u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2126500u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126504u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126508u32);
    emu.apc_no_count(1usize, 2126508u32, 86016u32, 2126512u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002072b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 1u32, 2126520u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126524u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126528u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126532u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126536u32);
    emu.apc_no_count(1usize, 2126536u32, 86016u32, 2126540u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002072d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 2u32, 2126548u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126552u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126556u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126560u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126564u32);
    emu.apc_no_count(1usize, 2126564u32, 86016u32, 2126568u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002072ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 3u32, 2126576u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126580u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126584u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126588u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126592u32);
    emu.apc_no_count(1usize, 2126592u32, 86016u32, 2126596u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126600u32;
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
pub fn block_0x00207308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2126604u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126608u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126612u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126616u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126620u32);
    emu.apc_no_count(1usize, 2126620u32, 86016u32, 2126624u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 5u32, 2126632u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126636u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126640u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126644u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126648u32);
    emu.apc_no_count(1usize, 2126648u32, 86016u32, 2126652u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 6u32, 2126660u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126664u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126668u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126672u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126676u32);
    emu.apc_no_count(1usize, 2126676u32, 86016u32, 2126680u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126684u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020735c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 7u32, 2126688u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126692u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126696u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126700u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126704u32);
    emu.apc_no_count(1usize, 2126704u32, 86016u32, 2126708u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 8u32, 2126716u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126720u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126724u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126728u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126732u32);
    emu.apc_no_count(1usize, 2126732u32, 86016u32, 2126736u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 9u32, 2126744u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126748u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126752u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126756u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126760u32);
    emu.apc_no_count(1usize, 2126760u32, 86016u32, 2126764u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126768u32;
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
pub fn block_0x002073b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 10u32, 2126772u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126776u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126780u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126784u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126788u32);
    emu.apc_no_count(1usize, 2126788u32, 86016u32, 2126792u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002073cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 11u32, 2126800u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126804u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126808u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126812u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126816u32);
    emu.apc_no_count(1usize, 2126816u32, 86016u32, 2126820u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002073e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 12u32, 2126828u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126832u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126836u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126840u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126844u32);
    emu.apc_no_count(1usize, 2126844u32, 86016u32, 2126848u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126852u32;
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
pub fn block_0x00207404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 13u32, 2126856u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126860u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126864u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126868u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126872u32);
    emu.apc_no_count(1usize, 2126872u32, 86016u32, 2126876u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 14u32, 2126884u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126888u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126892u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126896u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126900u32);
    emu.apc_no_count(1usize, 2126900u32, 86016u32, 2126904u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020743c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 15u32, 2126912u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126916u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126920u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126924u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126928u32);
    emu.apc_no_count(1usize, 2126928u32, 86016u32, 2126932u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 16u32, 2126940u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126944u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126948u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126952u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126956u32);
    emu.apc_no_count(1usize, 2126956u32, 86016u32, 2126960u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126964u32;
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
pub fn block_0x00207474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 17u32, 2126968u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2126972u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2126976u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2126980u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2126984u32);
    emu.apc_no_count(1usize, 2126984u32, 86016u32, 2126988u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 18u32, 2126996u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127000u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127004u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127008u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127012u32);
    emu.apc_no_count(1usize, 2127012u32, 86016u32, 2127016u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002074ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 19u32, 2127024u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127028u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127032u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127036u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127040u32);
    emu.apc_no_count(1usize, 2127040u32, 86016u32, 2127044u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002074c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 20u32, 2127052u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127056u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127060u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127064u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127068u32);
    emu.apc_no_count(1usize, 2127068u32, 86016u32, 2127072u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002074e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 21u32, 2127080u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127084u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127088u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127092u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127096u32);
    emu.apc_no_count(1usize, 2127096u32, 86016u32, 2127100u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 22u32, 2127108u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127112u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127116u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127120u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127124u32);
    emu.apc_no_count(1usize, 2127124u32, 86016u32, 2127128u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020751c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 23u32, 2127136u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127140u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127144u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127148u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127152u32);
    emu.apc_no_count(1usize, 2127152u32, 86016u32, 2127156u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 24u32, 2127164u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127168u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127172u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127176u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127180u32);
    emu.apc_no_count(1usize, 2127180u32, 86016u32, 2127184u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127188u32;
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
#[inline(always)]
pub fn block_0x00207554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 25u32, 2127192u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127196u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127200u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127204u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127208u32);
    emu.apc_no_count(1usize, 2127208u32, 86016u32, 2127212u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127216u32;
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
pub fn block_0x00207570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 26u32, 2127220u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127224u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127228u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127232u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127236u32);
    emu.apc_no_count(1usize, 2127236u32, 86016u32, 2127240u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020758c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 27u32, 2127248u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127252u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127256u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127260u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127264u32);
    emu.apc_no_count(1usize, 2127264u32, 86016u32, 2127268u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002075a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 28u32, 2127276u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127280u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127284u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127288u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127292u32);
    emu.apc_no_count(1usize, 2127292u32, 86016u32, 2127296u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002075c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 29u32, 2127304u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127308u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127312u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127316u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127320u32);
    emu.apc_no_count(1usize, 2127320u32, 86016u32, 2127324u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002075e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 30u32, 2127332u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2127336u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127340u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127344u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127348u32);
    emu.apc_no_count(1usize, 2127348u32, 86016u32, 2127352u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002075fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 8usize, 31u32, 2127360u32);
    emu.sw_no_count(8usize, 2usize, 16u32, 2127364u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2127368u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2127372u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2127376u32);
    emu.apc_no_count(1usize, 2127376u32, 86016u32, 2127380u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2127388u32);
    emu.apc_no_count(1usize, 2127388u32, 86016u32, 2127392u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2127400u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2127404u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2127408u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2127412u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127416u32;
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
pub fn block_0x00207638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2127420u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2127424u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2127428u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2127432u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2127436u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2127440u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2127444u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2127448u32);
    emu.lw_no_count(11usize, 12usize, 4u32, 2127452u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2127456u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2127528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076a8));
    } else {
        emu.pc = 2127460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207664));
    }
}
#[inline(always)]
pub fn block_0x00207664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 12usize, 8u32, 2127464u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2127528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076a8));
    } else {
        emu.pc = 2127468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020766c));
    }
}
#[inline(always)]
pub fn block_0x0020766c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 12usize, 0u32, 2127472u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2127476u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1212u32, 2127480u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2127484u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2127488u32);
    emu.apc_no_count(1usize, 2127488u32, 8192u32, 2127492u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2127572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076d4));
    } else {
        emu.pc = 2127500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020768c));
    }
}
#[inline(always)]
pub fn block_0x0020768c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2127504u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2127508u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2127512u32);
    emu.apc_no_count(1usize, 2127512u32, 4096u32, 2127516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002076a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2127524u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2127528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2127572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002076d4));
}
#[inline(always)]
pub fn block_0x002076a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2127568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076d0));
    } else {
        emu.pc = 2127532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076ac));
    }
}
#[inline(always)]
pub fn block_0x002076ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2127532u32, 4096u32, 2127536u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002076b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2127544u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1212u32, 2127548u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2127552u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2127556u32);
    emu.apc_no_count(1usize, 2127556u32, 8192u32, 2127560u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002076cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2127568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2127572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002076d4));
}
#[inline(always)]
pub fn block_0x002076d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2127572u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2127572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002076d4));
}
#[inline]
pub fn block_0x002076d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(11usize, 10usize, 1u32, 2127576u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2127580u32);
    emu.sw_no_count(11usize, 9usize, 0u32, 2127584u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2127588u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2127592u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2127596u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2127600u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2127604u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2127608u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2127612u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2127616u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2127620u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127624u32;
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
