pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2137816u32;
pub const PC_MAX: u32 = 2140744u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 127usize] = [
        block_0x00209ed8,
        block_0x00209ee0,
        block_0x00209ef4,
        block_0x00209f1c,
        block_0x00209f28,
        block_0x00209f30,
        block_0x00209f38,
        block_0x00209f4c,
        block_0x00209f80,
        block_0x00209f8c,
        block_0x00209fa4,
        block_0x00209fb8,
        block_0x00209fc0,
        block_0x00209fc8,
        block_0x00209fd0,
        block_0x00209fdc,
        block_0x0020a000,
        block_0x0020a01c,
        block_0x0020a07c,
        block_0x0020a0a0,
        block_0x0020a0d0,
        block_0x0020a0e0,
        block_0x0020a0e4,
        block_0x0020a114,
        block_0x0020a124,
        block_0x0020a130,
        block_0x0020a1a0,
        block_0x0020a1d4,
        block_0x0020a1e0,
        block_0x0020a1ec,
        block_0x0020a240,
        block_0x0020a24c,
        block_0x0020a264,
        block_0x0020a284,
        block_0x0020a294,
        block_0x0020a298,
        block_0x0020a2bc,
        block_0x0020a2cc,
        block_0x0020a2d8,
        block_0x0020a2e8,
        block_0x0020a300,
        block_0x0020a318,
        block_0x0020a31c,
        block_0x0020a320,
        block_0x0020a32c,
        block_0x0020a330,
        block_0x0020a33c,
        block_0x0020a368,
        block_0x0020a398,
        block_0x0020a3bc,
        block_0x0020a40c,
        block_0x0020a418,
        block_0x0020a424,
        block_0x0020a474,
        block_0x0020a47c,
        block_0x0020a49c,
        block_0x0020a4bc,
        block_0x0020a504,
        block_0x0020a510,
        block_0x0020a51c,
        block_0x0020a538,
        block_0x0020a53c,
        block_0x0020a55c,
        block_0x0020a560,
        block_0x0020a568,
        block_0x0020a574,
        block_0x0020a5dc,
        block_0x0020a628,
        block_0x0020a640,
        block_0x0020a64c,
        block_0x0020a650,
        block_0x0020a684,
        block_0x0020a69c,
        block_0x0020a6a0,
        block_0x0020a6b0,
        block_0x0020a6b8,
        block_0x0020a6e8,
        block_0x0020a6f0,
        block_0x0020a6f8,
        block_0x0020a6fc,
        block_0x0020a708,
        block_0x0020a718,
        block_0x0020a76c,
        block_0x0020a77c,
        block_0x0020a780,
        block_0x0020a78c,
        block_0x0020a7a8,
        block_0x0020a7c4,
        block_0x0020a7c8,
        block_0x0020a7e4,
        block_0x0020a7e8,
        block_0x0020a7f8,
        block_0x0020a800,
        block_0x0020a808,
        block_0x0020a818,
        block_0x0020a838,
        block_0x0020a84c,
        block_0x0020a860,
        block_0x0020a878,
        block_0x0020a8a0,
        block_0x0020a8c0,
        block_0x0020a8c8,
        block_0x0020a8d8,
        block_0x0020a8dc,
        block_0x0020a8e0,
        block_0x0020a8e4,
        block_0x0020a8f0,
        block_0x0020a8fc,
        block_0x0020a908,
        block_0x0020a90c,
        block_0x0020a91c,
        block_0x0020a930,
        block_0x0020a944,
        block_0x0020a94c,
        block_0x0020a960,
        block_0x0020a964,
        block_0x0020a968,
        block_0x0020a96c,
        block_0x0020a97c,
        block_0x0020a998,
        block_0x0020a9ac,
        block_0x0020a9c0,
        block_0x0020a9e4,
        block_0x0020a9e8,
        block_0x0020aa20,
        block_0x0020aa34,
        block_0x0020aa48,
    ];
    const IDX: [u16; 733usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 7u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 13u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16,
        16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        22u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        24u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 29u16,
        0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16,
        32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 34u16, 0u16, 0u16, 0u16, 35u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16,
        40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16,
        43u16, 44u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 52u16,
        0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 55u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 59u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 63u16, 64u16, 0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 70u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16,
        0u16, 79u16, 80u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 91u16, 0u16, 0u16, 0u16,
        92u16, 0u16, 93u16, 0u16, 94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16,
        98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16,
        102u16, 0u16, 0u16, 0u16, 103u16, 104u16, 105u16, 106u16, 0u16, 0u16, 107u16,
        0u16, 0u16, 108u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 111u16, 0u16,
        0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 114u16, 0u16,
        0u16, 0u16, 0u16, 115u16, 116u16, 117u16, 118u16, 0u16, 0u16, 0u16, 119u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16,
        0u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16,
        124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 125u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 127u16,
    ];
    if pc < 2137816u32 || pc > 2140744u32 {
        return None;
    }
    let word_offset = ((pc - 2137816u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00209ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2137816u32, 4096u32, 2137820u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2137824u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2137828u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2137832u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2137836u32);
    emu.apc_no_count(1usize, 2137836u32, 4096u32, 2137840u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2137848u32);
    emu.lw_no_count(10usize, 19usize, 0u32, 2137852u32)?;
    emu.lw_no_count(11usize, 19usize, 4u32, 2137856u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2137860u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2137864u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2137868u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2137872u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2137876u32);
    emu.apc_no_count(1usize, 2137876u32, 4096u32, 2137880u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 8u32, 2137888u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2137892u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2138020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fa4));
    } else {
        emu.pc = 2137896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f28));
    }
}
#[inline(always)]
pub fn block_0x00209f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2137900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2137996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f8c));
    } else {
        emu.pc = 2137904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f30));
    }
}
#[inline(always)]
pub fn block_0x00209f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2137908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fd0));
    } else {
        emu.pc = 2137912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f38));
    }
}
#[inline(always)]
pub fn block_0x00209f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137916u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 4294966464u32, 2137920u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2137924u32);
    emu.sb_no_count(10usize, 11usize, 4294966464u32, 2137928u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2138064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fd0));
    } else {
        emu.pc = 2137932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f4c));
    }
}
#[inline]
pub fn block_0x00209f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137936u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966472u32, 2137940u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2137944u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2137948u32);
    emu.lw_no_count(13usize, 18usize, 36u32, 2137952u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2137956u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2137960u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2137964u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2137968u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2137972u32);
    emu.adi_no_count(12usize, 2usize, 20u32, 2137976u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2137980u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2137984u32;
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
pub fn block_0x00209f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 12u32, 2137988u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2137992u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2137996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138056u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209fc8));
}
#[inline(always)]
pub fn block_0x00209f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2138000u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2138004u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2138008u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2138012u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2138016u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2138020u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138040u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209fb8));
}
#[inline(always)]
pub fn block_0x00209fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2138024u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2138028u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2138032u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2138036u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2138040u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2138040u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209fb8));
}
#[inline(always)]
pub fn block_0x00209fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2138040u32, 4096u32, 2138044u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 20u32, 2138052u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2138056u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2138056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209fc8));
}
#[inline(always)]
pub fn block_0x00209fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2138056u32, 4294963200u32, 2138060u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2138068u32);
    emu.apc_no_count(6usize, 2138068u32, 0u32, 2138072u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138076u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138080u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2138084u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138088u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2138092u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2138096u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2138100u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2138104u32);
    emu.apc_no_count(1usize, 2138104u32, 0u32, 2138108u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2138116u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2138120u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2138124u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2138128u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2138132u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138136u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0a0));
    } else {
        emu.pc = 2138140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a01c));
    }
}
#[inline]
pub fn block_0x0020a01c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2138144u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2138148u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2138152u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2138156u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2138160u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2138164u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2138168u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138172u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2138176u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2138180u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2138184u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2138188u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2138192u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2138196u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2138200u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2138204u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2138208u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2138212u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138216u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966480u32, 2138220u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2138224u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2138228u32);
    emu.apc_no_count(1usize, 2138228u32, 24576u32, 2138232u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138236u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a07c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2138240u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2138244u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2138248u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2138252u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2138256u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2138260u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2138264u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2138268u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2138272u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2138272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0a0));
}
#[inline]
pub fn block_0x0020a0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2138276u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2138280u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2138284u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2138288u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2138292u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2138296u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2138300u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2138304u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2138308u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2138312u32)?;
    emu.apc_no_count(1usize, 2138312u32, 4294934528u32, 2138316u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2138324u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138328u32);
    emu.apc_no_count(1usize, 2138328u32, 4294930432u32, 2138332u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(68u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a114));
    } else {
        emu.pc = 2138340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0e4));
    }
}
#[inline]
pub fn block_0x0020a0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2138344u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2138348u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2138352u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138356u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966504u32, 2138360u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2138364u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2138368u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2138372u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2138376u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2138380u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2138384u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138388u32;
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
pub fn block_0x0020a114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138392u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2138396u32);
    emu.apc_no_count(1usize, 2138396u32, 4096u32, 2138400u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2138408u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138412u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2138580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1d4));
    } else {
        emu.pc = 2138416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a130));
    }
}
#[inline(never)]
pub fn block_0x0020a130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2138420u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2138424u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2138428u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2138432u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2138436u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2138440u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2138444u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2138448u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2138452u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2138456u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2138460u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2138464u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2138468u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2138472u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2138476u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2138480u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2138484u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2138488u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2138492u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2138496u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2138500u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138504u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966480u32, 2138508u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2138512u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2138516u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2138520u32);
    emu.apc_no_count(1usize, 2138520u32, 24576u32, 2138524u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2138532u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2138536u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2138540u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2138544u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2138548u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2138552u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2138556u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2138560u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2138564u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2138568u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2138572u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2138576u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2138580u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2138580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a1d4));
}
#[inline(always)]
pub fn block_0x0020a1d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138584u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966504u32, 2138588u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138592u32;
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
pub fn block_0x0020a1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2138596u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2138600u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2138700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a24c));
    } else {
        emu.pc = 2138604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1ec));
    }
}
#[inline]
pub fn block_0x0020a1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2138608u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2138612u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2138616u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2138620u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2138624u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2138628u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2138632u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2138636u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2138640u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2138644u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2138648u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2138652u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2138656u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2138660u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2138664u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2138668u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2138672u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2138676u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2138680u32);
    emu.apc_no_count(1usize, 2138680u32, 24576u32, 2138684u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2138692u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2138696u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138700u32;
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
pub fn block_0x0020a24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2138704u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2138708u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2138712u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2138716u32);
    emu.apc_no_count(6usize, 2138716u32, 28672u32, 2138720u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138724u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138728u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2138732u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2138736u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2138740u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2138744u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2138748u32)?;
    emu.apc_no_count(1usize, 2138748u32, 4294934528u32, 2138752u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138756u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2138760u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138764u32);
    emu.apc_no_count(1usize, 2138764u32, 4294930432u32, 2138768u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966928u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2bc));
    } else {
        emu.pc = 2138776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a298));
    }
}
#[inline]
pub fn block_0x0020a298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966520u32, 2138784u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2138788u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2138792u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2138796u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2138800u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2138804u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2138808u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138812u32;
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
pub fn block_0x0020a2bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138816u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2138820u32);
    emu.apc_no_count(1usize, 2138820u32, 4096u32, 2138824u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138832u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966520u32, 2138836u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138840u32;
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
pub fn block_0x0020a2d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2138844u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2138848u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2138852u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138856u32;
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
pub fn block_0x0020a2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2138860u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138864u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2138868u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2138872u32);
    emu.apc_no_count(6usize, 2138872u32, 28672u32, 2138876u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138880u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138884u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2138888u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2138892u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2138896u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2138900u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2138924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a32c));
    } else {
        emu.pc = 2138904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a318));
    }
}
#[inline(always)]
pub fn block_0x0020a318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a368));
    } else {
        emu.pc = 2138908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a31c));
    }
}
#[inline(always)]
pub fn block_0x0020a31c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a368));
    } else {
        emu.pc = 2138912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a320));
    }
}
#[inline(always)]
pub fn block_0x0020a320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2138916u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2138920u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2138924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138940u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a33c));
}
#[inline(always)]
pub fn block_0x0020a32c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a368));
    } else {
        emu.pc = 2138928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a330));
    }
}
#[inline(always)]
pub fn block_0x0020a330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2138932u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2138936u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2138940u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2138940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a33c));
}
#[inline]
pub fn block_0x0020a33c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2138944u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138948u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2138952u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2138956u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2138960u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2138964u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138968u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966536u32, 2138972u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2138976u32);
    emu.apc_no_count(1usize, 2138976u32, 0u32, 2138980u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2138988u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138992u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2138996u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2139000u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2139004u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139008u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2139012u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139016u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966564u32, 2139020u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2139024u32);
    emu.apc_no_count(1usize, 2139024u32, 0u32, 2139028u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2139036u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2139040u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2139044u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2139048u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2139052u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2139056u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2139060u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2139064u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2139068u32;
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
pub fn block_0x0020a3bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2139072u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2139076u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2139080u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2139084u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139088u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2139092u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2139096u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139100u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2139104u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2139108u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139112u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2139116u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2139120u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139124u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2139128u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2139132u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2139136u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2139140u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2139144u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a418));
    } else {
        emu.pc = 2139148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a40c));
    }
}
#[inline(always)]
pub fn block_0x0020a40c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2139152u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2139156u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a47c));
}
#[inline(always)]
pub fn block_0x0020a418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2139164u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2139168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2139172u32;
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
pub fn block_0x0020a424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2139176u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2139180u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2139184u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2139188u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139192u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2139196u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2139200u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139204u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2139208u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2139212u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139216u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2139220u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2139224u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139228u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2139232u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2139236u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2139240u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2139244u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2139248u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a49c));
    } else {
        emu.pc = 2139252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a474));
    }
}
#[inline(always)]
pub fn block_0x0020a474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2139256u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2139260u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a47c));
}
#[inline(always)]
pub fn block_0x0020a47c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2139264u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2139268u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2139272u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2139276u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2139280u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2139284u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139288u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139292u32;
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
pub fn block_0x0020a49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139296u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966592u32, 2139300u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2139304u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2139308u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2139312u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2139316u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139320u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139324u32;
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
pub fn block_0x0020a4bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2139328u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2139332u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2139336u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2139340u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2139344u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2139348u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2139352u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2139356u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2139360u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2139364u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2139368u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2139372u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2139376u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2139380u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2139384u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2139388u32);
    emu.apc_no_count(1usize, 2139388u32, 4096u32, 2139392u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2139400u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2139404u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a560));
    } else {
        emu.pc = 2139408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a510));
    }
}
#[inline(always)]
pub fn block_0x0020a510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139412u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966452u32, 2139416u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2139900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6fc));
    } else {
        emu.pc = 2139420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a51c));
    }
}
#[inline(always)]
pub fn block_0x0020a51c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 4294966452u32, 2139424u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2139428u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2139432u32);
    emu.sw_no_count(11usize, 10usize, 4294966452u32, 2139436u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2139440u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2139444u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2139724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a64c));
    } else {
        emu.pc = 2139448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a538));
    }
}
#[inline(always)]
pub fn block_0x0020a538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139452u32;
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
pub fn block_0x0020a53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2139456u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139460u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2139464u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2139468u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2139472u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2139476u32);
    emu.apc_no_count(1usize, 2139476u32, 0u32, 2139480u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2139488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a684));
}
#[inline(always)]
pub fn block_0x0020a560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2139492u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2139612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5dc));
    } else {
        emu.pc = 2139496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a568));
    }
}
#[inline(always)]
pub fn block_0x0020a568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2139500u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2139504u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139508u32;
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
#[inline(never)]
pub fn block_0x0020a574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2139512u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2139516u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139520u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967068u32, 2139524u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2139528u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2139532u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966356u32, 2139536u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2139540u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966720u32, 2139544u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2139548u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2139552u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2139556u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2139560u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2139564u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139568u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2139572u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2139576u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2139580u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2139584u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2139588u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2139592u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139596u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2139600u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2139604u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2139608u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2139612u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a628));
}
#[inline]
pub fn block_0x0020a5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2139616u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139620u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967068u32, 2139624u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2139628u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139632u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966684u32, 2139636u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139640u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966632u32, 2139644u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2139648u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139652u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2139656u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2139660u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2139664u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2139668u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2139672u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139676u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2139680u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2139684u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2139688u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2139688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a628));
}
#[inline(always)]
pub fn block_0x0020a628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2139692u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2139696u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2139700u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2139704u32);
    emu.apc_no_count(1usize, 2139704u32, 0u32, 2139708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2139716u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2139720u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6f0));
}
#[inline(always)]
pub fn block_0x0020a64c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139728u32;
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
pub fn block_0x0020a650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2139732u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966452u32, 2139736u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2139740u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2139744u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2139748u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2139752u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139756u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2139760u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2139764u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2139768u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2139772u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2139776u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2139780u32;
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
pub fn block_0x0020a684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139784u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966452u32, 2139788u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2139792u32);
    emu.sw_no_count(11usize, 10usize, 4294966452u32, 2139796u32)?;
    emu.apc_no_count(1usize, 2139796u32, 4096u32, 2139800u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139804u32;
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
pub fn block_0x0020a69c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2139824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6b0));
    } else {
        emu.pc = 2139808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6a0));
    }
}
#[inline(always)]
pub fn block_0x0020a6a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2139812u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2139816u32);
    emu.apc_no_count(1usize, 2139816u32, 0u32, 2139820u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(96u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139828u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966792u32, 2139832u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6b8));
}
#[inline]
pub fn block_0x0020a6b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2139836u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139840u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2139844u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2139848u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139852u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2139856u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2139860u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2139864u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2139868u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2139872u32);
    emu.apc_no_count(1usize, 2139872u32, 0u32, 2139876u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139880u32;
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
pub fn block_0x0020a6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2139884u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2139888u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6f0));
}
#[inline(always)]
pub fn block_0x0020a6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2139888u32, 4294963200u32, 2139892u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a6f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2139896u32));
}
#[inline(always)]
pub fn block_0x0020a6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139904u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966940u32, 2139908u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139912u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6b8));
}
#[inline(always)]
pub fn block_0x0020a708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2139916u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2139920u32)?;
    emu.apc_no_count(1usize, 2139920u32, 4294934528u32, 2139924u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2139932u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2139936u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966404u32, 2139944u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2139948u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2139952u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2139956u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2139960u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2139964u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2139968u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139972u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2139976u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2139980u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2139984u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2139988u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2139992u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2139996u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2140000u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2140004u32);
    emu.apc_no_count(1usize, 2140004u32, 0u32, 2140008u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2140016u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2140020u32)?;
    emu.apc_no_count(1usize, 2140020u32, 4294963200u32, 2140024u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(12u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a77c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2140028u32));
}
#[inline(always)]
pub fn block_0x0020a780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2140036u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2140040u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140044u32;
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
pub fn block_0x0020a78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2140048u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2140052u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2140056u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2140060u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2140064u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2140068u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140072u32;
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
pub fn block_0x0020a7a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2140076u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2140080u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2140084u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2140088u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2140092u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2140096u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2140100u32;
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
pub fn block_0x0020a7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2140100u32));
}
#[inline(always)]
pub fn block_0x0020a7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140108u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140112u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140116u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2140120u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140124u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2140128u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2140132u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2140132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7e4));
}
#[inline(always)]
pub fn block_0x0020a7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2140160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a800));
    } else {
        emu.pc = 2140136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7e8));
    }
}
#[inline(always)]
pub fn block_0x0020a7e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2140140u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2140144u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2140148u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2140132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7e4));
    } else {
        emu.pc = 2140152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7f8));
    }
}
#[inline(always)]
pub fn block_0x0020a7f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2140156u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2140160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a808));
}
#[inline(always)]
pub fn block_0x0020a800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2140164u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140168u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2140168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a808));
}
#[inline(always)]
pub fn block_0x0020a808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2140172u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2140176u32);
    emu.apc_no_count(1usize, 2140176u32, 4294934528u32, 2140180u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140188u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140192u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2140196u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2140200u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140204u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2140208u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140212u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140216u32;
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
pub fn block_0x0020a838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140220u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140224u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140228u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140232u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2140256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a860));
    } else {
        emu.pc = 2140236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a84c));
    }
}
#[inline(always)]
pub fn block_0x0020a84c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2140240u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2140244u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2140248u32);
    emu.apc_no_count(1usize, 2140248u32, 4294934528u32, 2140252u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140256u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140260u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140264u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2140268u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140272u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140276u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140280u32;
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
pub fn block_0x0020a878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2140284u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2140288u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2140292u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2140296u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2140300u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2140304u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2140308u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2140312u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140316u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2140388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e4));
    } else {
        emu.pc = 2140320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8a0));
    }
}
#[inline(always)]
pub fn block_0x0020a8a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140324u32);
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2140328u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 13usize, 4294967295u32, 2140332u32);
    emu.sli_no_count(11usize, 13usize, 3u32, 2140336u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2140340u32);
    emu.anr_no_count(14usize, 14usize, 19usize, 2140344u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2140348u32);
    emu.adi_no_count(15usize, 12usize, 4u32, 2140352u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2140352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8c0));
}
#[inline(always)]
pub fn block_0x0020a8c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2140356u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2140380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8dc));
    } else {
        emu.pc = 2140360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8c8));
    }
}
#[inline(always)]
pub fn block_0x0020a8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2140364u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2140368u32);
    emu.adi_no_count(15usize, 15usize, 8u32, 2140372u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8c0));
    } else {
        emu.pc = 2140376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8d8));
    }
}
#[inline(always)]
pub fn block_0x0020a8d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2140380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8dc));
}
#[inline(always)]
pub fn block_0x0020a8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2140704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa20));
    } else {
        emu.pc = 2140384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e0));
    }
}
#[inline(always)]
pub fn block_0x0020a8e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2140400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8f0));
    } else {
        emu.pc = 2140388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e4));
    }
}
#[inline(always)]
pub fn block_0x0020a8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140392u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140396u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2140400u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9c0));
}
#[inline(always)]
pub fn block_0x0020a8f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 10usize, 3u32, 2140404u32);
    emu.adr_no_count(20usize, 12usize, 20usize, 2140408u32);
    emu.sbr_no_count(9usize, 13usize, 10usize, 2140412u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2140412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8fc));
}
#[inline(always)]
pub fn block_0x0020a8fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 9usize, 3u32, 2140416u32);
    emu.adi_no_count(10usize, 20usize, 4294967288u32, 2140420u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2140424u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2140424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a908));
}
#[inline(always)]
pub fn block_0x0020a908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a998));
    } else {
        emu.pc = 2140428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a90c));
    }
}
#[inline(always)]
pub fn block_0x0020a90c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2140432u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2140436u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2140440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2140424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a908));
    } else {
        emu.pc = 2140444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a91c));
    }
}
#[inline(always)]
pub fn block_0x0020a91c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2140448u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2140452u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2140456u32);
    emu.apc_no_count(1usize, 2140456u32, 4294934528u32, 2140460u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140468u32);
    emu.adr_no_count(11usize, 9usize, 19usize, 2140472u32);
    emu.anr_no_count(11usize, 11usize, 19usize, 2140476u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2140480u32);
    emu.adi_no_count(12usize, 20usize, 4u32, 2140484u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2140484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a944));
}
#[inline(always)]
pub fn block_0x0020a944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2140488u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2140516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a964));
    } else {
        emu.pc = 2140492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a94c));
    }
}
#[inline(always)]
pub fn block_0x0020a94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(18usize, 18usize, 13usize, 2140496u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2140500u32);
    emu.adi_no_count(21usize, 21usize, 4294967288u32, 2140504u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2140508u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2140484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a944));
    } else {
        emu.pc = 2140512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a960));
    }
}
#[inline(always)]
pub fn block_0x0020a960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2140516u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a964));
}
#[inline(always)]
pub fn block_0x0020a964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa34));
    } else {
        emu.pc = 2140520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a968));
    }
}
#[inline(always)]
pub fn block_0x0020a968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2140644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9e4));
    } else {
        emu.pc = 2140524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a96c));
    }
}
#[inline(always)]
pub fn block_0x0020a96c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 3u32, 2140528u32);
    emu.adr_no_count(20usize, 20usize, 11usize, 2140532u32);
    emu.lw_no_count(11usize, 20usize, 4u32, 2140536u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2140744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa48));
    } else {
        emu.pc = 2140540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a97c));
    }
}
#[inline(always)]
pub fn block_0x0020a97c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2140544u32)?;
    emu.sbr_no_count(9usize, 9usize, 10usize, 2140548u32);
    emu.sbr_no_count(10usize, 11usize, 18usize, 2140552u32);
    emu.adr_no_count(12usize, 12usize, 18usize, 2140556u32);
    emu.sw_no_count(12usize, 20usize, 0u32, 2140560u32)?;
    emu.sw_no_count(10usize, 20usize, 4u32, 2140564u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2140568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8fc));
}
#[inline(always)]
pub fn block_0x0020a998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2140572u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140576u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2140580u32);
    emu.apc_no_count(1usize, 2140580u32, 4294934528u32, 2140584u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140592u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294967088u32, 2140596u32)?;
    emu.lw_no_count(10usize, 10usize, 4294967092u32, 2140600u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2140604u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2140608u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2140608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9c0));
}
#[inline]
pub fn block_0x0020a9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2140612u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2140616u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2140620u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2140624u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2140628u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2140632u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2140636u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2140640u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140644u32;
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
pub fn block_0x0020a9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2140388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e4));
    } else {
        emu.pc = 2140648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9e8));
    }
}
#[inline]
pub fn block_0x0020a9e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140652u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966988u32, 2140656u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140660u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2140664u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2140668u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2140672u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2140676u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2140680u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2140684u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966996u32, 2140692u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2140696u32);
    emu.apc_no_count(1usize, 2140696u32, 8192u32, 2140700u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aa20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140708u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967028u32, 2140712u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140716u32);
    emu.apc_no_count(1usize, 2140716u32, 36864u32, 2140720u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aa34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140728u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967028u32, 2140732u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2140736u32);
    emu.apc_no_count(1usize, 2140736u32, 36864u32, 2140740u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140744u32;
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
pub fn block_0x0020aa48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140748u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967012u32, 2140752u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2140756u32);
    emu.apc_no_count(1usize, 2140756u32, 36864u32, 2140760u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
