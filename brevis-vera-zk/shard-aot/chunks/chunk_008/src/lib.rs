pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2137572u32;
pub const PC_MAX: u32 = 2140500u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 127usize] = [
        block_0x00209de4,
        block_0x00209dec,
        block_0x00209e00,
        block_0x00209e28,
        block_0x00209e34,
        block_0x00209e3c,
        block_0x00209e44,
        block_0x00209e58,
        block_0x00209e8c,
        block_0x00209e98,
        block_0x00209eb0,
        block_0x00209ec4,
        block_0x00209ecc,
        block_0x00209ed4,
        block_0x00209edc,
        block_0x00209ee8,
        block_0x00209f0c,
        block_0x00209f28,
        block_0x00209f88,
        block_0x00209fac,
        block_0x00209fdc,
        block_0x00209fec,
        block_0x00209ff0,
        block_0x0020a020,
        block_0x0020a030,
        block_0x0020a03c,
        block_0x0020a0ac,
        block_0x0020a0e0,
        block_0x0020a0ec,
        block_0x0020a0f8,
        block_0x0020a14c,
        block_0x0020a158,
        block_0x0020a170,
        block_0x0020a190,
        block_0x0020a1a0,
        block_0x0020a1a4,
        block_0x0020a1c8,
        block_0x0020a1d8,
        block_0x0020a1e4,
        block_0x0020a1f4,
        block_0x0020a20c,
        block_0x0020a224,
        block_0x0020a228,
        block_0x0020a22c,
        block_0x0020a238,
        block_0x0020a23c,
        block_0x0020a248,
        block_0x0020a274,
        block_0x0020a2a4,
        block_0x0020a2c8,
        block_0x0020a318,
        block_0x0020a324,
        block_0x0020a330,
        block_0x0020a380,
        block_0x0020a388,
        block_0x0020a3a8,
        block_0x0020a3c8,
        block_0x0020a410,
        block_0x0020a41c,
        block_0x0020a428,
        block_0x0020a444,
        block_0x0020a448,
        block_0x0020a468,
        block_0x0020a46c,
        block_0x0020a474,
        block_0x0020a480,
        block_0x0020a4e8,
        block_0x0020a534,
        block_0x0020a54c,
        block_0x0020a558,
        block_0x0020a55c,
        block_0x0020a590,
        block_0x0020a5a8,
        block_0x0020a5ac,
        block_0x0020a5bc,
        block_0x0020a5c4,
        block_0x0020a5f4,
        block_0x0020a5fc,
        block_0x0020a604,
        block_0x0020a608,
        block_0x0020a614,
        block_0x0020a624,
        block_0x0020a678,
        block_0x0020a688,
        block_0x0020a68c,
        block_0x0020a698,
        block_0x0020a6b4,
        block_0x0020a6d0,
        block_0x0020a6d4,
        block_0x0020a6f0,
        block_0x0020a6f4,
        block_0x0020a704,
        block_0x0020a70c,
        block_0x0020a714,
        block_0x0020a724,
        block_0x0020a744,
        block_0x0020a758,
        block_0x0020a76c,
        block_0x0020a784,
        block_0x0020a7ac,
        block_0x0020a7cc,
        block_0x0020a7d4,
        block_0x0020a7e4,
        block_0x0020a7e8,
        block_0x0020a7ec,
        block_0x0020a7f0,
        block_0x0020a7fc,
        block_0x0020a808,
        block_0x0020a814,
        block_0x0020a818,
        block_0x0020a828,
        block_0x0020a83c,
        block_0x0020a850,
        block_0x0020a858,
        block_0x0020a86c,
        block_0x0020a870,
        block_0x0020a874,
        block_0x0020a878,
        block_0x0020a888,
        block_0x0020a8a4,
        block_0x0020a8b8,
        block_0x0020a8cc,
        block_0x0020a8f0,
        block_0x0020a8f4,
        block_0x0020a92c,
        block_0x0020a940,
        block_0x0020a954,
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
    if pc < 2137572u32 || pc > 2140500u32 {
        return None;
    }
    let word_offset = ((pc - 2137572u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00209de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2137572u32, 4096u32, 2137576u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2137580u32;
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
pub fn block_0x00209dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2137584u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2137588u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2137592u32);
    emu.apc_no_count(1usize, 2137592u32, 4096u32, 2137596u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137600u32;
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
pub fn block_0x00209e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2137604u32);
    emu.lw_no_count(10usize, 19usize, 0u32, 2137608u32)?;
    emu.lw_no_count(11usize, 19usize, 4u32, 2137612u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2137616u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2137620u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2137624u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2137628u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2137632u32);
    emu.apc_no_count(1usize, 2137632u32, 4096u32, 2137636u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137640u32;
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
pub fn block_0x00209e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 8u32, 2137644u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2137648u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2137776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209eb0));
    } else {
        emu.pc = 2137652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e34));
    }
}
#[inline(always)]
pub fn block_0x00209e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2137656u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2137752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e98));
    } else {
        emu.pc = 2137660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e3c));
    }
}
#[inline(always)]
pub fn block_0x00209e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2137664u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2137820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209edc));
    } else {
        emu.pc = 2137668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e44));
    }
}
#[inline(always)]
pub fn block_0x00209e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137672u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 4294966224u32, 2137676u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2137680u32);
    emu.sb_no_count(10usize, 11usize, 4294966224u32, 2137684u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2137820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209edc));
    } else {
        emu.pc = 2137688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e58));
    }
}
#[inline]
pub fn block_0x00209e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137692u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966232u32, 2137696u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2137700u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2137704u32);
    emu.lw_no_count(13usize, 18usize, 36u32, 2137708u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2137712u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2137716u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2137720u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2137724u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2137728u32);
    emu.adi_no_count(12usize, 2usize, 20u32, 2137732u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2137736u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2137740u32;
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
pub fn block_0x00209e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 12u32, 2137744u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2137748u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2137752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137812u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ed4));
}
#[inline(always)]
pub fn block_0x00209e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2137756u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2137760u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137764u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2137768u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2137772u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2137776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137796u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ec4));
}
#[inline(always)]
pub fn block_0x00209eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2137780u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2137784u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2137788u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2137792u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2137796u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2137796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ec4));
}
#[inline(always)]
pub fn block_0x00209ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2137796u32, 4096u32, 2137800u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137804u32;
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
pub fn block_0x00209ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 20u32, 2137808u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2137812u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2137812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ed4));
}
#[inline(always)]
pub fn block_0x00209ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2137812u32, 4294963200u32, 2137816u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137820u32;
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
pub fn block_0x00209edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2137824u32);
    emu.apc_no_count(6usize, 2137824u32, 0u32, 2137828u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2137832u32;
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
pub fn block_0x00209ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2137836u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2137840u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2137844u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2137848u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2137852u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2137856u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2137860u32);
    emu.apc_no_count(1usize, 2137860u32, 0u32, 2137864u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137868u32;
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
pub fn block_0x00209f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2137872u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2137876u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2137880u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2137884u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2137888u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137892u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fac));
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
#[inline]
pub fn block_0x00209f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2137900u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2137904u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2137908u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2137912u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2137916u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2137920u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2137924u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2137928u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2137932u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2137936u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2137940u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2137944u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2137948u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2137952u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2137956u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2137960u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2137964u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2137968u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966240u32, 2137976u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2137980u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2137984u32);
    emu.apc_no_count(1usize, 2137984u32, 24576u32, 2137988u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137992u32;
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
pub fn block_0x00209f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2137996u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2138000u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2138004u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2138008u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2138012u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2138016u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2138020u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2138024u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2138028u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2138028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209fac));
}
#[inline]
pub fn block_0x00209fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2138032u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2138036u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2138040u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2138044u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2138048u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2138052u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2138056u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2138060u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2138064u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2138068u32)?;
    emu.apc_no_count(1usize, 2138068u32, 4294934528u32, 2138072u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2138080u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138084u32);
    emu.apc_no_count(1usize, 2138084u32, 4294930432u32, 2138088u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a020));
    } else {
        emu.pc = 2138096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ff0));
    }
}
#[inline]
pub fn block_0x00209ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2138100u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2138104u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2138108u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138112u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966264u32, 2138116u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2138120u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2138124u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2138128u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2138132u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2138136u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2138140u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138144u32;
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
pub fn block_0x0020a020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138148u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2138152u32);
    emu.apc_no_count(1usize, 2138152u32, 4096u32, 2138156u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138160u32;
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
pub fn block_0x0020a030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2138164u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138168u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2138336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0e0));
    } else {
        emu.pc = 2138172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a03c));
    }
}
#[inline(never)]
pub fn block_0x0020a03c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2138176u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2138180u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2138184u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2138188u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2138192u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2138196u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2138200u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2138204u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2138208u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2138212u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2138216u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2138220u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2138224u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2138228u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2138232u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2138236u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2138240u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2138244u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2138248u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2138252u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2138256u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138260u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966240u32, 2138264u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2138268u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2138272u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2138276u32);
    emu.apc_no_count(1usize, 2138276u32, 24576u32, 2138280u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138284u32;
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
pub fn block_0x0020a0ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2138288u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2138292u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2138296u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2138300u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2138304u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2138308u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2138312u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2138316u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2138320u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2138324u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2138328u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2138332u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2138336u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2138336u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0e0));
}
#[inline(always)]
pub fn block_0x0020a0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138340u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966264u32, 2138344u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138348u32;
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
pub fn block_0x0020a0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2138352u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2138356u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2138456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a158));
    } else {
        emu.pc = 2138360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0f8));
    }
}
#[inline]
pub fn block_0x0020a0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2138364u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2138368u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2138372u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2138376u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2138380u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2138384u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2138388u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2138392u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2138396u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2138400u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2138404u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2138408u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2138412u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2138416u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2138420u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2138424u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2138428u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2138432u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2138436u32);
    emu.apc_no_count(1usize, 2138436u32, 24576u32, 2138440u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138444u32;
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
pub fn block_0x0020a14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2138448u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2138452u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138456u32;
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
pub fn block_0x0020a158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2138460u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2138464u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2138468u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2138472u32);
    emu.apc_no_count(6usize, 2138472u32, 28672u32, 2138476u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138480u32;
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
pub fn block_0x0020a170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138484u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2138488u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2138492u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2138496u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2138500u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2138504u32)?;
    emu.apc_no_count(1usize, 2138504u32, 4294934528u32, 2138508u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2138516u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138520u32);
    emu.apc_no_count(1usize, 2138520u32, 4294930432u32, 2138524u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1c8));
    } else {
        emu.pc = 2138532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1a4));
    }
}
#[inline]
pub fn block_0x0020a1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138536u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966280u32, 2138540u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2138544u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2138548u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2138552u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2138556u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2138560u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2138564u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138568u32;
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
pub fn block_0x0020a1c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138572u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2138576u32);
    emu.apc_no_count(1usize, 2138576u32, 4096u32, 2138580u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138584u32;
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
pub fn block_0x0020a1d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138588u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966280u32, 2138592u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138596u32;
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
pub fn block_0x0020a1e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2138600u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2138604u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2138608u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138612u32;
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
pub fn block_0x0020a1f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2138616u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138620u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2138624u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2138628u32);
    emu.apc_no_count(6usize, 2138628u32, 28672u32, 2138632u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138636u32;
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
pub fn block_0x0020a20c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138640u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2138644u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2138648u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2138652u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2138656u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2138680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a238));
    } else {
        emu.pc = 2138660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a224));
    }
}
#[inline(always)]
pub fn block_0x0020a224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a274));
    } else {
        emu.pc = 2138664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a228));
    }
}
#[inline(always)]
pub fn block_0x0020a228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a274));
    } else {
        emu.pc = 2138668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a22c));
    }
}
#[inline(always)]
pub fn block_0x0020a22c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2138672u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2138676u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2138680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a248));
}
#[inline(always)]
pub fn block_0x0020a238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a274));
    } else {
        emu.pc = 2138684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a23c));
    }
}
#[inline(always)]
pub fn block_0x0020a23c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2138688u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2138692u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2138696u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2138696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a248));
}
#[inline]
pub fn block_0x0020a248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2138700u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138704u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2138708u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2138712u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2138716u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2138720u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966296u32, 2138728u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2138732u32);
    emu.apc_no_count(1usize, 2138732u32, 0u32, 2138736u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138740u32;
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
pub fn block_0x0020a274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2138744u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138748u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2138752u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2138756u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2138760u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138764u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2138768u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966324u32, 2138776u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2138780u32);
    emu.apc_no_count(1usize, 2138780u32, 0u32, 2138784u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138788u32;
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
pub fn block_0x0020a2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2138792u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2138796u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2138800u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2138804u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2138808u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2138812u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2138816u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2138820u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2138824u32;
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
pub fn block_0x0020a2c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2138828u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2138832u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2138836u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2138840u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138844u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2138848u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2138852u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138856u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2138860u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2138864u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138868u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2138872u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2138876u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138880u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2138884u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2138888u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2138892u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2138896u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138900u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a324));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2138908u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2138912u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2138916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a388));
}
#[inline(always)]
pub fn block_0x0020a324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2138920u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2138924u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2138928u32;
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
pub fn block_0x0020a330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2138932u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2138936u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2138940u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2138944u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138948u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2138952u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2138956u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138960u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2138964u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2138968u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138972u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2138976u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2138980u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138984u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2138988u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2138992u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2138996u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2139000u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2139004u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3a8));
    } else {
        emu.pc = 2139008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a380));
    }
}
#[inline(always)]
pub fn block_0x0020a380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2139012u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2139016u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a388));
}
#[inline(always)]
pub fn block_0x0020a388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2139020u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2139024u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2139028u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2139032u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2139036u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2139040u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139044u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139048u32;
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
pub fn block_0x0020a3a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139052u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966352u32, 2139056u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2139060u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2139064u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2139068u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2139072u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139076u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139080u32;
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
pub fn block_0x0020a3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2139084u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2139088u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2139092u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2139096u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2139100u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2139104u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2139108u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2139112u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2139116u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2139120u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2139124u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2139128u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2139132u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2139136u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2139140u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2139144u32);
    emu.apc_no_count(1usize, 2139144u32, 4096u32, 2139148u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139152u32;
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
pub fn block_0x0020a410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2139156u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2139160u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a46c));
    } else {
        emu.pc = 2139164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a41c));
    }
}
#[inline(always)]
pub fn block_0x0020a41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139168u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966212u32, 2139172u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2139656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a608));
    } else {
        emu.pc = 2139176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a428));
    }
}
#[inline(always)]
pub fn block_0x0020a428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 4294966212u32, 2139180u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2139184u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2139188u32);
    emu.sw_no_count(11usize, 10usize, 4294966212u32, 2139192u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2139196u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2139200u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2139480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a558));
    } else {
        emu.pc = 2139204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a444));
    }
}
#[inline(always)]
pub fn block_0x0020a444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139208u32;
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
pub fn block_0x0020a448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2139212u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139216u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2139220u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2139224u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2139228u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2139232u32);
    emu.apc_no_count(1usize, 2139232u32, 0u32, 2139236u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139240u32;
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
pub fn block_0x0020a468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2139244u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a590));
}
#[inline(always)]
pub fn block_0x0020a46c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2139248u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2139368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4e8));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2139256u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2139260u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139264u32;
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
pub fn block_0x0020a480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2139268u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2139272u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139276u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966824u32, 2139280u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2139284u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2139288u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966112u32, 2139292u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2139296u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966480u32, 2139300u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2139304u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2139308u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2139312u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2139316u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2139320u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139324u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2139328u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2139332u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2139336u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2139340u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2139344u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2139348u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139352u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2139356u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2139360u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2139364u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2139368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139444u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a534));
}
#[inline]
pub fn block_0x0020a4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2139372u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139376u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966824u32, 2139380u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2139384u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139388u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966440u32, 2139392u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139396u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966392u32, 2139400u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2139404u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139408u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2139412u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2139416u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2139420u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2139424u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2139428u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139432u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2139436u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2139440u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2139444u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2139444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a534));
}
#[inline(always)]
pub fn block_0x0020a534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2139448u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2139452u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2139456u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2139460u32);
    emu.apc_no_count(1usize, 2139460u32, 0u32, 2139464u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139468u32;
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
pub fn block_0x0020a54c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2139472u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2139476u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5fc));
}
#[inline(always)]
pub fn block_0x0020a558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139484u32;
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
pub fn block_0x0020a55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2139488u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966212u32, 2139492u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2139496u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2139500u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2139504u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2139508u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139512u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2139516u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2139520u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2139524u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2139528u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2139532u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2139536u32;
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
pub fn block_0x0020a590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139540u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966212u32, 2139544u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2139548u32);
    emu.sw_no_count(11usize, 10usize, 4294966212u32, 2139552u32)?;
    emu.apc_no_count(1usize, 2139552u32, 4096u32, 2139556u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139560u32;
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
pub fn block_0x0020a5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5bc));
    } else {
        emu.pc = 2139564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5ac));
    }
}
#[inline(always)]
pub fn block_0x0020a5ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2139568u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2139572u32);
    emu.apc_no_count(1usize, 2139572u32, 0u32, 2139576u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139580u32;
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
pub fn block_0x0020a5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139584u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966552u32, 2139588u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5c4));
}
#[inline]
pub fn block_0x0020a5c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2139592u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139596u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2139600u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2139604u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139608u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2139612u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2139616u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2139620u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2139624u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2139628u32);
    emu.apc_no_count(1usize, 2139628u32, 0u32, 2139632u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139636u32;
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
pub fn block_0x0020a5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2139640u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2139644u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139644u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5fc));
}
#[inline(always)]
pub fn block_0x0020a5fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2139644u32, 4294963200u32, 2139648u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139652u32;
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
pub fn block_0x0020a604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2139652u32));
}
#[inline(always)]
pub fn block_0x0020a608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139660u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966700u32, 2139664u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5c4));
}
#[inline(always)]
pub fn block_0x0020a614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2139672u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2139676u32)?;
    emu.apc_no_count(1usize, 2139676u32, 4294934528u32, 2139680u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2139688u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2139692u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139696u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966160u32, 2139700u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2139704u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966624u32, 2139708u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2139712u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2139716u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2139720u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2139724u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139728u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2139732u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2139736u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2139740u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2139744u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2139748u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2139752u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2139756u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2139760u32);
    emu.apc_no_count(1usize, 2139760u32, 0u32, 2139764u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139768u32;
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
pub fn block_0x0020a678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2139772u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2139776u32)?;
    emu.apc_no_count(1usize, 2139776u32, 4294963200u32, 2139780u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139784u32;
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
pub fn block_0x0020a688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2139784u32));
}
#[inline(always)]
pub fn block_0x0020a68c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2139792u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2139796u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139800u32;
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
pub fn block_0x0020a698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2139804u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2139808u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2139812u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2139816u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2139820u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2139824u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139828u32;
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
pub fn block_0x0020a6b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2139832u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2139836u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2139840u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2139844u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2139848u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2139852u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2139856u32;
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
pub fn block_0x0020a6d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2139856u32));
}
#[inline(always)]
pub fn block_0x0020a6d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2139864u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2139868u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2139872u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2139876u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2139880u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2139884u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2139888u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2139888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6f0));
}
#[inline(always)]
pub fn block_0x0020a6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a70c));
    } else {
        emu.pc = 2139892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6f4));
    }
}
#[inline(always)]
pub fn block_0x0020a6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2139896u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2139900u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2139904u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2139888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6f0));
    } else {
        emu.pc = 2139908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a704));
    }
}
#[inline(always)]
pub fn block_0x0020a704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2139912u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2139916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a714));
}
#[inline(always)]
pub fn block_0x0020a70c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2139920u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2139924u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a714));
}
#[inline(always)]
pub fn block_0x0020a714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2139928u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2139932u32);
    emu.apc_no_count(1usize, 2139932u32, 4294938624u32, 2139936u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2139944u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2139948u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2139952u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2139956u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2139960u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2139964u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2139968u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139972u32;
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
pub fn block_0x0020a744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2139976u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2139980u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2139984u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2139988u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2140012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a76c));
    } else {
        emu.pc = 2139992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a758));
    }
}
#[inline(always)]
pub fn block_0x0020a758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2139996u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2140000u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2140004u32);
    emu.apc_no_count(1usize, 2140004u32, 4294938624u32, 2140008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965248u32);
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
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140016u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140020u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2140024u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140028u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140032u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140036u32;
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
pub fn block_0x0020a784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2140040u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2140044u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2140048u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2140052u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2140056u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2140060u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2140064u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2140068u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140072u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2140144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7f0));
    } else {
        emu.pc = 2140076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7ac));
    }
}
#[inline(always)]
pub fn block_0x0020a7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140080u32);
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2140084u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 13usize, 4294967295u32, 2140088u32);
    emu.sli_no_count(11usize, 13usize, 3u32, 2140092u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2140096u32);
    emu.anr_no_count(14usize, 14usize, 19usize, 2140100u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2140104u32);
    emu.adi_no_count(15usize, 12usize, 4u32, 2140108u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2140108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7cc));
}
#[inline(always)]
pub fn block_0x0020a7cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2140112u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2140136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7e8));
    } else {
        emu.pc = 2140116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7d4));
    }
}
#[inline(always)]
pub fn block_0x0020a7d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2140120u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2140124u32);
    emu.adi_no_count(15usize, 15usize, 8u32, 2140128u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7cc));
    } else {
        emu.pc = 2140132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7e4));
    }
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
    emu.adi_no_count(10usize, 14usize, 0u32, 2140136u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7e8));
}
#[inline(always)]
pub fn block_0x0020a7e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a92c));
    } else {
        emu.pc = 2140140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7ec));
    }
}
#[inline(always)]
pub fn block_0x0020a7ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7fc));
    } else {
        emu.pc = 2140144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7f0));
    }
}
#[inline(always)]
pub fn block_0x0020a7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140148u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140152u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2140156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8cc));
}
#[inline(always)]
pub fn block_0x0020a7fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 10usize, 3u32, 2140160u32);
    emu.adr_no_count(20usize, 12usize, 20usize, 2140164u32);
    emu.sbr_no_count(9usize, 13usize, 10usize, 2140168u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2140168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a808));
}
#[inline(always)]
pub fn block_0x0020a808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 9usize, 3u32, 2140172u32);
    emu.adi_no_count(10usize, 20usize, 4294967288u32, 2140176u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2140180u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2140180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a814));
}
#[inline(always)]
pub fn block_0x0020a814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8a4));
    } else {
        emu.pc = 2140184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a818));
    }
}
#[inline(always)]
pub fn block_0x0020a818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2140188u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2140192u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2140196u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2140180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a814));
    } else {
        emu.pc = 2140200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a828));
    }
}
#[inline(always)]
pub fn block_0x0020a828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2140204u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2140208u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2140212u32);
    emu.apc_no_count(1usize, 2140212u32, 4294934528u32, 2140216u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a83c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140224u32);
    emu.adr_no_count(11usize, 9usize, 19usize, 2140228u32);
    emu.anr_no_count(11usize, 11usize, 19usize, 2140232u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2140236u32);
    emu.adi_no_count(12usize, 20usize, 4u32, 2140240u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2140240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a850));
}
#[inline(always)]
pub fn block_0x0020a850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2140244u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2140272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a870));
    } else {
        emu.pc = 2140248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a858));
    }
}
#[inline(always)]
pub fn block_0x0020a858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(18usize, 18usize, 13usize, 2140252u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2140256u32);
    emu.adi_no_count(21usize, 21usize, 4294967288u32, 2140260u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2140264u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2140240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a850));
    } else {
        emu.pc = 2140268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a86c));
    }
}
#[inline(always)]
pub fn block_0x0020a86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2140272u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a870));
}
#[inline(always)]
pub fn block_0x0020a870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a940));
    } else {
        emu.pc = 2140276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a874));
    }
}
#[inline(always)]
pub fn block_0x0020a874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8f0));
    } else {
        emu.pc = 2140280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a878));
    }
}
#[inline(always)]
pub fn block_0x0020a878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 3u32, 2140284u32);
    emu.adr_no_count(20usize, 20usize, 11usize, 2140288u32);
    emu.lw_no_count(11usize, 20usize, 4u32, 2140292u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2140500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a954));
    } else {
        emu.pc = 2140296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a888));
    }
}
#[inline(always)]
pub fn block_0x0020a888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2140300u32)?;
    emu.sbr_no_count(9usize, 9usize, 10usize, 2140304u32);
    emu.sbr_no_count(10usize, 11usize, 18usize, 2140308u32);
    emu.adr_no_count(12usize, 12usize, 18usize, 2140312u32);
    emu.sw_no_count(12usize, 20usize, 0u32, 2140316u32)?;
    emu.sw_no_count(10usize, 20usize, 4u32, 2140320u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2140324u32;
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
pub fn block_0x0020a8a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2140328u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140332u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2140336u32);
    emu.apc_no_count(1usize, 2140336u32, 4294934528u32, 2140340u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a8b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140348u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966848u32, 2140352u32)?;
    emu.lw_no_count(10usize, 10usize, 4294966852u32, 2140356u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2140360u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2140364u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2140364u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8cc));
}
#[inline]
pub fn block_0x0020a8cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2140368u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2140372u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2140376u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2140380u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2140384u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2140388u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2140392u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2140396u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140400u32;
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
pub fn block_0x0020a8f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7f0));
    } else {
        emu.pc = 2140404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8f4));
    }
}
#[inline]
pub fn block_0x0020a8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140408u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966748u32, 2140412u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140416u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2140420u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2140424u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2140428u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2140432u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2140436u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2140440u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140444u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2140448u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2140452u32);
    emu.apc_no_count(1usize, 2140452u32, 8192u32, 2140456u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140460u32;
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
pub fn block_0x0020a92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140464u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966788u32, 2140468u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140472u32);
    emu.apc_no_count(1usize, 2140472u32, 36864u32, 2140476u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140480u32;
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
pub fn block_0x0020a940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140484u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966788u32, 2140488u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2140492u32);
    emu.apc_no_count(1usize, 2140492u32, 36864u32, 2140496u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140500u32;
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
pub fn block_0x0020a954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140504u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966772u32, 2140508u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2140512u32);
    emu.apc_no_count(1usize, 2140512u32, 36864u32, 2140516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140520u32;
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
