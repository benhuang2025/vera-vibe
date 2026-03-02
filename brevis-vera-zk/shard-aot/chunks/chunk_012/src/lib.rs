pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2148112u32;
pub const PC_MAX: u32 = 2150372u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 122usize] = [
        block_0x0020c710,
        block_0x0020c738,
        block_0x0020c744,
        block_0x0020c74c,
        block_0x0020c774,
        block_0x0020c778,
        block_0x0020c794,
        block_0x0020c7a4,
        block_0x0020c7b8,
        block_0x0020c7bc,
        block_0x0020c7c8,
        block_0x0020c7d4,
        block_0x0020c7f4,
        block_0x0020c7fc,
        block_0x0020c808,
        block_0x0020c810,
        block_0x0020c81c,
        block_0x0020c844,
        block_0x0020c858,
        block_0x0020c85c,
        block_0x0020c880,
        block_0x0020c89c,
        block_0x0020c8b4,
        block_0x0020c8cc,
        block_0x0020c8e4,
        block_0x0020c910,
        block_0x0020c948,
        block_0x0020c954,
        block_0x0020c958,
        block_0x0020c978,
        block_0x0020c98c,
        block_0x0020c990,
        block_0x0020c9e4,
        block_0x0020c9ec,
        block_0x0020c9f4,
        block_0x0020ca04,
        block_0x0020ca0c,
        block_0x0020ca14,
        block_0x0020ca1c,
        block_0x0020ca24,
        block_0x0020ca28,
        block_0x0020ca3c,
        block_0x0020ca44,
        block_0x0020ca48,
        block_0x0020ca50,
        block_0x0020ca74,
        block_0x0020ca8c,
        block_0x0020cb24,
        block_0x0020cb38,
        block_0x0020cb54,
        block_0x0020cb5c,
        block_0x0020cb64,
        block_0x0020cb68,
        block_0x0020cb70,
        block_0x0020cb78,
        block_0x0020cb88,
        block_0x0020cb8c,
        block_0x0020cb98,
        block_0x0020cba0,
        block_0x0020cbac,
        block_0x0020cbb0,
        block_0x0020cbbc,
        block_0x0020cbc4,
        block_0x0020cbcc,
        block_0x0020cc00,
        block_0x0020cc08,
        block_0x0020cc0c,
        block_0x0020cc14,
        block_0x0020cc20,
        block_0x0020cc28,
        block_0x0020cc30,
        block_0x0020cc34,
        block_0x0020cc44,
        block_0x0020cc4c,
        block_0x0020cc58,
        block_0x0020cc5c,
        block_0x0020cc60,
        block_0x0020cc6c,
        block_0x0020cc70,
        block_0x0020cc80,
        block_0x0020cc90,
        block_0x0020cca4,
        block_0x0020cca8,
        block_0x0020ccac,
        block_0x0020ccb0,
        block_0x0020ccc8,
        block_0x0020cce4,
        block_0x0020cce8,
        block_0x0020ccec,
        block_0x0020ccf4,
        block_0x0020ccfc,
        block_0x0020cd00,
        block_0x0020cd3c,
        block_0x0020cd68,
        block_0x0020cd88,
        block_0x0020cd90,
        block_0x0020cdb0,
        block_0x0020cde0,
        block_0x0020ce1c,
        block_0x0020ce54,
        block_0x0020ce70,
        block_0x0020ce80,
        block_0x0020ce8c,
        block_0x0020ce90,
        block_0x0020ceb8,
        block_0x0020cec8,
        block_0x0020cf18,
        block_0x0020cf1c,
        block_0x0020cf34,
        block_0x0020cf38,
        block_0x0020cf48,
        block_0x0020cf4c,
        block_0x0020cf68,
        block_0x0020cf6c,
        block_0x0020cf74,
        block_0x0020cf88,
        block_0x0020cf90,
        block_0x0020cfa8,
        block_0x0020cfb0,
        block_0x0020cfcc,
        block_0x0020cfd4,
        block_0x0020cfe4,
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
    if pc < 2148112u32 || pc > 2150372u32 {
        return None;
    }
    let word_offset = ((pc - 2148112u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020c710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2148116u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2148120u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2148124u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2148128u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2148132u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2148136u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2148140u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2148144u32);
    emu.adi_no_count(11usize, 0usize, 1280u32, 2148148u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2148480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c880));
    } else {
        emu.pc = 2148152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c738));
    }
}
#[inline(always)]
pub fn block_0x0020c738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 160u32, 2148156u32)?;
    emu.sri_no_count(19usize, 8usize, 5u32, 2148160u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2148244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c794));
    } else {
        emu.pc = 2148164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c744));
    }
}
#[inline(always)]
pub fn block_0x0020c744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2148168u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2148532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8b4));
    } else {
        emu.pc = 2148172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c74c));
    }
}
#[inline]
pub fn block_0x0020c74c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2148176u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2148180u32);
    emu.adr_no_count(12usize, 12usize, 19usize, 2148184u32);
    emu.adr_no_count(14usize, 13usize, 10usize, 2148188u32);
    emu.adi_no_count(13usize, 12usize, 4294967295u32, 2148192u32);
    emu.sli_no_count(15usize, 12usize, 2u32, 2148196u32);
    emu.adi_no_count(12usize, 14usize, 4294967292u32, 2148200u32);
    emu.adr_no_count(14usize, 15usize, 10usize, 2148204u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2148208u32);
    emu.adi_no_count(15usize, 0usize, 39u32, 2148212u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2148212u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c774));
}
#[inline(always)]
pub fn block_0x0020c774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c89c));
    } else {
        emu.pc = 2148216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c778));
    }
}
#[inline(always)]
pub fn block_0x0020c778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 12usize, 0u32, 2148220u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2148224u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2148228u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2148232u32);
    emu.sw_no_count(16usize, 14usize, 0u32, 2148236u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2148240u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2148212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c774));
    } else {
        emu.pc = 2148244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c794));
    }
}
#[inline(always)]
pub fn block_0x0020c794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 8usize, 31u32, 2148248u32);
    emu.adi_no_count(11usize, 0usize, 32u32, 2148252u32);
    emu.sli_no_count(9usize, 19usize, 2u32, 2148256u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2148284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7bc));
    } else {
        emu.pc = 2148260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7a4));
    }
}
#[inline(always)]
pub fn block_0x0020c7a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2148264u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2148268u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2148272u32);
    emu.apc_no_count(1usize, 2148272u32, 4294926336u32, 2148276u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c7b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2148284u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c7bc));
}
#[inline(always)]
pub fn block_0x0020c7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 160u32, 2148288u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2148292u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2148440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c858));
    } else {
        emu.pc = 2148296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7c8));
    }
}
#[inline(always)]
pub fn block_0x0020c7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2148300u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2148304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2148508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c89c));
    } else {
        emu.pc = 2148308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7d4));
    }
}
#[inline(always)]
pub fn block_0x0020c7d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2148312u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2148316u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2148320u32)?;
    emu.sbr_no_count(12usize, 0usize, 8usize, 2148324u32);
    emu.srr_no_count(15usize, 11usize, 12usize, 2148328u32);
    emu.sli_no_count(13usize, 14usize, 2u32, 2148332u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2148336u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2148360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c808));
    } else {
        emu.pc = 2148340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7f4));
    }
}
#[inline(always)]
pub fn block_0x0020c7f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2148344u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2148556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8cc));
    } else {
        emu.pc = 2148348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7fc));
    }
}
#[inline(always)]
pub fn block_0x0020c7fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 13usize, 2148352u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2148356u32)?;
    emu.adi_no_count(11usize, 14usize, 1u32, 2148360u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2148360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c808));
}
#[inline(always)]
pub fn block_0x0020c808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2148364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2148420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c844));
    } else {
        emu.pc = 2148368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c810));
    }
}
#[inline(always)]
pub fn block_0x0020c810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 31u32, 2148372u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2148376u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2148380u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2148380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c81c));
}
#[inline]
pub fn block_0x0020c81c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 13usize, 0u32, 2148384u32)?;
    emu.lw_no_count(16usize, 13usize, 4294967292u32, 2148388u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2148392u32);
    emu.slr_no_count(15usize, 15usize, 20usize, 2148396u32);
    emu.srr_no_count(16usize, 16usize, 12usize, 2148400u32);
    emu.adi_no_count(17usize, 13usize, 4294967292u32, 2148404u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2148408u32);
    emu.sw_no_count(15usize, 13usize, 0u32, 2148412u32)?;
    emu.adi_no_count(13usize, 17usize, 0u32, 2148416u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2148380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c81c));
    } else {
        emu.pc = 2148420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c844));
    }
}
#[inline(always)]
pub fn block_0x0020c844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 10usize, 9usize, 2148424u32);
    emu.lw_no_count(12usize, 9usize, 0u32, 2148428u32)?;
    emu.slr_no_count(12usize, 12usize, 20usize, 2148432u32);
    emu.sw_no_count(12usize, 9usize, 0u32, 2148436u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2148440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148444u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c85c));
}
#[inline(always)]
pub fn block_0x0020c858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 0u32, 2148444u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c85c));
}
#[inline]
pub fn block_0x0020c85c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 160u32, 2148448u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2148452u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2148456u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2148460u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2148464u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2148468u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2148472u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2148476u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148480u32;
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
pub fn block_0x0020c880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2148484u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 162u32, 2148488u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2148492u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 120u32, 2148496u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2148500u32);
    emu.apc_no_count(1usize, 2148500u32, 0u32, 2148504u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148508u32;
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
pub fn block_0x0020c89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2148512u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 120u32, 2148516u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2148520u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2148524u32);
    emu.apc_no_count(1usize, 2148524u32, 0u32, 2148528u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148532u32;
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
pub fn block_0x0020c8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 4294967295u32, 2148536u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2148540u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 120u32, 2148544u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2148548u32);
    emu.apc_no_count(1usize, 2148548u32, 0u32, 2148552u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148556u32;
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
pub fn block_0x0020c8cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2148560u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 120u32, 2148564u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2148568u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2148572u32);
    emu.apc_no_count(1usize, 2148572u32, 0u32, 2148576u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148580u32;
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
pub fn block_0x0020c8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2148584u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2148588u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2148592u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2148596u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2148600u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2148604u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2148608u32)?;
    emu.sw_no_count(22usize, 2usize, 4u32, 2148612u32)?;
    emu.sli_no_count(12usize, 12usize, 2u32, 2148616u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2148620u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2148884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca14));
    } else {
        emu.pc = 2148624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c910));
    }
}
#[inline]
pub fn block_0x0020c910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 10usize, 0u32, 2148628u32);
    emu.adi_no_count(5usize, 0usize, 0u32, 2148632u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2148636u32);
    emu.sli_no_count(6usize, 14usize, 2u32, 2148640u32);
    emu.sltru_no_count(15usize, 0usize, 14usize, 2148644u32);
    emu.adi_no_count(17usize, 14usize, 1u32, 2148648u32);
    emu.adi_no_count(7usize, 14usize, 4294967295u32, 2148652u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2148656u32);
    emu.sli_no_count(15usize, 15usize, 2u32, 2148660u32);
    emu.sli_no_count(28usize, 7usize, 2u32, 2148664u32);
    emu.adr_no_count(7usize, 13usize, 15usize, 2148668u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2148672u32);
    emu.adi_no_count(28usize, 28usize, 1u32, 2148676u32);
    emu.adi_no_count(29usize, 0usize, 40u32, 2148680u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2148680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c948));
}
#[inline(always)]
pub fn block_0x0020c948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(9usize, 5usize, 2u32, 2148684u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2148688u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2148692u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2148692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c954));
}
#[inline(always)]
pub fn block_0x0020c954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca50));
    } else {
        emu.pc = 2148696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c958));
    }
}
#[inline(always)]
pub fn block_0x0020c958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 5usize, 0u32, 2148700u32);
    emu.adi_no_count(31usize, 9usize, 0u32, 2148704u32);
    emu.lw_no_count(8usize, 15usize, 0u32, 2148708u32)?;
    emu.adi_no_count(5usize, 5usize, 1u32, 2148712u32);
    emu.adi_no_count(11usize, 15usize, 4u32, 2148716u32);
    emu.adi_no_count(9usize, 9usize, 4u32, 2148720u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2148724u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2148692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c954));
    } else {
        emu.pc = 2148728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c978));
    }
}
#[inline(always)]
pub fn block_0x0020c978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2148732u32);
    emu.adi_no_count(9usize, 28usize, 0u32, 2148736u32);
    emu.adi_no_count(15usize, 30usize, 0u32, 2148740u32);
    emu.adi_no_count(20usize, 7usize, 0u32, 2148744u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2148748u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2148748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c98c));
}
#[inline(always)]
pub fn block_0x0020c98c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca74));
    } else {
        emu.pc = 2148752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c990));
    }
}
#[inline]
pub fn block_0x0020c990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 20usize, 0u32, 2148756u32);
    emu.lw_no_count(21usize, 21usize, 0u32, 2148760u32)?;
    emu.lw_no_count(20usize, 31usize, 0u32, 2148764u32)?;
    emu.mulhu_no_count(22usize, 21usize, 8usize, 2148768u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2148772u32);
    emu.sltru_no_count(20usize, 18usize, 20usize, 2148776u32);
    emu.adr_no_count(22usize, 20usize, 22usize, 2148780u32);
    emu.xrr_no_count(20usize, 19usize, 6usize, 2148784u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2148788u32);
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2148792u32);
    emu.sltru_no_count(20usize, 0usize, 20usize, 2148796u32);
    emu.sli_no_count(20usize, 20usize, 2u32, 2148800u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2148804u32);
    emu.mul_no_count(21usize, 21usize, 8usize, 2148808u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2148812u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2148816u32);
    emu.sw_no_count(21usize, 31usize, 0u32, 2148820u32)?;
    emu.adr_no_count(18usize, 22usize, 18usize, 2148824u32);
    emu.adi_no_count(31usize, 31usize, 4u32, 2148828u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2148832u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2148748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c98c));
    } else {
        emu.pc = 2148836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9e4));
    }
}
#[inline(always)]
pub fn block_0x0020c9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2148840u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2148868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca04));
    } else {
        emu.pc = 2148844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9ec));
    }
}
#[inline(always)]
pub fn block_0x0020c9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 30usize, 14usize, 2148848u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2148980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca74));
    } else {
        emu.pc = 2148852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9f4));
    }
}
#[inline(always)]
pub fn block_0x0020c9f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 2u32, 2148856u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2148860u32);
    emu.sw_no_count(18usize, 15usize, 0u32, 2148864u32)?;
    emu.adi_no_count(15usize, 17usize, 0u32, 2148868u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2148868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca04));
}
#[inline(always)]
pub fn block_0x0020ca04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 15usize, 30usize, 2148872u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2148680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c948));
    } else {
        emu.pc = 2148876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca0c));
    }
}
#[inline(always)]
pub fn block_0x0020ca0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 15usize, 0u32, 2148880u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2148884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c948));
}
#[inline(always)]
pub fn block_0x0020ca14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2148888u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2148892u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2148892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca1c));
}
#[inline(always)]
pub fn block_0x0020ca1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2148896u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2148900u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2148900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca24));
}
#[inline(always)]
pub fn block_0x0020ca24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca50));
    } else {
        emu.pc = 2148904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca28));
    }
}
#[inline(always)]
pub fn block_0x0020ca28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2148908u32)?;
    emu.adi_no_count(11usize, 14usize, 4u32, 2148912u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2148916u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2148920u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2148900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca24));
    } else {
        emu.pc = 2148924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca3c));
    }
}
#[inline(always)]
pub fn block_0x0020ca3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(14usize, 13usize, 4294967295u32, 2148928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2148936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca48));
    } else {
        emu.pc = 2148932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca44));
    }
}
#[inline(always)]
pub fn block_0x0020ca44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2148936u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca48));
}
#[inline(always)]
pub fn block_0x0020ca48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2148940u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2148944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca1c));
}
#[inline]
pub fn block_0x0020ca50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 28u32, 2148948u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2148952u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2148956u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2148960u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2148964u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2148968u32)?;
    emu.lw_no_count(22usize, 2usize, 4u32, 2148972u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2148976u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148980u32;
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
pub fn block_0x0020ca74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2148984u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 120u32, 2148988u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2148992u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2148996u32);
    emu.apc_no_count(1usize, 2148996u32, 4294963200u32, 2149000u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149004u32;
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
pub fn block_0x0020ca8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2149008u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2149012u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2149016u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2149020u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2149024u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2149028u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2149032u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2149036u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2149040u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2149044u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2149048u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2149052u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2149056u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2149060u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2149064u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2149068u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2149072u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2149076u32);
    emu.adi_no_count(25usize, 0usize, 0u32, 2149080u32);
    let a = 0u32.wrapping_add(168431616u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2149084u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149088u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 10usize, 0u32, 2149092u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2149096u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2149100u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2149104u32)?;
    emu.lw_no_count(22usize, 10usize, 8u32, 2149108u32)?;
    emu.adi_no_count(10usize, 9usize, 4294967295u32, 2149112u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2149116u32)?;
    emu.adi_no_count(10usize, 9usize, 4u32, 2149120u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2149124u32)?;
    emu.sbr_no_count(10usize, 0usize, 8usize, 2149128u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2149132u32)?;
    emu.adi_no_count(27usize, 11usize, 4294965770u32, 2149136u32);
    emu.adi_no_count(19usize, 12usize, 256u32, 2149140u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2149144u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149148u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 10usize, 128u32, 2149152u32);
    emu.add_memory_rw_events(38usize);
    let return_addr = 2149156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149212u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb5c));
}
#[inline(always)]
pub fn block_0x0020cb24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2149160u32)?;
    emu.adr_no_count(10usize, 10usize, 26usize, 2149164u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2149168u32);
    emu.adi_no_count(10usize, 10usize, 4294967286u32, 2149172u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2149176u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2149176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb38));
}
#[inline(always)]
pub fn block_0x0020cb38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 22usize, 0u32, 2149180u32);
    emu.lw_no_count(10usize, 2usize, 20u32, 2149184u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2149188u32)?;
    emu.sbr_no_count(12usize, 26usize, 20usize, 2149192u32);
    emu.adr_no_count(11usize, 9usize, 20usize, 2149196u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2149200u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2149204u32;
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
pub fn block_0x0020cb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2149208u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccfc));
    } else {
        emu.pc = 2149212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb5c));
    }
}
#[inline(always)]
pub fn block_0x0020cb5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 25usize, 1u32, 2149216u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccf4));
    } else {
        emu.pc = 2149220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb64));
    }
}
#[inline(always)]
pub fn block_0x0020cb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb78));
    } else {
        emu.pc = 2149224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb68));
    }
}
#[inline(always)]
pub fn block_0x0020cb68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 21usize, 0u32, 2149228u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149232u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccac));
}
#[inline(always)]
pub fn block_0x0020cb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 26usize, 0u32, 2149236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2149548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccac));
    } else {
        emu.pc = 2149240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb78));
    }
}
#[inline(always)]
pub fn block_0x0020cb78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 8usize, 21usize, 2149244u32);
    emu.adr_no_count(10usize, 9usize, 21usize, 2149248u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2149252u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2149296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbb0));
    } else {
        emu.pc = 2149256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb88));
    }
}
#[inline(always)]
pub fn block_0x0020cb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cca8));
    } else {
        emu.pc = 2149260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb8c));
    }
}
#[inline(always)]
pub fn block_0x0020cb8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2149264u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2149268u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2149272u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb98));
}
#[inline(always)]
pub fn block_0x0020cb98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2149276u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2149468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc5c));
    } else {
        emu.pc = 2149280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cba0));
    }
}
#[inline(always)]
pub fn block_0x0020cba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2149284u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2149288u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2149272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb98));
    } else {
        emu.pc = 2149292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbac));
    }
}
#[inline(always)]
pub fn block_0x0020cbac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2149296u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cca8));
}
#[inline(always)]
pub fn block_0x0020cbb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 3u32, 2149300u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2149304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2149388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc0c));
    } else {
        emu.pc = 2149308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbbc));
    }
}
#[inline(always)]
pub fn block_0x0020cbbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2149312u32);
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2149316u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149316u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbc4));
}
#[inline(always)]
pub fn block_0x0020cbc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 8u32, 2149320u32)?;
    emu.adr_no_count(14usize, 14usize, 21usize, 2149324u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbcc));
}
#[inline]
pub fn block_0x0020cbcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2149328u32);
    emu.adr_no_count(16usize, 14usize, 12usize, 2149332u32);
    emu.lw_no_count(15usize, 15usize, 0u32, 2149336u32)?;
    emu.lw_no_count(16usize, 16usize, 0u32, 2149340u32)?;
    emu.xrr_no_count(17usize, 15usize, 27usize, 2149344u32);
    emu.xrr_no_count(16usize, 16usize, 27usize, 2149348u32);
    emu.sbr_no_count(17usize, 19usize, 17usize, 2149352u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2149356u32);
    emu.sbr_no_count(17usize, 19usize, 16usize, 2149360u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2149364u32);
    emu.anr_no_count(15usize, 15usize, 16usize, 2149368u32);
    emu.anr_no_count(15usize, 15usize, 23usize, 2149372u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2149424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc30));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 8u32, 2149380u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2149324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbcc));
    } else {
        emu.pc = 2149384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc08));
    }
}
#[inline(always)]
pub fn block_0x0020cc08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2149388u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc30));
}
#[inline(always)]
pub fn block_0x0020cc0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2149392u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2149396u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc14));
}
#[inline(always)]
pub fn block_0x0020cc14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 10usize, 13usize, 2149400u32);
    emu.lbu_no_count(14usize, 14usize, 0u32, 2149404u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2149472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc60));
    } else {
        emu.pc = 2149408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc20));
    }
}
#[inline(always)]
pub fn block_0x0020cc20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2149412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2149396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc14));
    } else {
        emu.pc = 2149416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc28));
    }
}
#[inline(always)]
pub fn block_0x0020cc28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2149420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2149316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbc4));
    } else {
        emu.pc = 2149424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc30));
    }
}
#[inline(always)]
pub fn block_0x0020cc30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cca8));
    } else {
        emu.pc = 2149428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc34));
    }
}
#[inline(always)]
pub fn block_0x0020cc34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 12usize, 2149432u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2149436u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2149440u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2149444u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2149444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc44));
}
#[inline(always)]
pub fn block_0x0020cc44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2149448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2149488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc70));
    } else {
        emu.pc = 2149452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc4c));
    }
}
#[inline(always)]
pub fn block_0x0020cc4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149456u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2149460u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2149444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc44));
    } else {
        emu.pc = 2149464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc58));
    }
}
#[inline(always)]
pub fn block_0x0020cc58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2149468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cca8));
}
#[inline(always)]
pub fn block_0x0020cc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 11usize, 2149472u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149472u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc60));
}
#[inline(always)]
pub fn block_0x0020cc60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 13usize, 2149476u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2149480u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2149232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb70));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2149488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc80));
}
#[inline(always)]
pub fn block_0x0020cc70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2149492u32);
    emu.adr_no_count(10usize, 21usize, 13usize, 2149496u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2149500u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2149232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb70));
    } else {
        emu.pc = 2149504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc80));
    }
}
#[inline(always)]
pub fn block_0x0020cc80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 9usize, 21usize, 2149508u32);
    emu.adr_no_count(13usize, 21usize, 13usize, 2149512u32);
    emu.lbu_no_count(10usize, 13usize, 0u32, 2149516u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb70));
    } else {
        emu.pc = 2149520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc90));
    }
}
#[inline(always)]
pub fn block_0x0020cc90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2149524u32);
    emu.adi_no_count(18usize, 26usize, 0u32, 2149528u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2149532u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2149536u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2149608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cce8));
    } else {
        emu.pc = 2149540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cca4));
    }
}
#[inline(always)]
pub fn block_0x0020cca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2149544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccc8));
}
#[inline(always)]
pub fn block_0x0020cca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 8usize, 0u32, 2149548u32);
    emu.add_memory_rw_events(1usize);
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
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2149620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccf4));
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
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 1u32, 2149556u32);
    emu.adi_no_count(18usize, 20usize, 0u32, 2149560u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2149564u32);
    emu.adi_no_count(26usize, 8usize, 0u32, 2149568u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2149572u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2149608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cce8));
    } else {
        emu.pc = 2149576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccc8));
    }
}
#[inline(always)]
pub fn block_0x0020ccc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2149580u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2149584u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2149588u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2149592u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2149596u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 112u32, 2149600u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2149604u32;
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
pub fn block_0x0020cce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccfc));
    } else {
        emu.pc = 2149608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cce8));
    }
}
#[inline(always)]
pub fn block_0x0020cce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb24));
    } else {
        emu.pc = 2149612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccec));
    }
}
#[inline(always)]
pub fn block_0x0020ccec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2149616u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb38));
}
#[inline(always)]
pub fn block_0x0020ccf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2149624u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149632u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd00));
}
#[inline(always)]
pub fn block_0x0020ccfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2149632u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd00));
}
#[inline]
pub fn block_0x0020cd00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2149636u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2149640u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2149644u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2149648u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2149652u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2149656u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2149660u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2149664u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2149668u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2149672u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2149676u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2149680u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2149684u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2149688u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149692u32;
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
pub fn block_0x0020cd3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2149696u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2149700u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2149704u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2149708u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2149712u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2149716u32)?;
    emu.lw_no_count(9usize, 10usize, 8u32, 2149720u32)?;
    emu.lbu_no_count(12usize, 9usize, 0u32, 2149724u32);
    emu.lw_no_count(8usize, 10usize, 0u32, 2149728u32)?;
    emu.lw_no_count(18usize, 10usize, 4u32, 2149732u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2149808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdb0));
    } else {
        emu.pc = 2149736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd68));
    }
}
#[inline(always)]
pub fn block_0x0020cd68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 18usize, 12u32, 2149740u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2149744u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 112u32, 2149748u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2149752u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2149756u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2149760u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2149764u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2149768u32;
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
pub fn block_0x0020cd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2149772u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2149808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdb0));
    } else {
        emu.pc = 2149776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd90));
    }
}
#[inline(always)]
pub fn block_0x0020cd90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2149780u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2149784u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2149788u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2149792u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2149796u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2149800u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149804u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149808u32;
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
pub fn block_0x0020cdb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 4294967286u32, 2149812u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2149816u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2149820u32);
    emu.lw_no_count(6usize, 18usize, 16u32, 2149824u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2149828u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2149832u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2149836u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2149840u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2149844u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2149848u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149852u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2149856u32;
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
pub fn block_0x0020cde0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2149860u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2149864u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2149868u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2149872u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2149876u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2149880u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2149884u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2149888u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2149892u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2149896u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2149900u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2149904u32);
    emu.adi_no_count(21usize, 0usize, 1u32, 2149908u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2149912u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2149972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce54));
    } else {
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
    }
}
#[inline]
pub fn block_0x0020ce1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(20usize, 8usize, 4u32, 2149920u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2149924u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2149928u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2149932u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2149936u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2149940u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2149944u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2149948u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2149952u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2149956u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2149960u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2149964u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2149968u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149972u32;
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
pub fn block_0x0020ce54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 14usize, 0u32, 2149976u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2149980u32);
    emu.lw_no_count(19usize, 8usize, 0u32, 2149984u32)?;
    emu.lbu_no_count(10usize, 8usize, 5u32, 2149988u32);
    emu.lbu_no_count(13usize, 19usize, 10u32, 2149992u32);
    emu.ani_no_count(13usize, 13usize, 128u32, 2149996u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2150028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce8c));
    } else {
        emu.pc = 2150000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce70));
    }
}
#[inline(always)]
pub fn block_0x0020ce70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 10usize, 3u32, 2150004u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2150008u32);
    emu.adi_no_count(23usize, 12usize, 0u32, 2150012u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf6c));
    } else {
        emu.pc = 2150016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce80));
    }
}
#[inline(always)]
pub fn block_0x0020ce80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150020u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 244u32, 2150024u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2150028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf74));
}
#[inline(always)]
pub fn block_0x0020ce8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cec8));
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
#[inline]
pub fn block_0x0020ce90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 19usize, 4u32, 2150036u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150040u32)?;
    emu.lw_no_count(14usize, 13usize, 12u32, 2150044u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2150048u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 251u32, 2150052u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2150056u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2150060u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2150064u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2150068u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2150072u32;
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
pub fn block_0x0020ceb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2150076u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2150080u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2150084u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
    } else {
        emu.pc = 2150088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cec8));
    }
}
#[inline]
pub fn block_0x0020cec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2150092u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2150096u32);
    emu.lw_no_count(13usize, 19usize, 0u32, 2150100u32)?;
    emu.lw_no_count(14usize, 19usize, 4u32, 2150104u32)?;
    emu.lw_no_count(15usize, 19usize, 8u32, 2150108u32)?;
    emu.lw_no_count(16usize, 19usize, 12u32, 2150112u32)?;
    emu.adi_no_count(17usize, 2usize, 12u32, 2150116u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2150120u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2150124u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2150128u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2150132u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 220u32, 2150136u32);
    emu.sb_no_count(20usize, 2usize, 27u32, 2150140u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2150144u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2150148u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2150152u32)?;
    emu.sw_no_count(16usize, 2usize, 40u32, 2150156u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2150160u32);
    emu.apc_no_count(1usize, 2150160u32, 0u32, 2150164u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150168u32;
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
pub fn block_0x0020cf18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
    } else {
        emu.pc = 2150172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf1c));
    }
}
#[inline(always)]
pub fn block_0x0020cf1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 249u32, 2150180u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2150184u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2150188u32);
    emu.apc_no_count(1usize, 2150188u32, 0u32, 2150192u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150196u32;
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
pub fn block_0x0020cf34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
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
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2150204u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2150208u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2150212u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2150216u32;
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
pub fn block_0x0020cf48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
    } else {
        emu.pc = 2150220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf4c));
    }
}
#[inline(always)]
pub fn block_0x0020cf4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2150224u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2150228u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2150232u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150236u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 254u32, 2150240u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2150244u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150248u32;
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
pub fn block_0x0020cf68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cfe4));
}
#[inline(always)]
pub fn block_0x0020cf6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150256u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 247u32, 2150260u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2150260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf74));
}
#[inline(always)]
pub fn block_0x0020cf74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 4u32, 2150264u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150268u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2150272u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2150276u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2150280u32;
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
pub fn block_0x0020cf88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2150284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
    } else {
        emu.pc = 2150288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf90));
    }
}
#[inline(always)]
pub fn block_0x0020cf90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 0u32, 2150292u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2150296u32);
    emu.lw_no_count(13usize, 19usize, 4u32, 2150300u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150304u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2150308u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150312u32;
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
pub fn block_0x0020cfa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2150316u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
    } else {
        emu.pc = 2150320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfb0));
    }
}
#[inline(always)]
pub fn block_0x0020cfb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 19usize, 4u32, 2150324u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150328u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2150332u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150336u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 249u32, 2150340u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2150344u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150348u32;
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
pub fn block_0x0020cfcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2150352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce1c));
    } else {
        emu.pc = 2150356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfd4));
    }
}
#[inline(always)]
pub fn block_0x0020cfd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2150360u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2150364u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2150368u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2150372u32;
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
pub fn block_0x0020cfe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2150376u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149916u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce1c));
}
