pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2102020u32;
pub const PC_MAX: u32 = 2105036u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 139usize] = [
        block_0x00201304,
        block_0x00201324,
        block_0x00201334,
        block_0x0020133c,
        block_0x00201344,
        block_0x0020134c,
        block_0x00201354,
        block_0x00201364,
        block_0x00201394,
        block_0x002013a0,
        block_0x002013b4,
        block_0x002013c4,
        block_0x002013cc,
        block_0x002013d4,
        block_0x002013dc,
        block_0x002013e4,
        block_0x002013f0,
        block_0x00201458,
        block_0x00201464,
        block_0x00201484,
        block_0x00201490,
        block_0x00201498,
        block_0x002014a0,
        block_0x002014a8,
        block_0x002014b0,
        block_0x002014e4,
        block_0x002014ec,
        block_0x00201508,
        block_0x0020150c,
        block_0x00201520,
        block_0x0020152c,
        block_0x00201530,
        block_0x00201534,
        block_0x0020153c,
        block_0x00201554,
        block_0x0020155c,
        block_0x00201560,
        block_0x00201590,
        block_0x0020159c,
        block_0x002015a0,
        block_0x002015cc,
        block_0x002015d0,
        block_0x002015e0,
        block_0x002015f0,
        block_0x002015f4,
        block_0x002015fc,
        block_0x00201608,
        block_0x00201620,
        block_0x00201638,
        block_0x00201640,
        block_0x00201664,
        block_0x0020166c,
        block_0x00201678,
        block_0x002016a4,
        block_0x002016a8,
        block_0x002016b4,
        block_0x002016bc,
        block_0x002016c0,
        block_0x002016c8,
        block_0x002016d4,
        block_0x002016e8,
        block_0x00201700,
        block_0x00201708,
        block_0x0020172c,
        block_0x00201734,
        block_0x00201740,
        block_0x00201748,
        block_0x0020176c,
        block_0x00201778,
        block_0x00201790,
        block_0x002017bc,
        block_0x002017c4,
        block_0x002017d8,
        block_0x00201800,
        block_0x00201808,
        block_0x0020181c,
        block_0x00201844,
        block_0x0020184c,
        block_0x0020185c,
        block_0x00201868,
        block_0x00201874,
        block_0x00201894,
        block_0x002018ac,
        block_0x00201944,
        block_0x0020195c,
        block_0x00201974,
        block_0x00201988,
        block_0x00201990,
        block_0x00201994,
        block_0x002019a8,
        block_0x002019b4,
        block_0x002019d4,
        block_0x002019f0,
        block_0x00201a2c,
        block_0x00201a48,
        block_0x00201a4c,
        block_0x00201a68,
        block_0x00201a88,
        block_0x00201aa4,
        block_0x00201ae4,
        block_0x00201aec,
        block_0x00201b04,
        block_0x00201b1c,
        block_0x00201b44,
        block_0x00201b48,
        block_0x00201b70,
        block_0x00201b84,
        block_0x00201ba4,
        block_0x00201bb4,
        block_0x00201bb8,
        block_0x00201c24,
        block_0x00201c30,
        block_0x00201c44,
        block_0x00201c54,
        block_0x00201c60,
        block_0x00201c84,
        block_0x00201c8c,
        block_0x00201c9c,
        block_0x00201cbc,
        block_0x00201ccc,
        block_0x00201cd0,
        block_0x00201d3c,
        block_0x00201d48,
        block_0x00201d58,
        block_0x00201d68,
        block_0x00201d78,
        block_0x00201d90,
        block_0x00201d98,
        block_0x00201da0,
        block_0x00201dac,
        block_0x00201dcc,
        block_0x00201ddc,
        block_0x00201e48,
        block_0x00201e68,
        block_0x00201e80,
        block_0x00201e88,
        block_0x00201ea0,
        block_0x00201ea4,
        block_0x00201ecc,
    ];
    const IDX: [u16; 755usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 4u16, 0u16, 5u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        10u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 13u16, 0u16,
        14u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 22u16, 0u16, 23u16,
        0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16,
        29u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 31u16, 32u16, 33u16, 0u16,
        34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 36u16, 37u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16, 40u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 42u16, 0u16,
        0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 46u16, 0u16, 0u16,
        47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16,
        0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 52u16,
        0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        54u16, 55u16, 0u16, 0u16, 56u16, 0u16, 57u16, 58u16, 0u16, 59u16, 0u16, 0u16,
        60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16,
        0u16, 66u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16,
        0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16,
        73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 77u16, 0u16, 78u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 80u16, 0u16, 0u16,
        81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        95u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16,
        0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 113u16,
        0u16, 0u16, 0u16, 114u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 116u16, 0u16, 117u16, 0u16, 0u16, 0u16, 118u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 120u16, 121u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 122u16, 0u16,
        0u16, 123u16, 0u16, 0u16, 0u16, 124u16, 0u16, 0u16, 0u16, 125u16, 0u16, 0u16,
        0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 127u16, 0u16, 128u16, 0u16, 129u16,
        0u16, 0u16, 130u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 131u16, 0u16, 0u16,
        0u16, 132u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 133u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 134u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 135u16, 0u16, 136u16, 0u16, 0u16, 0u16, 0u16, 0u16, 137u16,
        138u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 139u16,
    ];
    if pc < 2102020u32 || pc > 2105036u32 {
        return None;
    }
    let word_offset = ((pc - 2102020u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00201304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2102024u32)?;
    emu.lw_no_count(10usize, 12usize, 4u32, 2102028u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2102032u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2102036u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2102040u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2102044u32);
    emu.apc_no_count(6usize, 2102044u32, 65536u32, 2102048u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102052u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2102056u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2102060u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2102064u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201344));
    } else {
        emu.pc = 2102068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201334));
    }
}
#[inline(always)]
pub fn block_0x00201334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2102072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020134c));
    } else {
        emu.pc = 2102076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020133c));
    }
}
#[inline(always)]
pub fn block_0x0020133c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102076u32, 40960u32, 2102080u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102084u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00201344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102084u32, 40960u32, 2102088u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102092u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020134c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102092u32, 40960u32, 2102096u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102100u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2102104u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2102108u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2102112u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2102176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013a0));
    } else {
        emu.pc = 2102116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201364));
    }
}
#[inline]
pub fn block_0x00201364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2102120u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2102124u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2102128u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2102132u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102136u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967024u32, 2102140u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2102144u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2102148u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2102152u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2102156u32);
    emu.apc_no_count(1usize, 2102156u32, 65536u32, 2102160u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2102168u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2102172u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102176u32;
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
pub fn block_0x002013a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102180u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967044u32, 2102184u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2102188u32);
    emu.apc_no_count(6usize, 2102188u32, 65536u32, 2102192u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102196u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2102200u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2102204u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2102208u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013d4));
    } else {
        emu.pc = 2102212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013c4));
    }
}
#[inline(always)]
pub fn block_0x002013c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2102216u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013dc));
    } else {
        emu.pc = 2102220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013cc));
    }
}
#[inline(always)]
pub fn block_0x002013cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102220u32, 40960u32, 2102224u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102228u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102228u32, 40960u32, 2102232u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102236u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102236u32, 40960u32, 2102240u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102244u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002013e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2102248u32)?;
    emu.apc_no_count(6usize, 2102248u32, 40960u32, 2102252u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102256u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002013f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2102260u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2102264u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2102268u32);
    emu.lw_no_count(15usize, 10usize, 0u32, 2102272u32)?;
    emu.adi_no_count(10usize, 15usize, 4u32, 2102276u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2102280u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102284u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967244u32, 2102288u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2102292u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2102296u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102300u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967260u32, 2102304u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2102308u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967269u32, 2102312u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2102316u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294967228u32, 2102320u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2102324u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294967280u32, 2102328u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2102332u32);
    emu.adi_no_count(14usize, 0usize, 11u32, 2102336u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2102340u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2102344u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2102348u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2102352u32);
    emu.apc_no_count(1usize, 2102352u32, 65536u32, 2102356u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2102364u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2102368u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102372u32;
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
pub fn block_0x00201464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2102376u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2102380u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2102384u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2102388u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2102392u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2102396u32);
    emu.apc_no_count(6usize, 2102396u32, 28672u32, 2102400u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102404u32;
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
#[inline(always)]
pub fn block_0x00201484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2102408u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2102412u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014a0));
    } else {
        emu.pc = 2102416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201490));
    }
}
#[inline(always)]
pub fn block_0x00201490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2102420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014a8));
    } else {
        emu.pc = 2102424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201498));
    }
}
#[inline(always)]
pub fn block_0x00201498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102424u32, 40960u32, 2102428u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102432u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102432u32, 40960u32, 2102436u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102440u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2102440u32, 40960u32, 2102444u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2102448u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002014b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2102452u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2102456u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2102460u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2102464u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2102468u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2102472u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2102476u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2102480u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2102484u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2102488u32);
    emu.lw_no_count(11usize, 13usize, 4u32, 2102492u32)?;
    emu.adi_no_count(18usize, 10usize, 0u32, 2102496u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2102576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201530));
    } else {
        emu.pc = 2102500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014e4));
    }
}
#[inline(always)]
pub fn block_0x002014e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 13usize, 8u32, 2102504u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2102576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201530));
    } else {
        emu.pc = 2102508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014ec));
    }
}
#[inline(always)]
pub fn block_0x002014ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 13usize, 0u32, 2102512u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102516u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967008u32, 2102520u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2102524u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2102528u32);
    emu.apc_no_count(1usize, 2102528u32, 8192u32, 2102532u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201554));
    } else {
        emu.pc = 2102540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020150c));
    }
}
#[inline(always)]
pub fn block_0x0020150c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 0u32, 2102544u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2102548u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2102552u32);
    emu.apc_no_count(1usize, 2102552u32, 8192u32, 2102556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2102564u32);
    emu.sltiu_no_count(11usize, 21usize, 1u32, 2102568u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2102620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020155c));
    } else {
        emu.pc = 2102572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020152c));
    }
}
#[inline(always)]
pub fn block_0x0020152c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201560));
}
#[inline(always)]
pub fn block_0x00201530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2102672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201590));
    } else {
        emu.pc = 2102580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201534));
    }
}
#[inline(always)]
pub fn block_0x00201534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2102580u32, 4096u32, 2102584u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020153c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102592u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967008u32, 2102596u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2102600u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2102604u32);
    emu.apc_no_count(1usize, 2102604u32, 8192u32, 2102608u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(11usize, 10usize, 1u32, 2102616u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201560));
    } else {
        emu.pc = 2102620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020155c));
    }
}
#[inline(always)]
pub fn block_0x0020155c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2102624u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201560));
}
#[inline]
pub fn block_0x00201560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 18usize, 0u32, 2102628u32)?;
    emu.sw_no_count(9usize, 18usize, 4u32, 2102632u32)?;
    emu.sw_no_count(8usize, 18usize, 8u32, 2102636u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2102640u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2102644u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2102648u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2102652u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2102656u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2102660u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2102664u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2102668u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102672u32;
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
pub fn block_0x00201590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2102676u32);
    emu.sltiu_no_count(11usize, 9usize, 1u32, 2102680u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2102620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020155c));
    } else {
        emu.pc = 2102684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020159c));
    }
}
#[inline(always)]
pub fn block_0x0020159c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201560));
}
#[inline]
pub fn block_0x002015a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2102692u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2102696u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2102700u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2102704u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2102708u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2102712u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2102716u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2102720u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2102724u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2102728u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2102736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015d0));
    } else {
        emu.pc = 2102732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015cc));
    }
}
#[inline(always)]
pub fn block_0x002015cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2102736u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102736u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015d0));
}
#[inline(always)]
pub fn block_0x002015d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 20u32, 2102740u32);
    emu.mulhu_no_count(12usize, 18usize, 11usize, 2102744u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2102748u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2102780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015fc));
    } else {
        emu.pc = 2102752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015e0));
    }
}
#[inline(always)]
pub fn block_0x002015e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 18usize, 11usize, 2102756u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102760u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 11usize, 4294967292u32, 2102764u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2102892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020166c));
    } else {
        emu.pc = 2102768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015f0));
    }
}
#[inline(always)]
pub fn block_0x002015f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201608));
    } else {
        emu.pc = 2102772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015f4));
    }
}
#[inline(always)]
pub fn block_0x002015f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2102776u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2102780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102816u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201620));
}
#[inline(always)]
pub fn block_0x002015fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2102784u32);
    emu.apc_no_count(1usize, 2102784u32, 40960u32, 2102788u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2102796u32)?;
    emu.adi_no_count(11usize, 0usize, 20u32, 2102800u32);
    emu.mul_no_count(11usize, 13usize, 11usize, 2102804u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2102808u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2102812u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2102816u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2102816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201620));
}
#[inline(always)]
pub fn block_0x00201620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2102820u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2102824u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2102828u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2102832u32);
    emu.apc_no_count(1usize, 2102832u32, 0u32, 2102836u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2102844u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201664));
    } else {
        emu.pc = 2102848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201640));
    }
}
#[inline]
pub fn block_0x00201640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2102852u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2102856u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2102860u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2102864u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2102868u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2102872u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2102876u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2102880u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102884u32;
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
pub fn block_0x00201664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2102888u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2102892u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2102892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020166c));
}
#[inline(always)]
pub fn block_0x0020166c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2102896u32);
    emu.apc_no_count(1usize, 2102896u32, 40960u32, 2102900u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2102908u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2102912u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2102916u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2102920u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2102924u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2102928u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2102932u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2102936u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2102940u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2102944u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2102952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016a8));
    } else {
        emu.pc = 2102948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016a4));
    }
}
#[inline(always)]
pub fn block_0x002016a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2102952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002016a8));
}
#[inline(always)]
pub fn block_0x002016a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 13usize, 26u32, 2102956u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2102960u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2102984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016c8));
    } else {
        emu.pc = 2102964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016b4));
    }
}
#[inline(always)]
pub fn block_0x002016b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 18usize, 5u32, 2102968u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2103092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201734));
    } else {
        emu.pc = 2102972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016bc));
    }
}
#[inline(always)]
pub fn block_0x002016bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016d4));
    } else {
        emu.pc = 2102976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016c0));
    }
}
#[inline(always)]
pub fn block_0x002016c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2102980u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2102984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002016e8));
}
#[inline(always)]
pub fn block_0x002016c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2102988u32);
    emu.apc_no_count(1usize, 2102988u32, 40960u32, 2102992u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102996u32;
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
#[inline(always)]
pub fn block_0x002016d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2103000u32)?;
    emu.sli_no_count(13usize, 13usize, 5u32, 2103004u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2103008u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2103012u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2103016u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2103016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002016e8));
}
#[inline(always)]
pub fn block_0x002016e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2103020u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2103024u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2103028u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2103032u32);
    emu.apc_no_count(1usize, 2103032u32, 0u32, 2103036u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2103044u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2103084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020172c));
    } else {
        emu.pc = 2103048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201708));
    }
}
#[inline]
pub fn block_0x00201708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2103052u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2103056u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2103060u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2103064u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2103068u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2103072u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2103076u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2103080u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103084u32;
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
pub fn block_0x0020172c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2103088u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2103092u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2103092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201734));
}
#[inline(always)]
pub fn block_0x00201734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2103096u32);
    emu.apc_no_count(1usize, 2103096u32, 40960u32, 2103100u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2103104u32, 4294963200u32, 2103108u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2103112u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2103116u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2103120u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2103124u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2103128u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2103132u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2103136u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2103140u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2103144u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2103444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201894));
    } else {
        emu.pc = 2103148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020176c));
    }
}
#[inline(always)]
pub fn block_0x0020176c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2103152u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2103156u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2103184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201790));
    } else {
        emu.pc = 2103160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201778));
    }
}
#[inline(always)]
pub fn block_0x00201778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2103164u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103168u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2103172u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2103176u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2103180u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2103184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020185c));
}
#[inline]
pub fn block_0x00201790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2103188u32)?;
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2103192u32);
    emu.adi_no_count(6usize, 10usize, 4u32, 2103196u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2103200u32);
    emu.lbu_no_count(5usize, 10usize, 1u32, 2103204u32);
    emu.lbu_no_count(16usize, 10usize, 2u32, 2103208u32);
    emu.lbu_no_count(17usize, 10usize, 3u32, 2103212u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2103216u32);
    emu.sw_no_count(6usize, 11usize, 0u32, 2103220u32)?;
    emu.sw_no_count(15usize, 11usize, 4u32, 2103224u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2103620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201944));
    } else {
        emu.pc = 2103228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017bc));
    }
}
#[inline(always)]
pub fn block_0x002017bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 4u32, 2103232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2103256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017d8));
    } else {
        emu.pc = 2103236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017c4));
    }
}
#[inline(always)]
pub fn block_0x002017c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103240u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2103244u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2103248u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2103252u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2103256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020185c));
}
#[inline]
pub fn block_0x002017d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 10usize, 8u32, 2103260u32);
    emu.adi_no_count(6usize, 12usize, 4294967288u32, 2103264u32);
    emu.lbu_no_count(15usize, 10usize, 4u32, 2103268u32);
    emu.lbu_no_count(29usize, 10usize, 5u32, 2103272u32);
    emu.lbu_no_count(7usize, 10usize, 6u32, 2103276u32);
    emu.lbu_no_count(28usize, 10usize, 7u32, 2103280u32);
    emu.adi_no_count(31usize, 0usize, 2u32, 2103284u32);
    emu.sw_no_count(30usize, 11usize, 0u32, 2103288u32)?;
    emu.sw_no_count(6usize, 11usize, 4u32, 2103292u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2103644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020195c));
    } else {
        emu.pc = 2103296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201800));
    }
}
#[inline(always)]
pub fn block_0x00201800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 0usize, 4u32, 2103300u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2103324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020181c));
    } else {
        emu.pc = 2103304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201808));
    }
}
#[inline(always)]
pub fn block_0x00201808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103308u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2103312u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2103316u32)?;
    emu.sw_no_count(30usize, 2usize, 4u32, 2103320u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2103324u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020185c));
}
#[inline]
pub fn block_0x0020181c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 12u32, 2103328u32);
    emu.adi_no_count(18usize, 12usize, 4294967284u32, 2103332u32);
    emu.lbu_no_count(6usize, 10usize, 8u32, 2103336u32);
    emu.lbu_no_count(9usize, 10usize, 9u32, 2103340u32);
    emu.lbu_no_count(30usize, 10usize, 10u32, 2103344u32);
    emu.lbu_no_count(31usize, 10usize, 11u32, 2103348u32);
    emu.adi_no_count(20usize, 0usize, 3u32, 2103352u32);
    emu.sw_no_count(19usize, 11usize, 0u32, 2103356u32)?;
    emu.sw_no_count(18usize, 11usize, 4u32, 2103360u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2103668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201974));
    } else {
        emu.pc = 2103364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201844));
    }
}
#[inline(always)]
pub fn block_0x00201844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 4u32, 2103368u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2103468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018ac));
    } else {
        emu.pc = 2103372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020184c));
    }
}
#[inline(always)]
pub fn block_0x0020184c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103376u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2103380u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2103384u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2103388u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2103388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020185c));
}
#[inline(always)]
pub fn block_0x0020185c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2103392u32);
    emu.apc_no_count(1usize, 2103392u32, 8192u32, 2103396u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103400u32;
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
pub fn block_0x00201868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2103404u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2103408u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2103412u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2103412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201874));
}
#[inline(always)]
pub fn block_0x00201874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2103416u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2103420u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2103424u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2103428u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2103432u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2103436u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2103440u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103444u32;
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
pub fn block_0x00201894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103448u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 96u32, 2103452u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 104u32, 2103460u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2103464u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2103468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201988));
}
#[inline(never)]
pub fn block_0x002018ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 8u32, 2103472u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2103476u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2103480u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2103484u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2103488u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2103492u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2103496u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2103500u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2103504u32);
    emu.orr_no_count(13usize, 5usize, 14usize, 2103508u32);
    emu.adi_no_count(14usize, 10usize, 16u32, 2103512u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2103516u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2103520u32);
    emu.orr_no_count(15usize, 29usize, 15usize, 2103524u32);
    emu.orr_no_count(17usize, 28usize, 7usize, 2103528u32);
    emu.lbu_no_count(5usize, 10usize, 12u32, 2103532u32);
    emu.lbu_no_count(7usize, 10usize, 13u32, 2103536u32);
    emu.lbu_no_count(28usize, 10usize, 14u32, 2103540u32);
    emu.lbu_no_count(10usize, 10usize, 15u32, 2103544u32);
    emu.orr_no_count(6usize, 9usize, 6usize, 2103548u32);
    emu.orr_no_count(29usize, 31usize, 30usize, 2103552u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2103556u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2103560u32)?;
    emu.orr_no_count(11usize, 16usize, 13usize, 2103564u32);
    emu.orr_no_count(12usize, 17usize, 15usize, 2103568u32);
    emu.orr_no_count(13usize, 29usize, 6usize, 2103572u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2103576u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2103580u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2103584u32);
    emu.orr_no_count(14usize, 7usize, 5usize, 2103588u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2103592u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2103596u32);
    emu.sh_no_count(0usize, 8usize, 0u32, 2103600u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2103604u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2103608u32)?;
    emu.sw_no_count(13usize, 8usize, 12u32, 2103612u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2103616u32)?;
    emu.add_memory_rw_events(38usize);
    let return_addr = 2103620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201874));
}
#[inline(always)]
pub fn block_0x00201944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103624u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 96u32, 2103628u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103632u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 104u32, 2103636u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103640u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2103644u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201988));
}
#[inline(always)]
pub fn block_0x0020195c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103648u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 96u32, 2103652u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103656u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 104u32, 2103660u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2103664u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2103668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201988));
}
#[inline(always)]
pub fn block_0x00201974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103672u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 96u32, 2103676u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103680u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 104u32, 2103684u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2103688u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2103688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201988));
}
#[inline(always)]
pub fn block_0x00201988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2103688u32, 0u32, 2103692u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2103700u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103400u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201868));
}
#[inline(always)]
pub fn block_0x00201994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2103704u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2103708u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2103712u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2103716u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2103852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201a2c));
    } else {
        emu.pc = 2103720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019a8));
    }
}
#[inline(always)]
pub fn block_0x002019a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2103724u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2103728u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2103792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019f0));
    } else {
        emu.pc = 2103732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019b4));
    }
}
#[inline(always)]
pub fn block_0x002019b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2103736u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103740u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2103744u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2103748u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2103752u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2103756u32);
    emu.apc_no_count(1usize, 2103756u32, 8192u32, 2103760u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002019d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2103768u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2103772u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2103776u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2103780u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2103784u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2103788u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103792u32;
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
pub fn block_0x002019f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 11usize, 0u32, 2103796u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2103800u32);
    emu.lbu_no_count(14usize, 13usize, 0u32, 2103804u32);
    emu.lbu_no_count(15usize, 13usize, 1u32, 2103808u32);
    emu.adi_no_count(13usize, 13usize, 2u32, 2103812u32);
    emu.sw_no_count(13usize, 11usize, 0u32, 2103816u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2103820u32)?;
    emu.sli_no_count(15usize, 15usize, 8u32, 2103824u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2103828u32);
    emu.sh_no_count(10usize, 8usize, 0u32, 2103832u32)?;
    emu.sh_no_count(14usize, 8usize, 2u32, 2103836u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2103840u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2103844u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2103848u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103852u32;
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
pub fn block_0x00201a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103856u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 184u32, 2103860u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103864u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 104u32, 2103868u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2103872u32);
    emu.apc_no_count(1usize, 2103872u32, 0u32, 2103876u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103880u32;
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
#[inline(always)]
pub fn block_0x00201a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2103884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002019d4));
}
#[inline(always)]
pub fn block_0x00201a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2103888u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2103892u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2103896u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2103900u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2103904u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2103908u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2103972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201aa4));
    } else {
        emu.pc = 2103912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201a68));
    }
}
#[inline(always)]
pub fn block_0x00201a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2103916u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103920u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2103924u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2103928u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2103932u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2103936u32);
    emu.apc_no_count(1usize, 2103936u32, 8192u32, 2103940u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2103948u32);
    emu.sb_no_count(11usize, 8usize, 0u32, 2103952u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2103956u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2103960u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2103964u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2103968u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103972u32;
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
pub fn block_0x00201aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2103976u32)?;
    emu.lbu_no_count(13usize, 10usize, 1u32, 2103980u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2103984u32);
    emu.lbu_no_count(15usize, 10usize, 2u32, 2103988u32);
    emu.lbu_no_count(16usize, 10usize, 3u32, 2103992u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2103996u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2104000u32);
    emu.adi_no_count(14usize, 12usize, 4294967292u32, 2104004u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2104008u32);
    emu.sli_no_count(15usize, 15usize, 16u32, 2104012u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2104016u32);
    emu.orr_no_count(12usize, 16usize, 15usize, 2104020u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2104024u32);
    emu.sw_no_count(10usize, 11usize, 0u32, 2104028u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2104032u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2104068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201b04));
    } else {
        emu.pc = 2104036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201ae4));
    }
}
#[inline(always)]
pub fn block_0x00201ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2104040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2104092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201b1c));
    } else {
        emu.pc = 2104044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201aec));
    }
}
#[inline(always)]
pub fn block_0x00201aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 0u32, 2104048u32);
    emu.sw_no_count(11usize, 8usize, 4u32, 2104052u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2104056u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2104060u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2104064u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104068u32;
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
pub fn block_0x00201b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2104072u32);
    emu.sw_no_count(11usize, 8usize, 4u32, 2104076u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2104080u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2104084u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2104088u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104092u32;
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
pub fn block_0x00201b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 2usize, 8u32, 2104096u32);
    emu.sw_no_count(12usize, 2usize, 16u32, 2104100u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2104104u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104108u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967140u32, 2104112u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104116u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967148u32, 2104120u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2104124u32);
    emu.apc_no_count(1usize, 2104124u32, 0u32, 2104128u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2104136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103944u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201a88));
}
#[inline]
pub fn block_0x00201b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2104140u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2104144u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2104148u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2104152u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2104156u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2104160u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2104164u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2104168u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2104172u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2104696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d78));
    } else {
        emu.pc = 2104176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201b70));
    }
}
#[inline(always)]
pub fn block_0x00201b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2104180u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2104184u32);
    emu.lw_no_count(10usize, 11usize, 4u32, 2104188u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2104192u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2104248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201bb8));
    } else {
        emu.pc = 2104196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201b84));
    }
}
#[inline(always)]
pub fn block_0x00201b84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2104200u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104204u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2104208u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2104212u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2104216u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2104220u32);
    emu.apc_no_count(1usize, 2104220u32, 8192u32, 2104224u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2104232u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2104236u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2104240u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2104368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c30));
    } else {
        emu.pc = 2104244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201bb4));
    }
}
#[inline(always)]
pub fn block_0x00201bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2104248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c54));
}
#[inline(never)]
pub fn block_0x00201bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2104252u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2104256u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2104260u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2104264u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2104268u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2104272u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2104276u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2104280u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2104284u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2104288u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2104292u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2104296u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2104300u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2104304u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2104308u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2104312u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2104316u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2104320u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2104324u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2104328u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2104332u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2104336u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2104340u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2104344u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2104348u32)?;
    emu.apc_no_count(1usize, 2104348u32, 8192u32, 2104352u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 11usize, 0u32, 2104360u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2104364u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2104404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c54));
    } else {
        emu.pc = 2104368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c30));
    }
}
#[inline(always)]
pub fn block_0x00201c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2104372u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2104376u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2104380u32);
    emu.apc_no_count(1usize, 2104380u32, 0u32, 2104384u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 2usize, 8u32, 2104392u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2104396u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104400u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2104452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c84));
    } else {
        emu.pc = 2104404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c54));
    }
}
#[inline(always)]
pub fn block_0x00201c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104408u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2104412u32)?;
    emu.sw_no_count(19usize, 8usize, 4u32, 2104416u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2104416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c60));
}
#[inline]
pub fn block_0x00201c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2104420u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2104424u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2104428u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2104432u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2104436u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2104440u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2104444u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2104448u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104452u32;
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
pub fn block_0x00201c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2104456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2104936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e68));
    } else {
        emu.pc = 2104460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c8c));
    }
}
#[inline(always)]
pub fn block_0x00201c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2104464u32)?;
    emu.lw_no_count(21usize, 2usize, 16u32, 2104468u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2104472u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2104528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201cd0));
    } else {
        emu.pc = 2104476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c9c));
    }
}
#[inline(always)]
pub fn block_0x00201c9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2104480u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104484u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2104488u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2104492u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2104496u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2104500u32);
    emu.apc_no_count(1usize, 2104500u32, 8192u32, 2104504u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2104512u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2104516u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2104520u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2104648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d48));
    } else {
        emu.pc = 2104524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201ccc));
    }
}
#[inline(always)]
pub fn block_0x00201ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2104528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201d68));
}
#[inline(never)]
pub fn block_0x00201cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2104532u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2104536u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2104540u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2104544u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2104548u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2104552u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2104556u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2104560u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2104564u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2104568u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2104572u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2104576u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2104580u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2104584u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2104588u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2104592u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2104596u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2104600u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2104604u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2104608u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2104612u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2104616u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2104620u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2104624u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2104628u32)?;
    emu.apc_no_count(1usize, 2104628u32, 8192u32, 2104632u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2104640u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2104644u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2104680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d68));
    } else {
        emu.pc = 2104648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d48));
    }
}
#[inline(always)]
pub fn block_0x00201d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2104652u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2104656u32);
    emu.apc_no_count(1usize, 2104656u32, 0u32, 2104660u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2104668u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2104672u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104676u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2104728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d98));
    } else {
        emu.pc = 2104680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d68));
    }
}
#[inline(always)]
pub fn block_0x00201d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104684u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2104688u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2104692u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2104696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c60));
}
#[inline(always)]
pub fn block_0x00201d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104700u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 28u32, 2104704u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2104708u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2104712u32);
    emu.apc_no_count(1usize, 2104712u32, 0u32, 2104716u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201d90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2104724u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2104728u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c54));
}
#[inline(always)]
pub fn block_0x00201d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2104732u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2104968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e88));
    } else {
        emu.pc = 2104736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201da0));
    }
}
#[inline(always)]
pub fn block_0x00201da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 4u32, 2104740u32)?;
    emu.adi_no_count(13usize, 0usize, 7u32, 2104744u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2104796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201ddc));
    } else {
        emu.pc = 2104748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201dac));
    }
}
#[inline(always)]
pub fn block_0x00201dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2104752u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104756u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2104760u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2104764u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2104768u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2104772u32);
    emu.apc_no_count(1usize, 2104772u32, 8192u32, 2104776u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104784u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2104788u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2104792u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2104796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c60));
}
#[inline(never)]
pub fn block_0x00201ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 9usize, 0u32, 2104800u32)?;
    emu.lbu_no_count(13usize, 14usize, 5u32, 2104804u32);
    emu.lbu_no_count(15usize, 14usize, 6u32, 2104808u32);
    emu.lbu_no_count(16usize, 14usize, 7u32, 2104812u32);
    emu.lbu_no_count(17usize, 14usize, 4u32, 2104816u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2104820u32);
    emu.sli_no_count(15usize, 15usize, 16u32, 2104824u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2104828u32);
    emu.orr_no_count(17usize, 13usize, 17usize, 2104832u32);
    emu.lbu_no_count(13usize, 14usize, 0u32, 2104836u32);
    emu.lbu_no_count(5usize, 14usize, 1u32, 2104840u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2104844u32);
    emu.lbu_no_count(16usize, 14usize, 2u32, 2104848u32);
    emu.lbu_no_count(6usize, 14usize, 3u32, 2104852u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2104856u32);
    emu.orr_no_count(5usize, 5usize, 13usize, 2104860u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2104864u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2104868u32);
    emu.orr_no_count(16usize, 6usize, 16usize, 2104872u32);
    emu.lw_no_count(13usize, 2usize, 16u32, 2104876u32)?;
    emu.adi_no_count(6usize, 11usize, 4294967288u32, 2104880u32);
    emu.adi_no_count(7usize, 14usize, 8u32, 2104884u32);
    emu.orr_no_count(14usize, 15usize, 17usize, 2104888u32);
    emu.orr_no_count(11usize, 16usize, 5usize, 2104892u32);
    emu.sw_no_count(7usize, 9usize, 0u32, 2104896u32)?;
    emu.sw_no_count(6usize, 9usize, 4u32, 2104900u32)?;
    emu.add_memory_rw_events(26usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2104996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201ea4));
    } else {
        emu.pc = 2104904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e48));
    }
}
#[inline(always)]
pub fn block_0x00201e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 8usize, 0u32, 2104908u32)?;
    emu.sw_no_count(19usize, 8usize, 4u32, 2104912u32)?;
    emu.sw_no_count(21usize, 8usize, 8u32, 2104916u32)?;
    emu.sw_no_count(10usize, 8usize, 12u32, 2104920u32)?;
    emu.sw_no_count(12usize, 8usize, 16u32, 2104924u32)?;
    emu.sw_no_count(13usize, 8usize, 20u32, 2104928u32)?;
    emu.sw_no_count(11usize, 8usize, 24u32, 2104932u32)?;
    emu.add_memory_rw_events(8usize);
    let return_addr = 2104936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c60));
}
#[inline(always)]
pub fn block_0x00201e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104940u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 28u32, 2104944u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2104948u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2104952u32);
    emu.apc_no_count(1usize, 2104952u32, 0u32, 2104956u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201e80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2104964u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2104968u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201d68));
}
#[inline(always)]
pub fn block_0x00201e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104972u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 28u32, 2104976u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2104980u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2104984u32);
    emu.apc_no_count(1usize, 2104984u32, 0u32, 2104988u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(168u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2104996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201dcc));
}
#[inline]
pub fn block_0x00201ea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2105000u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2105004u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2105008u32)?;
    emu.sw_no_count(14usize, 2usize, 20u32, 2105012u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105016u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 12u32, 2105020u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2105024u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2105028u32);
    emu.apc_no_count(1usize, 2105028u32, 0u32, 2105032u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105036u32;
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
pub fn block_0x00201ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105040u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201dcc));
}
