pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2148760u32;
pub const PC_MAX: u32 = 2151020u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 122usize] = [
        block_0x0020c998,
        block_0x0020c9c0,
        block_0x0020c9cc,
        block_0x0020c9d4,
        block_0x0020c9fc,
        block_0x0020ca00,
        block_0x0020ca1c,
        block_0x0020ca2c,
        block_0x0020ca40,
        block_0x0020ca44,
        block_0x0020ca50,
        block_0x0020ca5c,
        block_0x0020ca7c,
        block_0x0020ca84,
        block_0x0020ca90,
        block_0x0020ca98,
        block_0x0020caa4,
        block_0x0020cacc,
        block_0x0020cae0,
        block_0x0020cae4,
        block_0x0020cb08,
        block_0x0020cb24,
        block_0x0020cb3c,
        block_0x0020cb54,
        block_0x0020cb6c,
        block_0x0020cb98,
        block_0x0020cbd0,
        block_0x0020cbdc,
        block_0x0020cbe0,
        block_0x0020cc00,
        block_0x0020cc14,
        block_0x0020cc18,
        block_0x0020cc6c,
        block_0x0020cc74,
        block_0x0020cc7c,
        block_0x0020cc8c,
        block_0x0020cc94,
        block_0x0020cc9c,
        block_0x0020cca4,
        block_0x0020ccac,
        block_0x0020ccb0,
        block_0x0020ccc4,
        block_0x0020cccc,
        block_0x0020ccd0,
        block_0x0020ccd8,
        block_0x0020ccfc,
        block_0x0020cd14,
        block_0x0020cdac,
        block_0x0020cdc0,
        block_0x0020cddc,
        block_0x0020cde4,
        block_0x0020cdec,
        block_0x0020cdf0,
        block_0x0020cdf8,
        block_0x0020ce00,
        block_0x0020ce10,
        block_0x0020ce14,
        block_0x0020ce20,
        block_0x0020ce28,
        block_0x0020ce34,
        block_0x0020ce38,
        block_0x0020ce44,
        block_0x0020ce4c,
        block_0x0020ce54,
        block_0x0020ce88,
        block_0x0020ce90,
        block_0x0020ce94,
        block_0x0020ce9c,
        block_0x0020cea8,
        block_0x0020ceb0,
        block_0x0020ceb8,
        block_0x0020cebc,
        block_0x0020cecc,
        block_0x0020ced4,
        block_0x0020cee0,
        block_0x0020cee4,
        block_0x0020cee8,
        block_0x0020cef4,
        block_0x0020cef8,
        block_0x0020cf08,
        block_0x0020cf18,
        block_0x0020cf2c,
        block_0x0020cf30,
        block_0x0020cf34,
        block_0x0020cf38,
        block_0x0020cf50,
        block_0x0020cf6c,
        block_0x0020cf70,
        block_0x0020cf74,
        block_0x0020cf7c,
        block_0x0020cf84,
        block_0x0020cf88,
        block_0x0020cfc4,
        block_0x0020cff0,
        block_0x0020d010,
        block_0x0020d018,
        block_0x0020d038,
        block_0x0020d068,
        block_0x0020d0a4,
        block_0x0020d0dc,
        block_0x0020d0f8,
        block_0x0020d108,
        block_0x0020d114,
        block_0x0020d118,
        block_0x0020d140,
        block_0x0020d150,
        block_0x0020d1a0,
        block_0x0020d1a4,
        block_0x0020d1bc,
        block_0x0020d1c0,
        block_0x0020d1d0,
        block_0x0020d1d4,
        block_0x0020d1f0,
        block_0x0020d1f4,
        block_0x0020d1fc,
        block_0x0020d210,
        block_0x0020d218,
        block_0x0020d230,
        block_0x0020d238,
        block_0x0020d254,
        block_0x0020d25c,
        block_0x0020d26c,
    ];
    const IDX: [u16; 566usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16,
        3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16,
        6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 0u16,
        0u16, 0u16, 0u16, 9u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 16u16,
        0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16,
        0u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16,
        0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 31u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16,
        0u16, 34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 38u16,
        0u16, 39u16, 0u16, 40u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 43u16,
        44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 51u16, 0u16, 52u16, 53u16, 0u16, 54u16, 0u16, 55u16,
        0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 58u16, 0u16, 59u16, 0u16, 0u16,
        60u16, 61u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16, 67u16,
        0u16, 68u16, 0u16, 0u16, 69u16, 0u16, 70u16, 0u16, 71u16, 72u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 74u16, 0u16, 0u16, 75u16, 76u16, 77u16, 0u16, 0u16, 78u16,
        79u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        82u16, 83u16, 84u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 89u16, 0u16, 90u16, 0u16, 91u16, 92u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16,
        110u16, 0u16, 0u16, 0u16, 111u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        113u16, 114u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 117u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        120u16, 0u16, 121u16, 0u16, 0u16, 0u16, 122u16,
    ];
    if pc < 2148760u32 || pc > 2151020u32 {
        return None;
    }
    let word_offset = ((pc - 2148760u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020c998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2148764u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2148768u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2148772u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2148776u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2148780u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2148784u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2148788u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2148792u32);
    emu.adi_no_count(11usize, 0usize, 1280u32, 2148796u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2149128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb08));
    } else {
        emu.pc = 2148800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9c0));
    }
}
#[inline(always)]
pub fn block_0x0020c9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 160u32, 2148804u32)?;
    emu.sri_no_count(19usize, 8usize, 5u32, 2148808u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2148892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca1c));
    } else {
        emu.pc = 2148812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9cc));
    }
}
#[inline(always)]
pub fn block_0x0020c9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2148816u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2149180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb3c));
    } else {
        emu.pc = 2148820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9d4));
    }
}
#[inline]
pub fn block_0x0020c9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2148824u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2148828u32);
    emu.adr_no_count(12usize, 12usize, 19usize, 2148832u32);
    emu.adr_no_count(14usize, 13usize, 10usize, 2148836u32);
    emu.adi_no_count(13usize, 12usize, 4294967295u32, 2148840u32);
    emu.sli_no_count(15usize, 12usize, 2u32, 2148844u32);
    emu.adi_no_count(12usize, 14usize, 4294967292u32, 2148848u32);
    emu.adr_no_count(14usize, 15usize, 10usize, 2148852u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2148856u32);
    emu.adi_no_count(15usize, 0usize, 39u32, 2148860u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2148860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c9fc));
}
#[inline(always)]
pub fn block_0x0020c9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2149156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb24));
    } else {
        emu.pc = 2148864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca00));
    }
}
#[inline(always)]
pub fn block_0x0020ca00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 12usize, 0u32, 2148868u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2148872u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2148876u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2148880u32);
    emu.sw_no_count(16usize, 14usize, 0u32, 2148884u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2148888u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2148860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9fc));
    } else {
        emu.pc = 2148892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca1c));
    }
}
#[inline(always)]
pub fn block_0x0020ca1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 8usize, 31u32, 2148896u32);
    emu.adi_no_count(11usize, 0usize, 32u32, 2148900u32);
    emu.sli_no_count(9usize, 19usize, 2u32, 2148904u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2148932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca44));
    } else {
        emu.pc = 2148908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca2c));
    }
}
#[inline(always)]
pub fn block_0x0020ca2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2148912u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2148916u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2148920u32);
    emu.apc_no_count(1usize, 2148920u32, 4294926336u32, 2148924u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ca40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2148932u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148932u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca44));
}
#[inline(always)]
pub fn block_0x0020ca44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 160u32, 2148936u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2148940u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2149088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cae0));
    } else {
        emu.pc = 2148944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca50));
    }
}
#[inline(always)]
pub fn block_0x0020ca50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2148948u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2148952u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2149156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb24));
    } else {
        emu.pc = 2148956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca5c));
    }
}
#[inline(always)]
pub fn block_0x0020ca5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2148960u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2148964u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2148968u32)?;
    emu.sbr_no_count(12usize, 0usize, 8usize, 2148972u32);
    emu.srr_no_count(15usize, 11usize, 12usize, 2148976u32);
    emu.sli_no_count(13usize, 14usize, 2u32, 2148980u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2148984u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca90));
    } else {
        emu.pc = 2148988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca7c));
    }
}
#[inline(always)]
pub fn block_0x0020ca7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2148992u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2149204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb54));
    } else {
        emu.pc = 2148996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca84));
    }
}
#[inline(always)]
pub fn block_0x0020ca84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 13usize, 2149000u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2149004u32)?;
    emu.adi_no_count(11usize, 14usize, 1u32, 2149008u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149008u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca90));
}
#[inline(always)]
pub fn block_0x0020ca90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2149012u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2149068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cacc));
    } else {
        emu.pc = 2149016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca98));
    }
}
#[inline(always)]
pub fn block_0x0020ca98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 31u32, 2149020u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2149024u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2149028u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020caa4));
}
#[inline]
pub fn block_0x0020caa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 13usize, 0u32, 2149032u32)?;
    emu.lw_no_count(16usize, 13usize, 4294967292u32, 2149036u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2149040u32);
    emu.slr_no_count(15usize, 15usize, 20usize, 2149044u32);
    emu.srr_no_count(16usize, 16usize, 12usize, 2149048u32);
    emu.adi_no_count(17usize, 13usize, 4294967292u32, 2149052u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2149056u32);
    emu.sw_no_count(15usize, 13usize, 0u32, 2149060u32)?;
    emu.adi_no_count(13usize, 17usize, 0u32, 2149064u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2149028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caa4));
    } else {
        emu.pc = 2149068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cacc));
    }
}
#[inline(always)]
pub fn block_0x0020cacc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 10usize, 9usize, 2149072u32);
    emu.lw_no_count(12usize, 9usize, 0u32, 2149076u32)?;
    emu.slr_no_count(12usize, 12usize, 20usize, 2149080u32);
    emu.sw_no_count(12usize, 9usize, 0u32, 2149084u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2149088u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149092u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cae4));
}
#[inline(always)]
pub fn block_0x0020cae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 0u32, 2149092u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cae4));
}
#[inline]
pub fn block_0x0020cae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 160u32, 2149096u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2149100u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2149104u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2149108u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2149112u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2149116u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2149120u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149124u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149128u32;
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
pub fn block_0x0020cb08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149132u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 962u32, 2149136u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149140u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2149144u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2149148u32);
    emu.apc_no_count(1usize, 2149148u32, 0u32, 2149152u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cb24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149160u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2149164u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149168u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2149172u32);
    emu.apc_no_count(1usize, 2149172u32, 0u32, 2149176u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cb3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 4294967295u32, 2149184u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149188u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2149192u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149196u32);
    emu.apc_no_count(1usize, 2149196u32, 0u32, 2149200u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149208u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2149212u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149216u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2149220u32);
    emu.apc_no_count(1usize, 2149220u32, 0u32, 2149224u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020cb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2149232u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2149236u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2149240u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2149244u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2149248u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2149252u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2149256u32)?;
    emu.sw_no_count(22usize, 2usize, 4u32, 2149260u32)?;
    emu.sli_no_count(12usize, 12usize, 2u32, 2149264u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2149268u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2149532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc9c));
    } else {
        emu.pc = 2149272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb98));
    }
}
#[inline]
pub fn block_0x0020cb98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 10usize, 0u32, 2149276u32);
    emu.adi_no_count(5usize, 0usize, 0u32, 2149280u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2149284u32);
    emu.sli_no_count(6usize, 14usize, 2u32, 2149288u32);
    emu.sltru_no_count(15usize, 0usize, 14usize, 2149292u32);
    emu.adi_no_count(17usize, 14usize, 1u32, 2149296u32);
    emu.adi_no_count(7usize, 14usize, 4294967295u32, 2149300u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2149304u32);
    emu.sli_no_count(15usize, 15usize, 2u32, 2149308u32);
    emu.sli_no_count(28usize, 7usize, 2u32, 2149312u32);
    emu.adr_no_count(7usize, 13usize, 15usize, 2149316u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2149320u32);
    emu.adi_no_count(28usize, 28usize, 1u32, 2149324u32);
    emu.adi_no_count(29usize, 0usize, 40u32, 2149328u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2149328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbd0));
}
#[inline(always)]
pub fn block_0x0020cbd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(9usize, 5usize, 2u32, 2149332u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2149336u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2149340u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbdc));
}
#[inline(always)]
pub fn block_0x0020cbdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccd8));
    } else {
        emu.pc = 2149344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbe0));
    }
}
#[inline(always)]
pub fn block_0x0020cbe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 5usize, 0u32, 2149348u32);
    emu.adi_no_count(31usize, 9usize, 0u32, 2149352u32);
    emu.lw_no_count(8usize, 15usize, 0u32, 2149356u32)?;
    emu.adi_no_count(5usize, 5usize, 1u32, 2149360u32);
    emu.adi_no_count(11usize, 15usize, 4u32, 2149364u32);
    emu.adi_no_count(9usize, 9usize, 4u32, 2149368u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2149372u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2149340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbdc));
    } else {
        emu.pc = 2149376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc00));
    }
}
#[inline(always)]
pub fn block_0x0020cc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2149380u32);
    emu.adi_no_count(9usize, 28usize, 0u32, 2149384u32);
    emu.adi_no_count(15usize, 30usize, 0u32, 2149388u32);
    emu.adi_no_count(20usize, 7usize, 0u32, 2149392u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2149396u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2149396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc14));
}
#[inline(always)]
pub fn block_0x0020cc14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2149628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccfc));
    } else {
        emu.pc = 2149400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc18));
    }
}
#[inline]
pub fn block_0x0020cc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 20usize, 0u32, 2149404u32);
    emu.lw_no_count(21usize, 21usize, 0u32, 2149408u32)?;
    emu.lw_no_count(20usize, 31usize, 0u32, 2149412u32)?;
    emu.mulhu_no_count(22usize, 21usize, 8usize, 2149416u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2149420u32);
    emu.sltru_no_count(20usize, 18usize, 20usize, 2149424u32);
    emu.adr_no_count(22usize, 20usize, 22usize, 2149428u32);
    emu.xrr_no_count(20usize, 19usize, 6usize, 2149432u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2149436u32);
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2149440u32);
    emu.sltru_no_count(20usize, 0usize, 20usize, 2149444u32);
    emu.sli_no_count(20usize, 20usize, 2u32, 2149448u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2149452u32);
    emu.mul_no_count(21usize, 21usize, 8usize, 2149456u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2149460u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2149464u32);
    emu.sw_no_count(21usize, 31usize, 0u32, 2149468u32)?;
    emu.adr_no_count(18usize, 22usize, 18usize, 2149472u32);
    emu.adi_no_count(31usize, 31usize, 4u32, 2149476u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2149480u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2149396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc14));
    } else {
        emu.pc = 2149484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc6c));
    }
}
#[inline(always)]
pub fn block_0x0020cc6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2149488u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2149516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc8c));
    } else {
        emu.pc = 2149492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc74));
    }
}
#[inline(always)]
pub fn block_0x0020cc74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 30usize, 14usize, 2149496u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2149628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccfc));
    } else {
        emu.pc = 2149500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc7c));
    }
}
#[inline(always)]
pub fn block_0x0020cc7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 2u32, 2149504u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2149508u32);
    emu.sw_no_count(18usize, 15usize, 0u32, 2149512u32)?;
    emu.adi_no_count(15usize, 17usize, 0u32, 2149516u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2149516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc8c));
}
#[inline(always)]
pub fn block_0x0020cc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 15usize, 30usize, 2149520u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2149328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbd0));
    } else {
        emu.pc = 2149524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc94));
    }
}
#[inline(always)]
pub fn block_0x0020cc94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 15usize, 0u32, 2149528u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149532u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbd0));
}
#[inline(always)]
pub fn block_0x0020cc9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2149536u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2149540u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149540u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cca4));
}
#[inline(always)]
pub fn block_0x0020cca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2149544u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2149548u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccac));
}
#[inline(always)]
pub fn block_0x0020ccac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2149592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccd8));
    } else {
        emu.pc = 2149552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccb0));
    }
}
#[inline(always)]
pub fn block_0x0020ccb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2149556u32)?;
    emu.adi_no_count(11usize, 14usize, 4u32, 2149560u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2149564u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2149568u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccac));
    } else {
        emu.pc = 2149572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccc4));
    }
}
#[inline(always)]
pub fn block_0x0020ccc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(14usize, 13usize, 4294967295u32, 2149576u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2149584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccd0));
    } else {
        emu.pc = 2149580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cccc));
    }
}
#[inline(always)]
pub fn block_0x0020cccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2149584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccd0));
}
#[inline(always)]
pub fn block_0x0020ccd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2149588u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cca4));
}
#[inline]
pub fn block_0x0020ccd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 28u32, 2149596u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2149600u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2149604u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2149608u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2149612u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2149616u32)?;
    emu.lw_no_count(22usize, 2usize, 4u32, 2149620u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149624u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149628u32;
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
pub fn block_0x0020ccfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149632u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2149636u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149640u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2149644u32);
    emu.apc_no_count(1usize, 2149644u32, 4294963200u32, 2149648u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020cd14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2149656u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2149660u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2149664u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2149668u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2149672u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2149676u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2149680u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2149684u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2149688u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2149692u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2149696u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2149700u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2149704u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2149708u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2149712u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2149716u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2149720u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2149724u32);
    emu.adi_no_count(25usize, 0usize, 0u32, 2149728u32);
    let a = 0u32.wrapping_add(168431616u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2149732u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149736u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 10usize, 0u32, 2149740u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2149744u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2149748u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2149752u32)?;
    emu.lw_no_count(22usize, 10usize, 8u32, 2149756u32)?;
    emu.adi_no_count(10usize, 9usize, 4294967295u32, 2149760u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2149764u32)?;
    emu.adi_no_count(10usize, 9usize, 4u32, 2149768u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2149772u32)?;
    emu.sbr_no_count(10usize, 0usize, 8usize, 2149776u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2149780u32)?;
    emu.adi_no_count(27usize, 11usize, 4294965770u32, 2149784u32);
    emu.adi_no_count(19usize, 12usize, 256u32, 2149788u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2149792u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149796u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 10usize, 128u32, 2149800u32);
    emu.add_memory_rw_events(38usize);
    let return_addr = 2149804u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149860u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cde4));
}
#[inline(always)]
pub fn block_0x0020cdac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2149808u32)?;
    emu.adr_no_count(10usize, 10usize, 26usize, 2149812u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2149816u32);
    emu.adi_no_count(10usize, 10usize, 4294967286u32, 2149820u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2149824u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2149824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cdc0));
}
#[inline(always)]
pub fn block_0x0020cdc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 22usize, 0u32, 2149828u32);
    emu.lw_no_count(10usize, 2usize, 20u32, 2149832u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2149836u32)?;
    emu.sbr_no_count(12usize, 26usize, 20usize, 2149840u32);
    emu.adr_no_count(11usize, 9usize, 20usize, 2149844u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2149848u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2149852u32;
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
pub fn block_0x0020cddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2149856u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf84));
    } else {
        emu.pc = 2149860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cde4));
    }
}
#[inline(always)]
pub fn block_0x0020cde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 25usize, 1u32, 2149864u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf7c));
    } else {
        emu.pc = 2149868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdec));
    }
}
#[inline(always)]
pub fn block_0x0020cdec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2149888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce00));
    } else {
        emu.pc = 2149872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdf0));
    }
}
#[inline(always)]
pub fn block_0x0020cdf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 21usize, 0u32, 2149876u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf34));
}
#[inline(always)]
pub fn block_0x0020cdf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 26usize, 0u32, 2149884u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2150196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf34));
    } else {
        emu.pc = 2149888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce00));
    }
}
#[inline(always)]
pub fn block_0x0020ce00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 8usize, 21usize, 2149892u32);
    emu.adr_no_count(10usize, 9usize, 21usize, 2149896u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2149900u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2149944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce38));
    } else {
        emu.pc = 2149904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce10));
    }
}
#[inline(always)]
pub fn block_0x0020ce10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2150192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf30));
    } else {
        emu.pc = 2149908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce14));
    }
}
#[inline(always)]
pub fn block_0x0020ce14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2149912u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2149916u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2149920u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149920u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce20));
}
#[inline(always)]
pub fn block_0x0020ce20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2149924u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2150116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cee4));
    } else {
        emu.pc = 2149928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce28));
    }
}
#[inline(always)]
pub fn block_0x0020ce28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2149932u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2149936u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2149920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce20));
    } else {
        emu.pc = 2149940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce34));
    }
}
#[inline(always)]
pub fn block_0x0020ce34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2149944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150192u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf30));
}
#[inline(always)]
pub fn block_0x0020ce38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 3u32, 2149948u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2149952u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2150036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce94));
    } else {
        emu.pc = 2149956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce44));
    }
}
#[inline(always)]
pub fn block_0x0020ce44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2149960u32);
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2149964u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce4c));
}
#[inline(always)]
pub fn block_0x0020ce4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 8u32, 2149968u32)?;
    emu.adr_no_count(14usize, 14usize, 21usize, 2149972u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce54));
}
#[inline]
pub fn block_0x0020ce54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2149976u32);
    emu.adr_no_count(16usize, 14usize, 12usize, 2149980u32);
    emu.lw_no_count(15usize, 15usize, 0u32, 2149984u32)?;
    emu.lw_no_count(16usize, 16usize, 0u32, 2149988u32)?;
    emu.xrr_no_count(17usize, 15usize, 27usize, 2149992u32);
    emu.xrr_no_count(16usize, 16usize, 27usize, 2149996u32);
    emu.sbr_no_count(17usize, 19usize, 17usize, 2150000u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2150004u32);
    emu.sbr_no_count(17usize, 19usize, 16usize, 2150008u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2150012u32);
    emu.anr_no_count(15usize, 15usize, 16usize, 2150016u32);
    emu.anr_no_count(15usize, 15usize, 23usize, 2150020u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2150072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceb8));
    } else {
        emu.pc = 2150024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce88));
    }
}
#[inline(always)]
pub fn block_0x0020ce88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 8u32, 2150028u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2149972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce54));
    } else {
        emu.pc = 2150032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce90));
    }
}
#[inline(always)]
pub fn block_0x0020ce90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150036u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150072u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ceb8));
}
#[inline(always)]
pub fn block_0x0020ce94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2150040u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2150044u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2150044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce9c));
}
#[inline(always)]
pub fn block_0x0020ce9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 10usize, 13usize, 2150048u32);
    emu.lbu_no_count(14usize, 14usize, 0u32, 2150052u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2150120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cee8));
    } else {
        emu.pc = 2150056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cea8));
    }
}
#[inline(always)]
pub fn block_0x0020cea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2150060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2150044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce9c));
    } else {
        emu.pc = 2150064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceb0));
    }
}
#[inline(always)]
pub fn block_0x0020ceb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2150068u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2149964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce4c));
    } else {
        emu.pc = 2150072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceb8));
    }
}
#[inline(always)]
pub fn block_0x0020ceb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2150192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf30));
    } else {
        emu.pc = 2150076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cebc));
    }
}
#[inline(always)]
pub fn block_0x0020cebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 12usize, 2150080u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2150084u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2150088u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2150092u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2150092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cecc));
}
#[inline(always)]
pub fn block_0x0020cecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2150096u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2150136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cef8));
    } else {
        emu.pc = 2150100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ced4));
    }
}
#[inline(always)]
pub fn block_0x0020ced4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2150104u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2150108u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2150092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cecc));
    } else {
        emu.pc = 2150112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cee0));
    }
}
#[inline(always)]
pub fn block_0x0020cee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150192u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf30));
}
#[inline(always)]
pub fn block_0x0020cee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 11usize, 2150120u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150120u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cee8));
}
#[inline(always)]
pub fn block_0x0020cee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 13usize, 2150124u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2150128u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2149880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdf8));
    } else {
        emu.pc = 2150132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cef4));
    }
}
#[inline(always)]
pub fn block_0x0020cef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf08));
}
#[inline(always)]
pub fn block_0x0020cef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2150140u32);
    emu.adr_no_count(10usize, 21usize, 13usize, 2150144u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2150148u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2149880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdf8));
    } else {
        emu.pc = 2150152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf08));
    }
}
#[inline(always)]
pub fn block_0x0020cf08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 9usize, 21usize, 2150156u32);
    emu.adr_no_count(13usize, 21usize, 13usize, 2150160u32);
    emu.lbu_no_count(10usize, 13usize, 0u32, 2150164u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdf8));
    } else {
        emu.pc = 2150168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf18));
    }
}
#[inline(always)]
pub fn block_0x0020cf18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2150172u32);
    emu.adi_no_count(18usize, 26usize, 0u32, 2150176u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2150180u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2150184u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf70));
    } else {
        emu.pc = 2150188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf2c));
    }
}
#[inline(always)]
pub fn block_0x0020cf2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150224u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf50));
}
#[inline(always)]
pub fn block_0x0020cf30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 8usize, 0u32, 2150196u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf34));
}
#[inline(always)]
pub fn block_0x0020cf34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2150268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf7c));
    } else {
        emu.pc = 2150200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf38));
    }
}
#[inline(always)]
pub fn block_0x0020cf38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 1u32, 2150204u32);
    emu.adi_no_count(18usize, 20usize, 0u32, 2150208u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2150212u32);
    emu.adi_no_count(26usize, 8usize, 0u32, 2150216u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2150220u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf70));
    } else {
        emu.pc = 2150224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf50));
    }
}
#[inline(always)]
pub fn block_0x0020cf50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2150228u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2150232u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2150236u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2150240u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150244u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2150248u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150252u32;
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
pub fn block_0x0020cf6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf84));
    } else {
        emu.pc = 2150256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf70));
    }
}
#[inline(always)]
pub fn block_0x0020cf70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2149804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdac));
    } else {
        emu.pc = 2150260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf74));
    }
}
#[inline(always)]
pub fn block_0x0020cf74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2150264u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150268u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cdc0));
}
#[inline(always)]
pub fn block_0x0020cf7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2150272u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150280u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf88));
}
#[inline(always)]
pub fn block_0x0020cf84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2150280u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150280u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf88));
}
#[inline]
pub fn block_0x0020cf88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2150284u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2150288u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2150292u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2150296u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2150300u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2150304u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2150308u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2150312u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2150316u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2150320u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2150324u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2150328u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2150332u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2150336u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150340u32;
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
pub fn block_0x0020cfc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2150344u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2150348u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2150352u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2150356u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2150360u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2150364u32)?;
    emu.lw_no_count(9usize, 10usize, 8u32, 2150368u32)?;
    emu.lbu_no_count(12usize, 9usize, 0u32, 2150372u32);
    emu.lw_no_count(8usize, 10usize, 0u32, 2150376u32)?;
    emu.lw_no_count(18usize, 10usize, 4u32, 2150380u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2150456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d038));
    } else {
        emu.pc = 2150384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cff0));
    }
}
#[inline(always)]
pub fn block_0x0020cff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 18usize, 12u32, 2150388u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2150392u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2150396u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2150400u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2150404u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2150408u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2150412u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2150416u32;
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
pub fn block_0x0020d010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2150420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d038));
    } else {
        emu.pc = 2150424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d018));
    }
}
#[inline(always)]
pub fn block_0x0020d018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2150428u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2150432u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2150436u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2150440u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2150444u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2150448u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2150452u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150456u32;
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
pub fn block_0x0020d038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 4294967286u32, 2150460u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2150464u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2150468u32);
    emu.lw_no_count(6usize, 18usize, 16u32, 2150472u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2150476u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2150480u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2150484u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2150488u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2150492u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2150496u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2150500u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2150504u32;
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
pub fn block_0x0020d068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2150508u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2150512u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2150516u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2150520u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2150524u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2150528u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2150532u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2150536u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2150540u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2150544u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2150548u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2150552u32);
    emu.adi_no_count(21usize, 0usize, 1u32, 2150556u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2150560u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0dc));
    } else {
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    }
}
#[inline]
pub fn block_0x0020d0a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(20usize, 8usize, 4u32, 2150568u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2150572u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2150576u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2150580u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2150584u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2150588u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2150592u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2150596u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2150600u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2150604u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2150608u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2150612u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2150616u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150620u32;
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
pub fn block_0x0020d0dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 14usize, 0u32, 2150624u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2150628u32);
    emu.lw_no_count(19usize, 8usize, 0u32, 2150632u32)?;
    emu.lbu_no_count(10usize, 8usize, 5u32, 2150636u32);
    emu.lbu_no_count(13usize, 19usize, 10u32, 2150640u32);
    emu.ani_no_count(13usize, 13usize, 128u32, 2150644u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2150676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d114));
    } else {
        emu.pc = 2150648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0f8));
    }
}
#[inline(always)]
pub fn block_0x0020d0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 10usize, 3u32, 2150652u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2150656u32);
    emu.adi_no_count(23usize, 12usize, 0u32, 2150660u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1f4));
    } else {
        emu.pc = 2150664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d108));
    }
}
#[inline(always)]
pub fn block_0x0020d108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150668u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1044u32, 2150672u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2150676u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150908u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d1fc));
}
#[inline(always)]
pub fn block_0x0020d114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d150));
    } else {
        emu.pc = 2150680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d118));
    }
}
#[inline]
pub fn block_0x0020d118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 19usize, 4u32, 2150684u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150688u32)?;
    emu.lw_no_count(14usize, 13usize, 12u32, 2150692u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2150696u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1051u32, 2150700u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2150704u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2150708u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2150712u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2150716u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2150720u32;
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
pub fn block_0x0020d140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2150724u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2150728u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2150732u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    } else {
        emu.pc = 2150736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d150));
    }
}
#[inline]
pub fn block_0x0020d150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2150740u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2150744u32);
    emu.lw_no_count(13usize, 19usize, 0u32, 2150748u32)?;
    emu.lw_no_count(14usize, 19usize, 4u32, 2150752u32)?;
    emu.lw_no_count(15usize, 19usize, 8u32, 2150756u32)?;
    emu.lw_no_count(16usize, 19usize, 12u32, 2150760u32)?;
    emu.adi_no_count(17usize, 2usize, 12u32, 2150764u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2150768u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2150772u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2150776u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2150780u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1020u32, 2150784u32);
    emu.sb_no_count(20usize, 2usize, 27u32, 2150788u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2150792u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2150796u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2150800u32)?;
    emu.sw_no_count(16usize, 2usize, 40u32, 2150804u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2150808u32);
    emu.apc_no_count(1usize, 2150808u32, 0u32, 2150812u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    } else {
        emu.pc = 2150820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1a4));
    }
}
#[inline(always)]
pub fn block_0x0020d1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150824u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1049u32, 2150828u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2150832u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2150836u32);
    emu.apc_no_count(1usize, 2150836u32, 0u32, 2150840u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    } else {
        emu.pc = 2150848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1c0));
    }
}
#[inline(always)]
pub fn block_0x0020d1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2150852u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2150856u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2150860u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2150864u32;
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
pub fn block_0x0020d1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    } else {
        emu.pc = 2150868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d4));
    }
}
#[inline(always)]
pub fn block_0x0020d1d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2150872u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2150876u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2150880u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150884u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1054u32, 2150888u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2150892u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150896u32;
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
pub fn block_0x0020d1f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151020u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d26c));
}
#[inline(always)]
pub fn block_0x0020d1f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150904u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1047u32, 2150908u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2150908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d1fc));
}
#[inline(always)]
pub fn block_0x0020d1fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 4u32, 2150912u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150916u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2150920u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2150924u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2150928u32;
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
pub fn block_0x0020d210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2150932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    } else {
        emu.pc = 2150936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d218));
    }
}
#[inline(always)]
pub fn block_0x0020d218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 0u32, 2150940u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2150944u32);
    emu.lw_no_count(13usize, 19usize, 4u32, 2150948u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150952u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2150956u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150960u32;
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
pub fn block_0x0020d230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2150964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    } else {
        emu.pc = 2150968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d238));
    }
}
#[inline(always)]
pub fn block_0x0020d238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 19usize, 4u32, 2150972u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150976u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2150980u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150984u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1049u32, 2150988u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2150992u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150996u32;
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
pub fn block_0x0020d254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2151000u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    } else {
        emu.pc = 2151004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d25c));
    }
}
#[inline(always)]
pub fn block_0x0020d25c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2151008u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2151012u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2151016u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2151020u32;
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
pub fn block_0x0020d26c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2151024u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2151028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150564u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d0a4));
}
