pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2132844u32;
pub const PC_MAX: u32 = 2135016u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 103usize] = [
        block_0x00208b6c,
        block_0x00208b80,
        block_0x00208b88,
        block_0x00208b9c,
        block_0x00208ba0,
        block_0x00208bac,
        block_0x00208bb4,
        block_0x00208bb8,
        block_0x00208bc0,
        block_0x00208bc4,
        block_0x00208bc8,
        block_0x00208bcc,
        block_0x00208be8,
        block_0x00208bf8,
        block_0x00208bfc,
        block_0x00208c04,
        block_0x00208c14,
        block_0x00208c28,
        block_0x00208c40,
        block_0x00208c48,
        block_0x00208c68,
        block_0x00208c7c,
        block_0x00208c84,
        block_0x00208c94,
        block_0x00208d1c,
        block_0x00208d28,
        block_0x00208d38,
        block_0x00208d44,
        block_0x00208d58,
        block_0x00208d60,
        block_0x00208d70,
        block_0x00208d74,
        block_0x00208da0,
        block_0x00208da8,
        block_0x00208dac,
        block_0x00208db4,
        block_0x00208dc0,
        block_0x00208dd0,
        block_0x00208de0,
        block_0x00208de8,
        block_0x00208e08,
        block_0x00208e24,
        block_0x00208e28,
        block_0x00208e44,
        block_0x00208e4c,
        block_0x00208e78,
        block_0x00208eb0,
        block_0x00208edc,
        block_0x00208f0c,
        block_0x00208f20,
        block_0x00208f48,
        block_0x00208f68,
        block_0x00208f74,
        block_0x00208f80,
        block_0x00208f88,
        block_0x00208f90,
        block_0x00208f98,
        block_0x00208fa0,
        block_0x00208fa8,
        block_0x00208fb8,
        block_0x00208fbc,
        block_0x00208fdc,
        block_0x00208ffc,
        block_0x00209000,
        block_0x00209020,
        block_0x00209044,
        block_0x0020906c,
        block_0x00209078,
        block_0x0020908c,
        block_0x00209090,
        block_0x002090a8,
        block_0x002090d8,
        block_0x002090e0,
        block_0x002090e8,
        block_0x00209100,
        block_0x00209104,
        block_0x0020911c,
        block_0x00209124,
        block_0x00209140,
        block_0x00209164,
        block_0x00209184,
        block_0x0020918c,
        block_0x002091a4,
        block_0x002091c4,
        block_0x002091dc,
        block_0x002091e4,
        block_0x002091fc,
        block_0x0020925c,
        block_0x00209274,
        block_0x0020928c,
        block_0x002092ac,
        block_0x002092b4,
        block_0x00209308,
        block_0x00209318,
        block_0x00209320,
        block_0x00209338,
        block_0x00209348,
        block_0x00209350,
        block_0x00209358,
        block_0x00209368,
        block_0x0020937c,
        block_0x00209394,
        block_0x002093e8,
    ];
    const IDX: [u16; 544usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 4u16,
        5u16, 0u16, 0u16, 6u16, 0u16, 7u16, 8u16, 0u16, 9u16, 10u16, 11u16, 12u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 16u16,
        0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 19u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16,
        0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 0u16,
        0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 31u16,
        32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16,
        34u16, 35u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 38u16, 0u16,
        0u16, 0u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16,
        0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16,
        0u16, 0u16, 54u16, 0u16, 55u16, 0u16, 56u16, 0u16, 57u16, 0u16, 58u16, 0u16,
        59u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 64u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16,
        68u16, 0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        73u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 81u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16,
        86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16,
        92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16,
        98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 101u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16,
    ];
    if pc < 2132844u32 || pc > 2135016u32 {
        return None;
    }
    let word_offset = ((pc - 2132844u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00208b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2132848u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2132852u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2132856u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2132860u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2133096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c68));
    } else {
        emu.pc = 2132864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b80));
    }
}
#[inline(always)]
pub fn block_0x00208b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 11usize, 12usize, 2132868u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2133096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c68));
    } else {
        emu.pc = 2132872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b88));
    }
}
#[inline(always)]
pub fn block_0x00208b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2132876u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2132880u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2132884u32)?;
    emu.sli_no_count(10usize, 13usize, 1u32, 2132888u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2132896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ba0));
    } else {
        emu.pc = 2132892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b9c));
    }
}
#[inline(always)]
pub fn block_0x00208b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2132896u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2132896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208ba0));
}
#[inline(always)]
pub fn block_0x00208ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1025u32, 2132900u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2132904u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2132920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bb8));
    } else {
        emu.pc = 2132908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bac));
    }
}
#[inline(always)]
pub fn block_0x00208bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2132912u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2132928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bc0));
    } else {
        emu.pc = 2132916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bb4));
    }
}
#[inline(always)]
pub fn block_0x00208bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2132920u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2132932u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208bc4));
}
#[inline(always)]
pub fn block_0x00208bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2132924u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2132932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bc4));
    } else {
        emu.pc = 2132928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bc0));
    }
}
#[inline(always)]
pub fn block_0x00208bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2132932u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2132932u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208bc4));
}
#[inline(always)]
pub fn block_0x00208bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2132940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bcc));
    } else {
        emu.pc = 2132936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bc8));
    }
}
#[inline(always)]
pub fn block_0x00208bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2132940u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2132940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208bcc));
}
#[inline(always)]
pub fn block_0x00208bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 15usize, 14usize, 2132944u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2132948u32);
    emu.sbr_no_count(11usize, 0usize, 15usize, 2132952u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2132956u32);
    emu.mulhu_no_count(12usize, 11usize, 9usize, 2132960u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2132964u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2132996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c04));
    } else {
        emu.pc = 2132968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208be8));
    }
}
#[inline(always)]
pub fn block_0x00208be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 11usize, 9usize, 2132972u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132976u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 11usize, 15usize, 2132980u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2133124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c84));
    } else {
        emu.pc = 2132984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bf8));
    }
}
#[inline(always)]
pub fn block_0x00208bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2133012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c14));
    } else {
        emu.pc = 2132988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bfc));
    }
}
#[inline(always)]
pub fn block_0x00208bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2132992u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2132996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208c28));
}
#[inline(always)]
pub fn block_0x00208c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2133000u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1176u32, 2133004u32);
    emu.apc_no_count(1usize, 2133004u32, 8192u32, 2133008u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2133016u32)?;
    emu.mul_no_count(11usize, 13usize, 14usize, 2133020u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2133024u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2133028u32)?;
    emu.adi_no_count(10usize, 15usize, 0u32, 2133032u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2133032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208c28));
}
#[inline(always)]
pub fn block_0x00208c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2133036u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2133040u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2133044u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2133048u32);
    emu.apc_no_count(1usize, 2133048u32, 0u32, 2133052u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2133060u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2133116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c7c));
    } else {
        emu.pc = 2133064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c48));
    }
}
#[inline(always)]
pub fn block_0x00208c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2133068u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2133072u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2133076u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2133080u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2133084u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2133088u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2133092u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133096u32;
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
pub fn block_0x00208c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2133100u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2133104u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1176u32, 2133108u32);
    emu.apc_no_count(1usize, 2133108u32, 8192u32, 2133112u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2133120u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2133124u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2133124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208c84));
}
#[inline(always)]
pub fn block_0x00208c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2133128u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1176u32, 2133132u32);
    emu.apc_no_count(1usize, 2133132u32, 8192u32, 2133136u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133140u32;
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
#[inline(never)]
pub fn block_0x00208c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 34u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2133144u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2133148u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2133152u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2133156u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2133160u32)?;
    emu.adi_no_count(13usize, 2usize, 28u32, 2133164u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133168u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965708u32, 2133172u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2133176u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294965756u32, 2133180u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2133184u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1196u32, 2133188u32);
    emu.lw_no_count(17usize, 12usize, 0u32, 2133192u32)?;
    emu.lw_no_count(5usize, 12usize, 4u32, 2133196u32)?;
    emu.adi_no_count(6usize, 12usize, 8u32, 2133200u32);
    emu.adi_no_count(12usize, 12usize, 12u32, 2133204u32);
    emu.sw_no_count(13usize, 2usize, 36u32, 2133208u32)?;
    emu.sw_no_count(14usize, 2usize, 40u32, 2133212u32)?;
    emu.sw_no_count(6usize, 2usize, 44u32, 2133216u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2133220u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2133224u32)?;
    emu.sw_no_count(15usize, 2usize, 56u32, 2133228u32)?;
    emu.adi_no_count(12usize, 0usize, 3u32, 2133232u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2133236u32)?;
    emu.sw_no_count(17usize, 2usize, 28u32, 2133240u32)?;
    emu.sw_no_count(5usize, 2usize, 32u32, 2133244u32)?;
    emu.adi_no_count(13usize, 2usize, 36u32, 2133248u32);
    emu.sw_no_count(16usize, 2usize, 4u32, 2133252u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2133256u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2133260u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2133264u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2133268u32);
    emu.apc_no_count(1usize, 2133268u32, 28672u32, 2133272u32);
    emu.add_memory_rw_events(34usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(44u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2133280u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2133284u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133288u32;
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
pub fn block_0x00208d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2133292u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2133296u32)?;
    emu.apc_no_count(1usize, 2133296u32, 0u32, 2133300u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2133308u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2133312u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133316u32;
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
pub fn block_0x00208d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2133320u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2133324u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1256u32, 2133328u32);
    emu.apc_no_count(6usize, 2133328u32, 28672u32, 2133332u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133336u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2133340u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2133360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d70));
    } else {
        emu.pc = 2133344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d60));
    }
}
#[inline(always)]
pub fn block_0x00208d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2133348u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2133352u32);
    emu.apc_no_count(6usize, 2133352u32, 4294938624u32, 2133356u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133360u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133364u32;
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
pub fn block_0x00208d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2133368u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2133372u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2133376u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2133380u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2133384u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2133388u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2133392u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2133396u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2133400u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2133404u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2133416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208da8));
    } else {
        emu.pc = 2133408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208da0));
    }
}
#[inline(always)]
pub fn block_0x00208da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2133412u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2133416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208dc0));
}
#[inline(always)]
pub fn block_0x00208da8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2133428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208db4));
    } else {
        emu.pc = 2133420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208dac));
    }
}
#[inline(always)]
pub fn block_0x00208dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2133424u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2133428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208dc0));
}
#[inline(always)]
pub fn block_0x00208db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2133432u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2133436u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2133440u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2133440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208dc0));
}
#[inline(always)]
pub fn block_0x00208dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2133444u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2133448u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2133452u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2133480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208de8));
    } else {
        emu.pc = 2133456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208dd0));
    }
}
#[inline(always)]
pub fn block_0x00208dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2133460u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2133464u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2133468u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2133540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e24));
    } else {
        emu.pc = 2133472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208de0));
    }
}
#[inline(always)]
pub fn block_0x00208de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2133476u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2133480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208eb0));
}
#[inline(always)]
pub fn block_0x00208de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2133484u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2133488u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2133492u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2133496u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2133500u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2133504u32);
    emu.apc_no_count(1usize, 2133504u32, 0u32, 2133508u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2133516u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2133520u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2133524u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2133528u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2133532u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2133536u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2133472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208de0));
    } else {
        emu.pc = 2133540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e24));
    }
}
#[inline(always)]
pub fn block_0x00208e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2133572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e44));
    } else {
        emu.pc = 2133544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e28));
    }
}
#[inline(always)]
pub fn block_0x00208e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2133548u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2133552u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2133556u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2133560u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2133564u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2133568u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2133572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208eb0));
}
#[inline(always)]
pub fn block_0x00208e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2133576u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2133624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e78));
    } else {
        emu.pc = 2133580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e4c));
    }
}
#[inline]
pub fn block_0x00208e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2133584u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2133588u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2133592u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2133596u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2133600u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2133604u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2133608u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2133612u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2133616u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2133620u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2133624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208eb0));
}
#[inline]
pub fn block_0x00208e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2133628u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2133632u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2133636u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2133640u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2133644u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2133648u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2133652u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2133656u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2133660u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2133664u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2133668u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2133672u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2133676u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2133680u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2133680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208eb0));
}
#[inline]
pub fn block_0x00208eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2133684u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2133688u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2133692u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2133696u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2133700u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2133704u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2133708u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2133712u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2133716u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2133720u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133724u32;
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
pub fn block_0x00208edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2133728u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2133732u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2133736u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2133740u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2133744u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2133748u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2133752u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2133756u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2133760u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2133764u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2133768u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2133832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f48));
    } else {
        emu.pc = 2133772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f0c));
    }
}
#[inline(always)]
pub fn block_0x00208f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2133776u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2133780u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2133784u32);
    emu.apc_no_count(1usize, 2133784u32, 4294942720u32, 2133788u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2133796u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2133800u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2133804u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2133808u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2133812u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2133816u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2133820u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2133824u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2133828u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133832u32;
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
pub fn block_0x00208f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2133836u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2133840u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2133844u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2133848u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2133852u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2133856u32);
    emu.apc_no_count(1usize, 2133856u32, 0u32, 2133860u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2133868u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2133872u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2133876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208f0c));
}
#[inline(always)]
pub fn block_0x00208f74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2133880u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2133884u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2133904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f90));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2133892u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2133912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f98));
    } else {
        emu.pc = 2133896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f88));
    }
}
#[inline(always)]
pub fn block_0x00208f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2133896u32, 12288u32, 2133900u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133904u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2133904u32, 12288u32, 2133908u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133912u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2133912u32, 12288u32, 2133916u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133920u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2133924u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2133944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fb8));
    } else {
        emu.pc = 2133928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fa8));
    }
}
#[inline(always)]
pub fn block_0x00208fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2133932u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2133936u32);
    emu.apc_no_count(6usize, 2133936u32, 4294934528u32, 2133940u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133944u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133948u32;
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
pub fn block_0x00208fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2133952u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2133956u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2133960u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2133964u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2133968u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2133972u32);
    emu.apc_no_count(6usize, 2133972u32, 32768u32, 2133976u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133980u32;
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
pub fn block_0x00208fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2133984u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2133988u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2133992u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2133996u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2134000u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2134004u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2134008u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2134300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020911c));
    } else {
        emu.pc = 2134012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ffc));
    }
}
#[inline(always)]
pub fn block_0x00208ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2134436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091a4));
    } else {
        emu.pc = 2134016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209000));
    }
}
#[inline(always)]
pub fn block_0x00209000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2134020u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2134024u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134028u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1280u32, 2134032u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2134036u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2134040u32);
    emu.apc_no_count(1usize, 2134040u32, 32768u32, 2134044u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134052u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 76u32, 2134056u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1284u32, 2134064u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2134068u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2134072u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2134076u32);
    emu.apc_no_count(1usize, 2134076u32, 16384u32, 2134080u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134084u32;
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
#[inline]
pub fn block_0x00209044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 41u32, 2134088u32);
    emu.sb_no_count(11usize, 2usize, 35u32, 2134092u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134096u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 84u32, 2134100u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134104u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1300u32, 2134108u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2134112u32);
    emu.adi_no_count(13usize, 2usize, 35u32, 2134116u32);
    emu.apc_no_count(1usize, 2134116u32, 16384u32, 2134120u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134124u32;
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
pub fn block_0x0020906c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2134128u32);
    emu.apc_no_count(1usize, 2134128u32, 4294938624u32, 2134132u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2134140u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2134144u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2134148u32);
    emu.apc_no_count(1usize, 2134148u32, 4294934528u32, 2134152u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020908c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2134644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209274));
    } else {
        emu.pc = 2134160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209090));
    }
}
#[inline(always)]
pub fn block_0x00209090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2134164u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134168u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1236u32, 2134172u32);
    emu.adi_no_count(12usize, 0usize, 20u32, 2134176u32);
    emu.apc_no_count(1usize, 2134176u32, 4294942720u32, 2134180u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134184u32;
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
pub fn block_0x002090a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 36u32, 2134188u32)?;
    emu.sw_no_count(9usize, 2usize, 40u32, 2134192u32)?;
    emu.sw_no_count(18usize, 2usize, 44u32, 2134196u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134200u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1332u32, 2134204u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134208u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1316u32, 2134212u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2134216u32);
    emu.adi_no_count(13usize, 2usize, 36u32, 2134220u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2134224u32);
    emu.apc_no_count(1usize, 2134224u32, 16384u32, 2134228u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002090d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2134232u32, 16384u32, 2134236u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134240u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002090e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 36u32, 2134244u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209104));
    } else {
        emu.pc = 2134248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002090e8));
    }
}
#[inline(always)]
pub fn block_0x002090e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 40u32, 2134252u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2134256u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2134260u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2134264u32);
    emu.apc_no_count(1usize, 2134264u32, 4294934528u32, 2134268u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134272u32;
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
pub fn block_0x00209100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2134276u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2134276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209104));
}
#[inline(always)]
pub fn block_0x00209104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2134280u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2134284u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2134288u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2134292u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2134296u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134300u32;
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
pub fn block_0x0020911c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 2u32, 2134304u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2134524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091fc));
    } else {
        emu.pc = 2134308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209124));
    }
}
#[inline(always)]
pub fn block_0x00209124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 10usize, 4u32, 2134312u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134316u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1339u32, 2134320u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2134324u32);
    emu.adi_no_count(13usize, 0usize, 5u32, 2134328u32);
    emu.apc_no_count(1usize, 2134328u32, 32768u32, 2134332u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134336u32;
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
#[inline]
pub fn block_0x00209140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 8usize, 8u32, 2134340u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134344u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 84u32, 2134348u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134352u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1300u32, 2134356u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2134360u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2134364u32);
    emu.apc_no_count(1usize, 2134364u32, 16384u32, 2134368u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134376u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1332u32, 2134380u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134384u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1344u32, 2134388u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2134392u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2134396u32);
    emu.apc_no_count(1usize, 2134396u32, 16384u32, 2134400u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134404u32;
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
pub fn block_0x00209184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2134404u32, 16384u32, 2134408u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020918c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2134416u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2134420u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2134424u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2134428u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2134432u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134436u32;
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
pub fn block_0x002091a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 1u32, 2134440u32);
    emu.sb_no_count(10usize, 2usize, 24u32, 2134444u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134448u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 80u32, 2134452u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2134456u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2134460u32);
    emu.apc_no_count(1usize, 2134460u32, 32768u32, 2134464u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002091c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134472u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1300u32, 2134476u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2134480u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2134484u32);
    emu.apc_no_count(1usize, 2134484u32, 16384u32, 2134488u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002091dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2134492u32, 16384u32, 2134496u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(36u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002091e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2134504u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2134508u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2134512u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2134516u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2134520u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134524u32;
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
pub fn block_0x002091fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2134528u32)?;
    emu.adi_no_count(15usize, 10usize, 8u32, 2134532u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2134536u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134540u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1360u32, 2134544u32);
    emu.adi_no_count(6usize, 2usize, 36u32, 2134548u32);
    emu.adi_no_count(7usize, 0usize, 5u32, 2134552u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2134556u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 1376u32, 2134560u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2134564u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 84u32, 2134568u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2134572u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1300u32, 2134576u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2134580u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1382u32, 2134584u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2134588u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2134592u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2134596u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2134600u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2134604u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2134608u32);
    emu.adi_no_count(11usize, 5usize, 0u32, 2134612u32);
    emu.apc_no_count(1usize, 2134612u32, 32768u32, 2134616u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020925c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2134624u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2134628u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2134632u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2134636u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2134640u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134644u32;
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
pub fn block_0x00209274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134648u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1220u32, 2134652u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2134656u32);
    emu.adi_no_count(11usize, 0usize, 20u32, 2134660u32);
    emu.apc_no_count(1usize, 2134660u32, 8192u32, 2134664u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134668u32;
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
pub fn block_0x0020928c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2134672u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2134676u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2134680u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2134684u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2134688u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2134692u32);
    emu.apc_no_count(1usize, 2134692u32, 4294938624u32, 2134696u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002092ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2134704u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2134932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209394));
    } else {
        emu.pc = 2134708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002092b4));
    }
}
#[inline]
pub fn block_0x002092b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 40u32, 2134712u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2134716u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134720u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2134724u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134728u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1424u32, 2134732u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2134736u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2134740u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2134744u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2134748u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2134752u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2134756u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2134760u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2134764u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2134768u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2134772u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2134776u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2134780u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2134784u32);
    emu.apc_no_count(1usize, 2134784u32, 4294963200u32, 2134788u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 0u32, 2134796u32);
    emu.lw_no_count(8usize, 2usize, 4u32, 2134800u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2134804u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2134840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209338));
    } else {
        emu.pc = 2134808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209318));
    }
}
#[inline(always)]
pub fn block_0x00209318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2134812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2134840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209338));
    } else {
        emu.pc = 2134816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209320));
    }
}
#[inline(always)]
pub fn block_0x00209320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2134820u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2134824u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2134828u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2134832u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2134836u32);
    emu.add_memory_rw_events(6usize);
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
#[inline(always)]
pub fn block_0x00209338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2134844u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2134848u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2134852u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209350));
    } else {
        emu.pc = 2134856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209348));
    }
}
#[inline(always)]
pub fn block_0x00209348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2134860u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2134864u32;
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
pub fn block_0x00209350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2134868u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209368));
    } else {
        emu.pc = 2134872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209358));
    }
}
#[inline(always)]
pub fn block_0x00209358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2134876u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2134880u32);
    emu.apc_no_count(1usize, 2134880u32, 4294934528u32, 2134884u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2134892u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2134896u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2134900u32);
    emu.apc_no_count(1usize, 2134900u32, 4294934528u32, 2134904u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134908u32;
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
pub fn block_0x0020937c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2134912u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2134916u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2134920u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2134924u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2134928u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134932u32;
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
pub fn block_0x00209394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 0u32, 2134936u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2134940u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134944u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2134948u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2134952u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1456u32, 2134956u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2134960u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2134964u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2134968u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2134972u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2134976u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2134980u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2134984u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2134988u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2134992u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2134996u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135000u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1472u32, 2135004u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2135008u32);
    emu.apc_no_count(1usize, 2135008u32, 12288u32, 2135012u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135016u32;
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
#[inline]
pub fn block_0x002093e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 0u32, 2135020u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135024u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1976u32, 2135028u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135032u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965344u32, 2135036u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2135040u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2135044u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2135048u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2135052u32)?;
    emu.lw_no_count(13usize, 10usize, 0u32, 2135056u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2135060u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2135064u32);
    emu.apc_no_count(6usize, 2135064u32, 28672u32, 2135068u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135072u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
