pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2136120u32;
pub const PC_MAX: u32 = 2138572u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x00209838,
        block_0x0020986c,
        block_0x00209874,
        block_0x00209884,
        block_0x00209888,
        block_0x00209894,
        block_0x00209898,
        block_0x002098a8,
        block_0x002098ac,
        block_0x002098b8,
        block_0x002098c0,
        block_0x002098c4,
        block_0x002098ec,
        block_0x002098f4,
        block_0x002098fc,
        block_0x0020990c,
        block_0x00209934,
        block_0x0020993c,
        block_0x00209968,
        block_0x00209970,
        block_0x00209974,
        block_0x0020997c,
        block_0x00209988,
        block_0x00209998,
        block_0x002099a8,
        block_0x002099b0,
        block_0x002099d0,
        block_0x002099ec,
        block_0x002099f0,
        block_0x00209a0c,
        block_0x00209a14,
        block_0x00209a40,
        block_0x00209a78,
        block_0x00209aa4,
        block_0x00209ad4,
        block_0x00209ae8,
        block_0x00209b10,
        block_0x00209b30,
        block_0x00209b3c,
        block_0x00209b78,
        block_0x00209b90,
        block_0x00209bc4,
        block_0x00209be0,
        block_0x00209be8,
        block_0x00209c18,
        block_0x00209c30,
        block_0x00209c44,
        block_0x00209c54,
        block_0x00209c70,
        block_0x00209c78,
        block_0x00209c80,
        block_0x00209ca0,
        block_0x00209cac,
        block_0x00209cc0,
        block_0x00209cd4,
        block_0x00209ce4,
        block_0x00209d1c,
        block_0x00209d24,
        block_0x00209d60,
        block_0x00209d78,
        block_0x00209da8,
        block_0x00209dc4,
        block_0x00209dcc,
        block_0x00209df8,
        block_0x00209e10,
        block_0x00209e24,
        block_0x00209e34,
        block_0x00209e4c,
        block_0x00209e54,
        block_0x00209e74,
        block_0x00209e80,
        block_0x00209e94,
        block_0x00209ea8,
        block_0x00209eb8,
        block_0x00209ee8,
        block_0x00209ef4,
        block_0x00209efc,
        block_0x00209f04,
        block_0x00209f10,
        block_0x00209f18,
        block_0x00209f38,
        block_0x00209f64,
        block_0x00209f7c,
        block_0x00209f88,
        block_0x00209fa4,
        block_0x00209fc4,
        block_0x00209fd4,
        block_0x00209fe0,
        block_0x00209fe4,
        block_0x00209ffc,
        block_0x0020a004,
        block_0x0020a00c,
        block_0x0020a020,
        block_0x0020a048,
        block_0x0020a054,
        block_0x0020a05c,
        block_0x0020a064,
        block_0x0020a078,
        block_0x0020a0ac,
        block_0x0020a0b8,
        block_0x0020a0d0,
        block_0x0020a0e4,
        block_0x0020a0ec,
        block_0x0020a0f4,
        block_0x0020a0fc,
        block_0x0020a108,
        block_0x0020a12c,
        block_0x0020a148,
        block_0x0020a1a8,
        block_0x0020a1cc,
    ];
    const IDX: [u16; 614usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        2u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 6u16, 7u16, 0u16,
        0u16, 0u16, 8u16, 9u16, 0u16, 0u16, 10u16, 0u16, 11u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16,
        0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16,
        18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        20u16, 21u16, 0u16, 22u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 0u16,
        0u16, 0u16, 25u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16,
        0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16,
        47u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16,
        50u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16,
        53u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 57u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16,
        0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16,
        71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 0u16, 76u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16, 79u16, 0u16,
        80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16,
        0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 88u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 92u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 95u16, 0u16, 96u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16,
        0u16, 103u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16,
    ];
    if pc < 2136120u32 || pc > 2138572u32 {
        return None;
    }
    let word_offset = ((pc - 2136120u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00209838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2136124u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2136128u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2136132u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136136u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966129u32, 2136140u32);
    emu.adi_no_count(12usize, 12usize, 376u32, 2136144u32);
    emu.adi_no_count(13usize, 13usize, 44u32, 2136148u32);
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2136152u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2136156u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2136160u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2136164u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2136168u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136172u32;
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
pub fn block_0x0020986c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2136176u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209884));
    } else {
        emu.pc = 2136180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209874));
    }
}
#[inline(always)]
pub fn block_0x00209874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2136184u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2136188u32);
    emu.apc_no_count(6usize, 2136188u32, 4294934528u32, 2136192u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2136196u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
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
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136200u32;
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
pub fn block_0x00209888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2136204u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2136208u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098a8));
    } else {
        emu.pc = 2136212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209894));
    }
}
#[inline(always)]
pub fn block_0x00209894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2136232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098a8));
    } else {
        emu.pc = 2136216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209898));
    }
}
#[inline(always)]
pub fn block_0x00209898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2136220u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2136224u32);
    emu.apc_no_count(6usize, 2136224u32, 4294934528u32, 2136228u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2136232u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002098a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136236u32;
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
pub fn block_0x002098ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2136240u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2136244u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2136260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c4));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2136252u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2136260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c4));
    } else {
        emu.pc = 2136256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c0));
    }
}
#[inline(always)]
pub fn block_0x002098c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136260u32;
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
pub fn block_0x002098c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2136264u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2136268u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2136272u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2136276u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2136280u32)?;
    emu.lw_no_count(18usize, 11usize, 4u32, 2136284u32)?;
    emu.lw_no_count(12usize, 18usize, 0u32, 2136288u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2136292u32);
    emu.lw_no_count(9usize, 11usize, 0u32, 2136296u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2136308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098f4));
    } else {
        emu.pc = 2136300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098ec));
    }
}
#[inline(always)]
pub fn block_0x002098ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2136304u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2136308u32;
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
pub fn block_0x002098f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2136312u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020990c));
    } else {
        emu.pc = 2136316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098fc));
    }
}
#[inline(always)]
pub fn block_0x002098fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2136320u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2136324u32);
    emu.apc_no_count(1usize, 2136324u32, 4294934528u32, 2136328u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020990c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2136336u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2136340u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2136344u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2136348u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2136352u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2136356u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2136360u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2136364u32);
    emu.apc_no_count(6usize, 2136364u32, 4294934528u32, 2136368u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2136372u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2136376u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136380u32;
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
pub fn block_0x0020993c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136384u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2136388u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2136392u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2136396u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2136400u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2136404u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2136408u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2136412u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2136416u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2136420u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2136432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209970));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2136428u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209988));
}
#[inline(always)]
pub fn block_0x00209970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2136444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020997c));
    } else {
        emu.pc = 2136436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209974));
    }
}
#[inline(always)]
pub fn block_0x00209974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2136440u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209988));
}
#[inline(always)]
pub fn block_0x0020997c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2136448u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2136452u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2136456u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2136456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209988));
}
#[inline(always)]
pub fn block_0x00209988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2136460u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2136464u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136468u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2136496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099b0));
    } else {
        emu.pc = 2136472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209998));
    }
}
#[inline(always)]
pub fn block_0x00209998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2136476u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2136480u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2136484u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2136556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099ec));
    } else {
        emu.pc = 2136488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099a8));
    }
}
#[inline(always)]
pub fn block_0x002099a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2136492u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a78));
}
#[inline(always)]
pub fn block_0x002099b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2136500u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136504u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2136508u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2136512u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2136516u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2136520u32);
    emu.apc_no_count(1usize, 2136520u32, 4294963200u32, 2136524u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136528u32;
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
pub fn block_0x002099d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2136532u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2136536u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2136540u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2136544u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2136548u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2136552u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2136488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099a8));
    } else {
        emu.pc = 2136556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099ec));
    }
}
#[inline(always)]
pub fn block_0x002099ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2136588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a0c));
    } else {
        emu.pc = 2136560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099f0));
    }
}
#[inline(always)]
pub fn block_0x002099f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2136564u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2136568u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2136572u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2136576u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2136580u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2136584u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2136588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a78));
}
#[inline(always)]
pub fn block_0x00209a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2136592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2136640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a40));
    } else {
        emu.pc = 2136596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a14));
    }
}
#[inline]
pub fn block_0x00209a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2136600u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2136604u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2136608u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2136612u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2136616u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2136620u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2136624u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2136628u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2136632u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2136636u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2136640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a78));
}
#[inline]
pub fn block_0x00209a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2136644u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2136648u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2136652u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2136656u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2136660u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2136664u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2136668u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2136672u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2136676u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2136680u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2136684u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2136688u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2136692u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2136696u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2136696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a78));
}
#[inline]
pub fn block_0x00209a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2136700u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2136704u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2136708u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2136712u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2136716u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2136720u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2136724u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2136728u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2136732u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2136736u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136740u32;
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
pub fn block_0x00209aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136744u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2136748u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2136752u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2136756u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2136760u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2136764u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2136768u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2136772u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2136776u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2136780u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2136784u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2136848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b10));
    } else {
        emu.pc = 2136788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ad4));
    }
}
#[inline(always)]
pub fn block_0x00209ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2136792u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2136796u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136800u32);
    emu.apc_no_count(1usize, 2136800u32, 4294938624u32, 2136804u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2136812u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2136816u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2136820u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2136824u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2136828u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2136832u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2136836u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2136840u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2136844u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136848u32;
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
pub fn block_0x00209b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2136852u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136856u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2136860u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2136864u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2136868u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136872u32);
    emu.apc_no_count(1usize, 2136872u32, 4294963200u32, 2136876u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136880u32;
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
pub fn block_0x00209b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2136884u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2136888u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2136892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ad4));
}
#[inline]
pub fn block_0x00209b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136896u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2136900u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2136904u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2136908u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2136912u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2136916u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2136920u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2136924u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2136928u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2136932u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2136936u32)?;
    emu.lw_no_count(20usize, 9usize, 8u32, 2136940u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2136944u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2136948u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2137028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bc4));
    } else {
        emu.pc = 2136952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b78));
    }
}
#[inline(always)]
pub fn block_0x00209b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2136956u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2136960u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2136964u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136968u32);
    emu.apc_no_count(1usize, 2136968u32, 4294938624u32, 2136972u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 8usize, 2136980u32);
    emu.sw_no_count(20usize, 9usize, 8u32, 2136984u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2136988u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2136992u32);
    emu.sw_no_count(8usize, 18usize, 4u32, 2136996u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2137000u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2137004u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2137008u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2137012u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2137016u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2137020u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2137024u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137028u32;
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
pub fn block_0x00209bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137032u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137036u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2137040u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2137044u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2137048u32);
    emu.apc_no_count(1usize, 2137048u32, 4294963200u32, 2137052u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137056u32;
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
pub fn block_0x00209be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 9usize, 8u32, 2137060u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2137064u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136952u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b78));
}
#[inline]
pub fn block_0x00209be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2137068u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2137072u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2137076u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2137080u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2137084u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2137088u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2137092u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2137096u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2137100u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2137104u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2137108u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2137208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c78));
    } else {
        emu.pc = 2137112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c18));
    }
}
#[inline(always)]
pub fn block_0x00209c18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2137116u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2137120u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2137124u32);
    emu.sli_no_count(22usize, 13usize, 3u32, 2137128u32);
    emu.adr_no_count(22usize, 12usize, 22usize, 2137132u32);
    emu.adi_no_count(10usize, 12usize, 4u32, 2137136u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2137136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209c30));
}
#[inline(always)]
pub fn block_0x00209c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2137140u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2137144u32);
    emu.adr_no_count(9usize, 11usize, 9usize, 2137148u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2137152u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2137136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c30));
    } else {
        emu.pc = 2137156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c44));
    }
}
#[inline(always)]
pub fn block_0x00209c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2137160u32)?;
    emu.lw_no_count(20usize, 19usize, 8u32, 2137164u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2137168u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2137260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cac));
    } else {
        emu.pc = 2137172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c54));
    }
}
#[inline(always)]
pub fn block_0x00209c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137176u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137180u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2137184u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2137188u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2137192u32);
    emu.apc_no_count(1usize, 2137192u32, 4294963200u32, 2137196u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137200u32;
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
pub fn block_0x00209c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 19usize, 8u32, 2137204u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2137208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209cac));
}
#[inline(always)]
pub fn block_0x00209c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2137212u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2137216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137316u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ce4));
}
#[inline(always)]
pub fn block_0x00209c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137220u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137224u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2137228u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2137232u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2137236u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2137240u32);
    emu.apc_no_count(1usize, 2137240u32, 4294963200u32, 2137244u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137248u32;
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
pub fn block_0x00209ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 23usize, 0u32, 2137252u32);
    emu.lw_no_count(20usize, 19usize, 8u32, 2137256u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2137260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137280u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209cc0));
}
#[inline(always)]
pub fn block_0x00209cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2137264u32)?;
    emu.lw_no_count(21usize, 18usize, 4u32, 2137268u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2137272u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2137276u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2137216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c80));
    } else {
        emu.pc = 2137280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cc0));
    }
}
#[inline(always)]
pub fn block_0x00209cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 4u32, 2137284u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2137288u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2137292u32);
    emu.apc_no_count(1usize, 2137292u32, 4294938624u32, 2137296u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137300u32;
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
pub fn block_0x00209cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 21usize, 2137304u32);
    emu.adi_no_count(18usize, 18usize, 8u32, 2137308u32);
    emu.sw_no_count(20usize, 19usize, 8u32, 2137312u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2137260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cac));
    } else {
        emu.pc = 2137316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ce4));
    }
}
#[inline]
pub fn block_0x00209ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2137320u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2137324u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2137328u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2137332u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2137336u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2137340u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2137344u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2137348u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2137352u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2137356u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2137360u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2137364u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2137368u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137372u32;
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
pub fn block_0x00209d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2137376u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137380u32;
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
pub fn block_0x00209d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2137384u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2137388u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2137392u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2137396u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2137400u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2137404u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2137408u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2137412u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2137416u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2137420u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2137424u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2137428u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2137432u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2137436u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2137512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209da8));
    } else {
        emu.pc = 2137440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d60));
    }
}
#[inline(always)]
pub fn block_0x00209d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2137444u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2137448u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2137452u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2137456u32);
    emu.apc_no_count(1usize, 2137456u32, 4294938624u32, 2137460u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137464u32;
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
#[inline]
pub fn block_0x00209d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 20usize, 9usize, 2137468u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2137472u32);
    emu.sw_no_count(9usize, 8usize, 8u32, 2137476u32)?;
    emu.sb_no_count(10usize, 18usize, 0u32, 2137480u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2137484u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2137488u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2137492u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2137496u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2137500u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2137504u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2137508u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137512u32;
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
pub fn block_0x00209da8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137516u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137520u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2137524u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2137528u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2137532u32);
    emu.apc_no_count(1usize, 2137532u32, 4294963200u32, 2137536u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 8usize, 8u32, 2137544u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2137548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209d60));
}
#[inline]
pub fn block_0x00209dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2137552u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2137556u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2137560u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2137564u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2137568u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2137572u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2137576u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2137580u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2137584u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2137588u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2137784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209eb8));
    } else {
        emu.pc = 2137592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209df8));
    }
}
#[inline(always)]
pub fn block_0x00209df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2137596u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2137600u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2137604u32);
    emu.sli_no_count(21usize, 13usize, 3u32, 2137608u32);
    emu.adr_no_count(21usize, 9usize, 21usize, 2137612u32);
    emu.adi_no_count(10usize, 9usize, 4u32, 2137616u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2137616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e10));
}
#[inline(always)]
pub fn block_0x00209e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2137620u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2137624u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2137628u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2137632u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2137616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e10));
    } else {
        emu.pc = 2137636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e24));
    }
}
#[inline(always)]
pub fn block_0x00209e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2137640u32)?;
    emu.lw_no_count(19usize, 18usize, 8u32, 2137644u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2137648u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2137728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e80));
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
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137656u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137660u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2137664u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2137668u32);
    emu.apc_no_count(1usize, 2137668u32, 4294963200u32, 2137672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 8u32, 2137680u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2137684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e80));
}
#[inline(always)]
pub fn block_0x00209e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137688u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137692u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2137696u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2137700u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2137704u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2137708u32);
    emu.apc_no_count(1usize, 2137708u32, 4294963200u32, 2137712u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2137720u32);
    emu.lw_no_count(19usize, 18usize, 8u32, 2137724u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2137728u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e94));
}
#[inline(always)]
pub fn block_0x00209e80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2137732u32)?;
    emu.lw_no_count(20usize, 9usize, 4u32, 2137736u32)?;
    emu.lw_no_count(11usize, 9usize, 0u32, 2137740u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2137744u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2137684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e54));
    } else {
        emu.pc = 2137748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e94));
    }
}
#[inline(always)]
pub fn block_0x00209e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2137752u32)?;
    emu.adr_no_count(10usize, 10usize, 19usize, 2137756u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2137760u32);
    emu.apc_no_count(1usize, 2137760u32, 4294938624u32, 2137764u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 20usize, 2137772u32);
    emu.adi_no_count(9usize, 9usize, 8u32, 2137776u32);
    emu.sw_no_count(19usize, 18usize, 8u32, 2137780u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2137728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e80));
    } else {
        emu.pc = 2137784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209eb8));
    }
}
#[inline]
pub fn block_0x00209eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2137788u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2137792u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2137796u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2137800u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2137804u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2137808u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2137812u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2137816u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2137820u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2137824u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2137828u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137832u32;
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
pub fn block_0x00209ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2137836u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2137840u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137844u32;
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
pub fn block_0x00209ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2137848u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137852u32;
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
pub fn block_0x00209efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2137852u32, 4096u32, 2137856u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2137860u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209f04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2137864u32);
    emu.lbu_no_count(10usize, 10usize, 13u32, 2137868u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2137880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f18));
    } else {
        emu.pc = 2137872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f10));
    }
}
#[inline(always)]
pub fn block_0x00209f10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2137872u32, 4096u32, 2137876u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 8u32, 2137884u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2137888u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2137892u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2137896u32);
    emu.sb_no_count(13usize, 2usize, 7u32, 2137900u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2137904u32)?;
    emu.apc_no_count(1usize, 2137904u32, 0u32, 2137908u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137912u32;
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
#[inline]
pub fn block_0x00209f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2137916u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2137920u32)?;
    emu.adi_no_count(11usize, 2usize, 8u32, 2137924u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2137928u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137932u32;
    emu.update_insn_clock();
    emu.lbu_no_count(13usize, 10usize, 4294966680u32, 2137936u32);
    emu.adi_no_count(14usize, 2usize, 7u32, 2137940u32);
    emu.sw_no_count(11usize, 2usize, 20u32, 2137944u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2137948u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2137952u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2138084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fe4));
    } else {
        emu.pc = 2137956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f64));
    }
}
#[inline(always)]
pub fn block_0x00209f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2137960u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 18usize, 4294966684u32, 2137964u32)?;
    emu.adi_no_count(9usize, 0usize, 1u32, 2137968u32);
    emu.sb_no_count(9usize, 10usize, 4294966680u32, 2137972u32);
    emu.sw_no_count(0usize, 18usize, 4294966684u32, 2137976u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2138084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fe4));
    } else {
        emu.pc = 2137980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f7c));
    }
}
#[inline(always)]
pub fn block_0x00209f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 8u32, 2137984u32);
    emu.apc_no_count(1usize, 2137984u32, 4096u32, 2137988u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2137996u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2138000u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138004u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966552u32, 2138008u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2138012u32);
    emu.apc_no_count(1usize, 2138012u32, 0u32, 2138016u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2138024u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138028u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 18usize, 4294966684u32, 2138032u32)?;
    emu.sb_no_count(9usize, 11usize, 4294966680u32, 2138036u32);
    emu.sw_no_count(19usize, 18usize, 4294966684u32, 2138040u32)?;
    emu.sw_no_count(9usize, 2usize, 32u32, 2138044u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2138048u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2138108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ffc));
    } else {
        emu.pc = 2138052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fc4));
    }
}
#[inline(always)]
pub fn block_0x00209fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2138056u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2138060u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2138064u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2138108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ffc));
    } else {
        emu.pc = 2138068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fd4));
    }
}
#[inline(always)]
pub fn block_0x00209fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2138072u32);
    emu.apc_no_count(1usize, 2138072u32, 4096u32, 2138076u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2138084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ffc));
}
#[inline(always)]
pub fn block_0x00209fe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138088u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966592u32, 2138092u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2138096u32);
    emu.adi_no_count(11usize, 2usize, 43u32, 2138100u32);
    emu.apc_no_count(1usize, 2138100u32, 0u32, 2138104u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(16u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2138108u32, 4096u32, 2138112u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138116u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2138116u32, 4096u32, 2138120u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2138124u32;
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
pub fn block_0x0020a00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2138128u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2138132u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2138136u32);
    emu.apc_no_count(1usize, 2138136u32, 4096u32, 2138140u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138144u32;
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
pub fn block_0x0020a020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2138148u32);
    emu.lw_no_count(10usize, 19usize, 0u32, 2138152u32)?;
    emu.lw_no_count(11usize, 19usize, 4u32, 2138156u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2138160u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2138164u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2138168u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2138172u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2138176u32);
    emu.apc_no_count(1usize, 2138176u32, 4096u32, 2138180u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138184u32;
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
pub fn block_0x0020a048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 8u32, 2138188u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2138192u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2138320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0d0));
    } else {
        emu.pc = 2138196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a054));
    }
}
#[inline(always)]
pub fn block_0x0020a054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2138200u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2138296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0b8));
    } else {
        emu.pc = 2138204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a05c));
    }
}
#[inline(always)]
pub fn block_0x0020a05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2138208u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0fc));
    } else {
        emu.pc = 2138212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a064));
    }
}
#[inline(always)]
pub fn block_0x0020a064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138216u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 4294966704u32, 2138220u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2138224u32);
    emu.sb_no_count(10usize, 11usize, 4294966704u32, 2138228u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2138364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0fc));
    } else {
        emu.pc = 2138232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a078));
    }
}
#[inline]
pub fn block_0x0020a078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138236u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966712u32, 2138240u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2138244u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2138248u32);
    emu.lw_no_count(13usize, 18usize, 36u32, 2138252u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2138256u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2138260u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2138264u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2138268u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2138272u32);
    emu.adi_no_count(12usize, 2usize, 20u32, 2138276u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2138280u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2138284u32;
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
pub fn block_0x0020a0ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 12u32, 2138288u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2138292u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2138296u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138356u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0f4));
}
#[inline(always)]
pub fn block_0x0020a0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2138300u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2138304u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2138308u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2138312u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2138316u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2138320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0e4));
}
#[inline(always)]
pub fn block_0x0020a0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2138324u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2138328u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2138332u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2138336u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2138340u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2138340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0e4));
}
#[inline(always)]
pub fn block_0x0020a0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2138340u32, 4096u32, 2138344u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138348u32;
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
pub fn block_0x0020a0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 20u32, 2138352u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2138356u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2138356u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0f4));
}
#[inline(always)]
pub fn block_0x0020a0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2138356u32, 4294963200u32, 2138360u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138364u32;
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
pub fn block_0x0020a0fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2138368u32);
    emu.apc_no_count(6usize, 2138368u32, 0u32, 2138372u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138376u32;
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
pub fn block_0x0020a108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138380u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2138384u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138388u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2138392u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2138396u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2138400u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2138404u32);
    emu.apc_no_count(1usize, 2138404u32, 0u32, 2138408u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138412u32;
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
pub fn block_0x0020a12c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2138416u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2138420u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2138424u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2138428u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2138432u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138436u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1cc));
    } else {
        emu.pc = 2138440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a148));
    }
}
#[inline]
pub fn block_0x0020a148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2138444u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2138448u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2138452u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2138456u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2138460u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2138464u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2138468u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138472u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2138476u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2138480u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2138484u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2138488u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2138492u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2138496u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2138500u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2138504u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2138508u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2138512u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138516u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966720u32, 2138520u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2138524u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2138528u32);
    emu.apc_no_count(1usize, 2138528u32, 24576u32, 2138532u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138536u32;
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
pub fn block_0x0020a1a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2138540u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2138544u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2138548u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2138552u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2138556u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2138560u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2138564u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2138568u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2138572u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2138572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a1cc));
}
#[inline]
pub fn block_0x0020a1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2138576u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2138580u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2138584u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2138588u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2138592u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2138596u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2138600u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2138604u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2138608u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2138612u32)?;
    emu.apc_no_count(1usize, 2138612u32, 4294934528u32, 2138616u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
