pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2099200u32;
pub const PC_MAX: u32 = 2102488u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 147usize] = [
        block_0x00200800,
        block_0x0020081c,
        block_0x00200820,
        block_0x0020083c,
        block_0x00200858,
        block_0x0020089c,
        block_0x002008b4,
        block_0x002008cc,
        block_0x002008dc,
        block_0x002008f4,
        block_0x0020090c,
        block_0x00200914,
        block_0x00200950,
        block_0x00200958,
        block_0x00200960,
        block_0x00200998,
        block_0x002009c4,
        block_0x002009c8,
        block_0x002009fc,
        block_0x00200a24,
        block_0x00200a30,
        block_0x00200a34,
        block_0x00200a4c,
        block_0x00200a54,
        block_0x00200a5c,
        block_0x00200a60,
        block_0x00200a6c,
        block_0x00200a74,
        block_0x00200a78,
        block_0x00200a84,
        block_0x00200a90,
        block_0x00200a98,
        block_0x00200a9c,
        block_0x00200aa0,
        block_0x00200aa8,
        block_0x00200aac,
        block_0x00200ab8,
        block_0x00200abc,
        block_0x00200acc,
        block_0x00200afc,
        block_0x00200b10,
        block_0x00200b30,
        block_0x00200b40,
        block_0x00200b70,
        block_0x00200b7c,
        block_0x00200b90,
        block_0x00200b9c,
        block_0x00200bac,
        block_0x00200bb4,
        block_0x00200bbc,
        block_0x00200bc4,
        block_0x00200bcc,
        block_0x00200bdc,
        block_0x00200be4,
        block_0x00200bec,
        block_0x00200bf4,
        block_0x00200bfc,
        block_0x00200c64,
        block_0x00200c70,
        block_0x00200c90,
        block_0x00200cec,
        block_0x00200d60,
        block_0x00200d8c,
        block_0x00200d98,
        block_0x00200da0,
        block_0x00200dac,
        block_0x00200dc4,
        block_0x00200e50,
        block_0x00200e64,
        block_0x00200e70,
        block_0x00200e84,
        block_0x00200e98,
        block_0x00200ee0,
        block_0x00200ef4,
        block_0x00200efc,
        block_0x00201038,
        block_0x00201044,
        block_0x0020104c,
        block_0x00201054,
        block_0x0020105c,
        block_0x00201064,
        block_0x002010a8,
        block_0x002010cc,
        block_0x002010dc,
        block_0x002010f0,
        block_0x002010f4,
        block_0x0020110c,
        block_0x00201114,
        block_0x00201130,
        block_0x00201134,
        block_0x00201144,
        block_0x00201148,
        block_0x00201160,
        block_0x00201174,
        block_0x00201198,
        block_0x002011b4,
        block_0x002011d0,
        block_0x002011d8,
        block_0x002011e0,
        block_0x002011e4,
        block_0x002011e8,
        block_0x002011f4,
        block_0x002011fc,
        block_0x00201210,
        block_0x0020121c,
        block_0x0020122c,
        block_0x00201230,
        block_0x00201238,
        block_0x00201254,
        block_0x00201268,
        block_0x00201270,
        block_0x00201278,
        block_0x00201284,
        block_0x00201288,
        block_0x0020128c,
        block_0x00201290,
        block_0x002012ac,
        block_0x002012b0,
        block_0x002012b8,
        block_0x002012c0,
        block_0x002012d8,
        block_0x002012dc,
        block_0x002012ec,
        block_0x002012f0,
        block_0x00201304,
        block_0x00201324,
        block_0x00201340,
        block_0x0020135c,
        block_0x00201364,
        block_0x0020136c,
        block_0x00201370,
        block_0x00201374,
        block_0x00201380,
        block_0x00201388,
        block_0x0020139c,
        block_0x002013a8,
        block_0x002013b8,
        block_0x002013bc,
        block_0x00201404,
        block_0x00201408,
        block_0x00201418,
        block_0x0020141c,
        block_0x00201458,
        block_0x0020146c,
        block_0x00201480,
        block_0x002014ac,
        block_0x002014d8,
    ];
    const IDX: [u16; 823usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16,
        0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 18u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16, 22u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 25u16, 26u16, 0u16, 0u16,
        27u16, 0u16, 28u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16, 31u16, 0u16, 32u16,
        33u16, 34u16, 0u16, 35u16, 36u16, 0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16,
        0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16,
        0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 44u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16,
        47u16, 0u16, 0u16, 0u16, 48u16, 0u16, 49u16, 0u16, 50u16, 0u16, 51u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 55u16, 0u16, 56u16, 0u16,
        57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 64u16,
        0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16,
        0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16,
        0u16, 77u16, 0u16, 78u16, 0u16, 79u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16,
        0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        87u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16,
        0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 98u16,
        0u16, 99u16, 100u16, 101u16, 0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16,
        0u16, 104u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 106u16, 107u16, 0u16, 108u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16,
        111u16, 0u16, 112u16, 0u16, 0u16, 113u16, 114u16, 115u16, 116u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 117u16, 118u16, 0u16, 119u16, 0u16, 120u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 121u16, 122u16, 0u16, 0u16, 0u16, 123u16, 124u16, 0u16, 0u16,
        0u16, 0u16, 125u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 127u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 128u16, 0u16,
        129u16, 0u16, 130u16, 131u16, 132u16, 0u16, 0u16, 133u16, 0u16, 134u16, 0u16,
        0u16, 0u16, 0u16, 135u16, 0u16, 0u16, 136u16, 0u16, 0u16, 0u16, 137u16, 138u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 139u16, 140u16, 0u16, 0u16, 0u16, 141u16, 142u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        143u16, 0u16, 0u16, 0u16, 0u16, 144u16, 0u16, 0u16, 0u16, 0u16, 145u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 146u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 147u16,
    ];
    if pc < 2099200u32 || pc > 2102488u32 {
        return None;
    }
    let word_offset = ((pc - 2099200u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00200800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2099204u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2099208u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2099212u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2099216u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2099220u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2099224u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2099288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200858));
    } else {
        emu.pc = 2099228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020081c));
    }
}
#[inline(always)]
pub fn block_0x0020081c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2099232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2099232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200820));
}
#[inline(always)]
pub fn block_0x00200820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099236u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2099240u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2099244u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2099248u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2099252u32);
    emu.apc_no_count(1usize, 2099252u32, 12288u32, 2099256u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099260u32;
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
pub fn block_0x0020083c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 5u32, 2099264u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2099268u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2099272u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2099276u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099280u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099284u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099288u32;
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
pub fn block_0x00200858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2099292u32)?;
    emu.lbu_no_count(14usize, 10usize, 0u32, 2099296u32);
    emu.lbu_no_count(15usize, 10usize, 1u32, 2099300u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2099304u32);
    emu.lbu_no_count(16usize, 10usize, 2u32, 2099308u32);
    emu.lbu_no_count(17usize, 10usize, 3u32, 2099312u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2099316u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2099320u32);
    emu.adi_no_count(15usize, 0usize, 4u32, 2099324u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2099328u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2099332u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2099336u32);
    emu.adi_no_count(17usize, 10usize, 4u32, 2099340u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2099344u32);
    emu.sw_no_count(17usize, 11usize, 0u32, 2099348u32)?;
    emu.sw_no_count(13usize, 11usize, 4u32, 2099352u32)?;
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2099608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200998));
    } else {
        emu.pc = 2099356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020089c));
    }
}
#[inline(always)]
pub fn block_0x0020089c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 14usize, 2u32, 2099360u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2099364u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1432u32, 2099368u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2099372u32);
    emu.lw_no_count(14usize, 14usize, 0u32, 2099376u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2099380u32;
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
pub fn block_0x002008b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099384u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1512u32, 2099388u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2099392u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2099396u32);
    emu.apc_no_count(1usize, 2099396u32, 4096u32, 2099400u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099404u32;
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
pub fn block_0x002008cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2099408u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099412u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099416u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099420u32;
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
pub fn block_0x002008dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2099424u32);
    emu.sh_no_count(10usize, 8usize, 0u32, 2099428u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2099432u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099436u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099440u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099444u32;
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
pub fn block_0x002008f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2099448u32);
    emu.sh_no_count(10usize, 8usize, 0u32, 2099452u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2099456u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099460u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099464u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099468u32;
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
pub fn block_0x0020090c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 1u32, 2099472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2099544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200958));
    } else {
        emu.pc = 2099476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200914));
    }
}
#[inline]
pub fn block_0x00200914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 6u32, 2099480u32);
    emu.adi_no_count(12usize, 12usize, 4294967290u32, 2099484u32);
    emu.lbu_no_count(14usize, 10usize, 4u32, 2099488u32);
    emu.lbu_no_count(10usize, 10usize, 5u32, 2099492u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2099496u32);
    emu.sw_no_count(13usize, 11usize, 0u32, 2099500u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2099504u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2099508u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2099512u32);
    emu.sh_no_count(15usize, 8usize, 0u32, 2099516u32)?;
    emu.sh_no_count(10usize, 8usize, 2u32, 2099520u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2099524u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099528u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099532u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099536u32;
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
pub fn block_0x00200950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 1u32, 2099540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2099552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200960));
    } else {
        emu.pc = 2099544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200958));
    }
}
#[inline(always)]
pub fn block_0x00200958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2099548u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2099552u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200820));
}
#[inline]
pub fn block_0x00200960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 6u32, 2099556u32);
    emu.lbu_no_count(15usize, 10usize, 4u32, 2099560u32);
    emu.lbu_no_count(10usize, 10usize, 5u32, 2099564u32);
    emu.adi_no_count(12usize, 12usize, 4294967290u32, 2099568u32);
    emu.sw_no_count(13usize, 11usize, 0u32, 2099572u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2099576u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2099580u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2099584u32);
    emu.sh_no_count(14usize, 8usize, 0u32, 2099588u32)?;
    emu.sh_no_count(10usize, 8usize, 2u32, 2099592u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2099596u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099600u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099604u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099608u32;
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
pub fn block_0x00200998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2099612u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2099616u32);
    emu.sw_no_count(14usize, 2usize, 16u32, 2099620u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2099624u32)?;
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099628u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1568u32, 2099632u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099636u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1576u32, 2099640u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2099644u32);
    emu.apc_no_count(1usize, 2099644u32, 4096u32, 2099648u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002009c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020083c));
}
#[inline]
pub fn block_0x002009c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2099660u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2099664u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2099668u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2099672u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2099676u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2099680u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2099684u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2099688u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2099692u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2099696u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2099700u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2099704u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aac));
    } else {
        emu.pc = 2099708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002009fc));
    }
}
#[inline]
pub fn block_0x002009fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 13usize, 0u32, 2099712u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2099716u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2099720u32);
    emu.adi_no_count(21usize, 0usize, 4u32, 2099724u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2099728u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 1632u32, 2099732u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2099736u32);
    emu.adi_no_count(23usize, 0usize, 35u32, 2099740u32);
    emu.adi_no_count(24usize, 0usize, 2u32, 2099744u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2099748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a34));
}
#[inline(always)]
pub fn block_0x00200a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2099752u32)?;
    emu.lbu_no_count(11usize, 11usize, 8u32, 2099756u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2099900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200abc));
    } else {
        emu.pc = 2099760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a30));
    }
}
#[inline(always)]
pub fn block_0x00200a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aac));
    } else {
        emu.pc = 2099764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a34));
    }
}
#[inline(always)]
pub fn block_0x00200a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2099768u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2099772u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2099776u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2099780u32);
    emu.apc_no_count(1usize, 2099780u32, 12288u32, 2099784u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099788u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 0u32, 2099792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2099820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a6c));
    } else {
        emu.pc = 2099796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a54));
    }
}
#[inline(always)]
pub fn block_0x00200a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2099800u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2099896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ab8));
    } else {
        emu.pc = 2099804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a5c));
    }
}
#[inline(always)]
pub fn block_0x00200a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200afc));
    } else {
        emu.pc = 2099808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a60));
    }
}
#[inline(always)]
pub fn block_0x00200a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(9usize, 9usize, 10usize, 2099812u32);
    emu.adr_no_count(18usize, 18usize, 10usize, 2099816u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2099820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a30));
}
#[inline(always)]
pub fn block_0x00200a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2099824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2099868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a9c));
    } else {
        emu.pc = 2099828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a74));
    }
}
#[inline(always)]
pub fn block_0x00200a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a24));
    } else {
        emu.pc = 2099832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a78));
    }
}
#[inline(always)]
pub fn block_0x00200a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2099836u32)?;
    emu.lbu_no_count(12usize, 11usize, 8u32, 2099840u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2099900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200abc));
    } else {
        emu.pc = 2099844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a84));
    }
}
#[inline(always)]
pub fn block_0x00200a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 4u32, 2099848u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2099852u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2099760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a30));
    } else {
        emu.pc = 2099856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a90));
    }
}
#[inline(always)]
pub fn block_0x00200a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2099860u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2099864u32;
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
pub fn block_0x00200a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200a30));
}
#[inline(always)]
pub fn block_0x00200a9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200abc));
    } else {
        emu.pc = 2099872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aa0));
    }
}
#[inline(always)]
pub fn block_0x00200aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 1u32, 2099876u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2099760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a30));
    } else {
        emu.pc = 2099880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aa8));
    }
}
#[inline(always)]
pub fn block_0x00200aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099900u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200abc));
}
#[inline(always)]
pub fn block_0x00200aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2099888u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2099892u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2099896u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099916u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200acc));
}
#[inline(always)]
pub fn block_0x00200ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2099900u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2099900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200abc));
}
#[inline(always)]
pub fn block_0x00200abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2099904u32)?;
    emu.lw_no_count(10usize, 10usize, 4u32, 2099908u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2099912u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2099916u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2099916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200acc));
}
#[inline]
pub fn block_0x00200acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2099920u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2099924u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2099928u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2099932u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2099936u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2099940u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2099944u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2099948u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2099952u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2099956u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2099960u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099964u32;
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
pub fn block_0x00200afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099968u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 368u32, 2099972u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2099976u32);
    emu.apc_no_count(1usize, 2099976u32, 77824u32, 2099980u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2099988u32)?;
    emu.lw_no_count(10usize, 12usize, 4u32, 2099992u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2099996u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2100000u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2100004u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2100008u32);
    emu.apc_no_count(6usize, 2100008u32, 69632u32, 2100012u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100016u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2100020u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2100024u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2100028u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2100092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b7c));
    } else {
        emu.pc = 2100032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b40));
    }
}
#[inline]
pub fn block_0x00200b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2100036u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2100040u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2100044u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2100048u32)?;
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100052u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1468u32, 2100056u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2100060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 448u32, 2100064u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2100068u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2100072u32);
    emu.apc_no_count(1usize, 2100072u32, 65536u32, 2100076u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2100084u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2100088u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100092u32;
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
pub fn block_0x00200b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100096u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1488u32, 2100100u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2100104u32);
    emu.apc_no_count(6usize, 2100104u32, 65536u32, 2100108u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100112u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2100116u32)?;
    emu.apc_no_count(6usize, 2100116u32, 40960u32, 2100120u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100124u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2100128u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2100132u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2100136u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2100156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bbc));
    } else {
        emu.pc = 2100140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bac));
    }
}
#[inline(always)]
pub fn block_0x00200bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2100144u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2100164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bc4));
    } else {
        emu.pc = 2100148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bb4));
    }
}
#[inline(always)]
pub fn block_0x00200bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2100148u32, 45056u32, 2100152u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100156u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00200bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2100156u32, 45056u32, 2100160u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100164u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2100164u32, 45056u32, 2100168u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100172u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2100176u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2100180u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2100184u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2100204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bec));
    } else {
        emu.pc = 2100188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bdc));
    }
}
#[inline(always)]
pub fn block_0x00200bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2100192u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2100212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bf4));
    } else {
        emu.pc = 2100196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200be4));
    }
}
#[inline(always)]
pub fn block_0x00200be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2100196u32, 45056u32, 2100200u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100204u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2100204u32, 45056u32, 2100208u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100212u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2100212u32, 45056u32, 2100216u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100220u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(68u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00200bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2100224u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2100228u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2100232u32);
    emu.lw_no_count(15usize, 10usize, 0u32, 2100236u32)?;
    emu.adi_no_count(10usize, 15usize, 4u32, 2100240u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2100244u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100248u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 400u32, 2100252u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2100256u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2100260u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100264u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 416u32, 2100268u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2100272u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 425u32, 2100276u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2100280u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 384u32, 2100284u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2100288u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 436u32, 2100292u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2100296u32);
    emu.adi_no_count(14usize, 0usize, 11u32, 2100300u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2100304u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2100308u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2100312u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2100316u32);
    emu.apc_no_count(1usize, 2100316u32, 65536u32, 2100320u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2100328u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2100332u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100336u32;
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
pub fn block_0x00200c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2100340u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2100344u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2100348u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2100352u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2100356u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2100360u32);
    emu.apc_no_count(6usize, 2100360u32, 28672u32, 2100364u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2100368u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00200c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966944u32, 2100372u32);
    emu.sw_no_count(1usize, 2usize, 348u32, 2100376u32)?;
    emu.sw_no_count(8usize, 2usize, 344u32, 2100380u32)?;
    emu.sw_no_count(9usize, 2usize, 340u32, 2100384u32)?;
    emu.sw_no_count(18usize, 2usize, 336u32, 2100388u32)?;
    emu.sw_no_count(19usize, 2usize, 332u32, 2100392u32)?;
    emu.sw_no_count(20usize, 2usize, 328u32, 2100396u32)?;
    emu.sw_no_count(21usize, 2usize, 324u32, 2100400u32)?;
    emu.sw_no_count(22usize, 2usize, 320u32, 2100404u32)?;
    emu.sw_no_count(23usize, 2usize, 316u32, 2100408u32)?;
    emu.sw_no_count(24usize, 2usize, 312u32, 2100412u32)?;
    emu.sw_no_count(25usize, 2usize, 308u32, 2100416u32)?;
    emu.sw_no_count(26usize, 2usize, 304u32, 2100420u32)?;
    emu.sw_no_count(27usize, 2usize, 300u32, 2100424u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2100428u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2100432u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2100436u32);
    emu.adi_no_count(18usize, 2usize, 48u32, 2100440u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2100444u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2100448u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2100452u32);
    emu.apc_no_count(1usize, 2100452u32, 8192u32, 2100456u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100460u32;
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
#[inline(never)]
pub fn block_0x00200cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100464u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100468u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100472u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2100476u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2100480u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2100484u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2100488u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2100492u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2100496u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2100500u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2100504u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2100508u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2100512u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2100516u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2100520u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100524u32;
    emu.update_insn_clock();
    emu.sw_no_count(0usize, 2usize, 40u32, 2100528u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2100532u32)?;
    emu.adi_no_count(11usize, 14usize, 639u32, 2100536u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2100540u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2100544u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2100548u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2100552u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2100556u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2100560u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2100564u32)?;
    emu.sri_no_count(12usize, 9usize, 6u32, 2100568u32);
    emu.sb_no_count(0usize, 2usize, 112u32, 2100572u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d98));
    } else {
        emu.pc = 2100576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d60));
    }
}
#[inline]
pub fn block_0x00200d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100580u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967232u32, 2100584u32);
    emu.anr_no_count(10usize, 9usize, 10usize, 2100588u32);
    emu.ani_no_count(9usize, 9usize, 63u32, 2100592u32);
    emu.adr_no_count(20usize, 19usize, 10usize, 2100596u32);
    emu.sw_no_count(12usize, 2usize, 40u32, 2100600u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2100604u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2100608u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2100612u32);
    emu.apc_no_count(1usize, 2100612u32, 28672u32, 2100616u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2100624u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2100628u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2100632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100640u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200da0));
}
#[inline(always)]
pub fn block_0x00200d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2100636u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2100640u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2100640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200da0));
}
#[inline(always)]
pub fn block_0x00200da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 9usize, 0u32, 2100644u32);
    emu.apc_no_count(1usize, 2100644u32, 8192u32, 2100648u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 2usize, 112u32, 2100656u32);
    emu.adi_no_count(10usize, 2usize, 120u32, 2100660u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2100664u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2100668u32);
    emu.apc_no_count(1usize, 2100668u32, 8192u32, 2100672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00200dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 160u32, 2100680u32);
    emu.lbu_no_count(18usize, 2usize, 224u32, 2100684u32);
    emu.lw_no_count(10usize, 2usize, 152u32, 2100688u32)?;
    emu.lw_no_count(11usize, 2usize, 156u32, 2100692u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100696u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2100700u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2100704u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2100708u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2100712u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2100716u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2100720u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2100724u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2100728u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2100732u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2100736u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2100740u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2100744u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2100748u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2100752u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2100756u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2100760u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2100764u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2100768u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2100772u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2100776u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2100780u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2100784u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2100788u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2100792u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2100796u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2100800u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2100804u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2100808u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2100812u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2100848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e70));
    } else {
        emu.pc = 2100816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e50));
    }
}
#[inline(always)]
pub fn block_0x00200e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2100820u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2100824u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2100828u32);
    emu.apc_no_count(1usize, 2100828u32, 8192u32, 2100832u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100836u32;
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
pub fn block_0x00200e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2100840u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2100844u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2100960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ee0));
    } else {
        emu.pc = 2100848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e70));
    }
}
#[inline(always)]
pub fn block_0x00200e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2100852u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2100856u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2100860u32);
    emu.apc_no_count(1usize, 2100860u32, 28672u32, 2100864u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 236u32, 2100872u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2100876u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2100880u32);
    emu.apc_no_count(1usize, 2100880u32, 8192u32, 2100884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100888u32;
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
#[inline]
pub fn block_0x00200e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2100892u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2100896u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2100900u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2100904u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2100908u32);
    emu.sb_no_count(20usize, 2usize, 296u32, 2100912u32);
    emu.sb_no_count(12usize, 2usize, 297u32, 2100916u32);
    emu.sb_no_count(11usize, 2usize, 298u32, 2100920u32);
    emu.sb_no_count(10usize, 2usize, 299u32, 2100924u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2100928u32);
    emu.sb_no_count(19usize, 2usize, 292u32, 2100932u32);
    emu.sb_no_count(10usize, 2usize, 293u32, 2100936u32);
    emu.sb_no_count(14usize, 2usize, 294u32, 2100940u32);
    emu.sb_no_count(13usize, 2usize, 295u32, 2100944u32);
    emu.adi_no_count(10usize, 2usize, 120u32, 2100948u32);
    emu.adi_no_count(11usize, 2usize, 236u32, 2100952u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2100956u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2100960u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ef4));
}
#[inline(always)]
pub fn block_0x00200ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 216u32, 2100964u32)?;
    emu.sw_no_count(20usize, 2usize, 220u32, 2100968u32)?;
    emu.adi_no_count(10usize, 2usize, 120u32, 2100972u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2100976u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2100980u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2100980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ef4));
}
#[inline(always)]
pub fn block_0x00200ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2100980u32, 28672u32, 2100984u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100988u32;
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
#[inline(never)]
pub fn block_0x00200efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(28usize, 2usize, 120u32, 2100992u32)?;
    emu.lw_no_count(6usize, 2usize, 124u32, 2100996u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2101000u32)?;
    emu.lw_no_count(14usize, 2usize, 132u32, 2101004u32)?;
    emu.lw_no_count(13usize, 2usize, 136u32, 2101008u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2101012u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2101016u32)?;
    emu.lw_no_count(10usize, 2usize, 148u32, 2101020u32)?;
    emu.sri_no_count(31usize, 28usize, 24u32, 2101024u32);
    emu.sri_no_count(9usize, 6usize, 24u32, 2101028u32);
    emu.sri_no_count(30usize, 16usize, 24u32, 2101032u32);
    emu.sri_no_count(29usize, 14usize, 24u32, 2101036u32);
    emu.sri_no_count(7usize, 13usize, 24u32, 2101040u32);
    emu.sri_no_count(5usize, 12usize, 24u32, 2101044u32);
    emu.sri_no_count(17usize, 11usize, 24u32, 2101048u32);
    emu.sri_no_count(15usize, 10usize, 24u32, 2101052u32);
    emu.sri_no_count(18usize, 28usize, 8u32, 2101056u32);
    emu.sri_no_count(19usize, 28usize, 16u32, 2101060u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2101064u32);
    emu.sri_no_count(21usize, 6usize, 16u32, 2101068u32);
    emu.sri_no_count(22usize, 16usize, 8u32, 2101072u32);
    emu.sri_no_count(23usize, 16usize, 16u32, 2101076u32);
    emu.sri_no_count(24usize, 14usize, 8u32, 2101080u32);
    emu.sri_no_count(25usize, 14usize, 16u32, 2101084u32);
    emu.sri_no_count(26usize, 13usize, 8u32, 2101088u32);
    emu.sri_no_count(27usize, 13usize, 16u32, 2101092u32);
    emu.sri_no_count(1usize, 12usize, 8u32, 2101096u32);
    emu.sb_no_count(31usize, 8usize, 0u32, 2101100u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2101104u32);
    emu.sb_no_count(19usize, 8usize, 1u32, 2101108u32);
    emu.sri_no_count(19usize, 11usize, 8u32, 2101112u32);
    emu.sb_no_count(18usize, 8usize, 2u32, 2101116u32);
    emu.sb_no_count(28usize, 8usize, 3u32, 2101120u32);
    emu.sri_no_count(28usize, 11usize, 16u32, 2101124u32);
    emu.sb_no_count(9usize, 8usize, 4u32, 2101128u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2101132u32);
    emu.sb_no_count(20usize, 8usize, 6u32, 2101136u32);
    emu.sb_no_count(6usize, 8usize, 7u32, 2101140u32);
    emu.sri_no_count(6usize, 10usize, 8u32, 2101144u32);
    emu.sb_no_count(30usize, 8usize, 8u32, 2101148u32);
    emu.sb_no_count(23usize, 8usize, 9u32, 2101152u32);
    emu.sb_no_count(22usize, 8usize, 10u32, 2101156u32);
    emu.sb_no_count(16usize, 8usize, 11u32, 2101160u32);
    emu.sri_no_count(16usize, 10usize, 16u32, 2101164u32);
    emu.sb_no_count(29usize, 8usize, 12u32, 2101168u32);
    emu.sb_no_count(25usize, 8usize, 13u32, 2101172u32);
    emu.sb_no_count(24usize, 8usize, 14u32, 2101176u32);
    emu.sb_no_count(14usize, 8usize, 15u32, 2101180u32);
    emu.sb_no_count(7usize, 8usize, 16u32, 2101184u32);
    emu.sb_no_count(27usize, 8usize, 17u32, 2101188u32);
    emu.sb_no_count(26usize, 8usize, 18u32, 2101192u32);
    emu.sb_no_count(13usize, 8usize, 19u32, 2101196u32);
    emu.sb_no_count(5usize, 8usize, 20u32, 2101200u32);
    emu.sb_no_count(31usize, 8usize, 21u32, 2101204u32);
    emu.sb_no_count(1usize, 8usize, 22u32, 2101208u32);
    emu.sb_no_count(12usize, 8usize, 23u32, 2101212u32);
    emu.sb_no_count(17usize, 8usize, 24u32, 2101216u32);
    emu.sb_no_count(28usize, 8usize, 25u32, 2101220u32);
    emu.sb_no_count(19usize, 8usize, 26u32, 2101224u32);
    emu.sb_no_count(11usize, 8usize, 27u32, 2101228u32);
    emu.sb_no_count(15usize, 8usize, 28u32, 2101232u32);
    emu.sb_no_count(16usize, 8usize, 29u32, 2101236u32);
    emu.sb_no_count(6usize, 8usize, 30u32, 2101240u32);
    emu.sb_no_count(10usize, 8usize, 31u32, 2101244u32);
    emu.lw_no_count(1usize, 2usize, 348u32, 2101248u32)?;
    emu.lw_no_count(8usize, 2usize, 344u32, 2101252u32)?;
    emu.lw_no_count(9usize, 2usize, 340u32, 2101256u32)?;
    emu.lw_no_count(18usize, 2usize, 336u32, 2101260u32)?;
    emu.lw_no_count(19usize, 2usize, 332u32, 2101264u32)?;
    emu.lw_no_count(20usize, 2usize, 328u32, 2101268u32)?;
    emu.lw_no_count(21usize, 2usize, 324u32, 2101272u32)?;
    emu.lw_no_count(22usize, 2usize, 320u32, 2101276u32)?;
    emu.lw_no_count(23usize, 2usize, 316u32, 2101280u32)?;
    emu.lw_no_count(24usize, 2usize, 312u32, 2101284u32)?;
    emu.lw_no_count(25usize, 2usize, 308u32, 2101288u32)?;
    emu.lw_no_count(26usize, 2usize, 304u32, 2101292u32)?;
    emu.lw_no_count(27usize, 2usize, 300u32, 2101296u32)?;
    emu.adi_no_count(2usize, 2usize, 352u32, 2101300u32);
    emu.add_memory_rw_events(79usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101304u32;
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
pub fn block_0x00201038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2101308u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2101312u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2101332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201054));
    } else {
        emu.pc = 2101316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201044));
    }
}
#[inline(always)]
pub fn block_0x00201044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2101320u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2101340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020105c));
    } else {
        emu.pc = 2101324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020104c));
    }
}
#[inline(always)]
pub fn block_0x0020104c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2101324u32, 45056u32, 2101328u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2101332u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2101332u32, 45056u32, 2101336u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2101340u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020105c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2101340u32, 45056u32, 2101344u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2101348u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2101352u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2101356u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2101360u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2101364u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2101368u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2101372u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2101376u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2101380u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2101384u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2101388u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2101392u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2101396u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2101400u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2101404u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2101408u32);
    emu.apc_no_count(1usize, 2101408u32, 4096u32, 2101412u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101416u32;
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
#[inline]
pub fn block_0x002010a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 64u32, 2101420u32)?;
    emu.lw_no_count(11usize, 2usize, 68u32, 2101424u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2101428u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2101432u32)?;
    emu.adi_no_count(10usize, 2usize, 28u32, 2101436u32);
    emu.adi_no_count(11usize, 2usize, 16u32, 2101440u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2101444u32);
    emu.apc_no_count(1usize, 2101444u32, 4096u32, 2101448u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002010cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2101456u32)?;
    emu.lw_no_count(8usize, 2usize, 32u32, 2101460u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2101464u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201480));
    } else {
        emu.pc = 2101468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010dc));
    }
}
#[inline(always)]
pub fn block_0x002010dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2101472u32);
    emu.lw_no_count(21usize, 2usize, 52u32, 2101476u32)?;
    emu.sri_no_count(10usize, 21usize, 27u32, 2101480u32);
    emu.sli_no_count(18usize, 21usize, 5u32, 2101484u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2101576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201148));
    } else {
        emu.pc = 2101488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f0));
    }
}
#[inline(always)]
pub fn block_0x002010f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2101576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201148));
    } else {
        emu.pc = 2101492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f4));
    }
}
#[inline(always)]
pub fn block_0x002010f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 36u32, 2101496u32)?;
    emu.lw_no_count(22usize, 2usize, 44u32, 2101500u32)?;
    emu.lw_no_count(20usize, 2usize, 48u32, 2101504u32)?;
    emu.sw_no_count(18usize, 2usize, 12u32, 2101508u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2101512u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2101600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201160));
    } else {
        emu.pc = 2101516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020110c));
    }
}
#[inline(always)]
pub fn block_0x0020110c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2101516u32, 4096u32, 2101520u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101528u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 560u32, 2101532u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2101536u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2101540u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2101544u32);
    emu.apc_no_count(1usize, 2101544u32, 8192u32, 2101548u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2101576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201148));
    } else {
        emu.pc = 2101556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201134));
    }
}
#[inline(always)]
pub fn block_0x00201134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 16u32, 2101560u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2101564u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2101568u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2101620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201174));
    } else {
        emu.pc = 2101572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201144));
    }
}
#[inline(always)]
pub fn block_0x00201144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201230));
}
#[inline(always)]
pub fn block_0x00201148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101580u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 464u32, 2101584u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2101588u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2101592u32);
    emu.apc_no_count(1usize, 2101592u32, 40960u32, 2101596u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(884u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2101604u32);
    emu.sw_no_count(0usize, 2usize, 16u32, 2101608u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2101612u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2101616u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2101808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201230));
    } else {
        emu.pc = 2101620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201174));
    }
}
#[inline]
pub fn block_0x00201174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 0u32, 2101624u32);
    emu.adi_no_count(19usize, 0usize, 0u32, 2101628u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2101632u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2101636u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 544u32, 2101640u32);
    let a = 0u32.wrapping_add(4294901760u32);
    emu.write_reg_no_count(26usize, a);
    emu.pc = 2101644u32;
    emu.update_insn_clock();
    emu.adi_no_count(27usize, 21usize, 0u32, 2101648u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(25usize, a);
    emu.pc = 2101652u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(9usize);
    let return_addr = 2101656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002011d0));
}
#[inline(always)]
pub fn block_0x00201198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2101660u32)?;
    emu.sli_no_count(11usize, 18usize, 5u32, 2101664u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2101668u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2101672u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2101676u32);
    emu.apc_no_count(1usize, 2101676u32, 8192u32, 2101680u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101684u32;
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
pub fn block_0x002011b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2101688u32);
    emu.adr_no_count(19usize, 19usize, 23usize, 2101692u32);
    emu.adr_no_count(25usize, 25usize, 23usize, 2101696u32);
    emu.adi_no_count(27usize, 27usize, 4294967295u32, 2101700u32);
    emu.sw_no_count(18usize, 2usize, 24u32, 2101704u32)?;
    emu.adr_no_count(24usize, 24usize, 26usize, 2101708u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2101808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201230));
    } else {
        emu.pc = 2101712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011d0));
    }
}
#[inline(always)]
pub fn block_0x002011d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 9usize, 0u32, 2101716u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2101736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011e8));
    } else {
        emu.pc = 2101720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011d8));
    }
}
#[inline(always)]
pub fn block_0x002011d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 19usize, 23usize, 2101724u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2101748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011f4));
    } else {
        emu.pc = 2101728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011e0));
    }
}
#[inline(always)]
pub fn block_0x002011e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2101756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011fc));
    } else {
        emu.pc = 2101732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011e4));
    }
}
#[inline(always)]
pub fn block_0x002011e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201458));
}
#[inline(always)]
pub fn block_0x002011e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 25usize, 0u32, 2101740u32);
    emu.adr_no_count(11usize, 19usize, 23usize, 2101744u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2101728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011e0));
    } else {
        emu.pc = 2101748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011f4));
    }
}
#[inline(always)]
pub fn block_0x002011f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2101752u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2102360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201458));
    } else {
        emu.pc = 2101756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011fc));
    }
}
#[inline(always)]
pub fn block_0x002011fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 12usize, 24usize, 2101760u32);
    emu.adr_no_count(11usize, 8usize, 19usize, 2101764u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2101768u32);
    emu.apc_no_count(1usize, 2101768u32, 0u32, 2101772u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101776u32;
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
#[inline(always)]
pub fn block_0x00201210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2101780u32)?;
    emu.lw_no_count(18usize, 2usize, 24u32, 2101784u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2101656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201198));
    } else {
        emu.pc = 2101788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020121c));
    }
}
#[inline(always)]
pub fn block_0x0020121c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 16u32, 2101792u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2101796u32);
    emu.apc_no_count(1usize, 2101796u32, 0u32, 2101800u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101804u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1608u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020122c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201198));
}
#[inline(always)]
pub fn block_0x00201230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2101812u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2101936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012b0));
    } else {
        emu.pc = 2101816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201238));
    }
}
#[inline(always)]
pub fn block_0x00201238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2101820u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2101824u32);
    emu.adi_no_count(20usize, 0usize, 3u32, 2101828u32);
    emu.mul_no_count(23usize, 11usize, 10usize, 2101832u32);
    emu.adr_no_count(23usize, 22usize, 23usize, 2101836u32);
    emu.adi_no_count(24usize, 0usize, 5u32, 2101840u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2101844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201278));
}
#[inline(always)]
pub fn block_0x00201254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2101848u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2101852u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2101856u32);
    emu.apc_no_count(1usize, 2101856u32, 28672u32, 2101860u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 32u32, 2101868u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2101872u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2101872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201270));
}
#[inline(always)]
pub fn block_0x00201270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 20u32, 2101876u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2101936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012b0));
    } else {
        emu.pc = 2101880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201278));
    }
}
#[inline(always)]
pub fn block_0x00201278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 22usize, 0u32, 2101884u32)?;
    emu.lhu_no_count(13usize, 22usize, 2u32, 2101888u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201290));
    } else {
        emu.pc = 2101892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201284));
    }
}
#[inline(always)]
pub fn block_0x00201284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201254));
    } else {
        emu.pc = 2101896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201288));
    }
}
#[inline(always)]
pub fn block_0x00201288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2101872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201270));
    } else {
        emu.pc = 2101900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020128c));
    }
}
#[inline(always)]
pub fn block_0x0020128c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101904u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101936u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002012b0));
}
#[inline(always)]
pub fn block_0x00201290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 16u32, 2101908u32);
    emu.sai_no_count(13usize, 13usize, 1040u32, 2101912u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2101916u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2101920u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2101924u32);
    emu.apc_no_count(1usize, 2101924u32, 28672u32, 2101928u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101932u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002012ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101864u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201268));
}
#[inline(always)]
pub fn block_0x002012b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 12u32, 2101940u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2102000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012f0));
    } else {
        emu.pc = 2101944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012b8));
    }
}
#[inline(always)]
pub fn block_0x002012b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2101944u32, 4096u32, 2101948u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002012c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101956u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 560u32, 2101960u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2101964u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2101968u32);
    emu.apc_no_count(1usize, 2101968u32, 8192u32, 2101972u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002012d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014d8));
    } else {
        emu.pc = 2101980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012dc));
    }
}
#[inline(always)]
pub fn block_0x002012dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 60u32, 2101984u32)?;
    emu.sw_no_count(10usize, 2usize, 64u32, 2101988u32)?;
    emu.sw_no_count(0usize, 2usize, 68u32, 2101992u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2102020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201304));
    } else {
        emu.pc = 2101996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012ec));
    }
}
#[inline(always)]
pub fn block_0x002012ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102204u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002013bc));
}
#[inline(always)]
pub fn block_0x002012f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2102004u32);
    emu.sw_no_count(0usize, 2usize, 60u32, 2102008u32)?;
    emu.sw_no_count(10usize, 2usize, 64u32, 2102012u32)?;
    emu.sw_no_count(0usize, 2usize, 68u32, 2102016u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2102204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013bc));
    } else {
        emu.pc = 2102020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201304));
    }
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
    emu.adi_no_count(20usize, 0usize, 0u32, 2102024u32);
    emu.adi_no_count(18usize, 0usize, 0u32, 2102028u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2102032u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2102036u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 512u32, 2102040u32);
    let a = 0u32.wrapping_add(4294901760u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2102044u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2102048u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(8usize);
    let return_addr = 2102052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020135c));
}
#[inline(always)]
pub fn block_0x00201324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 64u32, 2102056u32)?;
    emu.sli_no_count(11usize, 25usize, 5u32, 2102060u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2102064u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2102068u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2102072u32);
    emu.apc_no_count(1usize, 2102072u32, 8192u32, 2102076u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966608u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2102084u32);
    emu.adr_no_count(18usize, 18usize, 22usize, 2102088u32);
    emu.adr_no_count(24usize, 24usize, 22usize, 2102092u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2102096u32);
    emu.sw_no_count(25usize, 2usize, 68u32, 2102100u32)?;
    emu.adr_no_count(20usize, 20usize, 23usize, 2102104u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2102204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013bc));
    } else {
        emu.pc = 2102108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020135c));
    }
}
#[inline(always)]
pub fn block_0x0020135c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 9usize, 0u32, 2102112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2102132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201374));
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
#[inline(always)]
pub fn block_0x00201364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 18usize, 22usize, 2102120u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2102144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201380));
    } else {
        emu.pc = 2102124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020136c));
    }
}
#[inline(always)]
pub fn block_0x0020136c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2102152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201388));
    } else {
        emu.pc = 2102128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201370));
    }
}
#[inline(always)]
pub fn block_0x00201370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102380u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020146c));
}
#[inline(always)]
pub fn block_0x00201374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 24usize, 0u32, 2102136u32);
    emu.adr_no_count(11usize, 18usize, 22usize, 2102140u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2102124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020136c));
    } else {
        emu.pc = 2102144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201380));
    }
}
#[inline(always)]
pub fn block_0x00201380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2102148u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2102380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020146c));
    } else {
        emu.pc = 2102152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201388));
    }
}
#[inline(always)]
pub fn block_0x00201388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 12usize, 20usize, 2102156u32);
    emu.adr_no_count(11usize, 8usize, 18usize, 2102160u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2102164u32);
    emu.apc_no_count(1usize, 2102164u32, 0u32, 2102168u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020139c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 60u32, 2102176u32)?;
    emu.lw_no_count(25usize, 2usize, 68u32, 2102180u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2102052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201324));
    } else {
        emu.pc = 2102184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013a8));
    }
}
#[inline(always)]
pub fn block_0x002013a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 60u32, 2102188u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2102192u32);
    emu.apc_no_count(1usize, 2102192u32, 0u32, 2102196u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102052u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201324));
}
#[inline]
pub fn block_0x002013bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2102208u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2102212u32)?;
    emu.lw_no_count(12usize, 2usize, 24u32, 2102216u32)?;
    emu.lw_no_count(13usize, 2usize, 60u32, 2102220u32)?;
    emu.lw_no_count(14usize, 2usize, 64u32, 2102224u32)?;
    emu.lw_no_count(15usize, 2usize, 68u32, 2102228u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2102232u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2102236u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2102240u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2102244u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2102248u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2102252u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2102256u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2102260u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2102264u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2102268u32);
    emu.apc_no_count(1usize, 2102268u32, 0u32, 2102272u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014ac));
    } else {
        emu.pc = 2102280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201408));
    }
}
#[inline(always)]
pub fn block_0x00201408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 40u32, 2102284u32);
    emu.adi_no_count(10usize, 2usize, 72u32, 2102288u32);
    emu.apc_no_count(1usize, 2102288u32, 0u32, 2102292u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014ac));
    } else {
        emu.pc = 2102300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020141c));
    }
}
#[inline]
pub fn block_0x0020141c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 124u32, 2102304u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2102308u32)?;
    emu.lw_no_count(9usize, 2usize, 116u32, 2102312u32)?;
    emu.lw_no_count(18usize, 2usize, 112u32, 2102316u32)?;
    emu.lw_no_count(19usize, 2usize, 108u32, 2102320u32)?;
    emu.lw_no_count(20usize, 2usize, 104u32, 2102324u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2102328u32)?;
    emu.lw_no_count(22usize, 2usize, 96u32, 2102332u32)?;
    emu.lw_no_count(23usize, 2usize, 92u32, 2102336u32)?;
    emu.lw_no_count(24usize, 2usize, 88u32, 2102340u32)?;
    emu.lw_no_count(25usize, 2usize, 84u32, 2102344u32)?;
    emu.lw_no_count(26usize, 2usize, 80u32, 2102348u32)?;
    emu.lw_no_count(27usize, 2usize, 76u32, 2102352u32)?;
    emu.adi_no_count(2usize, 2usize, 128u32, 2102356u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102360u32;
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
pub fn block_0x00201458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102364u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 528u32, 2102368u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2102372u32);
    emu.apc_no_count(1usize, 2102372u32, 77824u32, 2102376u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102380u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020146c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102384u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 496u32, 2102388u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2102392u32);
    emu.apc_no_count(1usize, 2102392u32, 77824u32, 2102396u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 28u32, 2102404u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102408u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1128u32, 2102412u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2102416u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1076u32, 2102420u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2102424u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1152u32, 2102428u32);
    emu.adi_no_count(11usize, 0usize, 22u32, 2102432u32);
    emu.adi_no_count(12usize, 2usize, 28u32, 2102436u32);
    emu.apc_no_count(1usize, 2102436u32, 57344u32, 2102440u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002014ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 72u32, 2102448u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102452u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1092u32, 2102456u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2102460u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1076u32, 2102464u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2102468u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1112u32, 2102472u32);
    emu.adi_no_count(11usize, 0usize, 20u32, 2102476u32);
    emu.adi_no_count(12usize, 2usize, 72u32, 2102480u32);
    emu.apc_no_count(1usize, 2102480u32, 57344u32, 2102484u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102488u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102492u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 480u32, 2102496u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2102500u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2102504u32);
    emu.apc_no_count(1usize, 2102504u32, 40960u32, 2102508u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
