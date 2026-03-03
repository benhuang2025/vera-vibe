pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2193792u32;
pub const PC_MAX: u32 = 2195904u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 118usize] = [
        block_0x00217980,
        block_0x002179b8,
        block_0x002179bc,
        block_0x002179c4,
        block_0x002179d4,
        block_0x002179ec,
        block_0x002179fc,
        block_0x00217a04,
        block_0x00217a14,
        block_0x00217a1c,
        block_0x00217a24,
        block_0x00217a34,
        block_0x00217a48,
        block_0x00217a60,
        block_0x00217a98,
        block_0x00217ad0,
        block_0x00217ad4,
        block_0x00217adc,
        block_0x00217aec,
        block_0x00217b04,
        block_0x00217b14,
        block_0x00217b1c,
        block_0x00217b2c,
        block_0x00217b34,
        block_0x00217b3c,
        block_0x00217b4c,
        block_0x00217b60,
        block_0x00217b78,
        block_0x00217bb0,
        block_0x00217bb4,
        block_0x00217bc8,
        block_0x00217bd0,
        block_0x00217bd8,
        block_0x00217c08,
        block_0x00217c1c,
        block_0x00217c44,
        block_0x00217c64,
        block_0x00217c70,
        block_0x00217cc4,
        block_0x00217cc8,
        block_0x00217cec,
        block_0x00217cf0,
        block_0x00217cfc,
        block_0x00217d14,
        block_0x00217d28,
        block_0x00217d30,
        block_0x00217d40,
        block_0x00217d48,
        block_0x00217d54,
        block_0x00217d7c,
        block_0x00217d8c,
        block_0x00217d94,
        block_0x00217d9c,
        block_0x00217dac,
        block_0x00217dc0,
        block_0x00217dc4,
        block_0x00217de0,
        block_0x00217df0,
        block_0x00217df8,
        block_0x00217e0c,
        block_0x00217e10,
        block_0x00217e20,
        block_0x00217e24,
        block_0x00217e2c,
        block_0x00217e3c,
        block_0x00217e40,
        block_0x00217e4c,
        block_0x00217e50,
        block_0x00217e54,
        block_0x00217e70,
        block_0x00217e78,
        block_0x00217e7c,
        block_0x00217e90,
        block_0x00217e98,
        block_0x00217eac,
        block_0x00217eb0,
        block_0x00217ebc,
        block_0x00217ec4,
        block_0x00217ec8,
        block_0x00217ed0,
        block_0x00217ed4,
        block_0x00217ed8,
        block_0x00217edc,
        block_0x00217ef8,
        block_0x00217f08,
        block_0x00217f0c,
        block_0x00217f14,
        block_0x00217f24,
        block_0x00217f38,
        block_0x00217f50,
        block_0x00217f58,
        block_0x00217f78,
        block_0x00217f8c,
        block_0x00217f94,
        block_0x00217fa4,
        block_0x0021802c,
        block_0x00218038,
        block_0x00218048,
        block_0x00218054,
        block_0x00218068,
        block_0x00218070,
        block_0x00218080,
        block_0x00218084,
        block_0x002180b0,
        block_0x002180b8,
        block_0x002180bc,
        block_0x002180c4,
        block_0x002180d0,
        block_0x002180e0,
        block_0x002180f0,
        block_0x002180f8,
        block_0x00218118,
        block_0x00218134,
        block_0x00218138,
        block_0x00218154,
        block_0x0021815c,
        block_0x00218188,
        block_0x002181c0,
    ];
    const IDX: [u16; 529usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 2u16, 3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 6u16, 0u16, 0u16, 0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16,
        10u16, 0u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16, 0u16, 18u16, 0u16, 0u16, 0u16,
        19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 21u16, 0u16, 22u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 31u16, 0u16, 32u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 39u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 42u16,
        0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16,
        45u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 0u16, 48u16, 0u16, 0u16, 49u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        51u16, 0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16,
        55u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16,
        0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16, 0u16, 0u16, 62u16,
        63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 67u16, 68u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 72u16, 0u16, 0u16,
        0u16, 0u16, 73u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16,
        77u16, 0u16, 78u16, 79u16, 0u16, 80u16, 81u16, 82u16, 83u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 86u16, 0u16, 87u16, 0u16, 0u16,
        0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 96u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 99u16,
        0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 102u16, 103u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16,
        106u16, 0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16,
        0u16, 110u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 115u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 117u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 118u16,
    ];
    if pc < 2193792u32 || pc > 2195904u32 {
        return None;
    }
    let word_offset = ((pc - 2193792u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00217980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2193796u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2193800u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2193804u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2193808u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2193812u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2193816u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2193820u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2193824u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2193828u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193832u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965996u32, 2193836u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2193840u32);
    emu.apc_no_count(1usize, 2193840u32, 20480u32, 2193844u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193848u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002179b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2193900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179ec));
    } else {
        emu.pc = 2193852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179bc));
    }
}
#[inline(always)]
pub fn block_0x002179bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2193856u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2194016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a60));
    } else {
        emu.pc = 2193860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179c4));
    }
}
#[inline(always)]
pub fn block_0x002179c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2193864u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2193868u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2193872u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2193876u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2193876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002179d4));
}
#[inline(always)]
pub fn block_0x002179d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2193880u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2193884u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2193888u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2193892u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2193896u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193900u32;
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
pub fn block_0x002179ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2193904u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2193908u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2193912u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2193924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a04));
    } else {
        emu.pc = 2193916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179fc));
    }
}
#[inline(always)]
pub fn block_0x002179fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2193920u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2193876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179d4));
    } else {
        emu.pc = 2193924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a04));
    }
}
#[inline(always)]
pub fn block_0x00217a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2193928u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2193932u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2193936u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a1c));
    } else {
        emu.pc = 2193940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a14));
    }
}
#[inline(always)]
pub fn block_0x00217a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2193944u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2193948u32;
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
pub fn block_0x00217a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2193952u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a34));
    } else {
        emu.pc = 2193956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a24));
    }
}
#[inline(always)]
pub fn block_0x00217a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2193960u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2193964u32);
    emu.apc_no_count(1usize, 2193964u32, 4294905856u32, 2193968u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2193976u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2193980u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2193984u32);
    emu.apc_no_count(1usize, 2193984u32, 4294905856u32, 2193988u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193992u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00217a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2193996u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2194000u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2194004u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2194008u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2194012u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194016u32;
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
pub fn block_0x00217a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2194020u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966208u32, 2194024u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2194028u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2194032u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2194036u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2194040u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2194044u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2194048u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2194052u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194056u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966216u32, 2194060u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2194064u32);
    emu.apc_no_count(1usize, 2194064u32, 16384u32, 2194068u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2194076u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2194080u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2194084u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2194088u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2194092u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2194096u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2194100u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2194104u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2194108u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194112u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966044u32, 2194116u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2194120u32);
    emu.apc_no_count(1usize, 2194120u32, 20480u32, 2194124u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b04));
    } else {
        emu.pc = 2194132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ad4));
    }
}
#[inline(always)]
pub fn block_0x00217ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2194136u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2194296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b78));
    } else {
        emu.pc = 2194140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217adc));
    }
}
#[inline(always)]
pub fn block_0x00217adc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2194144u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2194148u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2194152u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2194156u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2194156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217aec));
}
#[inline(always)]
pub fn block_0x00217aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2194160u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2194164u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2194168u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2194172u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2194176u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194180u32;
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
pub fn block_0x00217b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2194184u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2194188u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2194192u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2194204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b1c));
    } else {
        emu.pc = 2194196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b14));
    }
}
#[inline(always)]
pub fn block_0x00217b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2194200u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2194156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217aec));
    } else {
        emu.pc = 2194204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b1c));
    }
}
#[inline(always)]
pub fn block_0x00217b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2194208u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2194212u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2194216u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2194228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b34));
    } else {
        emu.pc = 2194220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b2c));
    }
}
#[inline(always)]
pub fn block_0x00217b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2194224u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2194228u32;
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
pub fn block_0x00217b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2194232u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2194252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b4c));
    } else {
        emu.pc = 2194236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b3c));
    }
}
#[inline(always)]
pub fn block_0x00217b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2194240u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2194244u32);
    emu.apc_no_count(1usize, 2194244u32, 4294901760u32, 2194248u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2194256u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2194260u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2194264u32);
    emu.apc_no_count(1usize, 2194264u32, 4294901760u32, 2194268u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194272u32;
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
#[inline(always)]
pub fn block_0x00217b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2194276u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2194280u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2194284u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2194288u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2194292u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194296u32;
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
pub fn block_0x00217b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2194300u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966208u32, 2194304u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2194308u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2194312u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2194316u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2194320u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2194324u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2194328u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2194332u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194336u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966216u32, 2194340u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2194344u32);
    emu.apc_no_count(1usize, 2194344u32, 16384u32, 2194348u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217bd0));
    } else {
        emu.pc = 2194356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217bb4));
    }
}
#[inline(always)]
pub fn block_0x00217bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2194360u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2194364u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2194368u32);
    emu.apc_no_count(1usize, 2194368u32, 4294905856u32, 2194372u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2194380u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2194384u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2194384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217bd0));
}
#[inline(always)]
pub fn block_0x00217bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2194388u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194392u32;
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
pub fn block_0x00217bd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2194396u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2194400u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2194404u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2194408u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2194412u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2194416u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2194420u32);
    emu.lw_no_count(9usize, 10usize, 8u32, 2194424u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2194428u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2194432u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2194436u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2194500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c44));
    } else {
        emu.pc = 2194440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c08));
    }
}
#[inline(always)]
pub fn block_0x00217c08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2194444u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2194448u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2194452u32);
    emu.apc_no_count(1usize, 2194452u32, 4294905856u32, 2194456u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2194464u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2194468u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2194472u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2194476u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2194480u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2194484u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2194488u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2194492u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2194496u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194500u32;
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
pub fn block_0x00217c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2194504u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2194508u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2194512u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2194516u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2194520u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2194524u32);
    emu.apc_no_count(1usize, 2194524u32, 0u32, 2194528u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2194536u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2194540u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2194544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217c08));
}
#[inline]
pub fn block_0x00217c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2194548u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2194552u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2194556u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2194560u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2194564u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2194568u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2194572u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2194576u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2194580u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2194584u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2194588u32);
    emu.lw_no_count(19usize, 10usize, 8u32, 2194592u32)?;
    emu.lw_no_count(10usize, 19usize, 4u32, 2194596u32)?;
    emu.lw_no_count(21usize, 19usize, 8u32, 2194600u32)?;
    emu.lw_no_count(20usize, 19usize, 12u32, 2194604u32)?;
    emu.lw_no_count(12usize, 19usize, 0u32, 2194608u32)?;
    emu.sltru_no_count(13usize, 21usize, 10usize, 2194612u32);
    emu.sltiu_no_count(14usize, 20usize, 1u32, 2194616u32);
    emu.anr_no_count(15usize, 14usize, 13usize, 2194620u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2194624u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2194632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cc8));
    } else {
        emu.pc = 2194628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cc4));
    }
}
#[inline(always)]
pub fn block_0x00217cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2194632u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cc8));
}
#[inline]
pub fn block_0x00217cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2194636u32);
    emu.orr_no_count(14usize, 14usize, 21usize, 2194640u32);
    emu.sbr_no_count(14usize, 10usize, 14usize, 2194644u32);
    emu.sltru_no_count(10usize, 10usize, 14usize, 2194648u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2194652u32);
    emu.anr_no_count(22usize, 10usize, 14usize, 2194656u32);
    emu.adr_no_count(10usize, 12usize, 13usize, 2194660u32);
    emu.adi_no_count(18usize, 22usize, 0u32, 2194664u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2194672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cf0));
    } else {
        emu.pc = 2194668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cec));
    }
}
#[inline(always)]
pub fn block_0x00217cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 9usize, 0u32, 2194672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cf0));
}
#[inline(always)]
pub fn block_0x00217cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 18usize, 0u32, 2194676u32);
    emu.apc_no_count(1usize, 2194676u32, 4294905856u32, 2194680u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(64u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 21usize, 18usize, 2194688u32);
    emu.sltru_no_count(10usize, 18usize, 21usize, 2194692u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2194696u32);
    emu.sw_no_count(18usize, 19usize, 8u32, 2194700u32)?;
    emu.sw_no_count(10usize, 19usize, 12u32, 2194704u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2194728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d28));
    } else {
        emu.pc = 2194708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d14));
    }
}
#[inline(always)]
pub fn block_0x00217d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2194712u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 10usize, 4294966112u32, 2194716u32)?;
    emu.ani_no_count(12usize, 19usize, 255u32, 2194720u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2194724u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2194736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d30));
    } else {
        emu.pc = 2194728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d28));
    }
}
#[inline(always)]
pub fn block_0x00217d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2194732u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d54));
}
#[inline(always)]
pub fn block_0x00217d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 4294966116u32, 2194740u32)?;
    emu.lbu_no_count(10usize, 8usize, 0u32, 2194744u32);
    emu.lw_no_count(9usize, 8usize, 4u32, 2194748u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2194812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d7c));
    } else {
        emu.pc = 2194752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d40));
    }
}
#[inline(always)]
pub fn block_0x00217d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2194756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2194812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d7c));
    } else {
        emu.pc = 2194760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d48));
    }
}
#[inline(always)]
pub fn block_0x00217d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2194764u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2194768u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2194772u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2194772u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d54));
}
#[inline]
pub fn block_0x00217d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2194776u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2194780u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2194784u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2194788u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2194792u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2194796u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2194800u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2194804u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2194808u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194812u32;
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
pub fn block_0x00217d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 9usize, 4u32, 2194816u32)?;
    emu.lw_no_count(11usize, 21usize, 0u32, 2194820u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2194824u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2194836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d94));
    } else {
        emu.pc = 2194828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d8c));
    }
}
#[inline(always)]
pub fn block_0x00217d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2194832u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2194836u32;
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
pub fn block_0x00217d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 21usize, 4u32, 2194840u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2194860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217dac));
    } else {
        emu.pc = 2194844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d9c));
    }
}
#[inline(always)]
pub fn block_0x00217d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2194848u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2194852u32);
    emu.apc_no_count(1usize, 2194852u32, 4294901760u32, 2194856u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2194864u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2194868u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2194872u32);
    emu.apc_no_count(1usize, 2194872u32, 4294901760u32, 2194876u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2194884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d48));
}
#[inline(always)]
pub fn block_0x00217dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2194888u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2194892u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2194896u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2194900u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2194904u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2194908u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2195020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e4c));
    } else {
        emu.pc = 2194912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217de0));
    }
}
#[inline(always)]
pub fn block_0x00217de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2194916u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2194920u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2194924u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2194976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e20));
    } else {
        emu.pc = 2194928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217df0));
    }
}
#[inline(always)]
pub fn block_0x00217df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2194932u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2194976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e20));
    } else {
        emu.pc = 2194936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217df8));
    }
}
#[inline(always)]
pub fn block_0x00217df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2194940u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2194944u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2194948u32);
    emu.apc_no_count(1usize, 2194948u32, 4294901760u32, 2194952u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e40));
    } else {
        emu.pc = 2194960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e10));
    }
}
#[inline(always)]
pub fn block_0x00217e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2194964u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2194968u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2194972u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2194976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217e54));
}
#[inline(always)]
pub fn block_0x00217e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e70));
    } else {
        emu.pc = 2194980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e24));
    }
}
#[inline(always)]
pub fn block_0x00217e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2194980u32, 4294901760u32, 2194984u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2194992u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2194996u32);
    emu.apc_no_count(1usize, 2194996u32, 4294901760u32, 2195000u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e10));
    } else {
        emu.pc = 2195008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e40));
    }
}
#[inline(always)]
pub fn block_0x00217e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2195012u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2195016u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2195020u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217e50));
}
#[inline(always)]
pub fn block_0x00217e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2195024u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2195024u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217e50));
}
#[inline(always)]
pub fn block_0x00217e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2195028u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2195028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217e54));
}
#[inline(always)]
pub fn block_0x00217e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2195032u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2195036u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2195040u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2195044u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2195048u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195052u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195056u32;
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
pub fn block_0x00217e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2195060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2194960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e10));
    } else {
        emu.pc = 2195064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e78));
    }
}
#[inline(always)]
pub fn block_0x00217e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2195068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195008u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217e40));
}
#[inline(always)]
pub fn block_0x00217e7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2195072u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2195076u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2195080u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2195084u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2195320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f78));
    } else {
        emu.pc = 2195088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e90));
    }
}
#[inline(always)]
pub fn block_0x00217e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 11usize, 12usize, 2195092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2195320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f78));
    } else {
        emu.pc = 2195096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e98));
    }
}
#[inline(always)]
pub fn block_0x00217e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2195100u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2195104u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2195108u32)?;
    emu.sli_no_count(10usize, 13usize, 1u32, 2195112u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2195120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217eb0));
    } else {
        emu.pc = 2195116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217eac));
    }
}
#[inline(always)]
pub fn block_0x00217eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2195120u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2195120u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217eb0));
}
#[inline(always)]
pub fn block_0x00217eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1025u32, 2195124u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2195128u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2195144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ec8));
    } else {
        emu.pc = 2195132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ebc));
    }
}
#[inline(always)]
pub fn block_0x00217ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2195136u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2195152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ed0));
    } else {
        emu.pc = 2195140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ec4));
    }
}
#[inline(always)]
pub fn block_0x00217ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2195144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217ed4));
}
#[inline(always)]
pub fn block_0x00217ec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2195148u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2195156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ed4));
    } else {
        emu.pc = 2195152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ed0));
    }
}
#[inline(always)]
pub fn block_0x00217ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2195156u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2195156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217ed4));
}
#[inline(always)]
pub fn block_0x00217ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217edc));
    } else {
        emu.pc = 2195160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ed8));
    }
}
#[inline(always)]
pub fn block_0x00217ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2195164u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2195164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217edc));
}
#[inline(always)]
pub fn block_0x00217edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 15usize, 14usize, 2195168u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2195172u32);
    emu.sbr_no_count(11usize, 0usize, 15usize, 2195176u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2195180u32);
    emu.mulhu_no_count(12usize, 11usize, 9usize, 2195184u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2195188u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2195220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f14));
    } else {
        emu.pc = 2195192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ef8));
    }
}
#[inline(always)]
pub fn block_0x00217ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 11usize, 9usize, 2195196u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195200u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 11usize, 15usize, 2195204u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2195348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f94));
    } else {
        emu.pc = 2195208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f08));
    }
}
#[inline(always)]
pub fn block_0x00217f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f24));
    } else {
        emu.pc = 2195212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f0c));
    }
}
#[inline(always)]
pub fn block_0x00217f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2195216u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2195220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217f38));
}
#[inline(always)]
pub fn block_0x00217f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2195224u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966232u32, 2195228u32);
    emu.apc_no_count(1usize, 2195228u32, 8192u32, 2195232u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195236u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2195240u32)?;
    emu.mul_no_count(11usize, 13usize, 14usize, 2195244u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2195248u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2195252u32)?;
    emu.adi_no_count(10usize, 15usize, 0u32, 2195256u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2195256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217f38));
}
#[inline(always)]
pub fn block_0x00217f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2195260u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2195264u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2195268u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2195272u32);
    emu.apc_no_count(1usize, 2195272u32, 0u32, 2195276u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195280u32;
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
pub fn block_0x00217f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2195284u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f8c));
    } else {
        emu.pc = 2195288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f58));
    }
}
#[inline(always)]
pub fn block_0x00217f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2195292u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2195296u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2195300u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2195304u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2195308u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2195312u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2195316u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195320u32;
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
pub fn block_0x00217f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2195324u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2195328u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966232u32, 2195332u32);
    emu.apc_no_count(1usize, 2195332u32, 8192u32, 2195336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2195344u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2195348u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2195348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217f94));
}
#[inline(always)]
pub fn block_0x00217f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2195352u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966232u32, 2195356u32);
    emu.apc_no_count(1usize, 2195356u32, 8192u32, 2195360u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00217fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 34u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2195368u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2195372u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2195376u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2195380u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2195384u32)?;
    emu.adi_no_count(13usize, 2usize, 28u32, 2195388u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2195392u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966492u32, 2195396u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2195400u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966756u32, 2195404u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2195408u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966252u32, 2195412u32);
    emu.lw_no_count(17usize, 12usize, 0u32, 2195416u32)?;
    emu.lw_no_count(5usize, 12usize, 4u32, 2195420u32)?;
    emu.adi_no_count(6usize, 12usize, 8u32, 2195424u32);
    emu.adi_no_count(12usize, 12usize, 12u32, 2195428u32);
    emu.sw_no_count(13usize, 2usize, 36u32, 2195432u32)?;
    emu.sw_no_count(14usize, 2usize, 40u32, 2195436u32)?;
    emu.sw_no_count(6usize, 2usize, 44u32, 2195440u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2195444u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2195448u32)?;
    emu.sw_no_count(15usize, 2usize, 56u32, 2195452u32)?;
    emu.adi_no_count(12usize, 0usize, 3u32, 2195456u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2195460u32)?;
    emu.sw_no_count(17usize, 2usize, 28u32, 2195464u32)?;
    emu.sw_no_count(5usize, 2usize, 32u32, 2195468u32)?;
    emu.adi_no_count(13usize, 2usize, 36u32, 2195472u32);
    emu.sw_no_count(16usize, 2usize, 4u32, 2195476u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2195480u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2195484u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2195488u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2195492u32);
    emu.apc_no_count(1usize, 2195492u32, 20480u32, 2195496u32);
    emu.add_memory_rw_events(34usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195500u32;
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
pub fn block_0x0021802c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2195504u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2195508u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195512u32;
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
pub fn block_0x00218038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2195516u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2195520u32)?;
    emu.apc_no_count(1usize, 2195520u32, 0u32, 2195524u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195528u32;
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
pub fn block_0x00218048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2195532u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195536u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195540u32;
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
pub fn block_0x00218054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2195544u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195548u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966312u32, 2195552u32);
    emu.apc_no_count(6usize, 2195552u32, 20480u32, 2195556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195560u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2195564u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218080));
    } else {
        emu.pc = 2195568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218070));
    }
}
#[inline(always)]
pub fn block_0x00218070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2195572u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2195576u32);
    emu.apc_no_count(6usize, 2195576u32, 4294901760u32, 2195580u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195584u32;
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
pub fn block_0x00218080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195588u32;
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
pub fn block_0x00218084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2195592u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2195596u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2195600u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2195604u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2195608u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2195612u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2195616u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2195620u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2195624u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2195628u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2195640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180b8));
    } else {
        emu.pc = 2195632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180b0));
    }
}
#[inline(always)]
pub fn block_0x002180b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2195636u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2195640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002180d0));
}
#[inline(always)]
pub fn block_0x002180b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180c4));
    } else {
        emu.pc = 2195644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180bc));
    }
}
#[inline(always)]
pub fn block_0x002180bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2195648u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2195652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002180d0));
}
#[inline(always)]
pub fn block_0x002180c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2195656u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2195660u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2195664u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2195664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002180d0));
}
#[inline(always)]
pub fn block_0x002180d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2195668u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2195672u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2195676u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2195704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180f8));
    } else {
        emu.pc = 2195680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180e0));
    }
}
#[inline(always)]
pub fn block_0x002180e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2195684u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2195688u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2195692u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2195764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218134));
    } else {
        emu.pc = 2195696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180f0));
    }
}
#[inline(always)]
pub fn block_0x002180f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2195700u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2195704u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002181c0));
}
#[inline(always)]
pub fn block_0x002180f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2195708u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2195712u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2195716u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2195720u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2195724u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2195728u32);
    emu.apc_no_count(1usize, 2195728u32, 0u32, 2195732u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195736u32;
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
pub fn block_0x00218118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2195740u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2195744u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2195748u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2195752u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2195756u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2195760u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2195696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180f0));
    } else {
        emu.pc = 2195764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218134));
    }
}
#[inline(always)]
pub fn block_0x00218134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218154));
    } else {
        emu.pc = 2195768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218138));
    }
}
#[inline(always)]
pub fn block_0x00218138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2195772u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2195776u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2195780u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2195784u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2195788u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2195792u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2195796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002181c0));
}
#[inline(always)]
pub fn block_0x00218154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2195800u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2195848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218188));
    } else {
        emu.pc = 2195804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021815c));
    }
}
#[inline]
pub fn block_0x0021815c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2195808u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2195812u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2195816u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2195820u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2195824u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2195828u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2195832u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2195836u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2195840u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2195844u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2195848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002181c0));
}
#[inline]
pub fn block_0x00218188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2195852u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2195856u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2195860u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2195864u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2195868u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2195872u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2195876u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2195880u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2195884u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2195888u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2195892u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2195896u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2195900u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2195904u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2195904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002181c0));
}
#[inline]
pub fn block_0x002181c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2195908u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2195912u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2195916u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2195920u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2195924u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2195928u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2195932u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2195936u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2195940u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2195944u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195948u32;
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
