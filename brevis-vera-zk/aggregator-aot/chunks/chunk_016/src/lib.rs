pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2188188u32;
pub const PC_MAX: u32 = 2191328u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 138usize] = [
        block_0x0021639c,
        block_0x002163a8,
        block_0x002163c8,
        block_0x00216418,
        block_0x00216424,
        block_0x00216438,
        block_0x00216444,
        block_0x002164a0,
        block_0x0021650c,
        block_0x00216524,
        block_0x00216530,
        block_0x00216548,
        block_0x00216554,
        block_0x00216564,
        block_0x00216594,
        block_0x002165b0,
        block_0x002165c4,
        block_0x002165d8,
        block_0x00216664,
        block_0x00216678,
        block_0x00216684,
        block_0x00216698,
        block_0x002166ac,
        block_0x002166f4,
        block_0x00216708,
        block_0x00216710,
        block_0x0021684c,
        block_0x00216898,
        block_0x002168a8,
        block_0x002168c4,
        block_0x00216920,
        block_0x00216958,
        block_0x0021698c,
        block_0x002169a0,
        block_0x002169dc,
        block_0x002169fc,
        block_0x00216a10,
        block_0x00216a28,
        block_0x00216a34,
        block_0x00216a50,
        block_0x00216a5c,
        block_0x00216a74,
        block_0x00216a84,
        block_0x00216aa0,
        block_0x00216aac,
        block_0x00216ac8,
        block_0x00216ad8,
        block_0x00216ae0,
        block_0x00216af4,
        block_0x00216af8,
        block_0x00216b08,
        block_0x00216b0c,
        block_0x00216b14,
        block_0x00216b24,
        block_0x00216b28,
        block_0x00216b34,
        block_0x00216b38,
        block_0x00216b3c,
        block_0x00216b58,
        block_0x00216b60,
        block_0x00216b64,
        block_0x00216b78,
        block_0x00216b80,
        block_0x00216b94,
        block_0x00216b98,
        block_0x00216ba4,
        block_0x00216bac,
        block_0x00216bb0,
        block_0x00216bb8,
        block_0x00216bbc,
        block_0x00216bc0,
        block_0x00216bc4,
        block_0x00216be0,
        block_0x00216bf0,
        block_0x00216bf4,
        block_0x00216bfc,
        block_0x00216c0c,
        block_0x00216c20,
        block_0x00216c38,
        block_0x00216c40,
        block_0x00216c60,
        block_0x00216c74,
        block_0x00216c7c,
        block_0x00216c8c,
        block_0x00216c90,
        block_0x00216cc0,
        block_0x00216cd4,
        block_0x00216cdc,
        block_0x00216d14,
        block_0x00216d24,
        block_0x00216d34,
        block_0x00216d3c,
        block_0x00216d58,
        block_0x00216d5c,
        block_0x00216d6c,
        block_0x00216d74,
        block_0x00216d7c,
        block_0x00216d84,
        block_0x00216d8c,
        block_0x00216d9c,
        block_0x00216da4,
        block_0x00216dac,
        block_0x00216db4,
        block_0x00216dbc,
        block_0x00216dc8,
        block_0x00216dd0,
        block_0x00216ddc,
        block_0x00216de8,
        block_0x00216dec,
        block_0x00216e04,
        block_0x00216e08,
        block_0x00216e24,
        block_0x00216e34,
        block_0x00216e38,
        block_0x00216e44,
        block_0x00216e6c,
        block_0x00216e78,
        block_0x00216e7c,
        block_0x00216e88,
        block_0x00216eac,
        block_0x00216eb8,
        block_0x00216eec,
        block_0x00216ef4,
        block_0x00216f0c,
        block_0x00216f14,
        block_0x00216f1c,
        block_0x00216f34,
        block_0x00216f3c,
        block_0x00216f44,
        block_0x00216f5c,
        block_0x00216f64,
        block_0x00216f7c,
        block_0x00216f84,
        block_0x00216f94,
        block_0x00216fa4,
        block_0x00216fb8,
        block_0x00216fc8,
        block_0x00216fe0,
    ];
    const IDX: [u16; 786usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16,
        6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16,
        18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16,
        0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16,
        26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        40u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16,
        43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 45u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 0u16, 48u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 53u16, 0u16,
        0u16, 0u16, 54u16, 55u16, 0u16, 0u16, 56u16, 57u16, 58u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 59u16, 0u16, 60u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 66u16, 0u16, 67u16,
        68u16, 0u16, 69u16, 70u16, 71u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        73u16, 0u16, 0u16, 0u16, 74u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16,
        0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 82u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16,
        97u16, 0u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 101u16, 0u16,
        102u16, 0u16, 103u16, 0u16, 104u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16,
        107u16, 0u16, 0u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 111u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 113u16, 114u16,
        0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16,
        0u16, 0u16, 117u16, 118u16, 0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 122u16, 0u16, 123u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 124u16, 0u16, 125u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 127u16,
        0u16, 128u16, 0u16, 129u16, 0u16, 0u16, 0u16, 0u16, 0u16, 130u16, 0u16, 131u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 132u16, 0u16, 133u16, 0u16, 0u16, 0u16, 134u16,
        0u16, 0u16, 0u16, 135u16, 0u16, 0u16, 0u16, 0u16, 136u16, 0u16, 0u16, 0u16,
        137u16, 0u16, 0u16, 0u16, 0u16, 0u16, 138u16,
    ];
    if pc < 2188188u32 || pc > 2191328u32 {
        return None;
    }
    let word_offset = ((pc - 2188188u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021639c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2188192u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2188196u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188200u32;
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
pub fn block_0x002163a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2188204u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2188208u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2188212u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2188216u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2188220u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2188224u32);
    emu.apc_no_count(6usize, 2188224u32, 32768u32, 2188228u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2188232u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002163c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2188236u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2188240u32)?;
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2188244u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966624u32, 2188248u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2188252u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966172u32, 2188256u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2188260u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2188264u32)?;
    emu.adi_no_count(15usize, 2usize, 36u32, 2188268u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2188272u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2188276u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2188280u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2188284u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2188288u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2188292u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2188296u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2188300u32)?;
    emu.adi_no_count(12usize, 2usize, 12u32, 2188304u32);
    emu.apc_no_count(1usize, 2188304u32, 28672u32, 2188308u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188312u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2188316u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2188320u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188324u32;
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
pub fn block_0x00216424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2188328u32);
    emu.sb_no_count(10usize, 2usize, 15u32, 2188332u32);
    emu.lbu_no_count(10usize, 2usize, 15u32, 2188336u32);
    emu.adi_no_count(2usize, 2usize, 16u32, 2188340u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188344u32;
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
pub fn block_0x00216438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2188348u32);
    emu.apc_no_count(6usize, 2188348u32, 0u32, 2188352u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2188356u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00216444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966944u32, 2188360u32);
    emu.sw_no_count(1usize, 2usize, 348u32, 2188364u32)?;
    emu.sw_no_count(8usize, 2usize, 344u32, 2188368u32)?;
    emu.sw_no_count(9usize, 2usize, 340u32, 2188372u32)?;
    emu.sw_no_count(18usize, 2usize, 336u32, 2188376u32)?;
    emu.sw_no_count(19usize, 2usize, 332u32, 2188380u32)?;
    emu.sw_no_count(20usize, 2usize, 328u32, 2188384u32)?;
    emu.sw_no_count(21usize, 2usize, 324u32, 2188388u32)?;
    emu.sw_no_count(22usize, 2usize, 320u32, 2188392u32)?;
    emu.sw_no_count(23usize, 2usize, 316u32, 2188396u32)?;
    emu.sw_no_count(24usize, 2usize, 312u32, 2188400u32)?;
    emu.sw_no_count(25usize, 2usize, 308u32, 2188404u32)?;
    emu.sw_no_count(26usize, 2usize, 304u32, 2188408u32)?;
    emu.sw_no_count(27usize, 2usize, 300u32, 2188412u32)?;
    emu.adi_no_count(18usize, 12usize, 0u32, 2188416u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2188420u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2188424u32);
    emu.adi_no_count(9usize, 2usize, 48u32, 2188428u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2188432u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2188436u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2188440u32);
    emu.apc_no_count(1usize, 2188440u32, 4294909952u32, 2188444u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188448u32;
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
pub fn block_0x002164a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2188452u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2188456u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2188460u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2188464u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2188468u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2188472u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2188476u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2188480u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2188484u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2188488u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2188492u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2188496u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2188500u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2188504u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2188508u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2188512u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2188516u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2188520u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2188524u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2188528u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2188532u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2188536u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2188540u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2188544u32)?;
    emu.sw_no_count(0usize, 2usize, 40u32, 2188548u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2188552u32)?;
    emu.add_memory_rw_events(26usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2188740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165c4));
    } else {
        emu.pc = 2188556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021650c));
    }
}
#[inline(always)]
pub fn block_0x0021650c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2188560u32);
    emu.adi_no_count(19usize, 19usize, 64u32, 2188564u32);
    emu.sli_no_count(20usize, 18usize, 5u32, 2188568u32);
    emu.adi_no_count(21usize, 0usize, 32u32, 2188572u32);
    emu.adi_no_count(22usize, 0usize, 64u32, 2188576u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2188580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2188616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216548));
}
#[inline(always)]
pub fn block_0x00216524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 32u32, 2188584u32);
    emu.apc_no_count(1usize, 2188584u32, 4294914048u32, 2188588u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 23usize, 32u32, 2188596u32);
    emu.ani_no_count(23usize, 18usize, 255u32, 2188600u32);
    emu.sb_no_count(18usize, 2usize, 112u32, 2188604u32);
    emu.adi_no_count(20usize, 20usize, 4294967264u32, 2188608u32);
    emu.adi_no_count(19usize, 19usize, 32u32, 2188612u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2188740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165c4));
    } else {
        emu.pc = 2188616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216548));
    }
}
#[inline(always)]
pub fn block_0x00216548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 4294967232u32, 2188620u32);
    emu.adr_no_count(10usize, 9usize, 23usize, 2188624u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2188580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216524));
    } else {
        emu.pc = 2188628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216554));
    }
}
#[inline(always)]
pub fn block_0x00216554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 22usize, 23usize, 2188632u32);
    emu.adi_no_count(18usize, 23usize, 4294967264u32, 2188636u32);
    emu.apc_no_count(1usize, 2188636u32, 4294909952u32, 2188640u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00216564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 40u32, 2188648u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2188652u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2188656u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2188660u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2188664u32);
    emu.sw_no_count(10usize, 2usize, 40u32, 2188668u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2188672u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2188676u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2188680u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2188684u32);
    emu.apc_no_count(1usize, 2188684u32, 0u32, 2188688u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 18usize, 4294967232u32, 2188696u32);
    emu.sbr_no_count(10usize, 19usize, 23usize, 2188700u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2188704u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2188708u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2188712u32);
    emu.apc_no_count(1usize, 2188712u32, 4294909952u32, 2188716u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002165b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(23usize, 18usize, 255u32, 2188724u32);
    emu.sb_no_count(18usize, 2usize, 112u32, 2188728u32);
    emu.adi_no_count(20usize, 20usize, 4294967264u32, 2188732u32);
    emu.adi_no_count(19usize, 19usize, 32u32, 2188736u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2188616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216548));
    } else {
        emu.pc = 2188740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165c4));
    }
}
#[inline(always)]
pub fn block_0x002165c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2188744u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2188748u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2188752u32);
    emu.apc_no_count(1usize, 2188752u32, 4294909952u32, 2188756u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002165d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 160u32, 2188764u32);
    emu.lbu_no_count(18usize, 2usize, 224u32, 2188768u32);
    emu.lw_no_count(10usize, 2usize, 152u32, 2188772u32)?;
    emu.lw_no_count(11usize, 2usize, 156u32, 2188776u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2188780u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2188784u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2188788u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2188792u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2188796u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2188800u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2188804u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2188808u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2188812u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2188816u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2188820u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2188824u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2188828u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2188832u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2188836u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2188840u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2188844u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2188848u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2188852u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2188856u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2188860u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2188864u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2188868u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2188872u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2188876u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2188880u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2188884u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2188888u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2188892u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2188896u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2188932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216684));
    } else {
        emu.pc = 2188900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216664));
    }
}
#[inline(always)]
pub fn block_0x00216664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2188904u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2188908u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2188912u32);
    emu.apc_no_count(1usize, 2188912u32, 4294909952u32, 2188916u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2188924u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2188928u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2189044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166f4));
    } else {
        emu.pc = 2188932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216684));
    }
}
#[inline(always)]
pub fn block_0x00216684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2188936u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2188940u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2188944u32);
    emu.apc_no_count(1usize, 2188944u32, 0u32, 2188948u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 236u32, 2188956u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2188960u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2188964u32);
    emu.apc_no_count(1usize, 2188964u32, 4294909952u32, 2188968u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002166ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2188976u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2188980u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2188984u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2188988u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2188992u32);
    emu.sb_no_count(20usize, 2usize, 296u32, 2188996u32);
    emu.sb_no_count(12usize, 2usize, 297u32, 2189000u32);
    emu.sb_no_count(11usize, 2usize, 298u32, 2189004u32);
    emu.sb_no_count(10usize, 2usize, 299u32, 2189008u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2189012u32);
    emu.sb_no_count(19usize, 2usize, 292u32, 2189016u32);
    emu.sb_no_count(10usize, 2usize, 293u32, 2189020u32);
    emu.sb_no_count(14usize, 2usize, 294u32, 2189024u32);
    emu.sb_no_count(13usize, 2usize, 295u32, 2189028u32);
    emu.adi_no_count(10usize, 2usize, 120u32, 2189032u32);
    emu.adi_no_count(11usize, 2usize, 236u32, 2189036u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2189040u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2189044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216708));
}
#[inline(always)]
pub fn block_0x002166f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 216u32, 2189048u32)?;
    emu.sw_no_count(20usize, 2usize, 220u32, 2189052u32)?;
    emu.adi_no_count(10usize, 2usize, 120u32, 2189056u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2189060u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2189064u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2189064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216708));
}
#[inline(always)]
pub fn block_0x00216708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2189064u32, 0u32, 2189068u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00216710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(28usize, 2usize, 120u32, 2189076u32)?;
    emu.lw_no_count(6usize, 2usize, 124u32, 2189080u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2189084u32)?;
    emu.lw_no_count(14usize, 2usize, 132u32, 2189088u32)?;
    emu.lw_no_count(13usize, 2usize, 136u32, 2189092u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2189096u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2189100u32)?;
    emu.lw_no_count(10usize, 2usize, 148u32, 2189104u32)?;
    emu.sri_no_count(31usize, 28usize, 24u32, 2189108u32);
    emu.sri_no_count(9usize, 6usize, 24u32, 2189112u32);
    emu.sri_no_count(30usize, 16usize, 24u32, 2189116u32);
    emu.sri_no_count(29usize, 14usize, 24u32, 2189120u32);
    emu.sri_no_count(7usize, 13usize, 24u32, 2189124u32);
    emu.sri_no_count(5usize, 12usize, 24u32, 2189128u32);
    emu.sri_no_count(17usize, 11usize, 24u32, 2189132u32);
    emu.sri_no_count(15usize, 10usize, 24u32, 2189136u32);
    emu.sri_no_count(18usize, 28usize, 8u32, 2189140u32);
    emu.sri_no_count(19usize, 28usize, 16u32, 2189144u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2189148u32);
    emu.sri_no_count(21usize, 6usize, 16u32, 2189152u32);
    emu.sri_no_count(22usize, 16usize, 8u32, 2189156u32);
    emu.sri_no_count(23usize, 16usize, 16u32, 2189160u32);
    emu.sri_no_count(24usize, 14usize, 8u32, 2189164u32);
    emu.sri_no_count(25usize, 14usize, 16u32, 2189168u32);
    emu.sri_no_count(26usize, 13usize, 8u32, 2189172u32);
    emu.sri_no_count(27usize, 13usize, 16u32, 2189176u32);
    emu.sri_no_count(1usize, 12usize, 8u32, 2189180u32);
    emu.sb_no_count(31usize, 8usize, 0u32, 2189184u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2189188u32);
    emu.sb_no_count(19usize, 8usize, 1u32, 2189192u32);
    emu.sri_no_count(19usize, 11usize, 8u32, 2189196u32);
    emu.sb_no_count(18usize, 8usize, 2u32, 2189200u32);
    emu.sb_no_count(28usize, 8usize, 3u32, 2189204u32);
    emu.sri_no_count(28usize, 11usize, 16u32, 2189208u32);
    emu.sb_no_count(9usize, 8usize, 4u32, 2189212u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2189216u32);
    emu.sb_no_count(20usize, 8usize, 6u32, 2189220u32);
    emu.sb_no_count(6usize, 8usize, 7u32, 2189224u32);
    emu.sri_no_count(6usize, 10usize, 8u32, 2189228u32);
    emu.sb_no_count(30usize, 8usize, 8u32, 2189232u32);
    emu.sb_no_count(23usize, 8usize, 9u32, 2189236u32);
    emu.sb_no_count(22usize, 8usize, 10u32, 2189240u32);
    emu.sb_no_count(16usize, 8usize, 11u32, 2189244u32);
    emu.sri_no_count(16usize, 10usize, 16u32, 2189248u32);
    emu.sb_no_count(29usize, 8usize, 12u32, 2189252u32);
    emu.sb_no_count(25usize, 8usize, 13u32, 2189256u32);
    emu.sb_no_count(24usize, 8usize, 14u32, 2189260u32);
    emu.sb_no_count(14usize, 8usize, 15u32, 2189264u32);
    emu.sb_no_count(7usize, 8usize, 16u32, 2189268u32);
    emu.sb_no_count(27usize, 8usize, 17u32, 2189272u32);
    emu.sb_no_count(26usize, 8usize, 18u32, 2189276u32);
    emu.sb_no_count(13usize, 8usize, 19u32, 2189280u32);
    emu.sb_no_count(5usize, 8usize, 20u32, 2189284u32);
    emu.sb_no_count(31usize, 8usize, 21u32, 2189288u32);
    emu.sb_no_count(1usize, 8usize, 22u32, 2189292u32);
    emu.sb_no_count(12usize, 8usize, 23u32, 2189296u32);
    emu.sb_no_count(17usize, 8usize, 24u32, 2189300u32);
    emu.sb_no_count(28usize, 8usize, 25u32, 2189304u32);
    emu.sb_no_count(19usize, 8usize, 26u32, 2189308u32);
    emu.sb_no_count(11usize, 8usize, 27u32, 2189312u32);
    emu.sb_no_count(15usize, 8usize, 28u32, 2189316u32);
    emu.sb_no_count(16usize, 8usize, 29u32, 2189320u32);
    emu.sb_no_count(6usize, 8usize, 30u32, 2189324u32);
    emu.sb_no_count(10usize, 8usize, 31u32, 2189328u32);
    emu.lw_no_count(1usize, 2usize, 348u32, 2189332u32)?;
    emu.lw_no_count(8usize, 2usize, 344u32, 2189336u32)?;
    emu.lw_no_count(9usize, 2usize, 340u32, 2189340u32)?;
    emu.lw_no_count(18usize, 2usize, 336u32, 2189344u32)?;
    emu.lw_no_count(19usize, 2usize, 332u32, 2189348u32)?;
    emu.lw_no_count(20usize, 2usize, 328u32, 2189352u32)?;
    emu.lw_no_count(21usize, 2usize, 324u32, 2189356u32)?;
    emu.lw_no_count(22usize, 2usize, 320u32, 2189360u32)?;
    emu.lw_no_count(23usize, 2usize, 316u32, 2189364u32)?;
    emu.lw_no_count(24usize, 2usize, 312u32, 2189368u32)?;
    emu.lw_no_count(25usize, 2usize, 308u32, 2189372u32)?;
    emu.lw_no_count(26usize, 2usize, 304u32, 2189376u32)?;
    emu.lw_no_count(27usize, 2usize, 300u32, 2189380u32)?;
    emu.adi_no_count(2usize, 2usize, 352u32, 2189384u32);
    emu.add_memory_rw_events(79usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189388u32;
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
pub fn block_0x0021684c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2189392u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2189396u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2189400u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2189404u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2189408u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2189412u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2189416u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2189420u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2189424u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2189428u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2189432u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2189436u32);
    emu.lw_no_count(11usize, 11usize, 52u32, 2189440u32)?;
    emu.lw_no_count(18usize, 9usize, 56u32, 2189444u32)?;
    emu.adi_no_count(19usize, 0usize, 1u32, 2189448u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2189452u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2189456u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2189460u32)?;
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2189788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002169dc));
    } else {
        emu.pc = 2189464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216898));
    }
}
#[inline(always)]
pub fn block_0x00216898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 19usize, 20usize, 2189468u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2189472u32);
    emu.apc_no_count(1usize, 2189472u32, 4294909952u32, 2189476u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002168a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 20usize, 18usize, 2189484u32);
    emu.lw_no_count(20usize, 9usize, 32u32, 2189488u32)?;
    emu.lw_no_count(18usize, 9usize, 36u32, 2189492u32)?;
    emu.sbr_no_count(10usize, 21usize, 11usize, 2189496u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2189500u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2189504u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2189840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216a10));
    } else {
        emu.pc = 2189508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002168c4));
    }
}
#[inline]
pub fn block_0x002168c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 19usize, 11usize, 2189512u32);
    emu.sri_no_count(10usize, 20usize, 24u32, 2189516u32);
    emu.sri_no_count(12usize, 20usize, 16u32, 2189520u32);
    emu.sri_no_count(13usize, 20usize, 8u32, 2189524u32);
    emu.sri_no_count(14usize, 18usize, 24u32, 2189528u32);
    emu.sri_no_count(15usize, 18usize, 16u32, 2189532u32);
    emu.sb_no_count(20usize, 11usize, 0u32, 2189536u32);
    emu.sb_no_count(13usize, 11usize, 1u32, 2189540u32);
    emu.sb_no_count(12usize, 11usize, 2u32, 2189544u32);
    emu.sb_no_count(10usize, 11usize, 3u32, 2189548u32);
    emu.sri_no_count(10usize, 18usize, 8u32, 2189552u32);
    emu.sb_no_count(18usize, 11usize, 4u32, 2189556u32);
    emu.sb_no_count(10usize, 11usize, 5u32, 2189560u32);
    emu.sb_no_count(15usize, 11usize, 6u32, 2189564u32);
    emu.sb_no_count(14usize, 11usize, 7u32, 2189568u32);
    emu.lw_no_count(15usize, 2usize, 16u32, 2189572u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2189576u32)?;
    emu.lw_no_count(20usize, 9usize, 40u32, 2189580u32)?;
    emu.adi_no_count(15usize, 15usize, 8u32, 2189584u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2189588u32);
    emu.adi_no_count(18usize, 0usize, 3u32, 2189592u32);
    emu.sw_no_count(15usize, 2usize, 16u32, 2189596u32)?;
    emu.add_memory_rw_events(22usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2189876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216a34));
    } else {
        emu.pc = 2189600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216920));
    }
}
#[inline]
pub fn block_0x00216920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 12u32, 2189604u32)?;
    emu.sri_no_count(13usize, 20usize, 24u32, 2189608u32);
    emu.sri_no_count(14usize, 20usize, 16u32, 2189612u32);
    emu.sri_no_count(16usize, 20usize, 8u32, 2189616u32);
    emu.adi_no_count(11usize, 15usize, 4u32, 2189620u32);
    emu.lw_no_count(19usize, 9usize, 44u32, 2189624u32)?;
    emu.adr_no_count(15usize, 12usize, 15usize, 2189628u32);
    emu.sb_no_count(20usize, 15usize, 0u32, 2189632u32);
    emu.sb_no_count(16usize, 15usize, 1u32, 2189636u32);
    emu.sb_no_count(14usize, 15usize, 2u32, 2189640u32);
    emu.sb_no_count(13usize, 15usize, 3u32, 2189644u32);
    emu.sbr_no_count(13usize, 10usize, 11usize, 2189648u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2189652u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2189916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216a5c));
    } else {
        emu.pc = 2189656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216958));
    }
}
#[inline]
pub fn block_0x00216958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 12usize, 11usize, 2189660u32);
    emu.sri_no_count(14usize, 19usize, 24u32, 2189664u32);
    emu.sri_no_count(15usize, 19usize, 16u32, 2189668u32);
    emu.sri_no_count(16usize, 19usize, 8u32, 2189672u32);
    emu.adi_no_count(18usize, 11usize, 4u32, 2189676u32);
    emu.sb_no_count(19usize, 13usize, 0u32, 2189680u32);
    emu.sb_no_count(16usize, 13usize, 1u32, 2189684u32);
    emu.sb_no_count(15usize, 13usize, 2u32, 2189688u32);
    emu.sb_no_count(14usize, 13usize, 3u32, 2189692u32);
    emu.sbr_no_count(10usize, 10usize, 18usize, 2189696u32);
    emu.adi_no_count(11usize, 0usize, 31u32, 2189700u32);
    emu.sw_no_count(18usize, 2usize, 16u32, 2189704u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2189956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216a84));
    } else {
        emu.pc = 2189708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021698c));
    }
}
#[inline(always)]
pub fn block_0x0021698c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 12usize, 18usize, 2189712u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2189716u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2189720u32);
    emu.apc_no_count(1usize, 2189720u32, 4294909952u32, 2189724u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002169a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2189732u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2189736u32)?;
    emu.adi_no_count(12usize, 18usize, 32u32, 2189740u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2189744u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2189748u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2189752u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2189756u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2189760u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2189764u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2189768u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2189772u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2189776u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2189780u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2189784u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189788u32;
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
pub fn block_0x002169dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2189792u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2189796u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2189800u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2189804u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2189808u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2189812u32);
    emu.apc_no_count(1usize, 2189812u32, 0u32, 2189816u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002169fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2189824u32);
    emu.lw_no_count(21usize, 2usize, 8u32, 2189828u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2189832u32)?;
    emu.lw_no_count(20usize, 2usize, 16u32, 2189836u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2189840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216898));
}
#[inline(always)]
pub fn block_0x00216a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2189844u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2189848u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2189852u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2189856u32);
    emu.apc_no_count(1usize, 2189856u32, 0u32, 2189860u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 12u32, 2189868u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2189872u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2189876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189508u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002168c4));
}
#[inline(always)]
pub fn block_0x00216a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2189880u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2189884u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2189888u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2189892u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2189896u32);
    emu.apc_no_count(1usize, 2189896u32, 0u32, 2189900u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2189908u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2189912u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2189916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216920));
}
#[inline(always)]
pub fn block_0x00216a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2189920u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2189924u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2189928u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2189932u32);
    emu.apc_no_count(1usize, 2189932u32, 0u32, 2189936u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2189944u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2189948u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2189952u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2189956u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216958));
}
#[inline(always)]
pub fn block_0x00216a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2189960u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2189964u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2189968u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2189972u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2189976u32);
    emu.apc_no_count(1usize, 2189976u32, 0u32, 2189980u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 12u32, 2189988u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2189992u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2189996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189708u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021698c));
}
#[inline(always)]
pub fn block_0x00216aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2190000u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2190004u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2190008u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2190012u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2190016u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2190020u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2190132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b34));
    } else {
        emu.pc = 2190024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216ac8));
    }
}
#[inline(always)]
pub fn block_0x00216ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2190028u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2190032u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2190036u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2190088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b08));
    } else {
        emu.pc = 2190040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216ad8));
    }
}
#[inline(always)]
pub fn block_0x00216ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2190044u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2190088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b08));
    } else {
        emu.pc = 2190048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216ae0));
    }
}
#[inline(always)]
pub fn block_0x00216ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2190052u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2190056u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2190060u32);
    emu.apc_no_count(1usize, 2190060u32, 4294909952u32, 2190064u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190068u32;
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
#[inline(always)]
pub fn block_0x00216af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2190120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b28));
    } else {
        emu.pc = 2190072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216af8));
    }
}
#[inline(always)]
pub fn block_0x00216af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2190076u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2190080u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2190084u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2190088u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b3c));
}
#[inline(always)]
pub fn block_0x00216b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2190168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b58));
    } else {
        emu.pc = 2190092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b0c));
    }
}
#[inline(always)]
pub fn block_0x00216b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2190092u32, 4294909952u32, 2190096u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2190104u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2190108u32);
    emu.apc_no_count(1usize, 2190108u32, 4294905856u32, 2190112u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2190072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216af8));
    } else {
        emu.pc = 2190120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b28));
    }
}
#[inline(always)]
pub fn block_0x00216b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2190124u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2190128u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2190132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b38));
}
#[inline(always)]
pub fn block_0x00216b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2190136u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2190136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b38));
}
#[inline(always)]
pub fn block_0x00216b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2190140u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2190140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b3c));
}
#[inline(always)]
pub fn block_0x00216b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2190144u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2190148u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2190152u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2190156u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2190160u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2190164u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190168u32;
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
pub fn block_0x00216b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2190172u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2190072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216af8));
    } else {
        emu.pc = 2190176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b60));
    }
}
#[inline(always)]
pub fn block_0x00216b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2190180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190120u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b28));
}
#[inline(always)]
pub fn block_0x00216b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2190184u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2190188u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2190192u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2190196u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2190432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216c60));
    } else {
        emu.pc = 2190200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b78));
    }
}
#[inline(always)]
pub fn block_0x00216b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 11usize, 12usize, 2190204u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2190432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216c60));
    } else {
        emu.pc = 2190208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b80));
    }
}
#[inline(always)]
pub fn block_0x00216b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2190212u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2190216u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2190220u32)?;
    emu.sli_no_count(10usize, 13usize, 1u32, 2190224u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2190232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b98));
    } else {
        emu.pc = 2190228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b94));
    }
}
#[inline(always)]
pub fn block_0x00216b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2190232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2190232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b98));
}
#[inline(always)]
pub fn block_0x00216b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1025u32, 2190236u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2190240u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2190256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bb0));
    } else {
        emu.pc = 2190244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216ba4));
    }
}
#[inline(always)]
pub fn block_0x00216ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2190248u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2190264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bb8));
    } else {
        emu.pc = 2190252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bac));
    }
}
#[inline(always)]
pub fn block_0x00216bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2190256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216bbc));
}
#[inline(always)]
pub fn block_0x00216bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2190260u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2190268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bbc));
    } else {
        emu.pc = 2190264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bb8));
    }
}
#[inline(always)]
pub fn block_0x00216bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2190268u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2190268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216bbc));
}
#[inline(always)]
pub fn block_0x00216bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2190276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bc4));
    } else {
        emu.pc = 2190272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bc0));
    }
}
#[inline(always)]
pub fn block_0x00216bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2190276u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2190276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216bc4));
}
#[inline(always)]
pub fn block_0x00216bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 15usize, 14usize, 2190280u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2190284u32);
    emu.sbr_no_count(11usize, 0usize, 15usize, 2190288u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2190292u32);
    emu.mulhu_no_count(12usize, 11usize, 9usize, 2190296u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2190300u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2190332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bfc));
    } else {
        emu.pc = 2190304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216be0));
    }
}
#[inline(always)]
pub fn block_0x00216be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 11usize, 9usize, 2190308u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2190312u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 11usize, 15usize, 2190316u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2190460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216c7c));
    } else {
        emu.pc = 2190320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bf0));
    }
}
#[inline(always)]
pub fn block_0x00216bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2190348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216c0c));
    } else {
        emu.pc = 2190324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216bf4));
    }
}
#[inline(always)]
pub fn block_0x00216bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2190328u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2190332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190368u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216c20));
}
#[inline(always)]
pub fn block_0x00216bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2190336u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965764u32, 2190340u32);
    emu.apc_no_count(1usize, 2190340u32, 16384u32, 2190344u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2190352u32)?;
    emu.mul_no_count(11usize, 13usize, 14usize, 2190356u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2190360u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2190364u32)?;
    emu.adi_no_count(10usize, 15usize, 0u32, 2190368u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2190368u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216c20));
}
#[inline(always)]
pub fn block_0x00216c20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2190372u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2190376u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2190380u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2190384u32);
    emu.apc_no_count(1usize, 2190384u32, 0u32, 2190388u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190392u32;
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
pub fn block_0x00216c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2190396u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2190452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216c74));
    } else {
        emu.pc = 2190400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216c40));
    }
}
#[inline(always)]
pub fn block_0x00216c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2190404u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2190408u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2190412u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2190416u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2190420u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2190424u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2190428u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190432u32;
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
pub fn block_0x00216c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2190436u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2190440u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965764u32, 2190444u32);
    emu.apc_no_count(1usize, 2190444u32, 16384u32, 2190448u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190452u32;
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
#[inline(always)]
pub fn block_0x00216c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2190456u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2190460u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2190460u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216c7c));
}
#[inline(always)]
pub fn block_0x00216c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2190464u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965764u32, 2190468u32);
    emu.apc_no_count(1usize, 2190468u32, 16384u32, 2190472u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2190680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d58));
    } else {
        emu.pc = 2190480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216c90));
    }
}
#[inline]
pub fn block_0x00216c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967008u32, 2190484u32);
    emu.sw_no_count(1usize, 2usize, 284u32, 2190488u32)?;
    emu.sw_no_count(8usize, 2usize, 280u32, 2190492u32)?;
    emu.sw_no_count(9usize, 2usize, 276u32, 2190496u32)?;
    emu.sw_no_count(18usize, 2usize, 272u32, 2190500u32)?;
    emu.sw_no_count(19usize, 2usize, 268u32, 2190504u32)?;
    emu.sw_no_count(20usize, 2usize, 264u32, 2190508u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2190512u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2190516u32);
    emu.adi_no_count(18usize, 0usize, 0u32, 2190520u32);
    emu.adi_no_count(19usize, 11usize, 3u32, 2190524u32);
    emu.adi_no_count(20usize, 2usize, 72u32, 2190528u32);
    emu.add_memory_rw_events(12usize);
    emu.pc = 2190528u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216cc0));
}
#[inline(always)]
pub fn block_0x00216cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2190532u32);
    emu.adi_no_count(12usize, 0usize, 256u32, 2190536u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2190540u32);
    emu.apc_no_count(1usize, 2190540u32, 4294909952u32, 2190544u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2190552u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2190556u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2190556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216cdc));
}
#[inline]
pub fn block_0x00216cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 4294967295u32, 2190560u32);
    emu.lbu_no_count(13usize, 10usize, 4294967294u32, 2190564u32);
    emu.lbu_no_count(14usize, 10usize, 4294967293u32, 2190568u32);
    emu.lbu_no_count(15usize, 10usize, 0u32, 2190572u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2190576u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2190580u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2190584u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2190588u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2190592u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2190596u32);
    emu.sw_no_count(12usize, 11usize, 0u32, 2190600u32)?;
    emu.adi_no_count(11usize, 11usize, 4u32, 2190604u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2190608u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2190556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216cdc));
    } else {
        emu.pc = 2190612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d14));
    }
}
#[inline(always)]
pub fn block_0x00216d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2190616u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2190620u32);
    emu.apc_no_count(1usize, 2190620u32, 4294909952u32, 2190624u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190628u32;
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
pub fn block_0x00216d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2190632u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2190636u32);
    emu.apc_no_count(1usize, 2190636u32, 4294909952u32, 2190640u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 64u32, 2190648u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2190528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216cc0));
    } else {
        emu.pc = 2190652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d3c));
    }
}
#[inline(always)]
pub fn block_0x00216d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 284u32, 2190656u32)?;
    emu.lw_no_count(8usize, 2usize, 280u32, 2190660u32)?;
    emu.lw_no_count(9usize, 2usize, 276u32, 2190664u32)?;
    emu.lw_no_count(18usize, 2usize, 272u32, 2190668u32)?;
    emu.lw_no_count(19usize, 2usize, 268u32, 2190672u32)?;
    emu.lw_no_count(20usize, 2usize, 264u32, 2190676u32)?;
    emu.adi_no_count(2usize, 2usize, 288u32, 2190680u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2190680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216d58));
}
#[inline(always)]
pub fn block_0x00216d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190684u32;
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
pub fn block_0x00216d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2190688u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2190692u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2190696u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2190716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d7c));
    } else {
        emu.pc = 2190700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d6c));
    }
}
#[inline(always)]
pub fn block_0x00216d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2190704u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2190724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d84));
    } else {
        emu.pc = 2190708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d74));
    }
}
#[inline(always)]
pub fn block_0x00216d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2190708u32, 16384u32, 2190712u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2190716u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00216d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2190716u32, 16384u32, 2190720u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2190724u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2190724u32, 16384u32, 2190728u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2190732u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2190736u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2190740u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2190744u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2190764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216dac));
    } else {
        emu.pc = 2190748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216d9c));
    }
}
#[inline(always)]
pub fn block_0x00216d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2190752u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2190772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216db4));
    } else {
        emu.pc = 2190756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216da4));
    }
}
#[inline(always)]
pub fn block_0x00216da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2190756u32, 16384u32, 2190760u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2190764u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00216dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2190764u32, 16384u32, 2190768u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2190772u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2190772u32, 16384u32, 2190776u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2190780u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2190784u32)?;
    emu.apc_no_count(6usize, 2190784u32, 0u32, 2190788u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2190792u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2190796u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2190884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e24));
    } else {
        emu.pc = 2190800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216dd0));
    }
}
#[inline(always)]
pub fn block_0x00216dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2190804u32)?;
    emu.lw_no_count(16usize, 11usize, 8u32, 2190808u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2190956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e6c));
    } else {
        emu.pc = 2190812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216ddc));
    }
}
#[inline(always)]
pub fn block_0x00216ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 16usize, 0u32, 2190816u32);
    emu.adi_no_count(14usize, 0usize, 39u32, 2190820u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2191204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f64));
    } else {
        emu.pc = 2190824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216de8));
    }
}
#[inline(always)]
pub fn block_0x00216de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2191020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216eac));
    } else {
        emu.pc = 2190828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216dec));
    }
}
#[inline(always)]
pub fn block_0x00216dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 16usize, 1u32, 2190832u32);
    emu.adr_no_count(14usize, 16usize, 12usize, 2190836u32);
    emu.lb_no_count(17usize, 14usize, 0u32, 2190840u32);
    emu.ani_no_count(15usize, 17usize, 127u32, 2190844u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2190848u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2191084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216eec));
    } else {
        emu.pc = 2190852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e04));
    }
}
#[inline(always)]
pub fn block_0x00216e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 1u32, 2190856u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2190856u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216e08));
}
#[inline(always)]
pub fn block_0x00216e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 16usize, 12usize, 2190860u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2190864u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2190868u32)?;
    emu.sw_no_count(14usize, 10usize, 4u32, 2190872u32)?;
    emu.sw_no_count(15usize, 10usize, 8u32, 2190876u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2190880u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190884u32;
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
pub fn block_0x00216e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2190888u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2190892u32);
    emu.adi_no_count(14usize, 0usize, 40u32, 2190896u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2191204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f64));
    } else {
        emu.pc = 2190900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e34));
    }
}
#[inline(always)]
pub fn block_0x00216e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2191304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fc8));
    } else {
        emu.pc = 2190904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e38));
    }
}
#[inline(always)]
pub fn block_0x00216e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 12usize, 1u32, 2190908u32);
    emu.adi_no_count(13usize, 0usize, 119u32, 2190912u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2190984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e88));
    } else {
        emu.pc = 2190916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e44));
    }
}
#[inline]
pub fn block_0x00216e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 1u32, 2190920u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2190924u32)?;
    emu.sw_no_count(0usize, 11usize, 4u32, 2190928u32)?;
    let a = 0u32.wrapping_add(107372544u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2190932u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1639u32, 2190936u32);
    emu.mulhu_no_count(11usize, 12usize, 11usize, 2190940u32);
    emu.sw_no_count(14usize, 10usize, 4u32, 2190944u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2190948u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2190952u32)?;
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190956u32;
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
pub fn block_0x00216e6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 16usize, 0u32, 2190960u32);
    emu.adi_no_count(12usize, 0usize, 40u32, 2190964u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2191204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f64));
    } else {
        emu.pc = 2190968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e78));
    }
}
#[inline(always)]
pub fn block_0x00216e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2191328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fe0));
    } else {
        emu.pc = 2190972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e7c));
    }
}
#[inline(always)]
pub fn block_0x00216e7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 16usize, 1u32, 2190976u32);
    emu.adi_no_count(13usize, 0usize, 119u32, 2190980u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2191032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216eb8));
    } else {
        emu.pc = 2190984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216e88));
    }
}
#[inline]
pub fn block_0x00216e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(107372544u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2190988u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1639u32, 2190992u32);
    emu.mulhu_no_count(11usize, 12usize, 11usize, 2190996u32);
    emu.sb_no_count(0usize, 10usize, 4u32, 2191000u32);
    emu.sb_no_count(12usize, 10usize, 5u32, 2191004u32);
    emu.sw_no_count(11usize, 10usize, 8u32, 2191008u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2191012u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2191016u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191020u32;
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
pub fn block_0x00216eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 4u32, 2191024u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2191028u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191032u32;
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
pub fn block_0x00216eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 1u32, 2191036u32);
    let a = 0u32.wrapping_add(107372544u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2191040u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 11usize, 0u32, 2191044u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2191048u32)?;
    emu.adi_no_count(11usize, 0usize, 40u32, 2191052u32);
    emu.adi_no_count(15usize, 15usize, 1639u32, 2191056u32);
    emu.mulhu_no_count(15usize, 12usize, 15usize, 2191060u32);
    emu.mul_no_count(11usize, 15usize, 11usize, 2191064u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2191068u32);
    emu.sw_no_count(14usize, 10usize, 4u32, 2191072u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2191076u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2191080u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191084u32;
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
pub fn block_0x00216eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 1u32, 2191088u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2191268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fa4));
    } else {
        emu.pc = 2191092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216ef4));
    }
}
#[inline(always)]
pub fn block_0x00216ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 16usize, 17usize, 2191096u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2191100u32);
    emu.sli_no_count(15usize, 15usize, 7u32, 2191104u32);
    emu.ani_no_count(5usize, 17usize, 127u32, 2191108u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2191112u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2191124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f14));
    } else {
        emu.pc = 2191116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f0c));
    }
}
#[inline(always)]
pub fn block_0x00216f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 2u32, 2191120u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2191124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190856u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216e08));
}
#[inline(always)]
pub fn block_0x00216f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 2u32, 2191128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2191268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fa4));
    } else {
        emu.pc = 2191132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f1c));
    }
}
#[inline(always)]
pub fn block_0x00216f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 16usize, 17usize, 2191136u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2191140u32);
    emu.sli_no_count(15usize, 15usize, 7u32, 2191144u32);
    emu.ani_no_count(5usize, 17usize, 127u32, 2191148u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2191152u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2191164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f3c));
    } else {
        emu.pc = 2191156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f34));
    }
}
#[inline(always)]
pub fn block_0x00216f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 3u32, 2191160u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2191164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190856u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216e08));
}
#[inline(always)]
pub fn block_0x00216f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 3u32, 2191168u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2191268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fa4));
    } else {
        emu.pc = 2191172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f44));
    }
}
#[inline(always)]
pub fn block_0x00216f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 16usize, 17usize, 2191176u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2191180u32);
    emu.sli_no_count(15usize, 15usize, 7u32, 2191184u32);
    emu.ani_no_count(5usize, 17usize, 127u32, 2191188u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2191192u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2191228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f7c));
    } else {
        emu.pc = 2191196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f5c));
    }
}
#[inline(always)]
pub fn block_0x00216f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 4u32, 2191200u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2191204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190856u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216e08));
}
#[inline(always)]
pub fn block_0x00216f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2191208u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965892u32, 2191212u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2191216u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2191220u32);
    emu.apc_no_count(1usize, 2191220u32, 28672u32, 2191224u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 4u32, 2191232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2191268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fa4));
    } else {
        emu.pc = 2191236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f84));
    }
}
#[inline(always)]
pub fn block_0x00216f84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 16usize, 17usize, 2191240u32);
    emu.lbu_no_count(13usize, 16usize, 0u32, 2191244u32);
    emu.adi_no_count(16usize, 0usize, 16u32, 2191248u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2191288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fb8));
    } else {
        emu.pc = 2191252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f94));
    }
}
#[inline(always)]
pub fn block_0x00216f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 7u32, 2191256u32);
    emu.orr_no_count(15usize, 15usize, 13usize, 2191260u32);
    emu.adi_no_count(16usize, 0usize, 5u32, 2191264u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2191268u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190856u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216e08));
}
#[inline(always)]
pub fn block_0x00216fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2191272u32);
    emu.sb_no_count(11usize, 10usize, 4u32, 2191276u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2191280u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2191284u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191288u32;
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
pub fn block_0x00216fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2191292u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2191296u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2191300u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191304u32;
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
pub fn block_0x00216fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2191308u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965812u32, 2191312u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2191316u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2191320u32);
    emu.apc_no_count(1usize, 2191320u32, 16384u32, 2191324u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2191332u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965828u32, 2191336u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2191340u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2191344u32);
    emu.apc_no_count(1usize, 2191344u32, 16384u32, 2191348u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
