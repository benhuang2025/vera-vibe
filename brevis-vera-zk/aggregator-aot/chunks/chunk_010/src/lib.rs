pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2134336u32;
pub const PC_MAX: u32 = 2137072u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 115usize] = [
        block_0x00209140,
        block_0x0020915c,
        block_0x00209168,
        block_0x00209170,
        block_0x00209194,
        block_0x002091a0,
        block_0x002091ac,
        block_0x002091e8,
        block_0x00209284,
        block_0x002092b0,
        block_0x002092bc,
        block_0x002092c8,
        block_0x002092cc,
        block_0x002092e8,
        block_0x00209318,
        block_0x00209338,
        block_0x0020935c,
        block_0x00209384,
        block_0x00209388,
        block_0x00209390,
        block_0x002093a0,
        block_0x002093b0,
        block_0x002093d8,
        block_0x00209400,
        block_0x00209414,
        block_0x0020942c,
        block_0x00209490,
        block_0x00209498,
        block_0x002094a8,
        block_0x002094ac,
        block_0x002094dc,
        block_0x002094ec,
        block_0x0020950c,
        block_0x0020951c,
        block_0x00209520,
        block_0x00209544,
        block_0x00209554,
        block_0x0020957c,
        block_0x00209580,
        block_0x00209598,
        block_0x002095a4,
        block_0x002095b0,
        block_0x002095c4,
        block_0x002095c8,
        block_0x002095d4,
        block_0x002095d8,
        block_0x002095ec,
        block_0x002095f4,
        block_0x00209604,
        block_0x00209608,
        block_0x00209630,
        block_0x00209654,
        block_0x00209664,
        block_0x0020966c,
        block_0x0020967c,
        block_0x00209680,
        block_0x00209688,
        block_0x0020968c,
        block_0x002096c0,
        block_0x002096c8,
        block_0x002096f8,
        block_0x00209708,
        block_0x00209710,
        block_0x00209718,
        block_0x00209720,
        block_0x00209728,
        block_0x0020975c,
        block_0x00209760,
        block_0x00209784,
        block_0x00209788,
        block_0x002097c0,
        block_0x002097c4,
        block_0x002097e8,
        block_0x002097ec,
        block_0x00209808,
        block_0x0020980c,
        block_0x00209820,
        block_0x00209824,
        block_0x00209850,
        block_0x00209854,
        block_0x0020987c,
        block_0x00209884,
        block_0x00209888,
        block_0x002098b8,
        block_0x002098bc,
        block_0x002098e4,
        block_0x002098e8,
        block_0x00209908,
        block_0x00209910,
        block_0x00209914,
        block_0x00209944,
        block_0x00209948,
        block_0x00209968,
        block_0x0020996c,
        block_0x002099b4,
        block_0x002099b8,
        block_0x002099fc,
        block_0x00209a28,
        block_0x00209a2c,
        block_0x00209a58,
        block_0x00209a5c,
        block_0x00209a98,
        block_0x00209a9c,
        block_0x00209ab8,
        block_0x00209abc,
        block_0x00209af0,
        block_0x00209af4,
        block_0x00209b20,
        block_0x00209b24,
        block_0x00209b6c,
        block_0x00209b70,
        block_0x00209ba0,
        block_0x00209ba4,
        block_0x00209bec,
        block_0x00209bf0,
    ];
    const IDX: [u16; 685usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 4u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 6u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 19u16, 0u16, 20u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16,
        34u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16,
        0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 41u16, 0u16, 0u16, 42u16, 0u16,
        0u16, 0u16, 0u16, 43u16, 44u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 0u16, 0u16,
        47u16, 0u16, 48u16, 0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 55u16, 56u16,
        0u16, 57u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 64u16, 0u16,
        65u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 67u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16,
        70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 71u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 74u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16, 0u16, 77u16,
        78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 80u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 82u16, 83u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 85u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16,
        99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 102u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 105u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16,
        107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
        109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 113u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 115u16,
    ];
    if pc < 2134336u32 || pc > 2137072u32 {
        return None;
    }
    let word_offset = ((pc - 2134336u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00209140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(3usize, 2134336u32, 4292898816u32, 2134340u32);
    emu.adi_no_count(3usize, 3usize, 1728u32, 2134344u32);
    emu.apc_no_count(2usize, 2134344u32, 98304u32, 2134348u32);
    emu.adi_no_count(2usize, 2usize, 1360u32, 2134352u32);
    emu.lw_no_count(2usize, 2usize, 0u32, 2134356u32)?;
    emu.apc_no_count(1usize, 2134356u32, 0u32, 2134360u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134364u32;
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
pub fn block_0x0020915c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134368u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 1032u32, 2134372u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2134384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209170));
    } else {
        emu.pc = 2134376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209168));
    }
}
#[inline(always)]
pub fn block_0x00209168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134380u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1268u32, 2134384u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2134384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209170));
}
#[inline]
pub fn block_0x00209170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967295u32, 2134388u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2134392u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2134396u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2134400u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2134404u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2134408u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2134412u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2134416u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2134444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091ac));
    } else {
        emu.pc = 2134420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209194));
    }
}
#[inline(always)]
pub fn block_0x00209194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2013265920u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134424u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2134428u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2134444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091ac));
    } else {
        emu.pc = 2134432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091a0));
    }
}
#[inline(always)]
pub fn block_0x002091a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134436u32;
    emu.update_insn_clock();
    emu.sw_no_count(12usize, 11usize, 1032u32, 2134440u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134444u32;
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
pub fn block_0x002091ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2134448u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134452u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1664u32, 2134456u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2134460u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2134464u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2134468u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2134472u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2134476u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2134480u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2134484u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134488u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1672u32, 2134492u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2134496u32);
    emu.apc_no_count(1usize, 2134496u32, 73728u32, 2134500u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134504u32;
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
#[inline(never)]
pub fn block_0x002091e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2134508u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2134512u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2134516u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2134520u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2134524u32)?;
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2134528u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 0usize, 1u32, 2134532u32);
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134536u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134540u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134544u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2134548u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134552u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2134556u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 8usize, 1040u32, 2134560u32);
    emu.adi_no_count(10usize, 10usize, 1639u32, 2134564u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2134568u32);
    emu.sw_no_count(9usize, 18usize, 40u32, 2134572u32)?;
    emu.sw_no_count(0usize, 18usize, 44u32, 2134576u32)?;
    emu.sw_no_count(10usize, 18usize, 48u32, 2134580u32)?;
    emu.sw_no_count(11usize, 18usize, 52u32, 2134584u32)?;
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134588u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 12usize, 882u32, 2134592u32);
    emu.adi_no_count(12usize, 13usize, 1338u32, 2134596u32);
    emu.adi_no_count(13usize, 14usize, 639u32, 2134600u32);
    emu.adi_no_count(14usize, 15usize, 4294965388u32, 2134604u32);
    emu.sw_no_count(11usize, 18usize, 56u32, 2134608u32)?;
    emu.sw_no_count(12usize, 18usize, 60u32, 2134612u32)?;
    emu.sw_no_count(13usize, 18usize, 64u32, 2134616u32)?;
    emu.sw_no_count(14usize, 18usize, 68u32, 2134620u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134624u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965675u32, 2134628u32);
    emu.adi_no_count(11usize, 11usize, 4294966553u32, 2134632u32);
    emu.sw_no_count(10usize, 18usize, 72u32, 2134636u32)?;
    emu.sw_no_count(11usize, 18usize, 76u32, 2134640u32)?;
    emu.adi_no_count(10usize, 18usize, 80u32, 2134644u32);
    emu.adi_no_count(12usize, 0usize, 73u32, 2134648u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2134652u32);
    emu.apc_no_count(1usize, 2134652u32, 0u32, 2134656u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134660u32;
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
pub fn block_0x00209284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 1040u32, 2134664u32)?;
    emu.sw_no_count(0usize, 8usize, 1044u32, 2134668u32)?;
    emu.sw_no_count(0usize, 18usize, 8u32, 2134672u32)?;
    emu.sw_no_count(0usize, 18usize, 12u32, 2134676u32)?;
    emu.sw_no_count(0usize, 18usize, 16u32, 2134680u32)?;
    emu.sw_no_count(0usize, 18usize, 20u32, 2134684u32)?;
    emu.sw_no_count(0usize, 18usize, 24u32, 2134688u32)?;
    emu.sw_no_count(0usize, 18usize, 28u32, 2134692u32)?;
    emu.sw_no_count(0usize, 18usize, 32u32, 2134696u32)?;
    emu.apc_no_count(1usize, 2134696u32, 4294946816u32, 2134700u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002092b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2134708u32);
    emu.apc_no_count(1usize, 2134708u32, 4294963200u32, 2134712u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002092bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134720u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 12usize, 262u32, 2134724u32);
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
pub fn block_0x002092c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134732u32;
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
pub fn block_0x002092cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2134736u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1692u32, 2134740u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2134744u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2134748u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2134752u32);
    emu.apc_no_count(6usize, 2134752u32, 81920u32, 2134756u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2134760u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002092e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2134764u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2134768u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2134772u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2134776u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2134780u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2134784u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2134788u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2134792u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2134796u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2134800u32);
    emu.apc_no_count(1usize, 2134800u32, 4294963200u32, 2134804u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2134812u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2134816u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2134820u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2134824u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2134828u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2134832u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2134836u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134840u32;
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
pub fn block_0x00209338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2134844u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2134848u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2134852u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2134856u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2134860u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2134864u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2134868u32);
    emu.apc_no_count(1usize, 2134868u32, 0u32, 2134872u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020935c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2134880u32);
    emu.ani_no_count(10usize, 10usize, 3u32, 2134884u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2134888u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2134892u32);
    emu.adr_no_count(10usize, 10usize, 9usize, 2134896u32);
    emu.ani_no_count(18usize, 10usize, 4294967292u32, 2134900u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2134904u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2134908u32);
    emu.apc_no_count(1usize, 2134908u32, 73728u32, 2134912u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134916u32;
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
#[inline(always)]
pub fn block_0x00209384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2135000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002093d8));
    } else {
        emu.pc = 2134920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209388));
    }
}
#[inline(always)]
pub fn block_0x00209388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2134920u32, 4294963200u32, 2134924u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(56u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2134932u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2134936u32);
    emu.apc_no_count(1usize, 2134936u32, 4294963200u32, 2134940u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002093a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2134948u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2134952u32);
    emu.apc_no_count(1usize, 2134952u32, 0u32, 2134956u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002093b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 0u32, 2134964u32)?;
    emu.sw_no_count(19usize, 8usize, 4u32, 2134968u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2134972u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2134976u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2134980u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2134984u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2134988u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2134992u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2134996u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135000u32;
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
pub fn block_0x002093d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135004u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965726u32, 2135008u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135012u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1704u32, 2135016u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2135020u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1720u32, 2135024u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2135028u32);
    emu.adi_no_count(12usize, 2usize, 11u32, 2135032u32);
    emu.apc_no_count(1usize, 2135032u32, 77824u32, 2135036u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2135044u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2135048u32)?;
    emu.sw_no_count(10usize, 2usize, 0u32, 2135052u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2135056u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2135084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020942c));
    } else {
        emu.pc = 2135060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209414));
    }
}
#[inline(always)]
pub fn block_0x00209414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2135064u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2135068u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2135072u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2135076u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135080u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135084u32;
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
pub fn block_0x0020942c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2135088u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135092u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 712u32, 2135096u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135100u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966084u32, 2135104u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135108u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966824u32, 2135112u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2135116u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1784u32, 2135120u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2135124u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2135128u32)?;
    emu.adi_no_count(16usize, 2usize, 44u32, 2135132u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2135136u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2135140u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2135144u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2135148u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2135152u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2135156u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2135160u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2135164u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2135168u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2135172u32);
    emu.adi_no_count(11usize, 2usize, 20u32, 2135176u32);
    emu.apc_no_count(1usize, 2135176u32, 69632u32, 2135180u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2135184u32, 4294963200u32, 2135188u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2135196u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2135200u32);
    emu.apc_no_count(1usize, 2135200u32, 4294963200u32, 2135204u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002094a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2135260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002094dc));
    } else {
        emu.pc = 2135212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002094ac));
    }
}
#[inline]
pub fn block_0x002094ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2135216u32);
    emu.lw_no_count(10usize, 2usize, 8u32, 2135220u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2135224u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2135228u32)?;
    emu.sw_no_count(10usize, 12usize, 0u32, 2135232u32)?;
    emu.sw_no_count(11usize, 12usize, 4u32, 2135236u32)?;
    emu.sw_no_count(13usize, 12usize, 8u32, 2135240u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2135244u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2135248u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2135252u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135256u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135260u32;
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
pub fn block_0x002094dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2135264u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2135268u32);
    emu.apc_no_count(1usize, 2135268u32, 69632u32, 2135272u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(40u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002094ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2135280u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2135284u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2135288u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2135292u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2135296u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2135300u32)?;
    emu.apc_no_count(1usize, 2135300u32, 4294963200u32, 2135304u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020950c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2135312u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2135316u32);
    emu.apc_no_count(1usize, 2135316u32, 4294963200u32, 2135320u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020951c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2135364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209544));
    } else {
        emu.pc = 2135328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209520));
    }
}
#[inline]
pub fn block_0x00209520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135332u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 10usize, 0u32, 2135336u32)?;
    emu.sw_no_count(8usize, 10usize, 4u32, 2135340u32)?;
    emu.sw_no_count(9usize, 10usize, 8u32, 2135344u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2135348u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2135352u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2135356u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2135360u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135364u32;
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
pub fn block_0x00209544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2135368u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2135372u32);
    emu.apc_no_count(1usize, 2135372u32, 69632u32, 2135376u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2135384u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2135388u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2135392u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2135396u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2135400u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2135404u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2135408u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2135412u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2135416u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2135448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209598));
    } else {
        emu.pc = 2135420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020957c));
    }
}
#[inline(always)]
pub fn block_0x0020957c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2135424u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2135424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209580));
}
#[inline(always)]
pub fn block_0x00209580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135428u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1868u32, 2135432u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2135436u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2135440u32);
    emu.apc_no_count(1usize, 2135440u32, 69632u32, 2135444u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135448u32;
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
pub fn block_0x00209598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2135452u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2135456u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2135508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095d4));
    } else {
        emu.pc = 2135460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095a4));
    }
}
#[inline(always)]
pub fn block_0x002095a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 12usize, 0u32, 2135464u32);
    emu.apc_no_count(1usize, 2135464u32, 4294963200u32, 2135468u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2135476u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2135480u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2135484u32);
    emu.apc_no_count(1usize, 2135484u32, 4294963200u32, 2135488u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2135424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209580));
    } else {
        emu.pc = 2135496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c8));
    }
}
#[inline(always)]
pub fn block_0x002095c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2135500u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2135504u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2135508u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2135512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002095d8));
}
#[inline(always)]
pub fn block_0x002095d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2135512u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2135512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002095d8));
}
#[inline(always)]
pub fn block_0x002095d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2135516u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2135520u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2135524u32);
    emu.apc_no_count(1usize, 2135524u32, 4294963200u32, 2135528u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2135532u32, 4294963200u32, 2135536u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2135544u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2135548u32);
    emu.apc_no_count(1usize, 2135548u32, 4294963200u32, 2135552u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135556u32;
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
pub fn block_0x00209604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2135636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209654));
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
#[inline]
pub fn block_0x00209608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2135564u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2135568u32)?;
    emu.sw_no_count(19usize, 10usize, 4u32, 2135572u32)?;
    emu.sw_no_count(8usize, 10usize, 8u32, 2135576u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135580u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1824u32, 2135584u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2135588u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2135592u32);
    emu.apc_no_count(1usize, 2135592u32, 65536u32, 2135596u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2135604u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2135608u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2135612u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2135616u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2135620u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2135624u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2135628u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2135632u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135636u32;
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
pub fn block_0x00209654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2135640u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2135644u32);
    emu.apc_no_count(1usize, 2135644u32, 69632u32, 2135648u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135652u32;
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
pub fn block_0x00209664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2135656u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2135676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020967c));
    } else {
        emu.pc = 2135660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020966c));
    }
}
#[inline(always)]
pub fn block_0x0020966c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2135664u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2135668u32);
    emu.apc_no_count(6usize, 2135668u32, 4294963200u32, 2135672u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135676u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020967c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135680u32;
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
pub fn block_0x00209680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2135684u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135688u32;
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
pub fn block_0x00209688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135692u32;
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
pub fn block_0x0020968c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1059942400u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135696u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2722881536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135700u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2725687296u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135704u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2304393216u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2135708u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966095u32, 2135712u32);
    emu.adi_no_count(12usize, 12usize, 1479u32, 2135716u32);
    emu.adi_no_count(13usize, 13usize, 4294966961u32, 2135720u32);
    emu.adi_no_count(14usize, 14usize, 4294965480u32, 2135724u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2135728u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2135732u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2135736u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2135740u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135744u32;
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
pub fn block_0x002096c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2135748u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135752u32;
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
pub fn block_0x002096c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2135756u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2135760u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2135764u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2135768u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2135772u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135776u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1884u32, 2135780u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2135784u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2135788u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2135792u32);
    emu.apc_no_count(1usize, 2135792u32, 73728u32, 2135796u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967080u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002096f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2135804u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2135808u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2135812u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2135832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209718));
    } else {
        emu.pc = 2135816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209708));
    }
}
#[inline(always)]
pub fn block_0x00209708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2135820u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2135840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209720));
    } else {
        emu.pc = 2135824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209710));
    }
}
#[inline(always)]
pub fn block_0x00209710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2135824u32, 69632u32, 2135828u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135832u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2135832u32, 69632u32, 2135836u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135840u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1012u32);
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
    emu.apc_no_count(6usize, 2135840u32, 69632u32, 2135844u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135848u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 12usize, 0u32, 2135852u32)?;
    emu.lw_no_count(15usize, 12usize, 4u32, 2135856u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2135860u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2135864u32)?;
    emu.lw_no_count(17usize, 11usize, 8u32, 2135868u32)?;
    emu.lw_no_count(30usize, 11usize, 12u32, 2135872u32)?;
    emu.lw_no_count(29usize, 12usize, 8u32, 2135876u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2135880u32)?;
    emu.adr_no_count(5usize, 15usize, 5usize, 2135884u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2135888u32);
    emu.sltru_no_count(28usize, 13usize, 14usize, 2135892u32);
    emu.adr_no_count(14usize, 5usize, 28usize, 2135896u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2135904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209760));
    } else {
        emu.pc = 2135900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020975c));
    }
}
#[inline(always)]
pub fn block_0x0020975c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 15usize, 2135904u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2135904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209760));
}
#[inline]
pub fn block_0x00209760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 11usize, 16u32, 2135908u32)?;
    emu.lw_no_count(7usize, 11usize, 20u32, 2135912u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2135916u32)?;
    emu.lw_no_count(15usize, 12usize, 20u32, 2135920u32)?;
    emu.adr_no_count(30usize, 16usize, 30usize, 2135924u32);
    emu.adr_no_count(31usize, 29usize, 17usize, 2135928u32);
    emu.sltru_no_count(17usize, 31usize, 29usize, 2135932u32);
    emu.adr_no_count(30usize, 30usize, 17usize, 2135936u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2135944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209788));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 30usize, 16usize, 2135944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2135944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209788));
}
#[inline]
pub fn block_0x00209788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2135948u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2135952u32)?;
    emu.adr_no_count(28usize, 31usize, 28usize, 2135956u32);
    emu.adr_no_count(7usize, 15usize, 7usize, 2135960u32);
    emu.adr_no_count(16usize, 6usize, 5usize, 2135964u32);
    emu.sltru_no_count(31usize, 28usize, 31usize, 2135968u32);
    emu.sltru_no_count(5usize, 16usize, 6usize, 2135972u32);
    emu.adr_no_count(29usize, 30usize, 31usize, 2135976u32);
    emu.sltru_no_count(6usize, 29usize, 30usize, 2135980u32);
    emu.anr_no_count(31usize, 31usize, 6usize, 2135984u32);
    emu.adr_no_count(31usize, 17usize, 31usize, 2135988u32);
    emu.adr_no_count(6usize, 7usize, 5usize, 2135992u32);
    emu.sltru_no_count(8usize, 31usize, 17usize, 2135996u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2136004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097c4));
    } else {
        emu.pc = 2136000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097c0));
    }
}
#[inline(always)]
pub fn block_0x002097c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 15usize, 2136004u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002097c4));
}
#[inline]
pub fn block_0x002097c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 11usize, 24u32, 2136008u32)?;
    emu.lw_no_count(30usize, 11usize, 28u32, 2136012u32)?;
    emu.lw_no_count(7usize, 12usize, 24u32, 2136016u32)?;
    emu.lw_no_count(11usize, 12usize, 28u32, 2136020u32)?;
    emu.adr_no_count(15usize, 16usize, 31usize, 2136024u32);
    emu.sltru_no_count(12usize, 15usize, 16usize, 2136028u32);
    emu.adr_no_count(16usize, 6usize, 8usize, 2136032u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2136036u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2136044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097ec));
    } else {
        emu.pc = 2136040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097e8));
    }
}
#[inline(always)]
pub fn block_0x002097e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 6usize, 2136044u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002097ec));
}
#[inline(always)]
pub fn block_0x002097ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 5usize, 12usize, 2136048u32);
    emu.adr_no_count(12usize, 11usize, 30usize, 2136052u32);
    emu.adr_no_count(30usize, 7usize, 17usize, 2136056u32);
    emu.sltru_no_count(17usize, 30usize, 7usize, 2136060u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2136064u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2136068u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2136076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020980c));
    } else {
        emu.pc = 2136072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209808));
    }
}
#[inline(always)]
pub fn block_0x00209808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 12usize, 11usize, 2136076u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020980c));
}
#[inline(always)]
pub fn block_0x0020980c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 30usize, 6usize, 2136080u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2136084u32);
    emu.adr_no_count(5usize, 12usize, 5usize, 2136088u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2136092u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2136100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209824));
    } else {
        emu.pc = 2136096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209820));
    }
}
#[inline(always)]
pub fn block_0x00209820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 12usize, 2136100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209824));
}
#[inline]
pub fn block_0x00209824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 1u32, 2136104u32);
    emu.sltiu_no_count(11usize, 12usize, 1u32, 2136108u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2136112u32);
    emu.orr_no_count(13usize, 12usize, 11usize, 2136116u32);
    emu.sltru_no_count(30usize, 0usize, 13usize, 2136120u32);
    emu.sbr_no_count(13usize, 0usize, 30usize, 2136124u32);
    emu.sbr_no_count(28usize, 28usize, 30usize, 2136128u32);
    emu.sltru_no_count(14usize, 28usize, 13usize, 2136132u32);
    emu.sbr_no_count(31usize, 29usize, 30usize, 2136136u32);
    emu.adr_no_count(31usize, 31usize, 14usize, 2136140u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2136148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209854));
    } else {
        emu.pc = 2136144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209850));
    }
}
#[inline(always)]
pub fn block_0x00209850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 13usize, 2136148u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209854));
}
#[inline]
pub fn block_0x00209854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(29usize, 14usize, 30usize, 2136152u32);
    emu.adi_no_count(14usize, 28usize, 1u32, 2136156u32);
    emu.sltru_no_count(8usize, 29usize, 13usize, 2136160u32);
    emu.sltiu_no_count(13usize, 14usize, 1u32, 2136164u32);
    emu.adr_no_count(13usize, 31usize, 13usize, 2136168u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2136172u32);
    emu.sbr_no_count(30usize, 8usize, 30usize, 2136176u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2136180u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2136184u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2136196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209884));
    } else {
        emu.pc = 2136188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020987c));
    }
}
#[inline(always)]
pub fn block_0x0020987c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 31usize, 2136192u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136200u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209888));
}
#[inline(always)]
pub fn block_0x00209884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 28usize, 2136200u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209888));
}
#[inline]
pub fn block_0x00209888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 29usize, 4294967295u32, 2136204u32);
    emu.sltiu_no_count(29usize, 29usize, 1u32, 2136208u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2136212u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2136216u32);
    emu.sltru_no_count(28usize, 28usize, 31usize, 2136220u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2136224u32);
    emu.sai_no_count(28usize, 28usize, 1055u32, 2136228u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2136232u32);
    emu.sltru_no_count(29usize, 15usize, 28usize, 2136236u32);
    emu.adr_no_count(16usize, 28usize, 16usize, 2136240u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2136244u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2136252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098bc));
    } else {
        emu.pc = 2136248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098b8));
    }
}
#[inline(always)]
pub fn block_0x002098b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 16usize, 28usize, 2136252u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002098bc));
}
#[inline]
pub fn block_0x002098bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 28usize, 29usize, 2136256u32);
    emu.sltru_no_count(29usize, 29usize, 28usize, 2136260u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2136264u32);
    emu.sai_no_count(30usize, 28usize, 1055u32, 2136268u32);
    emu.adr_no_count(31usize, 5usize, 30usize, 2136272u32);
    emu.adr_no_count(28usize, 6usize, 30usize, 2136276u32);
    emu.sltru_no_count(29usize, 28usize, 6usize, 2136280u32);
    emu.adr_no_count(31usize, 31usize, 29usize, 2136284u32);
    emu.adr_no_count(7usize, 17usize, 7usize, 2136288u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2136296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098e8));
    } else {
        emu.pc = 2136292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098e4));
    }
}
#[inline(always)]
pub fn block_0x002098e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 31usize, 5usize, 2136296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002098e8));
}
#[inline(always)]
pub fn block_0x002098e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 7usize, 17usize, 2136300u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2136304u32);
    emu.sltru_no_count(5usize, 0usize, 28usize, 2136308u32);
    emu.sltru_no_count(17usize, 29usize, 30usize, 2136312u32);
    emu.adr_no_count(5usize, 31usize, 5usize, 2136316u32);
    emu.adr_no_count(30usize, 30usize, 17usize, 2136320u32);
    emu.adi_no_count(17usize, 28usize, 4294967295u32, 2136324u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2136336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209910));
    } else {
        emu.pc = 2136328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209908));
    }
}
#[inline(always)]
pub fn block_0x00209908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 5usize, 31usize, 2136332u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209914));
}
#[inline(always)]
pub fn block_0x00209910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 17usize, 28usize, 2136340u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209914));
}
#[inline]
pub fn block_0x00209914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 29usize, 4294967295u32, 2136344u32);
    emu.sltiu_no_count(29usize, 29usize, 1u32, 2136348u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2136352u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2136356u32);
    emu.sltru_no_count(28usize, 28usize, 31usize, 2136360u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2136364u32);
    emu.sai_no_count(28usize, 28usize, 1055u32, 2136368u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2136372u32);
    emu.sltru_no_count(7usize, 7usize, 28usize, 2136376u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2136380u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2136384u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2136392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209948));
    } else {
        emu.pc = 2136388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209944));
    }
}
#[inline(always)]
pub fn block_0x00209944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 6usize, 28usize, 2136392u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209948));
}
#[inline(always)]
pub fn block_0x00209948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 28usize, 7usize, 2136396u32);
    emu.sltru_no_count(7usize, 6usize, 28usize, 2136400u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2136404u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2136408u32);
    emu.sltru_no_count(28usize, 12usize, 6usize, 2136412u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2136416u32);
    emu.adr_no_count(11usize, 11usize, 28usize, 2136420u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020996c));
    } else {
        emu.pc = 2136424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209968));
    }
}
#[inline(always)]
pub fn block_0x00209968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 11usize, 7usize, 2136428u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020996c));
}
#[inline]
pub fn block_0x0020996c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 6usize, 14usize, 2136432u32);
    emu.sltru_no_count(30usize, 29usize, 6usize, 2136436u32);
    emu.adr_no_count(14usize, 29usize, 28usize, 2136440u32);
    emu.adr_no_count(28usize, 13usize, 30usize, 2136444u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2136448u32);
    emu.sltru_no_count(13usize, 0usize, 28usize, 2136452u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2136456u32);
    emu.anr_no_count(30usize, 13usize, 30usize, 2136460u32);
    emu.adr_no_count(13usize, 28usize, 29usize, 2136464u32);
    emu.sltru_no_count(28usize, 13usize, 28usize, 2136468u32);
    emu.anr_no_count(28usize, 29usize, 28usize, 2136472u32);
    emu.adr_no_count(28usize, 30usize, 28usize, 2136476u32);
    emu.sltru_no_count(29usize, 28usize, 30usize, 2136480u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2136484u32);
    emu.sltru_no_count(28usize, 15usize, 28usize, 2136488u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2136492u32);
    emu.adr_no_count(16usize, 16usize, 28usize, 2136496u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2136504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099b8));
    } else {
        emu.pc = 2136500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099b4));
    }
}
#[inline(always)]
pub fn block_0x002099b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 16usize, 29usize, 2136504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002099b8));
}
#[inline]
pub fn block_0x002099b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(6usize, 6usize, 1u32, 2136508u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2136512u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2136516u32)?;
    emu.sw_no_count(11usize, 10usize, 4u32, 2136520u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2136524u32)?;
    emu.sw_no_count(13usize, 10usize, 12u32, 2136528u32)?;
    emu.adr_no_count(17usize, 6usize, 17usize, 2136532u32);
    emu.sltru_no_count(11usize, 17usize, 6usize, 2136536u32);
    emu.adr_no_count(28usize, 17usize, 28usize, 2136540u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2136544u32);
    emu.sltru_no_count(12usize, 28usize, 17usize, 2136548u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2136552u32);
    emu.sw_no_count(15usize, 10usize, 16u32, 2136556u32)?;
    emu.sw_no_count(16usize, 10usize, 20u32, 2136560u32)?;
    emu.sw_no_count(28usize, 10usize, 24u32, 2136564u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2136568u32)?;
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136572u32;
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
pub fn block_0x002099fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 11usize, 0u32, 2136576u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2136580u32)?;
    emu.lw_no_count(13usize, 11usize, 8u32, 2136584u32)?;
    emu.lw_no_count(7usize, 11usize, 12u32, 2136588u32)?;
    emu.lw_no_count(16usize, 12usize, 0u32, 2136592u32)?;
    emu.lw_no_count(6usize, 12usize, 4u32, 2136596u32)?;
    emu.lw_no_count(30usize, 12usize, 8u32, 2136600u32)?;
    emu.lw_no_count(29usize, 12usize, 12u32, 2136604u32)?;
    emu.sltru_no_count(17usize, 14usize, 16usize, 2136608u32);
    emu.adi_no_count(15usize, 17usize, 0u32, 2136612u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2136620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a2c));
    } else {
        emu.pc = 2136616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a28));
    }
}
#[inline(always)]
pub fn block_0x00209a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 5usize, 6usize, 2136620u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a2c));
}
#[inline]
pub fn block_0x00209a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136624u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2136628u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2136632u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2136636u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2136640u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2136644u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2136648u32)?;
    emu.sltru_no_count(8usize, 13usize, 30usize, 2136652u32);
    emu.sbr_no_count(28usize, 7usize, 29usize, 2136656u32);
    emu.sbr_no_count(28usize, 28usize, 8usize, 2136660u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2136668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a5c));
    } else {
        emu.pc = 2136664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a58));
    }
}
#[inline(always)]
pub fn block_0x00209a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 7usize, 29usize, 2136668u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a5c));
}
#[inline]
pub fn block_0x00209a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 11usize, 16u32, 2136672u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2136676u32)?;
    emu.lw_no_count(29usize, 12usize, 16u32, 2136680u32)?;
    emu.lw_no_count(9usize, 12usize, 20u32, 2136684u32)?;
    emu.sbr_no_count(18usize, 0usize, 8usize, 2136688u32);
    emu.sbr_no_count(13usize, 13usize, 30usize, 2136692u32);
    emu.adr_no_count(30usize, 15usize, 8usize, 2136696u32);
    emu.sbr_no_count(20usize, 28usize, 15usize, 2136700u32);
    emu.sbr_no_count(8usize, 0usize, 30usize, 2136704u32);
    emu.sbr_no_count(15usize, 13usize, 15usize, 2136708u32);
    emu.sltru_no_count(18usize, 8usize, 18usize, 2136712u32);
    emu.sltru_no_count(19usize, 15usize, 13usize, 2136716u32);
    emu.adr_no_count(13usize, 20usize, 19usize, 2136720u32);
    emu.sbr_no_count(18usize, 18usize, 30usize, 2136724u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2136732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a9c));
    } else {
        emu.pc = 2136728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a98));
    }
}
#[inline(always)]
pub fn block_0x00209a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(19usize, 13usize, 28usize, 2136732u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a9c));
}
#[inline(always)]
pub fn block_0x00209a9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(28usize, 19usize, 30usize, 2136736u32);
    emu.sltru_no_count(28usize, 28usize, 8usize, 2136740u32);
    emu.adr_no_count(28usize, 18usize, 28usize, 2136744u32);
    emu.sltru_no_count(21usize, 7usize, 29usize, 2136748u32);
    emu.sai_no_count(19usize, 28usize, 1055u32, 2136752u32);
    emu.adi_no_count(20usize, 21usize, 0u32, 2136756u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2136764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209abc));
    } else {
        emu.pc = 2136760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ab8));
    }
}
#[inline(always)]
pub fn block_0x00209ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 31usize, 9usize, 2136764u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209abc));
}
#[inline]
pub fn block_0x00209abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(28usize, 11usize, 24u32, 2136768u32)?;
    emu.lw_no_count(8usize, 11usize, 28u32, 2136772u32)?;
    emu.lw_no_count(30usize, 12usize, 24u32, 2136776u32)?;
    emu.lw_no_count(18usize, 12usize, 28u32, 2136780u32)?;
    emu.sbr_no_count(11usize, 31usize, 9usize, 2136784u32);
    emu.sbr_no_count(12usize, 7usize, 29usize, 2136788u32);
    emu.sbr_no_count(31usize, 11usize, 21usize, 2136792u32);
    emu.adr_no_count(11usize, 12usize, 19usize, 2136796u32);
    emu.adr_no_count(7usize, 31usize, 19usize, 2136800u32);
    emu.sltru_no_count(29usize, 11usize, 12usize, 2136804u32);
    emu.adr_no_count(12usize, 7usize, 29usize, 2136808u32);
    emu.sbr_no_count(7usize, 0usize, 20usize, 2136812u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2136820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209af4));
    } else {
        emu.pc = 2136816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209af0));
    }
}
#[inline(always)]
pub fn block_0x00209af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 12usize, 31usize, 2136820u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209af4));
}
#[inline]
pub fn block_0x00209af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 5usize, 6usize, 2136824u32);
    emu.sbr_no_count(6usize, 19usize, 20usize, 2136828u32);
    emu.adr_no_count(29usize, 6usize, 29usize, 2136832u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2136836u32);
    emu.sltru_no_count(29usize, 29usize, 6usize, 2136840u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2136844u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2136848u32);
    emu.sltru_no_count(29usize, 28usize, 30usize, 2136852u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2136856u32);
    emu.adi_no_count(7usize, 29usize, 0u32, 2136860u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2136868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b24));
    } else {
        emu.pc = 2136864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b20));
    }
}
#[inline(always)]
pub fn block_0x00209b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 8usize, 18usize, 2136868u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b24));
}
#[inline]
pub fn block_0x00209b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 5usize, 17usize, 2136872u32);
    emu.sbr_no_count(17usize, 14usize, 16usize, 2136876u32);
    emu.sbr_no_count(14usize, 8usize, 18usize, 2136880u32);
    emu.sbr_no_count(16usize, 28usize, 30usize, 2136884u32);
    emu.sbr_no_count(29usize, 14usize, 29usize, 2136888u32);
    emu.adr_no_count(14usize, 16usize, 6usize, 2136892u32);
    emu.adr_no_count(28usize, 29usize, 6usize, 2136896u32);
    emu.sltru_no_count(30usize, 14usize, 16usize, 2136900u32);
    emu.adr_no_count(16usize, 28usize, 30usize, 2136904u32);
    emu.sbr_no_count(28usize, 0usize, 7usize, 2136908u32);
    emu.lw_no_count(8usize, 2usize, 28u32, 2136912u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2136916u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2136920u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2136924u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2136928u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2136932u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2136936u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2136944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b70));
    } else {
        emu.pc = 2136940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b6c));
    }
}
#[inline(always)]
pub fn block_0x00209b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 16usize, 29usize, 2136944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b70));
}
#[inline]
pub fn block_0x00209b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(6usize, 6usize, 7usize, 2136948u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2136952u32);
    emu.sltru_no_count(7usize, 6usize, 28usize, 2136956u32);
    emu.sltru_no_count(28usize, 30usize, 6usize, 2136960u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2136964u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2136968u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2136972u32);
    emu.adr_no_count(17usize, 6usize, 17usize, 2136976u32);
    emu.sltru_no_count(7usize, 17usize, 6usize, 2136980u32);
    emu.adr_no_count(5usize, 6usize, 5usize, 2136984u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2136988u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2136996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ba4));
    } else {
        emu.pc = 2136992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ba0));
    }
}
#[inline(always)]
pub fn block_0x00209ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 6usize, 2136996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2136996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ba4));
}
#[inline]
pub fn block_0x00209ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 6usize, 15usize, 2137000u32);
    emu.sltru_no_count(29usize, 28usize, 6usize, 2137004u32);
    emu.adr_no_count(15usize, 28usize, 7usize, 2137008u32);
    emu.adr_no_count(7usize, 13usize, 29usize, 2137012u32);
    emu.sltru_no_count(28usize, 15usize, 28usize, 2137016u32);
    emu.sltru_no_count(13usize, 0usize, 7usize, 2137020u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2137024u32);
    emu.anr_no_count(29usize, 13usize, 29usize, 2137028u32);
    emu.adr_no_count(13usize, 7usize, 28usize, 2137032u32);
    emu.sltru_no_count(7usize, 13usize, 7usize, 2137036u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2137040u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2137044u32);
    emu.sltru_no_count(28usize, 7usize, 29usize, 2137048u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2137052u32);
    emu.sltru_no_count(7usize, 11usize, 7usize, 2137056u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2137060u32);
    emu.adr_no_count(12usize, 12usize, 7usize, 2137064u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2137072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bf0));
    } else {
        emu.pc = 2137068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bec));
    }
}
#[inline(always)]
pub fn block_0x00209bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 28usize, 2137072u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209bf0));
}
#[inline]
pub fn block_0x00209bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(28usize, 6usize, 1u32, 2137076u32);
    emu.sbr_no_count(14usize, 14usize, 6usize, 2137080u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2137084u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2137088u32)?;
    emu.sw_no_count(5usize, 10usize, 4u32, 2137092u32)?;
    emu.sw_no_count(15usize, 10usize, 8u32, 2137096u32)?;
    emu.sw_no_count(13usize, 10usize, 12u32, 2137100u32)?;
    emu.sltru_no_count(13usize, 14usize, 28usize, 2137104u32);
    emu.adr_no_count(7usize, 14usize, 7usize, 2137108u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2137112u32);
    emu.sltru_no_count(14usize, 7usize, 14usize, 2137116u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2137120u32);
    emu.sw_no_count(11usize, 10usize, 16u32, 2137124u32)?;
    emu.sw_no_count(12usize, 10usize, 20u32, 2137128u32)?;
    emu.sw_no_count(7usize, 10usize, 24u32, 2137132u32)?;
    emu.sw_no_count(13usize, 10usize, 28u32, 2137136u32)?;
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137140u32;
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
