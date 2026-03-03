pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2195704u32;
pub const PC_MAX: u32 = 2198348u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 117usize] = [
        block_0x002180f8,
        block_0x00218128,
        block_0x0021813c,
        block_0x00218164,
        block_0x00218184,
        block_0x00218190,
        block_0x0021819c,
        block_0x002181a4,
        block_0x002181ac,
        block_0x002181b4,
        block_0x002181bc,
        block_0x002181c4,
        block_0x002181d4,
        block_0x002181d8,
        block_0x002181f8,
        block_0x00218218,
        block_0x0021821c,
        block_0x0021823c,
        block_0x00218260,
        block_0x00218288,
        block_0x00218294,
        block_0x002182a8,
        block_0x002182ac,
        block_0x002182c4,
        block_0x002182f4,
        block_0x002182fc,
        block_0x00218304,
        block_0x0021831c,
        block_0x00218320,
        block_0x00218338,
        block_0x00218340,
        block_0x0021835c,
        block_0x00218380,
        block_0x002183a0,
        block_0x002183a8,
        block_0x002183c0,
        block_0x002183e0,
        block_0x002183f8,
        block_0x00218400,
        block_0x00218418,
        block_0x00218478,
        block_0x00218490,
        block_0x002184a8,
        block_0x002184c8,
        block_0x002184d0,
        block_0x00218524,
        block_0x00218534,
        block_0x0021853c,
        block_0x00218554,
        block_0x00218564,
        block_0x0021856c,
        block_0x00218574,
        block_0x00218584,
        block_0x00218598,
        block_0x002185b0,
        block_0x00218604,
        block_0x0021863c,
        block_0x0021864c,
        block_0x00218654,
        block_0x00218668,
        block_0x0021866c,
        block_0x002186a0,
        block_0x002186d4,
        block_0x002186dc,
        block_0x002186ec,
        block_0x002186f0,
        block_0x002186fc,
        block_0x00218700,
        block_0x00218710,
        block_0x00218714,
        block_0x00218720,
        block_0x00218728,
        block_0x0021872c,
        block_0x00218754,
        block_0x0021875c,
        block_0x00218764,
        block_0x00218774,
        block_0x0021879c,
        block_0x002187a4,
        block_0x002187d0,
        block_0x002187d8,
        block_0x002187dc,
        block_0x002187e4,
        block_0x002187f0,
        block_0x00218800,
        block_0x00218810,
        block_0x00218818,
        block_0x00218838,
        block_0x00218854,
        block_0x00218858,
        block_0x00218874,
        block_0x0021887c,
        block_0x002188a8,
        block_0x002188e0,
        block_0x0021890c,
        block_0x0021893c,
        block_0x00218950,
        block_0x00218978,
        block_0x00218998,
        block_0x002189a4,
        block_0x002189e0,
        block_0x002189f8,
        block_0x00218a2c,
        block_0x00218a48,
        block_0x00218a50,
        block_0x00218a80,
        block_0x00218a98,
        block_0x00218aac,
        block_0x00218abc,
        block_0x00218ad8,
        block_0x00218ae0,
        block_0x00218ae8,
        block_0x00218b08,
        block_0x00218b14,
        block_0x00218b28,
        block_0x00218b3c,
        block_0x00218b4c,
    ];
    const IDX: [u16; 662usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16,
        0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 6u16,
        0u16, 0u16, 7u16, 0u16, 8u16, 0u16, 9u16, 0u16, 10u16, 0u16, 11u16, 0u16, 12u16,
        0u16, 0u16, 0u16, 13u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 26u16,
        0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 0u16,
        48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 50u16, 0u16, 51u16,
        0u16, 52u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16,
        0u16, 0u16, 0u16, 58u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        64u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 67u16, 68u16, 0u16, 0u16,
        0u16, 69u16, 70u16, 0u16, 0u16, 71u16, 0u16, 72u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16,
        79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16,
        81u16, 82u16, 0u16, 83u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 0u16,
        0u16, 0u16, 86u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 91u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 100u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16,
        108u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16,
        0u16, 111u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16,
        0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16,
        116u16, 0u16, 0u16, 0u16, 117u16,
    ];
    if pc < 2195704u32 || pc > 2198348u32 {
        return None;
    }
    let word_offset = ((pc - 2195704u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x002180f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2195708u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2195712u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2195716u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2195720u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2195724u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2195728u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2195732u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2195736u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2195740u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2195744u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2195748u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2195812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218164));
    } else {
        emu.pc = 2195752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218128));
    }
}
#[inline(always)]
pub fn block_0x00218128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2195756u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2195760u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2195764u32);
    emu.apc_no_count(1usize, 2195764u32, 4294905856u32, 2195768u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021813c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2195776u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2195780u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2195784u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2195788u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2195792u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2195796u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2195800u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2195804u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2195808u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195812u32;
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
pub fn block_0x00218164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2195816u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2195820u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2195824u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2195828u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2195832u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2195836u32);
    emu.apc_no_count(1usize, 2195836u32, 0u32, 2195840u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195844u32;
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
pub fn block_0x00218184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2195848u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2195852u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2195856u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195752u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218128));
}
#[inline(always)]
pub fn block_0x00218190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2195860u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2195864u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2195884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002181ac));
    } else {
        emu.pc = 2195868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021819c));
    }
}
#[inline(always)]
pub fn block_0x0021819c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2195872u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2195892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002181b4));
    } else {
        emu.pc = 2195876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002181a4));
    }
}
#[inline(always)]
pub fn block_0x002181a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2195876u32, 12288u32, 2195880u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195884u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002181ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2195884u32, 12288u32, 2195888u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195892u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002181b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2195892u32, 12288u32, 2195896u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195900u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002181bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2195904u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002181d4));
    } else {
        emu.pc = 2195908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002181c4));
    }
}
#[inline(always)]
pub fn block_0x002181c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2195912u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2195916u32);
    emu.apc_no_count(6usize, 2195916u32, 4294901760u32, 2195920u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195924u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002181d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195928u32;
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
pub fn block_0x002181d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2195932u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2195936u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2195940u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2195944u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2195948u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2195952u32);
    emu.apc_no_count(6usize, 2195952u32, 24576u32, 2195956u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195960u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002181f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2195964u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2195968u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2195972u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2195976u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2195980u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2195984u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2195988u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2196280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218338));
    } else {
        emu.pc = 2195992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218218));
    }
}
#[inline(always)]
pub fn block_0x00218218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2196416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002183c0));
    } else {
        emu.pc = 2195996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021821c));
    }
}
#[inline(always)]
pub fn block_0x0021821c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2196000u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2196004u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2196008u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966096u32, 2196012u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2196016u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2196020u32);
    emu.apc_no_count(1usize, 2196020u32, 20480u32, 2196024u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021823c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196032u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965776u32, 2196036u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196040u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966100u32, 2196044u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2196048u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2196052u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2196056u32);
    emu.apc_no_count(1usize, 2196056u32, 16384u32, 2196060u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 41u32, 2196068u32);
    emu.sb_no_count(11usize, 2usize, 35u32, 2196072u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196076u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965784u32, 2196080u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196084u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966116u32, 2196088u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2196092u32);
    emu.adi_no_count(13usize, 2usize, 35u32, 2196096u32);
    emu.apc_no_count(1usize, 2196096u32, 16384u32, 2196100u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2196108u32);
    emu.apc_no_count(1usize, 2196108u32, 4294901760u32, 2196112u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2196120u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2196124u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2196128u32);
    emu.apc_no_count(1usize, 2196128u32, 4294901760u32, 2196132u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002182a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2196624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218490));
    } else {
        emu.pc = 2196140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002182ac));
    }
}
#[inline(always)]
pub fn block_0x002182ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2196144u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966052u32, 2196152u32);
    emu.adi_no_count(12usize, 0usize, 20u32, 2196156u32);
    emu.apc_no_count(1usize, 2196156u32, 4294905856u32, 2196160u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002182c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 36u32, 2196168u32)?;
    emu.sw_no_count(9usize, 2usize, 40u32, 2196172u32)?;
    emu.sw_no_count(18usize, 2usize, 44u32, 2196176u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196180u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966148u32, 2196184u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196188u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966132u32, 2196192u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2196196u32);
    emu.adi_no_count(13usize, 2usize, 36u32, 2196200u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2196204u32);
    emu.apc_no_count(1usize, 2196204u32, 16384u32, 2196208u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002182f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2196212u32, 16384u32, 2196216u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002182fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 36u32, 2196224u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2196256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218320));
    } else {
        emu.pc = 2196228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218304));
    }
}
#[inline(always)]
pub fn block_0x00218304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 40u32, 2196232u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2196236u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2196240u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2196244u32);
    emu.apc_no_count(1usize, 2196244u32, 4294901760u32, 2196248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(24u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021831c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2196256u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2196256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218320));
}
#[inline(always)]
pub fn block_0x00218320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2196260u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2196264u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2196268u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2196272u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2196276u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196280u32;
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
pub fn block_0x00218338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 2u32, 2196284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2196504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218418));
    } else {
        emu.pc = 2196288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218340));
    }
}
#[inline(always)]
pub fn block_0x00218340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 10usize, 4u32, 2196292u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2196296u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966155u32, 2196300u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2196304u32);
    emu.adi_no_count(13usize, 0usize, 5u32, 2196308u32);
    emu.apc_no_count(1usize, 2196308u32, 20480u32, 2196312u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021835c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 8usize, 8u32, 2196320u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196324u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965784u32, 2196328u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196332u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966116u32, 2196336u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2196340u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2196344u32);
    emu.apc_no_count(1usize, 2196344u32, 16384u32, 2196348u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196356u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966148u32, 2196360u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196364u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966160u32, 2196368u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2196372u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2196376u32);
    emu.apc_no_count(1usize, 2196376u32, 16384u32, 2196380u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002183a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2196384u32, 16384u32, 2196388u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196392u32;
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
pub fn block_0x002183a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2196396u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2196400u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2196404u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2196408u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2196412u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196416u32;
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
pub fn block_0x002183c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 1u32, 2196420u32);
    emu.sb_no_count(10usize, 2usize, 24u32, 2196424u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2196428u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965780u32, 2196432u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2196436u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2196440u32);
    emu.apc_no_count(1usize, 2196440u32, 20480u32, 2196444u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002183e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2196452u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966116u32, 2196456u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2196460u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2196464u32);
    emu.apc_no_count(1usize, 2196464u32, 16384u32, 2196468u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002183f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2196472u32, 16384u32, 2196476u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2196484u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2196488u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2196492u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2196496u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2196500u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196504u32;
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
pub fn block_0x00218418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2196508u32)?;
    emu.adi_no_count(15usize, 10usize, 8u32, 2196512u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2196516u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2196520u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966176u32, 2196524u32);
    emu.adi_no_count(6usize, 2usize, 36u32, 2196528u32);
    emu.adi_no_count(7usize, 0usize, 5u32, 2196532u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2196536u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966192u32, 2196540u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196544u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965784u32, 2196548u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2196552u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966116u32, 2196556u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2196560u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966198u32, 2196564u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2196568u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2196572u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2196576u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2196580u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2196584u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2196588u32);
    emu.adi_no_count(11usize, 5usize, 0u32, 2196592u32);
    emu.apc_no_count(1usize, 2196592u32, 20480u32, 2196596u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2196604u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2196608u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2196612u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2196616u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2196620u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196624u32;
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
pub fn block_0x00218490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2196628u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966036u32, 2196632u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2196636u32);
    emu.adi_no_count(11usize, 0usize, 20u32, 2196640u32);
    emu.apc_no_count(1usize, 2196640u32, 8192u32, 2196644u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002184a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2196652u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2196656u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2196660u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2196664u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2196668u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2196672u32);
    emu.apc_no_count(1usize, 2196672u32, 4294901760u32, 2196676u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002184c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2196684u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2196912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002185b0));
    } else {
        emu.pc = 2196688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002184d0));
    }
}
#[inline]
pub fn block_0x002184d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 40u32, 2196692u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2196696u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966512u32, 2196704u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2196708u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966240u32, 2196712u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2196716u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2196720u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2196724u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2196728u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2196732u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2196736u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2196740u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2196744u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2196748u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2196752u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2196756u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2196760u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2196764u32);
    emu.apc_no_count(1usize, 2196764u32, 4294963200u32, 2196768u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196772u32;
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
pub fn block_0x00218524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 0u32, 2196776u32);
    emu.lw_no_count(8usize, 2usize, 4u32, 2196780u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2196784u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2196820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218554));
    } else {
        emu.pc = 2196788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218534));
    }
}
#[inline(always)]
pub fn block_0x00218534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2196792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2196820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218554));
    } else {
        emu.pc = 2196796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021853c));
    }
}
#[inline(always)]
pub fn block_0x0021853c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2196800u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2196804u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2196808u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2196812u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2196816u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196820u32;
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
pub fn block_0x00218554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2196824u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2196828u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2196832u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2196844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021856c));
    } else {
        emu.pc = 2196836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218564));
    }
}
#[inline(always)]
pub fn block_0x00218564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2196840u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2196844u32;
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
pub fn block_0x0021856c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2196848u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2196868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218584));
    } else {
        emu.pc = 2196852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218574));
    }
}
#[inline(always)]
pub fn block_0x00218574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2196856u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2196860u32);
    emu.apc_no_count(1usize, 2196860u32, 4294901760u32, 2196864u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2196872u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2196876u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2196880u32);
    emu.apc_no_count(1usize, 2196880u32, 4294901760u32, 2196884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2196892u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2196896u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2196900u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2196904u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2196908u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196912u32;
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
pub fn block_0x002185b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 0u32, 2196916u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2196920u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196924u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966512u32, 2196928u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2196932u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966272u32, 2196936u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2196940u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2196944u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2196948u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2196952u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2196956u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2196960u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2196964u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2196968u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2196972u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2196976u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196980u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966288u32, 2196984u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2196988u32);
    emu.apc_no_count(1usize, 2196988u32, 12288u32, 2196992u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 0u32, 2197000u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2197004u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966792u32, 2197008u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197012u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966960u32, 2197016u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2197020u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2197024u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2197028u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2197032u32)?;
    emu.lw_no_count(13usize, 10usize, 0u32, 2197036u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2197040u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2197044u32);
    emu.apc_no_count(6usize, 2197044u32, 20480u32, 2197048u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197052u32;
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
pub fn block_0x0021863c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2197056u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 12usize, 896u32, 2197060u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2197064u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2197076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218654));
    } else {
        emu.pc = 2197068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021864c));
    }
}
#[inline(always)]
pub fn block_0x0021864c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197072u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1192u32, 2197076u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2197076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218654));
}
#[inline(always)]
pub fn block_0x00218654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2197080u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2197084u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2197088u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2197092u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2197096u32;
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
pub fn block_0x00218668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2197096u32));
}
#[inline]
pub fn block_0x0021866c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2197104u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2197108u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197112u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2197116u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 428u32, 2197120u32);
    emu.adi_no_count(12usize, 12usize, 1473u32, 2197124u32);
    emu.adi_no_count(13usize, 13usize, 4294965685u32, 2197128u32);
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2197132u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2197136u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2197140u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2197144u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2197148u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197152u32;
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
pub fn block_0x002186a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2197156u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2197160u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197164u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2197168u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966129u32, 2197172u32);
    emu.adi_no_count(12usize, 12usize, 376u32, 2197176u32);
    emu.adi_no_count(13usize, 13usize, 44u32, 2197180u32);
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2197184u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2197188u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2197192u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2197196u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2197200u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197204u32;
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
pub fn block_0x002186d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2197208u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2197228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002186ec));
    } else {
        emu.pc = 2197212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002186dc));
    }
}
#[inline(always)]
pub fn block_0x002186dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2197216u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2197220u32);
    emu.apc_no_count(6usize, 2197220u32, 4294901760u32, 2197224u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197228u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002186ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197232u32;
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
pub fn block_0x002186f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2197236u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2197240u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2197264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218710));
    } else {
        emu.pc = 2197244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002186fc));
    }
}
#[inline(always)]
pub fn block_0x002186fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2197264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218710));
    } else {
        emu.pc = 2197248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218700));
    }
}
#[inline(always)]
pub fn block_0x00218700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2197252u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2197256u32);
    emu.apc_no_count(6usize, 2197256u32, 4294901760u32, 2197260u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197264u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197268u32;
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
pub fn block_0x00218714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2197272u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2197276u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2197292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021872c));
    } else {
        emu.pc = 2197280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218720));
    }
}
#[inline(always)]
pub fn block_0x00218720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2197284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2197292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021872c));
    } else {
        emu.pc = 2197288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218728));
    }
}
#[inline(always)]
pub fn block_0x00218728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197292u32;
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
pub fn block_0x0021872c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2197296u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2197300u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2197304u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2197308u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2197312u32)?;
    emu.lw_no_count(18usize, 11usize, 4u32, 2197316u32)?;
    emu.lw_no_count(12usize, 18usize, 0u32, 2197320u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2197324u32);
    emu.lw_no_count(9usize, 11usize, 0u32, 2197328u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2197340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021875c));
    } else {
        emu.pc = 2197332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218754));
    }
}
#[inline(always)]
pub fn block_0x00218754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2197336u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2197340u32;
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
pub fn block_0x0021875c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2197344u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2197364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218774));
    } else {
        emu.pc = 2197348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218764));
    }
}
#[inline(always)]
pub fn block_0x00218764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2197352u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2197356u32);
    emu.apc_no_count(1usize, 2197356u32, 4294901760u32, 2197360u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2197368u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2197372u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2197376u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2197380u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2197384u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2197388u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2197392u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2197396u32);
    emu.apc_no_count(6usize, 2197396u32, 4294901760u32, 2197400u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197404u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966168u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021879c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2197408u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197412u32;
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
pub fn block_0x002187a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2197416u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2197420u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2197424u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2197428u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2197432u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2197436u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2197440u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2197444u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2197448u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2197452u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2197464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187d8));
    } else {
        emu.pc = 2197456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187d0));
    }
}
#[inline(always)]
pub fn block_0x002187d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2197460u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2197464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002187f0));
}
#[inline(always)]
pub fn block_0x002187d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187e4));
    } else {
        emu.pc = 2197468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187dc));
    }
}
#[inline(always)]
pub fn block_0x002187dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2197472u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2197476u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002187f0));
}
#[inline(always)]
pub fn block_0x002187e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2197480u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2197484u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2197488u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2197488u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002187f0));
}
#[inline(always)]
pub fn block_0x002187f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2197492u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2197496u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2197500u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2197528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218818));
    } else {
        emu.pc = 2197504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218800));
    }
}
#[inline(always)]
pub fn block_0x00218800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2197508u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2197512u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2197516u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2197588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218854));
    } else {
        emu.pc = 2197520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218810));
    }
}
#[inline(always)]
pub fn block_0x00218810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2197524u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2197528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002188e0));
}
#[inline(always)]
pub fn block_0x00218818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2197532u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2197536u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2197540u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2197544u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2197548u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2197552u32);
    emu.apc_no_count(1usize, 2197552u32, 4294963200u32, 2197556u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2197564u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2197568u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2197572u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2197576u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2197580u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2197584u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2197520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218810));
    } else {
        emu.pc = 2197588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218854));
    }
}
#[inline(always)]
pub fn block_0x00218854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218874));
    } else {
        emu.pc = 2197592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218858));
    }
}
#[inline(always)]
pub fn block_0x00218858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2197596u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2197600u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2197604u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2197608u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2197612u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2197616u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2197620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002188e0));
}
#[inline(always)]
pub fn block_0x00218874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2197624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2197672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002188a8));
    } else {
        emu.pc = 2197628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021887c));
    }
}
#[inline]
pub fn block_0x0021887c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2197632u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2197636u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2197640u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2197644u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2197648u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2197652u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2197656u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2197660u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2197664u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2197668u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2197672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002188e0));
}
#[inline]
pub fn block_0x002188a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2197676u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2197680u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2197684u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2197688u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2197692u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2197696u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2197700u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2197704u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2197708u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2197712u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2197716u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2197720u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2197724u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2197728u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2197728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002188e0));
}
#[inline]
pub fn block_0x002188e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2197732u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2197736u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2197740u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2197744u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2197748u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2197752u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2197756u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2197760u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2197764u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2197768u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197772u32;
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
pub fn block_0x0021890c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2197776u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2197780u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2197784u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2197788u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2197792u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2197796u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2197800u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2197804u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2197808u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2197812u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2197816u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2197880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218978));
    } else {
        emu.pc = 2197820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021893c));
    }
}
#[inline(always)]
pub fn block_0x0021893c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2197824u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2197828u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2197832u32);
    emu.apc_no_count(1usize, 2197832u32, 4294901760u32, 2197836u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2197844u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2197848u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2197852u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2197856u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2197860u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2197864u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2197868u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2197872u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2197876u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197880u32;
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
pub fn block_0x00218978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2197884u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2197888u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2197892u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2197896u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2197900u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2197904u32);
    emu.apc_no_count(1usize, 2197904u32, 4294963200u32, 2197908u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2197916u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2197920u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2197924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021893c));
}
#[inline]
pub fn block_0x002189a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2197928u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2197932u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2197936u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2197940u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2197944u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2197948u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2197952u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2197956u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2197960u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2197964u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2197968u32)?;
    emu.lw_no_count(20usize, 9usize, 8u32, 2197972u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2197976u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2197980u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2198060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a2c));
    } else {
        emu.pc = 2197984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002189e0));
    }
}
#[inline(always)]
pub fn block_0x002189e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2197988u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2197992u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2197996u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2198000u32);
    emu.apc_no_count(1usize, 2198000u32, 4294901760u32, 2198004u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198008u32;
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
#[inline]
pub fn block_0x002189f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 8usize, 2198012u32);
    emu.sw_no_count(20usize, 9usize, 8u32, 2198016u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2198020u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2198024u32);
    emu.sw_no_count(8usize, 18usize, 4u32, 2198028u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2198032u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2198036u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2198040u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2198044u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2198048u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2198052u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2198056u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198060u32;
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
pub fn block_0x00218a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2198064u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2198068u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2198072u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2198076u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2198080u32);
    emu.apc_no_count(1usize, 2198080u32, 4294963200u32, 2198084u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 9usize, 8u32, 2198092u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2198096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002189e0));
}
#[inline]
pub fn block_0x00218a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2198100u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2198104u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2198108u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2198112u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2198116u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2198120u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2198124u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2198128u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2198132u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2198136u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2198140u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2198240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ae0));
    } else {
        emu.pc = 2198144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a80));
    }
}
#[inline(always)]
pub fn block_0x00218a80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2198148u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2198152u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2198156u32);
    emu.sli_no_count(22usize, 13usize, 3u32, 2198160u32);
    emu.adr_no_count(22usize, 12usize, 22usize, 2198164u32);
    emu.adi_no_count(10usize, 12usize, 4u32, 2198168u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2198168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218a98));
}
#[inline(always)]
pub fn block_0x00218a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2198172u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2198176u32);
    emu.adr_no_count(9usize, 11usize, 9usize, 2198180u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2198184u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2198168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a98));
    } else {
        emu.pc = 2198188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218aac));
    }
}
#[inline(always)]
pub fn block_0x00218aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2198192u32)?;
    emu.lw_no_count(20usize, 19usize, 8u32, 2198196u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2198200u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2198292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b14));
    } else {
        emu.pc = 2198204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218abc));
    }
}
#[inline(always)]
pub fn block_0x00218abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2198208u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2198212u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2198216u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2198220u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2198224u32);
    emu.apc_no_count(1usize, 2198224u32, 4294963200u32, 2198228u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 19usize, 8u32, 2198236u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2198240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218b14));
}
#[inline(always)]
pub fn block_0x00218ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2198244u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2198248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218b4c));
}
#[inline(always)]
pub fn block_0x00218ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2198252u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2198256u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2198260u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2198264u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2198268u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2198272u32);
    emu.apc_no_count(1usize, 2198272u32, 4294963200u32, 2198276u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 23usize, 0u32, 2198284u32);
    emu.lw_no_count(20usize, 19usize, 8u32, 2198288u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2198292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198312u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218b28));
}
#[inline(always)]
pub fn block_0x00218b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2198296u32)?;
    emu.lw_no_count(21usize, 18usize, 4u32, 2198300u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2198304u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2198308u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2198248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ae8));
    } else {
        emu.pc = 2198312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b28));
    }
}
#[inline(always)]
pub fn block_0x00218b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 4u32, 2198316u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2198320u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2198324u32);
    emu.apc_no_count(1usize, 2198324u32, 4294901760u32, 2198328u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 21usize, 2198336u32);
    emu.adi_no_count(18usize, 18usize, 8u32, 2198340u32);
    emu.sw_no_count(20usize, 19usize, 8u32, 2198344u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2198292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b14));
    } else {
        emu.pc = 2198348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b4c));
    }
}
#[inline]
pub fn block_0x00218b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2198352u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2198356u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2198360u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2198364u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2198368u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2198372u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2198376u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2198380u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2198384u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2198388u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2198392u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2198396u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2198400u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198404u32;
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
