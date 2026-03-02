pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2118532u32;
pub const PC_MAX: u32 = 2121452u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 117usize] = [
        block_0x00205384,
        block_0x002053ac,
        block_0x002053b0,
        block_0x002053c0,
        block_0x002053c4,
        block_0x002053d4,
        block_0x002053d8,
        block_0x002053e8,
        block_0x002053ec,
        block_0x002053fc,
        block_0x00205400,
        block_0x00205410,
        block_0x0020543c,
        block_0x0020545c,
        block_0x00205468,
        block_0x00205478,
        block_0x00205490,
        block_0x002054c8,
        block_0x002054dc,
        block_0x0020551c,
        block_0x00205540,
        block_0x00205550,
        block_0x00205568,
        block_0x00205584,
        block_0x0020559c,
        block_0x002055b4,
        block_0x002055b8,
        block_0x002055c4,
        block_0x002055d4,
        block_0x002055e4,
        block_0x002055f4,
        block_0x00205618,
        block_0x00205628,
        block_0x0020562c,
        block_0x00205650,
        block_0x00205680,
        block_0x002056c8,
        block_0x002056cc,
        block_0x002056d0,
        block_0x002056dc,
        block_0x002056f4,
        block_0x002056f8,
        block_0x00205714,
        block_0x0020576c,
        block_0x00205778,
        block_0x0020578c,
        block_0x00205794,
        block_0x00205820,
        block_0x00205830,
        block_0x00205834,
        block_0x00205844,
        block_0x00205860,
        block_0x00205870,
        block_0x002058a4,
        block_0x002058bc,
        block_0x0020590c,
        block_0x00205910,
        block_0x00205914,
        block_0x00205920,
        block_0x00205938,
        block_0x0020593c,
        block_0x00205950,
        block_0x002059b0,
        block_0x002059f0,
        block_0x002059fc,
        block_0x00205a10,
        block_0x00205a18,
        block_0x00205aac,
        block_0x00205ab4,
        block_0x00205b78,
        block_0x00205b8c,
        block_0x00205b90,
        block_0x00205ba8,
        block_0x00205bb8,
        block_0x00205bd4,
        block_0x00205c10,
        block_0x00205c28,
        block_0x00205c6c,
        block_0x00205c70,
        block_0x00205c74,
        block_0x00205c7c,
        block_0x00205c94,
        block_0x00205c98,
        block_0x00205cc4,
        block_0x00205ce0,
        block_0x00205ce8,
        block_0x00205d04,
        block_0x00205d14,
        block_0x00205d18,
        block_0x00205d28,
        block_0x00205d44,
        block_0x00205d64,
        block_0x00205d70,
        block_0x00205da0,
        block_0x00205db8,
        block_0x00205dec,
        block_0x00205e14,
        block_0x00205e20,
        block_0x00205e24,
        block_0x00205e3c,
        block_0x00205e44,
        block_0x00205e4c,
        block_0x00205e50,
        block_0x00205e5c,
        block_0x00205e64,
        block_0x00205e68,
        block_0x00205e74,
        block_0x00205e80,
        block_0x00205e88,
        block_0x00205e8c,
        block_0x00205e90,
        block_0x00205e98,
        block_0x00205e9c,
        block_0x00205ea8,
        block_0x00205eac,
        block_0x00205ebc,
        block_0x00205eec,
    ];
    const IDX: [u16; 731usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 3u16, 0u16,
        0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16, 8u16,
        9u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 22u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 27u16, 0u16,
        0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16,
        33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 37u16, 38u16, 39u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        41u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16,
        0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 56u16, 57u16, 58u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 60u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 65u16,
        0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16,
        79u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16,
        0u16, 0u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 97u16, 0u16, 0u16, 98u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        100u16, 0u16, 101u16, 0u16, 102u16, 103u16, 0u16, 0u16, 104u16, 0u16, 105u16,
        106u16, 0u16, 0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 109u16, 110u16, 111u16,
        0u16, 112u16, 113u16, 0u16, 0u16, 114u16, 115u16, 0u16, 0u16, 0u16, 116u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16,
    ];
    if pc < 2118532u32 || pc > 2121452u32 {
        return None;
    }
    let word_offset = ((pc - 2118532u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00205384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2118536u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2118540u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2118544u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2118548u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2118552u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2118556u32)?;
    emu.adi_no_count(11usize, 8usize, 12u32, 2118560u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2118564u32);
    emu.apc_no_count(1usize, 2118564u32, 0u32, 2118568u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002053ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205410));
    } else {
        emu.pc = 2118576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002053b0));
    }
}
#[inline(always)]
pub fn block_0x002053b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 44u32, 2118580u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2118584u32);
    emu.apc_no_count(1usize, 2118584u32, 0u32, 2118588u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002053c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205410));
    } else {
        emu.pc = 2118596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002053c4));
    }
}
#[inline(always)]
pub fn block_0x002053c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 76u32, 2118600u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2118604u32);
    emu.apc_no_count(1usize, 2118604u32, 0u32, 2118608u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118612u32;
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
pub fn block_0x002053d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205410));
    } else {
        emu.pc = 2118616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002053d8));
    }
}
#[inline(always)]
pub fn block_0x002053d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2118620u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2118624u32);
    emu.apc_no_count(1usize, 2118624u32, 4294959104u32, 2118628u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002053e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205410));
    } else {
        emu.pc = 2118636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002053ec));
    }
}
#[inline(always)]
pub fn block_0x002053ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 108u32, 2118640u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2118644u32);
    emu.apc_no_count(1usize, 2118644u32, 0u32, 2118648u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118652u32;
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
#[inline(always)]
pub fn block_0x002053fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205410));
    } else {
        emu.pc = 2118656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205400));
    }
}
#[inline(always)]
pub fn block_0x00205400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2118660u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2118664u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2118668u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118672u32;
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
pub fn block_0x00205410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 0u32, 2118676u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118680u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967268u32, 2118684u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2118688u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967252u32, 2118692u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2118696u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967288u32, 2118700u32);
    emu.adi_no_count(11usize, 0usize, 20u32, 2118704u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2118708u32);
    emu.apc_no_count(1usize, 2118708u32, 98304u32, 2118712u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020543c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966944u32, 2118720u32);
    emu.sw_no_count(1usize, 2usize, 348u32, 2118724u32)?;
    emu.sw_no_count(8usize, 2usize, 344u32, 2118728u32)?;
    emu.sw_no_count(9usize, 2usize, 340u32, 2118732u32)?;
    emu.sw_no_count(18usize, 2usize, 336u32, 2118736u32)?;
    emu.sw_no_count(19usize, 2usize, 332u32, 2118740u32)?;
    emu.sw_no_count(20usize, 2usize, 328u32, 2118744u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2119208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205628));
    } else {
        emu.pc = 2118748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020545c));
    }
}
#[inline(always)]
pub fn block_0x0020545c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2118752u32);
    emu.adi_no_count(14usize, 0usize, 5u32, 2118756u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2119208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205628));
    } else {
        emu.pc = 2118760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205468));
    }
}
#[inline(always)]
pub fn block_0x00205468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 61u32, 2118764u32);
    emu.srr_no_count(14usize, 14usize, 13usize, 2118768u32);
    emu.ani_no_count(14usize, 14usize, 1u32, 2118772u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2119208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205628));
    } else {
        emu.pc = 2118776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205478));
    }
}
#[inline(always)]
pub fn block_0x00205478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2118780u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2118784u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1844u32, 2118788u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2118792u32);
    emu.lw_no_count(13usize, 13usize, 0u32, 2118796u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2119208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205628));
    } else {
        emu.pc = 2118800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205490));
    }
}
#[inline]
pub fn block_0x00205490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2118804u32);
    emu.sltiu_no_count(10usize, 12usize, 65u32, 2118808u32);
    emu.adi_no_count(13usize, 0usize, 65u32, 2118812u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2118816u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118820u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2118824u32);
    emu.adi_no_count(10usize, 2usize, 68u32, 2118828u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2118832u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2118836u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2118840u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2118844u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2118848u32);
    emu.apc_no_count(1usize, 2118848u32, 16384u32, 2118852u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002054c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 68u32, 2118860u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2118864u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2118868u32);
    emu.apc_no_count(1usize, 2118868u32, 16384u32, 2118872u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002054dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 69u32, 2118880u32);
    emu.lbu_no_count(11usize, 2usize, 68u32, 2118884u32);
    emu.lbu_no_count(12usize, 2usize, 70u32, 2118888u32);
    emu.lbu_no_count(8usize, 2usize, 71u32, 2118892u32);
    emu.sli_no_count(10usize, 10usize, 8u32, 2118896u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118900u32);
    emu.lbu_no_count(9usize, 2usize, 72u32, 2118904u32);
    emu.lbu_no_count(19usize, 2usize, 73u32, 2118908u32);
    emu.lbu_no_count(20usize, 2usize, 74u32, 2118912u32);
    emu.sh_no_count(10usize, 2usize, 2u32, 2118916u32)?;
    emu.sb_no_count(12usize, 2usize, 4u32, 2118920u32);
    emu.adi_no_count(11usize, 2usize, 75u32, 2118924u32);
    emu.adi_no_count(10usize, 2usize, 204u32, 2118928u32);
    emu.adi_no_count(12usize, 0usize, 58u32, 2118932u32);
    emu.apc_no_count(1usize, 2118932u32, 16384u32, 2118936u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020551c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 2usize, 5u32, 2118944u32);
    emu.sb_no_count(9usize, 2usize, 6u32, 2118948u32);
    emu.sb_no_count(19usize, 2usize, 7u32, 2118952u32);
    emu.sb_no_count(20usize, 2usize, 8u32, 2118956u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2118960u32);
    emu.adi_no_count(11usize, 2usize, 204u32, 2118964u32);
    emu.adi_no_count(12usize, 0usize, 58u32, 2118968u32);
    emu.apc_no_count(1usize, 2118968u32, 16384u32, 2118972u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 68u32, 2118980u32);
    emu.adi_no_count(11usize, 2usize, 2u32, 2118984u32);
    emu.apc_no_count(1usize, 2118984u32, 4294955008u32, 2118988u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2118996u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1776u32, 2119000u32);
    emu.adi_no_count(10usize, 2usize, 204u32, 2119004u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2119008u32);
    emu.apc_no_count(1usize, 2119008u32, 16384u32, 2119012u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119016u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(8usize, 2usize, 136u32, 2119020u32);
    emu.adi_no_count(10usize, 2usize, 140u32, 2119024u32);
    emu.adi_no_count(11usize, 2usize, 204u32, 2119028u32);
    emu.adi_no_count(12usize, 2usize, 68u32, 2119032u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2119036u32);
    emu.apc_no_count(1usize, 2119036u32, 49152u32, 2119040u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119044u32;
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
#[inline(always)]
pub fn block_0x00205584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 236u32, 2119048u32);
    emu.adi_no_count(12usize, 2usize, 100u32, 2119052u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2119056u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2119060u32);
    emu.apc_no_count(1usize, 2119060u32, 49152u32, 2119064u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119068u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020559c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 2u32, 2119072u32);
    emu.lbu_no_count(9usize, 2usize, 268u32, 2119076u32);
    emu.lbu_no_count(19usize, 2usize, 132u32, 2119080u32);
    emu.adi_no_count(11usize, 10usize, 4294967294u32, 2119084u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2119088u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2119096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055b8));
    } else {
        emu.pc = 2119092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055b4));
    }
}
#[inline(always)]
pub fn block_0x002055b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2119248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205650));
    } else {
        emu.pc = 2119096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055b8));
    }
}
#[inline(always)]
pub fn block_0x002055b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2119100u32);
    emu.apc_no_count(1usize, 2119100u32, 73728u32, 2119104u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002055c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2119112u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2119116u32);
    emu.apc_no_count(1usize, 2119116u32, 73728u32, 2119120u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002055d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 136u32, 2119128u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2119132u32);
    emu.apc_no_count(1usize, 2119132u32, 73728u32, 2119136u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119140u32;
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
pub fn block_0x002055e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 10usize, 255u32, 2119144u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2119148u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2119152u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2119212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020562c));
    } else {
        emu.pc = 2119156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055f4));
    }
}
#[inline]
pub fn block_0x002055f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(11usize, 19usize, 9usize, 2119160u32);
    emu.sbr_no_count(12usize, 0usize, 8usize, 2119164u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2119168u32);
    emu.xrr_no_count(9usize, 11usize, 9usize, 2119172u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2119176u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2119180u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2119184u32);
    emu.apc_no_count(1usize, 2119184u32, 16384u32, 2119188u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2119196u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2119200u32);
    emu.sb_no_count(9usize, 18usize, 68u32, 2119204u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2119208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2119212u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020562c));
}
#[inline(always)]
pub fn block_0x00205628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2119212u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2119212u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020562c));
}
#[inline]
pub fn block_0x0020562c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2119216u32)?;
    emu.lw_no_count(1usize, 2usize, 348u32, 2119220u32)?;
    emu.lw_no_count(8usize, 2usize, 344u32, 2119224u32)?;
    emu.lw_no_count(9usize, 2usize, 340u32, 2119228u32)?;
    emu.lw_no_count(18usize, 2usize, 336u32, 2119232u32)?;
    emu.lw_no_count(19usize, 2usize, 332u32, 2119236u32)?;
    emu.lw_no_count(20usize, 2usize, 328u32, 2119240u32)?;
    emu.adi_no_count(2usize, 2usize, 352u32, 2119244u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119248u32;
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
pub fn block_0x00205650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 7u32, 2119252u32);
    emu.sw_no_count(10usize, 2usize, 272u32, 2119256u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2119260u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965768u32, 2119264u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2119268u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965752u32, 2119272u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2119276u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965780u32, 2119280u32);
    emu.adi_no_count(11usize, 0usize, 11u32, 2119284u32);
    emu.adi_no_count(12usize, 2usize, 272u32, 2119288u32);
    emu.apc_no_count(1usize, 2119288u32, 94208u32, 2119292u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967136u32, 2119300u32);
    emu.sw_no_count(1usize, 2usize, 156u32, 2119304u32)?;
    emu.sw_no_count(8usize, 2usize, 152u32, 2119308u32)?;
    emu.sw_no_count(9usize, 2usize, 148u32, 2119312u32)?;
    emu.sw_no_count(18usize, 2usize, 144u32, 2119316u32)?;
    emu.sw_no_count(19usize, 2usize, 140u32, 2119320u32)?;
    emu.sw_no_count(20usize, 2usize, 136u32, 2119324u32)?;
    emu.sw_no_count(21usize, 2usize, 132u32, 2119328u32)?;
    emu.sw_no_count(22usize, 2usize, 128u32, 2119332u32)?;
    emu.sw_no_count(23usize, 2usize, 124u32, 2119336u32)?;
    emu.sw_no_count(24usize, 2usize, 120u32, 2119340u32)?;
    emu.sw_no_count(25usize, 2usize, 116u32, 2119344u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2119348u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2119352u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2119356u32);
    let a = 0u32.wrapping_add(32768u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2119360u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 12usize, 0u32, 2119364u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056cc));
    } else {
        emu.pc = 2119368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056c8));
    }
}
#[inline(always)]
pub fn block_0x002056c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(32768u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2119372u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2119372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002056cc));
}
#[inline(always)]
pub fn block_0x002056cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2119732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205834));
    } else {
        emu.pc = 2119376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056d0));
    }
}
#[inline(always)]
pub fn block_0x002056d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(19usize, 20usize, 5u32, 2119380u32);
    emu.apc_no_count(1usize, 2119380u32, 16384u32, 2119384u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002056dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2119392u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965652u32, 2119396u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2119400u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2119404u32);
    emu.apc_no_count(1usize, 2119404u32, 16384u32, 2119408u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002056f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2119844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002058a4));
    } else {
        emu.pc = 2119416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056f8));
    }
}
#[inline(always)]
pub fn block_0x002056f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 12u32, 2119420u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2119424u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2119428u32)?;
    emu.adi_no_count(20usize, 2usize, 88u32, 2119432u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2119436u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 244u32, 2119440u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2119444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2119544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205778));
}
#[inline]
pub fn block_0x00205714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2119448u32);
    emu.sli_no_count(25usize, 25usize, 8u32, 2119452u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2119456u32)?;
    emu.sli_no_count(11usize, 21usize, 5u32, 2119460u32);
    emu.sri_no_count(12usize, 23usize, 24u32, 2119464u32);
    emu.orr_no_count(13usize, 25usize, 24usize, 2119468u32);
    emu.sri_no_count(14usize, 23usize, 16u32, 2119472u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2119476u32);
    emu.sri_no_count(11usize, 23usize, 8u32, 2119480u32);
    emu.sb_no_count(11usize, 10usize, 4u32, 2119484u32);
    emu.sb_no_count(14usize, 10usize, 5u32, 2119488u32);
    emu.sb_no_count(12usize, 10usize, 6u32, 2119492u32);
    emu.sri_no_count(11usize, 13usize, 8u32, 2119496u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2119500u32);
    emu.sb_no_count(11usize, 10usize, 1u32, 2119504u32);
    emu.sb_no_count(22usize, 10usize, 2u32, 2119508u32);
    emu.sb_no_count(23usize, 10usize, 3u32, 2119512u32);
    emu.adi_no_count(10usize, 10usize, 7u32, 2119516u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2119520u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2119524u32);
    emu.apc_no_count(1usize, 2119524u32, 16384u32, 2119528u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020576c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 21usize, 1u32, 2119536u32);
    emu.sw_no_count(21usize, 2usize, 20u32, 2119540u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2119748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205844));
    } else {
        emu.pc = 2119544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205778));
    }
}
#[inline(always)]
pub fn block_0x00205778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 80u32, 2119548u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2119552u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2119556u32);
    emu.apc_no_count(1usize, 2119556u32, 4294946816u32, 2119560u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020578c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 80u32, 2119568u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2119776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205860));
    } else {
        emu.pc = 2119572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205794));
    }
}
#[inline(never)]
pub fn block_0x00205794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 20usize, 0u32, 2119576u32)?;
    emu.lw_no_count(11usize, 20usize, 4u32, 2119580u32)?;
    emu.lw_no_count(12usize, 20usize, 8u32, 2119584u32)?;
    emu.lw_no_count(13usize, 20usize, 12u32, 2119588u32)?;
    emu.lw_no_count(14usize, 20usize, 16u32, 2119592u32)?;
    emu.lw_no_count(15usize, 20usize, 20u32, 2119596u32)?;
    emu.lbu_no_count(16usize, 20usize, 24u32, 2119600u32);
    emu.lw_no_count(17usize, 2usize, 12u32, 2119604u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2119608u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2119612u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2119616u32)?;
    emu.sw_no_count(12usize, 2usize, 60u32, 2119620u32)?;
    emu.sw_no_count(13usize, 2usize, 64u32, 2119624u32)?;
    emu.sw_no_count(14usize, 2usize, 68u32, 2119628u32)?;
    emu.sw_no_count(15usize, 2usize, 72u32, 2119632u32)?;
    emu.sb_no_count(16usize, 2usize, 76u32, 2119636u32);
    emu.lw_no_count(10usize, 2usize, 52u32, 2119640u32)?;
    emu.lw_no_count(11usize, 2usize, 56u32, 2119644u32)?;
    emu.lw_no_count(12usize, 2usize, 60u32, 2119648u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2119652u32)?;
    emu.lw_no_count(14usize, 2usize, 68u32, 2119656u32)?;
    emu.lw_no_count(15usize, 2usize, 72u32, 2119660u32)?;
    emu.lbu_no_count(16usize, 2usize, 76u32, 2119664u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2119668u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2119672u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2119676u32)?;
    emu.sw_no_count(13usize, 2usize, 36u32, 2119680u32)?;
    emu.lbu_no_count(24usize, 2usize, 81u32, 2119684u32);
    emu.lbu_no_count(25usize, 2usize, 82u32, 2119688u32);
    emu.lbu_no_count(22usize, 2usize, 83u32, 2119692u32);
    emu.lw_no_count(23usize, 2usize, 84u32, 2119696u32)?;
    emu.sw_no_count(14usize, 2usize, 40u32, 2119700u32)?;
    emu.sw_no_count(15usize, 2usize, 44u32, 2119704u32)?;
    emu.sb_no_count(16usize, 2usize, 48u32, 2119708u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2119444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205714));
    } else {
        emu.pc = 2119712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205820));
    }
}
#[inline(always)]
pub fn block_0x00205820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2119716u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2119720u32);
    emu.apc_no_count(1usize, 2119720u32, 12288u32, 2119724u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2119732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2119444u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205714));
}
#[inline(always)]
pub fn block_0x00205834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2119736u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2119740u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2119744u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2119748u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2119748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205844));
}
#[inline(always)]
pub fn block_0x00205844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2119752u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2119756u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2119760u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2119764u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2119768u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2119772u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2119776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2119792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205870));
}
#[inline(always)]
pub fn block_0x00205860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 84u32, 2119780u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2119784u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2119788u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2119792u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2119792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205870));
}
#[inline]
pub fn block_0x00205870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 156u32, 2119796u32)?;
    emu.lw_no_count(8usize, 2usize, 152u32, 2119800u32)?;
    emu.lw_no_count(9usize, 2usize, 148u32, 2119804u32)?;
    emu.lw_no_count(18usize, 2usize, 144u32, 2119808u32)?;
    emu.lw_no_count(19usize, 2usize, 140u32, 2119812u32)?;
    emu.lw_no_count(20usize, 2usize, 136u32, 2119816u32)?;
    emu.lw_no_count(21usize, 2usize, 132u32, 2119820u32)?;
    emu.lw_no_count(22usize, 2usize, 128u32, 2119824u32)?;
    emu.lw_no_count(23usize, 2usize, 124u32, 2119828u32)?;
    emu.lw_no_count(24usize, 2usize, 120u32, 2119832u32)?;
    emu.lw_no_count(25usize, 2usize, 116u32, 2119836u32)?;
    emu.adi_no_count(2usize, 2usize, 160u32, 2119840u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119844u32;
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
pub fn block_0x002058a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2119848u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 228u32, 2119852u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2119856u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2119860u32);
    emu.apc_no_count(1usize, 2119860u32, 86016u32, 2119864u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002058bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967072u32, 2119872u32);
    emu.sw_no_count(1usize, 2usize, 220u32, 2119876u32)?;
    emu.sw_no_count(8usize, 2usize, 216u32, 2119880u32)?;
    emu.sw_no_count(9usize, 2usize, 212u32, 2119884u32)?;
    emu.sw_no_count(18usize, 2usize, 208u32, 2119888u32)?;
    emu.sw_no_count(19usize, 2usize, 204u32, 2119892u32)?;
    emu.sw_no_count(20usize, 2usize, 200u32, 2119896u32)?;
    emu.sw_no_count(21usize, 2usize, 196u32, 2119900u32)?;
    emu.sw_no_count(22usize, 2usize, 192u32, 2119904u32)?;
    emu.sw_no_count(23usize, 2usize, 188u32, 2119908u32)?;
    emu.sw_no_count(24usize, 2usize, 184u32, 2119912u32)?;
    emu.sw_no_count(25usize, 2usize, 180u32, 2119916u32)?;
    emu.sw_no_count(26usize, 2usize, 176u32, 2119920u32)?;
    emu.sw_no_count(27usize, 2usize, 172u32, 2119924u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2119928u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2119932u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2119936u32)?;
    let a = 0u32.wrapping_add(16384u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2119940u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 12usize, 0u32, 2119944u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205910));
    } else {
        emu.pc = 2119948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020590c));
    }
}
#[inline(always)]
pub fn block_0x0020590c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16384u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2119952u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2119952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205910));
}
#[inline(always)]
pub fn block_0x00205910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2120616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ba8));
    } else {
        emu.pc = 2119956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205914));
    }
}
#[inline(always)]
pub fn block_0x00205914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(19usize, 20usize, 6u32, 2119960u32);
    emu.apc_no_count(1usize, 2119960u32, 12288u32, 2119964u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2119972u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965652u32, 2119976u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2119980u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2119984u32);
    emu.apc_no_count(1usize, 2119984u32, 16384u32, 2119988u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119992u32;
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
pub fn block_0x00205938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2120720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205c10));
    } else {
        emu.pc = 2119996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020593c));
    }
}
#[inline(always)]
pub fn block_0x0020593c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 12u32, 2120000u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2120004u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2120008u32)?;
    emu.adi_no_count(20usize, 2usize, 144u32, 2120012u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2120016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2120188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002059fc));
}
#[inline]
pub fn block_0x00205950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2120020u32);
    emu.sli_no_count(24usize, 24usize, 8u32, 2120024u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2120028u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2120032u32)?;
    emu.sli_no_count(11usize, 25usize, 6u32, 2120036u32);
    emu.orr_no_count(12usize, 24usize, 23usize, 2120040u32);
    emu.sri_no_count(13usize, 22usize, 24u32, 2120044u32);
    emu.orr_no_count(8usize, 8usize, 19usize, 2120048u32);
    emu.sri_no_count(14usize, 22usize, 16u32, 2120052u32);
    emu.adr_no_count(19usize, 10usize, 11usize, 2120056u32);
    emu.sri_no_count(10usize, 22usize, 8u32, 2120060u32);
    emu.sb_no_count(10usize, 19usize, 4u32, 2120064u32);
    emu.sb_no_count(14usize, 19usize, 5u32, 2120068u32);
    emu.sb_no_count(13usize, 19usize, 6u32, 2120072u32);
    emu.sri_no_count(10usize, 12usize, 8u32, 2120076u32);
    emu.sb_no_count(12usize, 19usize, 0u32, 2120080u32);
    emu.sb_no_count(10usize, 19usize, 1u32, 2120084u32);
    emu.sb_no_count(21usize, 19usize, 2u32, 2120088u32);
    emu.sb_no_count(22usize, 19usize, 3u32, 2120092u32);
    emu.adi_no_count(10usize, 19usize, 7u32, 2120096u32);
    emu.adi_no_count(11usize, 2usize, 52u32, 2120100u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2120104u32);
    emu.apc_no_count(1usize, 2120104u32, 16384u32, 2120108u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120112u32;
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
#[inline]
pub fn block_0x002059b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 24u32, 2120116u32);
    emu.sri_no_count(11usize, 27usize, 16u32, 2120120u32);
    emu.sri_no_count(12usize, 27usize, 8u32, 2120124u32);
    emu.sb_no_count(12usize, 19usize, 36u32, 2120128u32);
    emu.sb_no_count(11usize, 19usize, 37u32, 2120132u32);
    emu.sb_no_count(10usize, 19usize, 38u32, 2120136u32);
    emu.sri_no_count(10usize, 8usize, 8u32, 2120140u32);
    emu.sb_no_count(8usize, 19usize, 32u32, 2120144u32);
    emu.sb_no_count(10usize, 19usize, 33u32, 2120148u32);
    emu.sb_no_count(26usize, 19usize, 34u32, 2120152u32);
    emu.sb_no_count(27usize, 19usize, 35u32, 2120156u32);
    emu.adi_no_count(10usize, 19usize, 39u32, 2120160u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2120164u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2120168u32);
    emu.apc_no_count(1usize, 2120168u32, 16384u32, 2120172u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120176u32;
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
pub fn block_0x002059f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2120180u32);
    emu.sw_no_count(25usize, 2usize, 20u32, 2120184u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2120632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bb8));
    } else {
        emu.pc = 2120188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002059fc));
    }
}
#[inline(always)]
pub fn block_0x002059fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 136u32, 2120192u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2120196u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2120200u32);
    emu.apc_no_count(1usize, 2120200u32, 4294946816u32, 2120204u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 136u32, 2120212u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2120592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205b90));
    } else {
        emu.pc = 2120216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a18));
    }
}
#[inline(never)]
pub fn block_0x00205a18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 20usize, 0u32, 2120220u32)?;
    emu.lw_no_count(11usize, 20usize, 4u32, 2120224u32)?;
    emu.lw_no_count(12usize, 20usize, 8u32, 2120228u32)?;
    emu.lw_no_count(13usize, 20usize, 12u32, 2120232u32)?;
    emu.lw_no_count(14usize, 20usize, 16u32, 2120236u32)?;
    emu.lw_no_count(15usize, 20usize, 20u32, 2120240u32)?;
    emu.lbu_no_count(16usize, 20usize, 24u32, 2120244u32);
    emu.sw_no_count(10usize, 2usize, 108u32, 2120248u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2120252u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2120256u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2120260u32)?;
    emu.sw_no_count(14usize, 2usize, 124u32, 2120264u32)?;
    emu.sw_no_count(15usize, 2usize, 128u32, 2120268u32)?;
    emu.sb_no_count(16usize, 2usize, 132u32, 2120272u32);
    emu.lw_no_count(10usize, 2usize, 108u32, 2120276u32)?;
    emu.lw_no_count(11usize, 2usize, 112u32, 2120280u32)?;
    emu.lw_no_count(12usize, 2usize, 116u32, 2120284u32)?;
    emu.lw_no_count(13usize, 2usize, 120u32, 2120288u32)?;
    emu.lw_no_count(14usize, 2usize, 124u32, 2120292u32)?;
    emu.lw_no_count(15usize, 2usize, 128u32, 2120296u32)?;
    emu.lbu_no_count(16usize, 2usize, 132u32, 2120300u32);
    emu.sw_no_count(10usize, 2usize, 80u32, 2120304u32)?;
    emu.sw_no_count(11usize, 2usize, 84u32, 2120308u32)?;
    emu.sw_no_count(12usize, 2usize, 88u32, 2120312u32)?;
    emu.sw_no_count(13usize, 2usize, 92u32, 2120316u32)?;
    emu.lbu_no_count(23usize, 2usize, 137u32, 2120320u32);
    emu.lbu_no_count(24usize, 2usize, 138u32, 2120324u32);
    emu.lbu_no_count(21usize, 2usize, 139u32, 2120328u32);
    emu.lw_no_count(22usize, 2usize, 140u32, 2120332u32)?;
    emu.sw_no_count(14usize, 2usize, 96u32, 2120336u32)?;
    emu.sw_no_count(15usize, 2usize, 100u32, 2120340u32)?;
    emu.sb_no_count(16usize, 2usize, 104u32, 2120344u32);
    emu.adi_no_count(10usize, 2usize, 136u32, 2120348u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2120352u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2120356u32);
    emu.apc_no_count(1usize, 2120356u32, 4294946816u32, 2120360u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 136u32, 2120368u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2120592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205b90));
    } else {
        emu.pc = 2120372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ab4));
    }
}
#[inline(never)]
pub fn block_0x00205ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 49u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 20usize, 0u32, 2120376u32)?;
    emu.lw_no_count(11usize, 20usize, 4u32, 2120380u32)?;
    emu.lw_no_count(12usize, 20usize, 8u32, 2120384u32)?;
    emu.lw_no_count(13usize, 20usize, 12u32, 2120388u32)?;
    emu.lw_no_count(14usize, 20usize, 16u32, 2120392u32)?;
    emu.lw_no_count(15usize, 20usize, 20u32, 2120396u32)?;
    emu.lbu_no_count(16usize, 20usize, 24u32, 2120400u32);
    emu.lw_no_count(17usize, 2usize, 80u32, 2120404u32)?;
    emu.lw_no_count(5usize, 2usize, 84u32, 2120408u32)?;
    emu.lw_no_count(6usize, 2usize, 88u32, 2120412u32)?;
    emu.lw_no_count(7usize, 2usize, 92u32, 2120416u32)?;
    emu.lw_no_count(28usize, 2usize, 96u32, 2120420u32)?;
    emu.lw_no_count(29usize, 2usize, 100u32, 2120424u32)?;
    emu.lbu_no_count(30usize, 2usize, 104u32, 2120428u32);
    emu.sw_no_count(10usize, 2usize, 108u32, 2120432u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2120436u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2120440u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2120444u32)?;
    emu.lw_no_count(10usize, 2usize, 12u32, 2120448u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2120452u32)?;
    emu.sw_no_count(14usize, 2usize, 124u32, 2120456u32)?;
    emu.sw_no_count(15usize, 2usize, 128u32, 2120460u32)?;
    emu.sb_no_count(16usize, 2usize, 132u32, 2120464u32);
    emu.sw_no_count(17usize, 2usize, 52u32, 2120468u32)?;
    emu.sw_no_count(5usize, 2usize, 56u32, 2120472u32)?;
    emu.sw_no_count(6usize, 2usize, 60u32, 2120476u32)?;
    emu.sw_no_count(7usize, 2usize, 64u32, 2120480u32)?;
    emu.lw_no_count(11usize, 2usize, 108u32, 2120484u32)?;
    emu.lw_no_count(12usize, 2usize, 112u32, 2120488u32)?;
    emu.lw_no_count(13usize, 2usize, 116u32, 2120492u32)?;
    emu.lw_no_count(14usize, 2usize, 120u32, 2120496u32)?;
    emu.lw_no_count(15usize, 2usize, 124u32, 2120500u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2120504u32)?;
    emu.lbu_no_count(17usize, 2usize, 132u32, 2120508u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2120512u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2120516u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2120520u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2120524u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2120528u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2120532u32)?;
    emu.sb_no_count(17usize, 2usize, 48u32, 2120536u32);
    emu.lbu_no_count(19usize, 2usize, 137u32, 2120540u32);
    emu.lbu_no_count(8usize, 2usize, 138u32, 2120544u32);
    emu.lbu_no_count(26usize, 2usize, 139u32, 2120548u32);
    emu.lw_no_count(27usize, 2usize, 140u32, 2120552u32)?;
    emu.sw_no_count(28usize, 2usize, 68u32, 2120556u32)?;
    emu.sw_no_count(29usize, 2usize, 72u32, 2120560u32)?;
    emu.sb_no_count(30usize, 2usize, 76u32, 2120564u32);
    emu.add_memory_rw_events(48usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2120016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205950));
    } else {
        emu.pc = 2120568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205b78));
    }
}
#[inline(always)]
pub fn block_0x00205b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2120572u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120576u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 244u32, 2120580u32);
    emu.apc_no_count(1usize, 2120580u32, 12288u32, 2120584u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(28u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2120592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2120016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205950));
}
#[inline(always)]
pub fn block_0x00205b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 140u32, 2120596u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120600u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 2usize, 8u32, 2120604u32)?;
    emu.sw_no_count(11usize, 12usize, 0u32, 2120608u32)?;
    emu.sw_no_count(10usize, 12usize, 4u32, 2120612u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2120616u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2120660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205bd4));
}
#[inline(always)]
pub fn block_0x00205ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2120620u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2120624u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2120628u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2120632u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2120632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205bb8));
}
#[inline(always)]
pub fn block_0x00205bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2120636u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2120640u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2120644u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2120648u32)?;
    emu.sw_no_count(10usize, 13usize, 0u32, 2120652u32)?;
    emu.sw_no_count(11usize, 13usize, 4u32, 2120656u32)?;
    emu.sw_no_count(12usize, 13usize, 8u32, 2120660u32)?;
    emu.add_memory_rw_events(7usize);
    emu.pc = 2120660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205bd4));
}
#[inline]
pub fn block_0x00205bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 220u32, 2120664u32)?;
    emu.lw_no_count(8usize, 2usize, 216u32, 2120668u32)?;
    emu.lw_no_count(9usize, 2usize, 212u32, 2120672u32)?;
    emu.lw_no_count(18usize, 2usize, 208u32, 2120676u32)?;
    emu.lw_no_count(19usize, 2usize, 204u32, 2120680u32)?;
    emu.lw_no_count(20usize, 2usize, 200u32, 2120684u32)?;
    emu.lw_no_count(21usize, 2usize, 196u32, 2120688u32)?;
    emu.lw_no_count(22usize, 2usize, 192u32, 2120692u32)?;
    emu.lw_no_count(23usize, 2usize, 188u32, 2120696u32)?;
    emu.lw_no_count(24usize, 2usize, 184u32, 2120700u32)?;
    emu.lw_no_count(25usize, 2usize, 180u32, 2120704u32)?;
    emu.lw_no_count(26usize, 2usize, 176u32, 2120708u32)?;
    emu.lw_no_count(27usize, 2usize, 172u32, 2120712u32)?;
    emu.adi_no_count(2usize, 2usize, 224u32, 2120716u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120720u32;
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
pub fn block_0x00205c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2120724u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 228u32, 2120728u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2120732u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2120736u32);
    emu.apc_no_count(1usize, 2120736u32, 86016u32, 2120740u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120744u32;
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
#[inline]
pub fn block_0x00205c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2120748u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2120752u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2120756u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2120760u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2120764u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2120768u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2120772u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2120776u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2120780u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2120784u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2120788u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2120792u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2120796u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2120800u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120804u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 12usize, 0u32, 2120808u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2120816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205c70));
    } else {
        emu.pc = 2120812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205c6c));
    }
}
#[inline(always)]
pub fn block_0x00205c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2120816u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2120816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205c70));
}
#[inline(always)]
pub fn block_0x00205c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2120984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205d18));
    } else {
        emu.pc = 2120820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205c74));
    }
}
#[inline(always)]
pub fn block_0x00205c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2120820u32, 12288u32, 2120824u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120832u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965652u32, 2120836u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2120840u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2120844u32);
    emu.apc_no_count(1usize, 2120844u32, 16384u32, 2120848u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120852u32;
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
pub fn block_0x00205c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205da0));
    } else {
        emu.pc = 2120856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205c98));
    }
}
#[inline]
pub fn block_0x00205c98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2120860u32);
    emu.lw_no_count(21usize, 18usize, 0u32, 2120864u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2120868u32)?;
    emu.sw_no_count(19usize, 2usize, 4u32, 2120872u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2120876u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2120880u32)?;
    emu.sbr_no_count(22usize, 0usize, 23usize, 2120884u32);
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2120888u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2120892u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 244u32, 2120896u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2120900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2120928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205ce0));
}
#[inline(always)]
pub fn block_0x00205cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2120904u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2120908u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2120912u32);
    emu.sb_no_count(24usize, 10usize, 0u32, 2120916u32);
    emu.sw_no_count(20usize, 2usize, 12u32, 2120920u32)?;
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2120924u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2121000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205d28));
    } else {
        emu.pc = 2120928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ce0));
    }
}
#[inline(always)]
pub fn block_0x00205ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 22usize, 20usize, 2120932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2121028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205d44));
    } else {
        emu.pc = 2120936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ce8));
    }
}
#[inline(always)]
pub fn block_0x00205ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 20usize, 2120940u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2120944u32)?;
    emu.lbu_no_count(24usize, 10usize, 0u32, 2120948u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2120952u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2120956u32)?;
    emu.sw_no_count(23usize, 18usize, 4u32, 2120960u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2120900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205cc4));
    } else {
        emu.pc = 2120964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205d04));
    }
}
#[inline(always)]
pub fn block_0x00205d04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2120968u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2120972u32);
    emu.apc_no_count(1usize, 2120972u32, 86016u32, 2120976u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2120984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2120900u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205cc4));
}
#[inline(always)]
pub fn block_0x00205d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2120988u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2120992u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2120996u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2121000u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2121000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205d28));
}
#[inline(always)]
pub fn block_0x00205d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2121004u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2121008u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2121012u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2121016u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2121020u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2121024u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2121028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121072u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205d70));
}
#[inline(always)]
pub fn block_0x00205d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2121032u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121036u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2121040u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2121044u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2121048u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2121052u32);
    emu.apc_no_count(1usize, 2121052u32, 16384u32, 2121056u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121064u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2121068u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2121072u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2121072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205d70));
}
#[inline]
pub fn block_0x00205d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2121076u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2121080u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2121084u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2121088u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2121092u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2121096u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2121100u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2121104u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2121108u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2121112u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2121116u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121120u32;
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
pub fn block_0x00205da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2121124u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 228u32, 2121128u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2121132u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2121136u32);
    emu.apc_no_count(1usize, 2121136u32, 86016u32, 2121140u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121144u32;
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
#[inline]
pub fn block_0x00205db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2121148u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2121152u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2121156u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2121160u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2121164u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2121168u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2121172u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2121176u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2121180u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2121184u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2121188u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2121192u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2121372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e9c));
    } else {
        emu.pc = 2121196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205dec));
    }
}
#[inline]
pub fn block_0x00205dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 13usize, 0u32, 2121200u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2121204u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2121208u32);
    emu.adi_no_count(21usize, 0usize, 4u32, 2121212u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2121216u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 300u32, 2121220u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2121224u32);
    emu.adi_no_count(23usize, 0usize, 35u32, 2121228u32);
    emu.adi_no_count(24usize, 0usize, 2u32, 2121232u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2121236u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205e24));
}
#[inline(always)]
pub fn block_0x00205e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2121240u32)?;
    emu.lbu_no_count(11usize, 11usize, 8u32, 2121244u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2121388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205eac));
    } else {
        emu.pc = 2121248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e20));
    }
}
#[inline(always)]
pub fn block_0x00205e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e9c));
    } else {
        emu.pc = 2121252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e24));
    }
}
#[inline(always)]
pub fn block_0x00205e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2121256u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2121260u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2121264u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2121268u32);
    emu.apc_no_count(1usize, 2121268u32, 16384u32, 2121272u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121276u32;
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
#[inline(always)]
pub fn block_0x00205e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 0u32, 2121280u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2121308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e5c));
    } else {
        emu.pc = 2121284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e44));
    }
}
#[inline(always)]
pub fn block_0x00205e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2121288u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2121384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ea8));
    } else {
        emu.pc = 2121292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e4c));
    }
}
#[inline(always)]
pub fn block_0x00205e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2121452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205eec));
    } else {
        emu.pc = 2121296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e50));
    }
}
#[inline(always)]
pub fn block_0x00205e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(9usize, 9usize, 10usize, 2121300u32);
    emu.adr_no_count(18usize, 18usize, 10usize, 2121304u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2121308u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121248u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205e20));
}
#[inline(always)]
pub fn block_0x00205e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2121312u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2121356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e8c));
    } else {
        emu.pc = 2121316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e64));
    }
}
#[inline(always)]
pub fn block_0x00205e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2121236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e14));
    } else {
        emu.pc = 2121320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e68));
    }
}
#[inline(always)]
pub fn block_0x00205e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2121324u32)?;
    emu.lbu_no_count(12usize, 11usize, 8u32, 2121328u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2121388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205eac));
    } else {
        emu.pc = 2121332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e74));
    }
}
#[inline(always)]
pub fn block_0x00205e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 4u32, 2121336u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2121340u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2121248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e20));
    } else {
        emu.pc = 2121344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e80));
    }
}
#[inline(always)]
pub fn block_0x00205e80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2121348u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2121352u32;
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
pub fn block_0x00205e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2121356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121248u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205e20));
}
#[inline(always)]
pub fn block_0x00205e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2121388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205eac));
    } else {
        emu.pc = 2121360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e90));
    }
}
#[inline(always)]
pub fn block_0x00205e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 1u32, 2121364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2121248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e20));
    } else {
        emu.pc = 2121368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e98));
    }
}
#[inline(always)]
pub fn block_0x00205e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2121372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205eac));
}
#[inline(always)]
pub fn block_0x00205e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2121376u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2121380u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2121384u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205ebc));
}
#[inline(always)]
pub fn block_0x00205ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2121388u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2121388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205eac));
}
#[inline(always)]
pub fn block_0x00205eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2121392u32)?;
    emu.lw_no_count(10usize, 10usize, 4u32, 2121396u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2121400u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2121404u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2121404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205ebc));
}
#[inline]
pub fn block_0x00205ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2121408u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2121412u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2121416u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2121420u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2121424u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2121428u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2121432u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2121436u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2121440u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2121444u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2121448u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121452u32;
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
pub fn block_0x00205eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2121456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 308u32, 2121460u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2121464u32);
    emu.apc_no_count(1usize, 2121464u32, 102400u32, 2121468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
